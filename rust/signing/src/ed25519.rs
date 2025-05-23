use crate::{internal_error, Signer, SigningError};
use core::convert::TryFrom;
use core::str::FromStr;
use m10_protos::sdk::signature::Algorithm;
use ring::{
    rand,
    signature::{Ed25519KeyPair, KeyPair},
};
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};

/// An Ed25519 key-pair
#[derive(serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Ed25519 {
    #[serde(flatten)]
    key_pair: Ed25519KeyPair,
}

impl Ed25519 {
    /// Generates an ED25519 key-pair, and if the path is passed writes it to disk as a PKCS8 document
    pub fn new_key_pair(path: Option<&str>) -> Result<Self, SigningError> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|e| internal_error(e, "Ed25519::new_key_pair: generate_pkcs8"))?;
        if let Some(p) = path {
            let mut key_file = File::create(p)?;
            key_file.write_all(pkcs8_bytes.as_ref())?;
        }
        Ok(Self {
            key_pair: Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
                .map_err(|e| internal_error(e, "Ed25519::new_key_pair: from_pkcs8"))?,
        })
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
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|e| internal_error(e, "Ed25519::new_key_pair_exportable: generate_pkcs8"))?;
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|e| internal_error(e, "Ed25519::new_key_pair_exportable: from_pkcs8"))?;
        Ok((pkcs8_bytes.as_ref().to_vec(), Self { key_pair }))
    }

    /// Returns a new [`Ed25519`] key-pair from a PKCS8 document
    pub fn from_pkcs8(bytes: &[u8]) -> Result<Self, SigningError> {
        let key_pair = Ed25519KeyPair::from_pkcs8(bytes)
            .map_err(|e| internal_error(e, "Ed25519::from_pkcs8"))?;
        Ok(Self { key_pair })
    }
}

#[async_trait::async_trait]
impl Signer for Ed25519 {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, super::SigningError> {
        Ok(self.key_pair.sign(msg).as_ref().to_vec())
    }

    fn public_key(&self) -> &[u8] {
        self.key_pair.public_key().as_ref()
    }

    fn algorithm(&self) -> Algorithm {
        Algorithm::Ed25519
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

impl Ed25519 {
    /// Parses Ed25519 key-pair from base64-encoded 64-byte raw key (32-byte private + 32-byte public)
    pub fn from_keypair(b64: &str) -> Result<Self, SigningError> {
        let bytes = base64::decode(b64).map_err(|_| {
            SigningError::KeyInvalid("Failed to decode base64-encoded key".to_string())
        })?;
        if bytes.len() != 64 {
            return Err(SigningError::KeyInvalid(
                "Invalid key length, expected 64 bytes".to_string(),
            ));
        }
        let secret = &bytes[..32];
        let public = &bytes[32..];

        let key_pair = Ed25519KeyPair::from_seed_and_public_key(secret, public)
            .map_err(|_| SigningError::KeyInvalid("Invalid key".to_string()))?;
        Ok(Self { key_pair })
    }
}
