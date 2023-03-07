use core::fmt;
use core::ops::Deref;
use core::str::FromStr;

use prost::Message;

pub trait Pack: Default + Message + Sized {
    const COLLECTION: Collection;

    fn pack(&self) -> Vec<u8> {
        self.encode_to_vec()
    }

    fn set_id(&mut self, id: Vec<u8>);

    fn id(&self) -> &[u8];
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Collection {
    AccountMetadata,
    AccountSets,
    RoleBindings,
    Roles,
    Banks,
}

impl Deref for Collection {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Collection::AccountMetadata => "account-metadata",
            Collection::AccountSets => "account-sets",
            Collection::RoleBindings => "role-bindings",
            Collection::Roles => "roles",
            Collection::Banks => "banks",
        }
    }
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }
}

impl From<Collection> for String {
    fn from(collection: Collection) -> Self {
        collection.to_string()
    }
}

impl FromStr for Collection {
    type Err = UnsupportedCollection;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account-metadata" => Ok(Collection::AccountMetadata),
            "account-sets" => Ok(Collection::AccountSets),
            "role-bindings" => Ok(Collection::RoleBindings),
            "roles" => Ok(Collection::Roles),
            "banks" => Ok(Collection::Banks),
            _unsupported => Err(UnsupportedCollection()),
        }
    }
}

#[derive(Debug)]
pub struct UnsupportedCollection();

impl fmt::Display for UnsupportedCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unsupported collection")
    }
}

impl std::error::Error for UnsupportedCollection {}
