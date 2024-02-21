//! M10's helper library for signing messages with ED25519 or P256 elliptic-curve signatures
//!
//! This library contains a set of wrappers and traits that allow users to easily sign and verify
//! signatures
use core::str::FromStr;
use std::fmt;

use m10_protos::{prost::Message, sdk};

pub use ed25519::Ed25519;
pub use p256::P256;

mod ed25519;
mod p256;

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

#[derive(thiserror::Error, Debug)]
pub enum SigningError {
    #[error("internal")]
    Internal,
    #[error("malformed signature")]
    MalFormedSignature,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    KeyRejected(#[from] ring::error::KeyRejected),
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
        let signature = self.sign_payload(&payload).await?;
        let request_envelope = sdk::RequestEnvelope {
            payload,
            signature: Some(signature),
        };
        Ok(SignedRequest {
            request_envelope,
            data,
        })
    }

    /// Adds an endorsement signatured to a [`sdk::Contract`]
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
            KeyPair::P256(key_pair) => key_pair.sign(msg).await,
            KeyPair::Ed25519(key_pair) => key_pair.sign(msg).await,
        }
    }

    fn public_key(&self) -> &[u8] {
        match self {
            KeyPair::P256(key_pair) => key_pair.public_key(),
            KeyPair::Ed25519(key_pair) => key_pair.public_key(),
        }
    }

    fn algorithm(&self) -> sdk::signature::Algorithm {
        match self {
            KeyPair::P256(key_pair) => key_pair.algorithm(),
            KeyPair::Ed25519(key_pair) => key_pair.algorithm(),
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
            KeyPair::P256(key_pair) => write!(f, "P256({:?})", key_pair.public_key()),
            KeyPair::Ed25519(key_pair) => write!(f, "Ed25519({:?})", key_pair.public_key()),
        }
    }
}
