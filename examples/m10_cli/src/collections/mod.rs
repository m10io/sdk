use std::str::FromStr;
use uuid::Uuid;

pub(crate) mod account_sets;
pub(crate) mod accounts;
pub(crate) mod actions;
pub(crate) mod banks;
pub(crate) mod contracts;
pub(crate) mod role_bindings;
pub(crate) mod roles;
pub(crate) mod transfers;
pub(crate) mod tx;

#[derive(Clone, Debug)]
pub enum PrettyId {
    Hex(bytes::Bytes),
    Uuid(Uuid),
}

impl From<PrettyId> for bytes::Bytes {
    fn from(val: PrettyId) -> Self {
        match val {
            PrettyId::Hex(h) => h,
            PrettyId::Uuid(u) => u.as_bytes().to_vec().into(),
        }
    }
}

impl From<bytes::Bytes> for PrettyId {
    fn from(b: bytes::Bytes) -> Self {
        if let Ok(u) = Uuid::from_slice(b.as_ref()) {
            PrettyId::Uuid(u)
        } else {
            PrettyId::Hex(b)
        }
    }
}

impl std::fmt::Display for PrettyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrettyId::Hex(h) => {
                let h = hex::encode(h);
                f.write_str(&h)
            }
            PrettyId::Uuid(u) => std::fmt::Display::fmt(u, f),
        }
    }
}

impl FromStr for PrettyId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(u) = Uuid::parse_str(s) {
            Ok(PrettyId::Uuid(u))
        } else if let Ok(h) = hex::decode(s) {
            Ok(PrettyId::Hex(h.into()))
        } else {
            Err(anyhow::anyhow!("id must be a uuid or hex"))
        }
    }
}
