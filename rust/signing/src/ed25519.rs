use crate::{internal_error, PreparedDigest, PreparedTransaction, Signer, SigningError};
use core::convert::TryFrom;
use core::str::FromStr;
use ed25519_dalek::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use ed25519_dalek::{KEYPAIR_LENGTH, SECRET_KEY_LENGTH};
use m10_protos::sdk::signature::Algorithm;
use sha2::Digest as _;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};

/// An Ed25519 key-pair
#[derive(serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Ed25519 {
    #[serde(skip)]
    key: ed25519_dalek::SigningKey,
    public_key: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH],
    #[serde(skip)]
    algorithm: Algorithm,
}

fn make_ed25519(key: ed25519_dalek::SigningKey) -> Ed25519 {
    let public_key = *key.verifying_key().as_bytes();
    Ed25519 {
        key,
        public_key,
        algorithm: Algorithm::Ed25519,
    }
}

impl Ed25519 {
    /// Generates an ED25519 key-pair, and if the path is passed writes it to disk as a PKCS8 document
    pub fn new_key_pair(path: Option<&str>) -> Result<Self, SigningError> {
        let key = ed25519_dalek::SigningKey::generate(&mut rand_core::OsRng);
        if let Some(p) = path {
            let doc = key
                .to_pkcs8_der()
                .map_err(|e| internal_error(e, "Ed25519::new_key_pair: to_pkcs8_der"))?;
            let mut key_file = File::create(p)?;
            key_file.write_all(doc.as_bytes())?;
        }
        Ok(make_ed25519(key))
    }

    /// Loads an Ed25519 key-pair from a PKCS8 formatted file
    pub fn load_key_pair(path: &str) -> Result<Self, SigningError> {
        let mut key_file = File::open(path)?;
        let mut pkcs8_bytes: Vec<u8> = Vec::new();
        key_file.read_to_end(&mut pkcs8_bytes)?;
        Ed25519::from_pkcs8(&pkcs8_bytes)
    }

    /// Generates a new key-pair, and returns both the key-pair and a PKCS8 document containing the key-pair
    pub fn new_key_pair_exportable() -> Result<(Vec<u8>, Self), SigningError> {
        let key = ed25519_dalek::SigningKey::generate(&mut rand_core::OsRng);
        let doc = key
            .to_pkcs8_der()
            .map_err(|e| internal_error(e, "Ed25519::new_key_pair_exportable: to_pkcs8_der"))?;
        let pkcs8_bytes = doc.as_bytes().to_vec();
        Ok((pkcs8_bytes, make_ed25519(key)))
    }

    /// Returns a new [`Ed25519`] key-pair from a PKCS8 document
    pub fn from_pkcs8(bytes: &[u8]) -> Result<Self, SigningError> {
        let key = ed25519_dalek::SigningKey::from_pkcs8_der(bytes)
            .map_err(|e| internal_error(e, "Ed25519::from_pkcs8"))?;
        Ok(make_ed25519(key))
    }

    /// Parses Ed25519 key-pair from base64-encoded 64-byte raw key (32-byte private + 32-byte public)
    pub fn from_keypair(b64: &str) -> Result<Self, SigningError> {
        let bytes = base64::decode(b64).map_err(|_| {
            SigningError::KeyInvalid("Failed to decode base64-encoded key".to_string())
        })?;
        if bytes.len() != KEYPAIR_LENGTH {
            return Err(SigningError::KeyInvalid(format!(
                "Invalid key length: expected {KEYPAIR_LENGTH} bytes, got {}",
                bytes.len()
            )));
        }
        let seed: &[u8; SECRET_KEY_LENGTH] = bytes[..SECRET_KEY_LENGTH]
            .try_into()
            .map_err(|_| SigningError::KeyInvalid("Invalid seed slice".to_string()))?;
        let key = ed25519_dalek::SigningKey::from_bytes(seed);
        Ok(make_ed25519(key))
    }

    pub fn new_key_pair_ph(path: Option<&str>) -> Result<Self, SigningError> {
        Self::new_key_pair(path).map(|k| Self {
            algorithm: Algorithm::Ed25519PhSha512,
            ..k
        })
    }

    pub fn load_key_pair_ph(path: &str) -> Result<Self, SigningError> {
        Self::load_key_pair(path).map(|k| Self {
            algorithm: Algorithm::Ed25519PhSha512,
            ..k
        })
    }

    pub fn from_pkcs8_ph(bytes: &[u8]) -> Result<Self, SigningError> {
        Self::from_pkcs8(bytes).map(|k| Self {
            algorithm: Algorithm::Ed25519PhSha512,
            ..k
        })
    }
}

#[async_trait::async_trait]
impl Signer for Ed25519 {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, super::SigningError> {
        if self.algorithm == Algorithm::Ed25519PhSha512 {
            return Err(SigningError::KeyInvalid(
                "Ed25519Ph keys must use sign_prepared_transaction".into(),
            ));
        }
        use ed25519_dalek::Signer as _;
        Ok(self.key.sign(msg).to_bytes().to_vec())
    }

    fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    async fn sign_prepared_transaction(
        &self,
        prepared: &PreparedTransaction,
    ) -> Result<Vec<u8>, super::SigningError> {
        let hash = match (prepared.algorithm, &prepared.digest) {
            (Algorithm::Ed25519PhSha512, PreparedDigest::Ed25519PhSha512(h)) => h,
            _ => {
                return Err(internal_error(
                    "algorithm and digest type mismatch or unsupported",
                    "Ed25519::sign_prepared_transaction",
                ))
            }
        };

        let sig = self
            .key
            .sign_prehashed(sha2::Sha512::new_with_prefix(hash), None)
            .map_err(|e| internal_error(e, "Ed25519::sign_prepared_transaction: sign_prehashed"))?;

        Ok(sig.to_bytes().to_vec())
    }
}

impl fmt::Debug for Ed25519 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ed25519({:?})", self.public_key())
    }
}

impl FromStr for Ed25519 {
    type Err = SigningError;
    fn from_str(key_pair_enc: &str) -> Result<Self, Self::Err> {
        let pkcs8_bytes = base64::decode(key_pair_enc).unwrap_or_default();
        Ed25519::from_pkcs8(&pkcs8_bytes)
    }
}

impl TryFrom<String> for Ed25519 {
    type Error = SigningError;
    fn try_from(key_pair: String) -> Result<Self, Self::Error> {
        key_pair.parse()
    }
}
