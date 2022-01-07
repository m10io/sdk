use m10_sdk_protos::{prost::Message, sdk};

pub use ed25519::Ed25519;
pub use p256::P256;

mod ed25519;
mod p256;

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
pub enum Error {
    #[error("internal")]
    Internal,
    #[error("malformed signature")]
    MalFormedSignature,
}

#[async_trait::async_trait]
pub trait Signer: Send + Sync {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, Error>;
    fn public_key(&self) -> &[u8];
    fn algorithm(&self) -> sdk::signature::Algorithm;

    async fn sign_payload(&self, payload: &[u8]) -> Result<sdk::Signature, Error> {
        Ok(sdk::Signature {
            algorithm: self.algorithm().into(),
            public_key: self.public_key().into(),
            signature: self.sign(payload).await?,
        })
    }

    async fn sign_request<P: Message>(&self, data: P) -> Result<SignedRequest<P>, Error> {
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

    async fn endorse(&self, contract: &mut sdk::Contract, ledger_id: String) -> Result<(), Error> {
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

pub enum KeyPair {
    P256(P256),
    Ed25519(Ed25519),
}

#[async_trait::async_trait]
impl Signer for KeyPair {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, Error> {
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
