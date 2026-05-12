use crate::{internal_error, PreparedDigest, PreparedTransaction, Signer, SigningError};
use core::convert::TryFrom;
use core::str::FromStr;
use m10_protos::sdk::signature::Algorithm;
use p256::ecdsa::signature::hazmat::PrehashSigner;
use p256::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use std::fs::File;
use std::io::{Read, Write};

/// A P256 key-pair
#[derive(serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct P256 {
    #[serde(skip)]
    key: p256::ecdsa::SigningKey,
    public_key: [u8; 65],
}

fn make_p256(key: p256::ecdsa::SigningKey) -> P256 {
    let ep = key.verifying_key().to_encoded_point(false);
    let public_key: [u8; 65] = ep
        .as_bytes()
        .try_into()
        .expect("P-256 uncompressed point is always 65 bytes");
    P256 { key, public_key }
}

impl P256 {
    /// Generates a P256 key-pair, and if the path is passed writes it to disk as a PKCS8 document
    pub fn new_key_pair(path: Option<&str>) -> Result<Self, SigningError> {
        let key = p256::ecdsa::SigningKey::random(&mut rand_core::OsRng);
        if let Some(p) = path {
            let doc = key
                .to_pkcs8_der()
                .map_err(|e| internal_error(e, "P256::new_key_pair: to_pkcs8_der"))?;
            let mut key_file = File::create(p)?;
            key_file.write_all(doc.as_bytes())?;
        }
        Ok(make_p256(key))
    }

    /// Loads a P256 key-pair from a PKCS8 formatted file
    pub fn load_key_pair(path: &str) -> Result<Self, SigningError> {
        let mut key_file = File::open(path)?;
        let mut pkcs8_bytes: Vec<u8> = Vec::new();
        key_file.read_to_end(&mut pkcs8_bytes)?;
        P256::from_pkcs8(&pkcs8_bytes)
    }

    /// Generates a new key-pair, and returns both the key-pair and a PKCS8 document containing the key-pair
    pub fn new_key_pair_exportable() -> Result<(Vec<u8>, Self), SigningError> {
        let key = p256::ecdsa::SigningKey::random(&mut rand_core::OsRng);
        let doc = key
            .to_pkcs8_der()
            .map_err(|e| internal_error(e, "P256::new_key_pair_exportable: to_pkcs8_der"))?;
        let pkcs8_bytes = doc.as_bytes().to_vec();
        Ok((pkcs8_bytes, make_p256(key)))
    }

    /// Returns a new [`P256`] key-pair from a PKCS8 document
    pub fn from_pkcs8(bytes: &[u8]) -> Result<Self, SigningError> {
        let key = p256::ecdsa::SigningKey::from_pkcs8_der(bytes)
            .map_err(|e| internal_error(e, "P256::from_pkcs8"))?;
        Ok(make_p256(key))
    }
}

#[async_trait::async_trait]
impl Signer for P256 {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, super::SigningError> {
        use p256::ecdsa::signature::Signer as _;
        let sig: p256::ecdsa::Signature = self.key.sign(msg);
        Ok(sig.to_der().to_bytes().to_vec())
    }

    fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    fn algorithm(&self) -> Algorithm {
        Algorithm::P256Sha256Asn1
    }

    async fn sign_prepared_transaction(
        &self,
        prepared: &PreparedTransaction,
    ) -> Result<Vec<u8>, super::SigningError> {
        let hash = match (prepared.algorithm, &prepared.digest) {
            (Algorithm::P256Sha256Asn1, PreparedDigest::P256Sha256(h)) => h,
            _ => {
                return Err(internal_error(
                    "algorithm and digest type mismatch or unsupported",
                    "P256::sign_prepared_transaction",
                ))
            }
        };

        let sig: p256::ecdsa::Signature = self
            .key
            .sign_prehash(hash)
            .map_err(|e| internal_error(e, "P256::sign_prepared_transaction: sign_prehash"))?;

        Ok(sig.to_der().to_bytes().to_vec())
    }
}

impl FromStr for P256 {
    type Err = SigningError;
    fn from_str(key_pair_enc: &str) -> Result<Self, Self::Err> {
        let pkcs8_bytes = base64::decode(key_pair_enc).unwrap_or_default();
        P256::from_pkcs8(&pkcs8_bytes)
    }
}

impl TryFrom<String> for P256 {
    type Error = SigningError;
    fn try_from(key_pair: String) -> Result<Self, Self::Error> {
        key_pair.parse()
    }
}
