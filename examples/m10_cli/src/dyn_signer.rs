use std::sync::Arc;

use m10_protos::prost;
use m10_protos::sdk::signature::Algorithm;
use m10_protos::sdk::Contract;
use m10_protos::sdk::Endorsement;
use m10_protos::sdk::RequestEnvelope;
use m10_protos::sdk::Signature;
use m10_sdk::{SignedRequest, Signer, SigningError, VaultTransit};
use tonic::async_trait;

#[async_trait]
pub trait DynSigner: Send + Sync {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError>;

    fn public_key(&self) -> &[u8];

    fn algorithm(&self) -> Algorithm;
}

#[async_trait]
impl DynSigner for m10_sdk::KeyPair {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError> {
        Signer::sign(self, msg).await
    }

    fn public_key(&self) -> &[u8] {
        Signer::public_key(self)
    }

    fn algorithm(&self) -> Algorithm {
        Signer::algorithm(self)
    }
}

#[async_trait]
impl DynSigner for VaultTransit {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SigningError> {
        Signer::sign(self, msg).await
    }

    fn public_key(&self) -> &[u8] {
        Signer::public_key(self)
    }

    fn algorithm(&self) -> Algorithm {
        Signer::algorithm(self)
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
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, m10_sdk::SigningError> {
        self.0.sign(msg).await
    }

    fn public_key(&self) -> &[u8] {
        self.0.public_key()
    }

    fn algorithm(&self) -> Algorithm {
        self.0.algorithm()
    }

    async fn sign_payload(&self, payload: &[u8]) -> Result<Signature, m10_sdk::SigningError> {
        let sig = self.sign(payload).await?;
        Ok(Signature {
            algorithm: self.algorithm().into(),
            public_key: self.public_key().into(),
            signature: sig,
        })
    }

    async fn sign_request<P: prost::Message>(
        &self,
        data: P,
    ) -> Result<SignedRequest<P>, m10_sdk::SigningError> {
        let payload = data.encode_to_vec();
        let signature = self.sign_payload(&payload).await?;
        let envelope = RequestEnvelope {
            payload,
            signature: Some(signature),
        };
        Ok(SignedRequest {
            request_envelope: envelope,
            data,
        })
    }

    async fn endorse(
        &self,
        contract: &mut Contract,
        ledger_id: String,
    ) -> Result<(), m10_sdk::SigningError> {
        let public_key = self.public_key();
        let already_signed = contract
            .endorsements
            .iter()
            .filter_map(|e| e.signature.as_ref())
            .any(|s| s.public_key == public_key);
        if !already_signed {
            let sig = self.sign_payload(&contract.transactions).await?;
            contract.endorsements.push(Endorsement {
                ledger_id,
                signature: Some(sig),
            });
        }
        Ok(())
    }
}
