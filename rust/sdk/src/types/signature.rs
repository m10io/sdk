use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use crate::PublicKey;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Sign(pub Vec<u8>);

impl Sign {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl std::fmt::Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", base64::encode(&self.0))
    }
}

impl Serialize for Sign {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&base64::display::Base64Display::with_config(
            &self.0,
            base64::STANDARD,
        ))
    }
}

impl<'de> Deserialize<'de> for Sign {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis;
        impl serde::de::Visitor<'_> for Vis {
            type Value = Sign;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a base64 string")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                base64::decode(v).map(Sign).map_err(Error::custom)
            }
        }
        deserializer.deserialize_str(Vis)
    }
}

impl FromStr for Sign {
    type Err = base64::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = base64::decode(s)?;
        Ok(Self(key))
    }
}

impl From<Sign> for Vec<u8> {
    fn from(val: Sign) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub public_key: PublicKey,
    pub signature: Sign,
    pub algorithm: i32,
}

impl From<Signature> for m10_protos::sdk::transaction::Signature {
    fn from(val: Signature) -> Self {
        m10_protos::sdk::transaction::Signature {
            public_key: val.public_key.to_vec(),
            signature: val.signature.to_vec(),
            algorithm: val.algorithm,
        }
    }
}

impl From<m10_protos::sdk::transaction::Signature> for Signature {
    fn from(val: m10_protos::sdk::transaction::Signature) -> Self {
        Signature {
            public_key: PublicKey(val.public_key),
            signature: Sign(val.signature),
            algorithm: val.algorithm,
        }
    }
}
