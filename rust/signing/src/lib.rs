//! M10's helper library for signing messages with ED25519 or P256 elliptic-curve signatures
//!
//! This library contains a set of wrappers and traits that allow users to easily sign and verify
//! signatures
use async_trait::async_trait;
use core::str::FromStr;
use m10_protos::{prost::Message, sdk};
use std::fmt;
use std::sync::Arc;

pub use ed25519::Ed25519;
pub use p256::P256;
pub use vault::VaultTransit;

mod ed25519;
mod p256;
mod vault;
pub use crate::vault::extract_public_key;

pub fn verify_ed25519ph(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<(), SigningError> {
    use sha2::Digest as _;
    let pk_bytes: &[u8; 32] = public_key
        .try_into()
        .map_err(|e| internal_error(e, "verify_ed25519ph: invalid public key length"))?;
    let vk = ed25519_dalek::VerifyingKey::from_bytes(pk_bytes)
        .map_err(|e| internal_error(e, "verify_ed25519ph: invalid public key"))?;
    let sig = ed25519_dalek::Signature::from_slice(signature)
        .map_err(|e| internal_error(e, "verify_ed25519ph: invalid signature bytes"))?;
    vk.verify_prehashed_strict(
        sha2::Sha512::new_with_prefix(sha2::Sha512::digest(message)),
        None,
        &sig,
    )
    .map_err(|e| internal_error(e, "verify_ed25519ph: signature verification failed"))
}

/// A signed request containing both the serialized and signed payload; and the original message
#[derive(Default, Clone)]
pub struct SignedRequest<P: Message> {
    pub request_envelope: sdk::RequestEnvelope,
    pub data: P,
}

impl<P: Message> From<SignedRequest<P>> for sdk::RequestEnvelope {
    fn from(signed_request: SignedRequest<P>) -> Self {
        signed_request.request_envelope
    }
}

impl<P: Message> AsRef<P> for SignedRequest<P> {
    fn as_ref(&self) -> &P {
        &self.data
    }
}

/// Algorithm-bound pre-computed digest of a transaction payload
#[derive(Debug, PartialEq)]
pub enum PreparedDigest {
    P256Sha256([u8; 32]),
    Ed25519PhSha512([u8; 64]),
}

impl PreparedDigest {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            PreparedDigest::P256Sha256(d) => d,
            PreparedDigest::Ed25519PhSha512(d) => d,
        }
    }
}

/// A transaction payload with its pre-computed algorithm-bound digest
#[derive(Debug)]
pub struct PreparedTransaction {
    pub payload: Vec<u8>,
    pub algorithm: sdk::signature::Algorithm,
    pub digest: PreparedDigest,
}

impl PreparedTransaction {
    /// Creates a new prepared transaction, computing the digest for the given algorithm
    pub fn new(
        payload: Vec<u8>,
        algorithm: sdk::signature::Algorithm,
    ) -> Result<Self, SigningError> {
        use sha2::Digest as _;
        let digest = match algorithm {
            sdk::signature::Algorithm::P256Sha256Asn1 => {
                PreparedDigest::P256Sha256(sha2::Sha256::digest(&payload).into())
            }
            sdk::signature::Algorithm::Ed25519PhSha512 => {
                PreparedDigest::Ed25519PhSha512(sha2::Sha512::digest(&payload).into())
            }
            sdk::signature::Algorithm::Ed25519 => {
                return Err(SigningError::KeyInvalid(
                    "ED25519 does not support digest signing".to_string(),
                ))
            }
        };
        Ok(Self {
            payload,
            algorithm,
            digest,
        })
    }

