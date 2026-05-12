use crate::{debug_verbose, internal_error};
use crate::{Signer, SigningError};

use async_trait::async_trait;
use m10_protos::sdk::signature::Algorithm;
use p256::{elliptic_curve::sec1::ToEncodedPoint, pkcs8::DecodePublicKey};
use vaultrs::api::transit::requests::SignDataRequestBuilder;
use vaultrs::api::transit::responses::{ReadKeyData, ReadKeyResponse};
use vaultrs::api::transit::HashAlgorithm;
use vaultrs::api::transit::MarshalingAlgorithm;
use vaultrs::{
    api::transit::{requests::CreateKeyRequestBuilder, KeyType},
    client::{VaultClient, VaultClientSettingsBuilder},
    transit,
};

pub struct VaultTransit {
    client: VaultClient,
    key_name: String,
    mount: String,
    public_key: Vec<u8>,
    algorithm: Algorithm,
}

impl VaultTransit {
    pub async fn new(
        vault_addr: &str,
        token: &str,
        key_name: String,
        mount: Option<String>,
        algorithm: Option<Algorithm>,
        namespace: Option<String>,
    ) -> Result<Self, SigningError> {
        let settings = VaultClientSettingsBuilder::default()
            .address(vault_addr)
            .token(token)
            .namespace(namespace)
            .build()
            .map_err(|err| internal_error(err, "VaultTransit::new: building client settings"))?;

        let client = VaultClient::new(settings)
            .map_err(|err| internal_error(err, "VaultTransit::new: creating VaultClient"))?;

        let mount = mount.unwrap_or_else(|| "transit".to_string());

        let algorithm = algorithm.unwrap_or(Algorithm::Ed25519);
        let key_type = match algorithm {
            Algorithm::Ed25519 | Algorithm::Ed25519PhSha512 => KeyType::Ed25519,
            Algorithm::P256Sha256Asn1 => KeyType::EcdsaP256,
        };

        let public_key = match transit::key::read(&client, &mount, &key_name).await {
            Ok(response) => extract_public_key(&response)?,
            Err(_) => {
                transit::key::create(
                    &client,
                    &mount,
                    &key_name,
                    Some(
                        &mut CreateKeyRequestBuilder::default()
                            .key_type(key_type)
                            .convergent_encryption(false)
                            .exportable(true)
                            .allow_plaintext_backup(false),
                    ),
                )
                .await
                .map_err(|err| internal_error(err, "VaultTransit::new: creating key"))?;

                let response = transit::key::read(&client, &mount, &key_name)
                    .await
                    .map_err(|err| {
                        internal_error(err, "VaultTransit::new: reading key after creation")
                    })?;

                extract_public_key(&response)?
            }
        };

        Ok(Self {
            client,
            key_name,
            mount,
            public_key,
            algorithm,
        })
    }
}

pub fn extract_public_key(response: &ReadKeyResponse) -> Result<Vec<u8>, SigningError> {
    let keys = match &response.keys {
        ReadKeyData::Asymmetric(keys) => keys,
        _ => {
            return Err(internal_error(
                "unexpected key data type",
                "extract_public_key",
            ))
        }
    };

    let latest_key = keys
        .iter()
        .max_by_key(|(id, _)| id.parse::<u64>().unwrap_or_default())
        .ok_or_else(|| internal_error("missing keys", "extract_public_key"))?;

    let raw_key = &latest_key.1.public_key;

    match response.key_type {
        KeyType::Ed25519 => {
            // Raw 32-byte Ed25519 public key base64
            base64::decode(raw_key.trim())
                .map_err(|err| internal_error(err, "extract_public_key: ed25519 base64"))
        }
        KeyType::EcdsaP256 => {
            // PEM-encoded SubjectPublicKeyInfo for P-256 key
            let key = p256::PublicKey::from_public_key_pem(raw_key)
                .map_err(|err| internal_error(err, "extract_public_key: p256 pem parse"))?;
            Ok(key.to_encoded_point(false).as_bytes().to_vec())
        }
        _ => Err(internal_error("unsupported key type", "extract_public_key")),
    }
}

