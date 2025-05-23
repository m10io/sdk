use crate::{debug_verbose, internal_error};
use crate::{Signer, SigningError};

use async_trait::async_trait;
use m10_protos::sdk::signature::Algorithm;
use vaultrs::api::transit::responses::ReadKeyData;
use vaultrs::api::transit::responses::ReadKeyResponse;
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
            Algorithm::Ed25519 => KeyType::Ed25519,
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
    match &response.keys {
        ReadKeyData::Asymmetric(keys) => {
            let latest_key = keys
                .iter()
                .max_by_key(|(id, _)| id.parse::<u64>().unwrap_or_default())
                .ok_or_else(|| internal_error("missing keys", "extract_public_key"))?;

            base64::decode(&latest_key.1.public_key).map_err(|_| SigningError::Internal)
        }
        _ => Err(internal_error(
            "unexpected key data type",
            "extract_public_key",
        )),
    }
}

#[async_trait]
impl Signer for VaultTransit {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError> {
        debug_verbose!("VaultTransit::sign: signing payload: {:?}", msg);

        let input = base64::encode(msg);
        let sign_response =
            transit::data::sign(&self.client, &self.mount, &self.key_name, &input, None)
                .await
                .map_err(|err| internal_error(err, "VaultTransit::sign: signing payload"))?;

        let signature_str = sign_response.signature;
        let prefix = "vault:v1:";
        if !signature_str.starts_with(prefix) {
            return Err(SigningError::Internal);
        }
        let b64_signature = &signature_str[prefix.len()..];

        let decoded_signature = base64::decode(b64_signature)
            .map_err(|err| internal_error(err, "VaultTransit::sign: decoding signature"))?;

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