    /// Verifies that the stored digest matches a fresh recomputation from the payload
    pub fn verify_integrity(&self) -> Result<(), SigningError> {
        use sha2::Digest as _;
        let valid = match &self.digest {
            PreparedDigest::P256Sha256(stored) => {
                &sha2::Sha256::digest(&self.payload)[..] == stored
            }
            PreparedDigest::Ed25519PhSha512(stored) => {
                &sha2::Sha512::digest(&self.payload)[..] == stored
            }
        };
        if !valid {
            return Err(internal_error(
                "transaction digest mismatch",
                "PreparedTransaction::verify_integrity",
            ));
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SigningError {
    #[error("internal")]
    Internal,
    #[error("malformed signature")]
    MalFormedSignature,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("key invalid: {0}")]
    KeyInvalid(String),
}

/// Internal helper for mapping lower-level errors to SigningError::Internal.
/// When the "verbose_errors" feature flag is enabled, the helper logs the
/// underlying error details with the provided context.
#[inline]
#[allow(unused_variables)]
fn internal_error<E: std::fmt::Debug>(err: E, context: &'static str) -> SigningError {
    #[cfg(feature = "verbose_errors")]
    {
        log::error!("{}: {:?}", context, err);
    }
    SigningError::Internal
}

#[macro_export]
macro_rules! debug_verbose {
    ($($arg:tt)*) => {
        #[cfg(feature = "verbose_errors")]
        {
            log::debug!($($arg)*);
        }
    };
}

/// A trait repersenting a service or key that can sign a message
///
/// Typically this trait would be implemented by a key pair (like in [`KeyPair`]), an HSM, or some secure service like Vault
#[async_trait::async_trait]
pub trait Signer: Send + Sync {
    /// Signs the passed message, and returns the signature.
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError>;

    /// Returns the public key associated with the signer
    fn public_key(&self) -> &[u8];

    /// Returns the signing algorithm used by the signer
    fn algorithm(&self) -> sdk::signature::Algorithm;

    /// Signs the payload, and returns a signature structure containing the algorithm, public-key, and signature
    async fn sign_payload(&self, payload: &[u8]) -> Result<sdk::Signature, SigningError> {
        Ok(sdk::Signature {
            algorithm: self.algorithm().into(),
            public_key: self.public_key().into(),
            signature: self.sign(payload).await?,
        })
    }

    /// Signs a [`Message`] and returns a [`SignedRequest`]
    async fn sign_request<P: Message>(&self, data: P) -> Result<SignedRequest<P>, SigningError> {
        let payload = data.encode_to_vec();
        match self.algorithm() {
            sdk::signature::Algorithm::Ed25519 => {
                let signature = self.sign_payload(&payload).await?;
                Ok(SignedRequest {
                    request_envelope: sdk::RequestEnvelope {
                        payload,
                        signature: Some(signature),
                    },
                    data,
                })
            }
            algorithm => {
                let prepared = PreparedTransaction::new(payload, algorithm)?;
                prepared.verify_integrity()?;
                let sig_bytes = self.sign_prepared_transaction(&prepared).await?;
                let signature = sdk::Signature {
                    algorithm: algorithm.into(),
                    public_key: self.public_key().to_vec(),
                    signature: sig_bytes,
                };
                Ok(SignedRequest {
                    request_envelope: sdk::RequestEnvelope {
                        payload: prepared.payload,
                        signature: Some(signature),
                    },
                    data,
                })
            }
        }
    }

    /// Signs a pre-computed transaction digest
    async fn sign_prepared_transaction(
        &self,
        _prepared: &PreparedTransaction,
    ) -> Result<Vec<u8>, SigningError> {
        Err(SigningError::Internal)
    }

    /// Adds an endorsement signatured to a [`sdk::Contract`]
    /// TODO: endorse with the digest-signing path is not supported
    /// Ed25519PhSha512-mode keys cannot use this method
    async fn endorse(
        &self,
        contract: &mut sdk::Contract,
        ledger_id: String,
    ) -> Result<(), SigningError> {
        let public_key = self.public_key();
        let already_signed = contract
            .endorsements
            .iter()
            .filter_map(|endorsement| endorsement.signature.as_ref())
            .any(|signature| signature.public_key == public_key);
        if !already_signed {
            contract.endorsements.push(sdk::Endorsement {
                ledger_id,
                signature: Some(self.sign_payload(&contract.transactions).await?),
            })
        }
        Ok(())
    }
}

/// A P256 or ED25519 key pair
#[derive(serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum KeyPair {
    P256(P256),
    Ed25519(Ed25519),
}

#[async_trait::async_trait]
impl Signer for KeyPair {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError> {
        match self {
            KeyPair::P256(key_pair) => Signer::sign(key_pair, msg).await,
            KeyPair::Ed25519(key_pair) => Signer::sign(key_pair, msg).await,
        }
    }

    fn public_key(&self) -> &[u8] {
        match self {
            KeyPair::P256(key_pair) => Signer::public_key(key_pair),
            KeyPair::Ed25519(key_pair) => Signer::public_key(key_pair),
        }
    }

