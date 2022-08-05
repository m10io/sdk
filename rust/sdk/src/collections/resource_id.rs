use crate::account::AccountIdError;
use crate::error::M10Error;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum ResourceId {
    Hex(bytes::Bytes),
    Uuid(Uuid),
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
        } else if let Ok(h) = hex::decode(value) {
            Ok(ResourceId::Hex(h.into()))
        } else {
            Err(M10Error::InvalidAccountId(AccountIdError::InvalidLen))
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
