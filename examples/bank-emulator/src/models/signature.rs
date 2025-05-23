use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub public_key: String,
    pub signature: String,
    pub algorithm: i32,
}

impl TryFrom<Signature> for m10_sdk::Signature {
    type Error = crate::error::Error;

    fn try_from(val: Signature) -> Result<Self, Self::Error> {
        let public_key = m10_sdk::PublicKey::from_str(&val.public_key)?;
        let signature = m10_sdk::Sign::from_str(&val.signature)?;

        Ok(m10_sdk::Signature {
            public_key,
            signature,
            algorithm: val.algorithm,
        })
    }
}

impl From<m10_sdk::Signature> for Signature {
    fn from(val: m10_sdk::Signature) -> Self {
        Signature {
            public_key: val.public_key.to_string(),
            signature: val.signature.to_string(),
            algorithm: val.algorithm,
        }
    }
}

impl From<m10_sdk::sdk::transaction::Signature> for Signature {
    fn from(val: m10_sdk::sdk::transaction::Signature) -> Self {
        m10_sdk::Signature::from(val).into()
    }
}
