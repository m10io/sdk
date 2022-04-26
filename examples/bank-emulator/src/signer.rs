#![allow(dead_code)]
use std::sync::Arc;

use m10_sdk::{prost::Message, SignedRequest, Signer, SigningError};

use crate::config::Config;

#[derive(Clone)]
pub struct ProxySigner(Arc<m10_sdk::Ed25519>);

impl ProxySigner {
    pub fn new(config: &Config) -> Self {
        Self(config.key_pair.clone())
    }
}

#[async_trait::async_trait]
impl Signer for ProxySigner {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError> {
        self.0.sign(msg).await
    }

    fn public_key(&self) -> &[u8] {
        self.0.public_key()
    }

    fn algorithm(&self) -> m10_sdk::sdk::signature::Algorithm {
        self.0.algorithm()
    }

    async fn sign_request<P: Message>(&self, data: P) -> Result<SignedRequest<P>, SigningError> {
        self.0.sign_request(data).await
    }
}
