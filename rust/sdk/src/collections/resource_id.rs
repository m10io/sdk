use std::str::FromStr;

use crate::account::AccountIdError;
use crate::error::M10Error;
use m10_protos::sdk;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum ResourceId {
    Hex(bytes::Bytes),
    Uuid(Uuid),
}

impl ResourceId {
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            ResourceId::Hex(h) => h.to_vec(),
            ResourceId::Uuid(u) => u.as_bytes().to_vec(),
        }
    }
}

impl Serialize for ResourceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let id = match self {
            ResourceId::Hex(h) => hex::encode(h),
            ResourceId::Uuid(u) => u.to_string(),
        };
        serializer.serialize_str(&id)
    }
}

struct ResourceIdVisitor;

impl serde::de::Visitor<'_> for ResourceIdVisitor {
    type Value = ResourceId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Hex-encoded byte buffer")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let id = ResourceId::from_str(v).map_err(E::custom)?;
        Ok(id)
    }
}

impl<'de> Deserialize<'de> for ResourceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ResourceIdVisitor)
    }
}

impl From<ResourceId> for bytes::Bytes {
    fn from(val: ResourceId) -> Self {
        match val {
            ResourceId::Hex(h) => h,
            ResourceId::Uuid(u) => bytes::Bytes::copy_from_slice(u.as_bytes()),
        }
    }
}

impl From<bytes::Bytes> for ResourceId {
    fn from(b: bytes::Bytes) -> Self {
        if let Ok(u) = Uuid::from_slice(b.as_ref()) {
            ResourceId::Uuid(u)
        } else {
            ResourceId::Hex(b)
        }
    }
}

impl From<sdk::value::Value> for ResourceId {
    fn from(v: sdk::value::Value) -> Self {
        match v {
            sdk::value::Value::BoolValue(v) => ResourceId::Hex(if v {
                bytes::Bytes::from_static(&[1])
            } else {
                bytes::Bytes::from_static(&[0])
            }),
            sdk::value::Value::BytesValue(v) => ResourceId::Hex(v),
            sdk::value::Value::DoubleValue(v) => {
                ResourceId::Hex(bytes::Bytes::copy_from_slice(v.to_be_bytes().as_slice()))
            }
            sdk::value::Value::FloatValue(v) => {
                ResourceId::Hex(bytes::Bytes::copy_from_slice(v.to_be_bytes().as_slice()))
            }
            sdk::value::Value::Int16Value(v) => {
                ResourceId::Hex(bytes::Bytes::copy_from_slice(v.to_be_bytes().as_slice()))
            }
            sdk::value::Value::Int32Value(v) => {
                ResourceId::Hex(bytes::Bytes::copy_from_slice(v.to_be_bytes().as_slice()))
            }
            sdk::value::Value::Int64Value(v) => {
                ResourceId::Hex(bytes::Bytes::copy_from_slice(v.to_be_bytes().as_slice()))
            }
            sdk::value::Value::Int8Value(v) => {
                ResourceId::Hex(bytes::Bytes::copy_from_slice(v.to_be_bytes().as_slice()))
            }
            sdk::value::Value::StringValue(v) => ResourceId::Hex(bytes::Bytes::from(v)),
            sdk::value::Value::Uint16Value(v) => {
                ResourceId::Hex(bytes::Bytes::copy_from_slice(v.to_be_bytes().as_slice()))
            }
            sdk::value::Value::Uint32Value(v) => {
                ResourceId::Hex(bytes::Bytes::copy_from_slice(v.to_be_bytes().as_slice()))
            }
            sdk::value::Value::Uint64Value(v) => {
                ResourceId::Hex(bytes::Bytes::copy_from_slice(v.to_be_bytes().as_slice()))
            }
            sdk::value::Value::Uint8Value(v) => {
                ResourceId::Hex(bytes::Bytes::copy_from_slice(v.to_be_bytes().as_slice()))
            }
        }
    }
}

#[cfg(feature = "format")]
impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceId::Hex(h) => {
                let h = hex::encode(h);
                f.write_str(&h)
            }
            ResourceId::Uuid(u) => std::fmt::Display::fmt(u, f),
        }
    }
}

impl TryFrom<&[u8]> for ResourceId {
    type Error = M10Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if let Ok(u) = Uuid::from_slice(value) {
            Ok(ResourceId::Uuid(u))
        } else {
            Ok(ResourceId::Hex(bytes::Bytes::copy_from_slice(value)))
        }
    }
}

#[cfg(feature = "format")]
impl std::str::FromStr for ResourceId {
    type Err = M10Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(u) = Uuid::parse_str(s) {
            Ok(ResourceId::Uuid(u))
        } else if let Ok(h) = hex::decode(s) {
            Ok(ResourceId::Hex(h.into()))
        } else {
            Err(M10Error::InvalidAccountId(AccountIdError::InvalidLen))
        }
    }
}