pub fn parse_vault_signature(signature_str: &str) -> Result<Vec<u8>, SigningError> {
    let prefix = "vault:v1:";
    if !signature_str.starts_with(prefix) {
        return Err(SigningError::Internal);
    }

    let b64_signature = &signature_str[prefix.len()..];
    base64::decode(b64_signature).map_err(|_| SigningError::Internal)
}

#[async_trait]
impl Signer for VaultTransit {
    async fn sign_prepared_transaction(
        &self,
        prepared: &crate::PreparedTransaction,
    ) -> Result<Vec<u8>, SigningError> {
        let (digest_bytes, hash_alg, marshaling_alg): (
            &[u8],
            HashAlgorithm,
            Option<MarshalingAlgorithm>,
        ) = match (self.algorithm, &prepared.digest) {
            (Algorithm::P256Sha256Asn1, crate::PreparedDigest::P256Sha256(bytes)) => (
                bytes,
                HashAlgorithm::Sha2_256,
                Some(MarshalingAlgorithm::Asn1),
            ),
            (Algorithm::Ed25519PhSha512, crate::PreparedDigest::Ed25519PhSha512(bytes)) => {
                (bytes, HashAlgorithm::Sha2_512, None)
            }
            (Algorithm::Ed25519, _) => {
                return Err(SigningError::KeyInvalid(
                    "plain Ed25519 does not support digest signing".into(),
                ));
            }
            _ => {
                return Err(internal_error(
                    "algorithm and digest type mismatch or unsupported",
                    "VaultTransit",
                ));
            }
        };

        let input = base64::encode(digest_bytes);
        let mut opts = SignDataRequestBuilder::default();
        opts.prehashed(true).hash_algorithm(hash_alg);
        if let Some(ma) = marshaling_alg {
            opts.marshaling_algorithm(ma);
        }

        debug_verbose!("VaultTransit::sign_prepared: sending digest to Vault");
        let sign_response = transit::data::sign(
            &self.client,
            &self.mount,
            &self.key_name,
            &input,
            Some(&mut opts),
        )
        .await
        .map_err(|err| {
            internal_error(err, "VaultTransit::sign_prepared: signing digest via Vault")
        })?;
        parse_vault_signature(&sign_response.signature)
    }

    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError> {
        if self.algorithm == Algorithm::Ed25519PhSha512 {
            return Err(SigningError::KeyInvalid(
                "Ed25519Ph keys must use sign_prepared_transaction".into(),
            ));
        }

        debug_verbose!("VaultTransit::sign: signing payload: {:?}", msg);

        let input = base64::encode(msg);
        let sign_response =
            transit::data::sign(&self.client, &self.mount, &self.key_name, &input, None)
                .await
                .map_err(|err| internal_error(err, "VaultTransit::sign: signing payload"))?;

        let decoded_signature = parse_vault_signature(&sign_response.signature)?;

        debug_verbose!(
            "VaultTransit::sign: decoded signature: {:?}",
            decoded_signature
        );

        Ok(decoded_signature)
    }

    fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    fn algorithm(&self) -> Algorithm {
        self.algorithm
    }
}

#[cfg(test)]
impl VaultTransit {
    pub fn new_mock(address: &str, algorithm: m10_protos::sdk::signature::Algorithm) -> Self {
        let settings = vaultrs::client::VaultClientSettingsBuilder::default()
            .address(address)
            .token("some-token")
            .build()
            .unwrap();

        Self {
            client: vaultrs::client::VaultClient::new(settings).unwrap(),
            key_name: "test-key".to_string(),
            mount: "transit".to_string(),
            public_key: vec![],
            algorithm,
        }
    }
}
