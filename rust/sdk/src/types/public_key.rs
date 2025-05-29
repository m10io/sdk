use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PublicKey(pub Vec<u8>);

impl PublicKey {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }
}

#[cfg(feature = "format")]
impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", base64::encode(&self.0))
    }
}

impl Serialize for PublicKey {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&base64::display::Base64Display::with_config(
            &self.0,
            base64::STANDARD,
        ))
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis;
        impl serde::de::Visitor<'_> for Vis {
            type Value = PublicKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a base64 string")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                base64::decode(v).map(PublicKey).map_err(Error::custom)
            }
        }
        deserializer.deserialize_str(Vis)
    }
}

impl FromStr for PublicKey {
    type Err = base64::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = base64::decode(s)?;
        Ok(Self(key))
    }
}

impl From<PublicKey> for Vec<u8> {
    fn from(val: PublicKey) -> Self {
        val.0
    }
}
