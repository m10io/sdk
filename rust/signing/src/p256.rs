use crate::Signer;
use core::convert::TryFrom;
use core::str::FromStr;
use m10_sdk_protos::sdk::signature::Algorithm;
use ring::{
    rand,
    signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_ASN1_SIGNING},
};
use std::fs::File;
use std::io::{Read, Write};

#[derive(serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct P256 {
    #[serde(flatten)]
    key_pair: EcdsaKeyPair,
    #[serde(skip)]
    rng: rand::SystemRandom,
}

impl P256 {
    pub fn new_key_pair(path: Option<&str>) -> Result<Self, anyhow::Error> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &rng)?;
        if let Some(p) = path {
            let mut key_file = File::create(p)?;
            key_file.write_all(pkcs8_bytes.as_ref())?;
        }
        Ok(Self {
            key_pair: EcdsaKeyPair::from_pkcs8(
                &ECDSA_P256_SHA256_ASN1_SIGNING,
                pkcs8_bytes.as_ref(),
            )?,
            rng,
        })
    }

    pub fn load_key_pair(path: &str) -> Result<Self, anyhow::Error> {
        let mut key_file = File::open(path)?;
        let mut pkcs8_bytes: Vec<u8> = Vec::new();
        key_file.read_to_end(&mut pkcs8_bytes)?;
        let rng = rand::SystemRandom::new();
        Ok(Self {
            key_pair: EcdsaKeyPair::from_pkcs8(
                &ECDSA_P256_SHA256_ASN1_SIGNING,
                pkcs8_bytes.as_ref(),
            )?,
            rng,
        })
    }

    pub fn new_key_pair_exportable() -> Result<(Vec<u8>, Self), anyhow::Error> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &rng)?;
        let key_pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, pkcs8_bytes.as_ref())?;
        Ok((pkcs8_bytes.as_ref().to_vec(), Self { key_pair, rng }))
    }

    pub fn from_pkcs8(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        let rng = rand::SystemRandom::new();
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, bytes)?;
        Ok(Self { key_pair, rng })
    }
}

#[async_trait::async_trait]
impl Signer for P256 {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, super::Error> {
        self.key_pair
            .sign(&self.rng, msg)
            .map(|x| x.as_ref().to_vec())
            .map_err(|_| super::Error::Internal)
    }

    fn public_key(&self) -> &[u8] {
        self.key_pair.public_key().as_ref()
    }

    fn algorithm(&self) -> Algorithm {
        Algorithm::P256Sha256Asn1
    }
}

impl FromStr for P256 {
    type Err = anyhow::Error;
    fn from_str(key_pair_enc: &str) -> Result<Self, Self::Err> {
        let pkcs8_bytes = base64::decode(key_pair_enc).unwrap_or_default();
        P256::from_pkcs8(&pkcs8_bytes)
    }
}

impl TryFrom<String> for P256 {
    type Error = anyhow::Error;
    fn try_from(key_pair: String) -> Result<Self, Self::Error> {
        key_pair.parse()
    }
}
