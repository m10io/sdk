use std::str::FromStr;
use uuid::Uuid;

pub(crate) mod account_sets;
pub(crate) mod accounts;
pub(crate) mod banks;
pub(crate) mod contracts;
pub(crate) mod role_bindings;
pub(crate) mod roles;

pub(crate) const LEDGER_ACCOUNTS: &str = "ledger-accounts";
pub(crate) const ROLES: &str = "roles";
pub(crate) const ROLE_BINDINGS: &str = "role-bindings";

#[derive(Clone, Debug)]
pub enum PrettyId {
    Hex(bytes::Bytes),
    Uuid(Uuid),
}

impl PrettyId {
    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Hex(h) => h.to_vec(),
            Self::Uuid(u) => u.as_bytes().to_vec(),
        }
    }

    #[inline]
    pub fn from_slice(b: &[u8]) -> Self {
        if let Ok(u) = Uuid::from_slice(b) {
            Self::Uuid(u)
        } else {
            Self::Hex(bytes::Bytes::copy_from_slice(b))
        }
    }
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