    fn algorithm(&self) -> sdk::signature::Algorithm {
        match self {
            KeyPair::P256(key_pair) => Signer::algorithm(key_pair),
            KeyPair::Ed25519(key_pair) => Signer::algorithm(key_pair),
        }
    }

    async fn sign_prepared_transaction(
        &self,
        prepared: &PreparedTransaction,
    ) -> Result<Vec<u8>, SigningError> {
        match self {
            KeyPair::P256(key_pair) => Signer::sign_prepared_transaction(key_pair, prepared).await,
            KeyPair::Ed25519(key_pair) => {
                Signer::sign_prepared_transaction(key_pair, prepared).await
            }
        }
    }
}

impl FromStr for KeyPair {
    type Err = SigningError;
    fn from_str(key_pair_enc: &str) -> Result<Self, Self::Err> {
        Ed25519::from_str(key_pair_enc)
            .map(KeyPair::Ed25519)
            .or_else(|_| P256::from_str(key_pair_enc).map(KeyPair::P256))
    }
}

impl TryFrom<String> for KeyPair {
    type Error = SigningError;
    fn try_from(key_pair: String) -> Result<Self, Self::Error> {
        key_pair.parse()
    }
}

impl fmt::Debug for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyPair::P256(key_pair) => write!(f, "P256({:?})", Signer::public_key(key_pair)),
            KeyPair::Ed25519(key_pair) => {
                write!(f, "Ed25519({:?})", Signer::public_key(key_pair))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ArcKeyPair(pub Arc<KeyPair>);

impl<'de> serde::Deserialize<'de> for ArcKeyPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        let keypair = Ed25519::from_keypair(&encoded)
            .map(KeyPair::Ed25519)
            .map_err(serde::de::Error::custom)?;
        Ok(Self(Arc::new(keypair)))
    }
}

impl std::ops::Deref for ArcKeyPair {
    type Target = Arc<KeyPair>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
pub trait DynSigner: Send + Sync {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError>;

    fn public_key(&self) -> &[u8];

    fn algorithm(&self) -> sdk::signature::Algorithm;

    async fn sign_prepared_transaction(
        &self,
        prepared: &PreparedTransaction,
    ) -> Result<Vec<u8>, SigningError>;
}

#[async_trait]
impl<T: Signer> DynSigner for T {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError> {
        Signer::sign(self, msg).await
    }

    fn public_key(&self) -> &[u8] {
        Signer::public_key(self)
    }

    fn algorithm(&self) -> sdk::signature::Algorithm {
        Signer::algorithm(self)
    }

    async fn sign_prepared_transaction(
        &self,
        prepared: &PreparedTransaction,
    ) -> Result<Vec<u8>, SigningError> {
        Signer::sign_prepared_transaction(self, prepared).await
    }
}

#[derive(Clone)]
pub struct DynSignerWrapper(pub Arc<dyn DynSigner>);

impl DynSignerWrapper {
    pub fn new(signer: Arc<dyn DynSigner>) -> Self {
        Self(signer)
    }
}

#[async_trait]
impl Signer for DynSignerWrapper {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError> {
        self.0.sign(msg).await
    }

    fn public_key(&self) -> &[u8] {
        self.0.public_key()
    }

    fn algorithm(&self) -> sdk::signature::Algorithm {
        self.0.algorithm()
    }

    async fn sign_payload(&self, payload: &[u8]) -> Result<sdk::Signature, SigningError> {
        let sig = self.0.sign(payload).await?;
        Ok(sdk::Signature {
            algorithm: self.0.algorithm().into(),
            public_key: self.0.public_key().into(),
            signature: sig,
        })
    }

    async fn sign_prepared_transaction(
        &self,
        prepared: &PreparedTransaction,
    ) -> Result<Vec<u8>, SigningError> {
        self.0.sign_prepared_transaction(prepared).await
    }

    async fn endorse(
        &self,
        contract: &mut sdk::Contract,
        ledger_id: String,
    ) -> Result<(), SigningError> {
        let public_key = self.0.public_key();
        let already_signed = contract
            .endorsements
            .iter()
            .filter_map(|e| e.signature.as_ref())
            .any(|s| s.public_key == public_key);
        if !already_signed {
            let sig = self.sign_payload(&contract.transactions).await?;
            contract.endorsements.push(sdk::Endorsement {
                ledger_id,
                signature: Some(sig),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
