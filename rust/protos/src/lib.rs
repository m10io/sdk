/// Includes generated protocol buffer code.
macro_rules! include_proto {
    ($package:tt) => {
        include!(concat!(env!("OUT_DIR"), "/", $package, ".rs"));
    };
}

pub mod directory {
    include_proto!("m10.directory");
    use core::fmt;
    use core::str::FromStr;

    #[derive(Debug)]
    pub struct InvalidAliasType();

    impl fmt::Display for InvalidAliasType {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            f.write_str("invalid alias type")
        }
    }

    impl std::error::Error for InvalidAliasType {}

    impl FromStr for alias::Type {
        type Err = InvalidAliasType;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "handle" => Ok(alias::Type::Handle),
                "email" => Ok(alias::Type::Email),
                "phone" => Ok(alias::Type::Phone),
                _ => Err(InvalidAliasType()),
            }
        }
    }

    impl AsRef<str> for alias::Type {
        fn as_ref(&self) -> &str {
            match self {
                alias::Type::Handle => "handle",
                alias::Type::Email => "email",
                alias::Type::Phone => "phone",
            }
        }
    }

    impl fmt::Display for alias::Type {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            f.write_str(self.as_ref())
        }
    }
}

pub mod sdk {
    include_proto!("m10.sdk");

    pub const FILE_DESCRIPTOR_SET_BYTES: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/m10.sdk.bin"));
    pub static FILE_DESCRIPTOR_SET: once_cell::sync::Lazy<prost_types::FileDescriptorSet> =
        once_cell::sync::Lazy::new(|| {
            prost::Message::decode(FILE_DESCRIPTOR_SET_BYTES).expect("file descriptor parse failed")
        });

    pub mod model {
        include_proto!("m10.sdk.model");
        pub const FILE_DESCRIPTOR_SET_BYTES: &[u8] =
            include_bytes!(concat!(env!("OUT_DIR"), "/m10.model.pb"));
        pub static FILE_DESCRIPTOR_SET: once_cell::sync::Lazy<prost_types::FileDescriptorSet> =
            once_cell::sync::Lazy::new(|| {
                prost::Message::decode(FILE_DESCRIPTOR_SET_BYTES)
                    .expect("file descriptor parse failed")
            });
    }
    pub mod transaction {
        include_proto!("m10.sdk.transaction");
    }
    pub mod metadata {
        include_proto!("m10.sdk.metadata");
    }
    pub use metadata::*;
    pub use model::*;
    use prost::Message;
    pub use transaction::*;

    use core::{fmt, str};
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    use crate::{sdk, Collection, Pack};

    impl str::FromStr for AccountRef {
        type Err = AccountRefParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s = s.split('/');
            let ledger_id = s.next().ok_or(AccountRefParseError())?.to_string();
            let account_id = s.next().ok_or(AccountRefParseError())?;
            let account_id = hex::decode(account_id).map_err(|_| AccountRefParseError())?;
            Ok(Self {
                ledger_id,
                account_id,
            })
        }
    }

    impl Serialize for AccountRef {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_str(self)
        }
    }

    impl<'de> Deserialize<'de> for AccountRef {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(de::Error::custom)
        }
    }

    impl fmt::Display for AccountRef {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}/{}", self.ledger_id, hex::encode(&self.account_id))
        }
    }

    #[derive(Debug)]
    pub struct AccountRefParseError();

    impl fmt::Display for AccountRefParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("failed to parse account ref")
        }
    }

    impl std::error::Error for AccountRefParseError {}

    impl Pack for AccountSet {
        const COLLECTION: Collection = Collection::AccountSets;
        fn set_id(&mut self, id: Vec<u8>) {
            self.id = id;
        }
        fn id(&self) -> &[u8] {
            &self.id
        }
    }

    impl Pack for Account {
        const COLLECTION: Collection = Collection::Accounts;
        fn set_id(&mut self, id: Vec<u8>) {
            self.id = id;
        }
        fn id(&self) -> &[u8] {
            &self.id
        }
    }

    use transaction_data::Data;

    impl From<CreateTransfer> for Data {
        fn from(create_transfer: CreateTransfer) -> Self {
            Self::Transfer(create_transfer)
        }
    }

    impl From<CreateLedgerAccount> for Data {
        fn from(request: CreateLedgerAccount) -> Self {
            Self::CreateLedgerAccount(request)
        }
    }

    impl From<SetFreezeState> for Data {
        fn from(request: SetFreezeState) -> Self {
            Self::SetFreezeState(request)
        }
    }

    impl From<InvokeAction> for Data {
        fn from(request: InvokeAction) -> Self {
            Self::InvokeAction(request)
        }
    }

    impl From<CommitTransfer> for Data {
        fn from(request: CommitTransfer) -> Self {
            Self::CommitTransfer(request)
        }
    }

    impl From<sdk::DocumentOperations> for Data {
        fn from(operations: sdk::DocumentOperations) -> Self {
            Self::DocumentOperations(operations)
        }
    }

    impl From<Vec<sdk::Operation>> for Data {
        fn from(operations: Vec<sdk::Operation>) -> Self {
            Self::from(sdk::DocumentOperations { operations })
        }
    }

    impl From<sdk::Operation> for Data {
        fn from(operation: sdk::Operation) -> Self {
            Self::from(vec![operation])
        }
    }

    impl From<CreateLedgerTransfers> for Contract {
        fn from(transfers: CreateLedgerTransfers) -> Self {
            Self {
                transactions: transfers.encode_to_vec(),
                ..Default::default()
            }
        }
    }

    impl TransactionResponse {
        pub fn tx_error(self) -> Result<Self, TransactionError> {
            match self.error {
                Some(err) => Err(err),
                None => Ok(self),
            }
        }
    }

    impl fmt::Display for TransactionError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}: {}", self.code(), self.message)
        }
    }

    impl std::error::Error for TransactionError {}

    impl From<prost::bytes::Bytes> for Value {
        fn from(bytes: prost::bytes::Bytes) -> Value {
            Value {
                value: Some(value::Value::BytesValue(bytes)),
            }
        }
    }

    impl Operation {
        pub fn insert<D: Pack>(document: D) -> Self {
            Self {
                operation: Some(operation::Operation::InsertDocument(
                    operation::InsertDocument {
                        collection: D::COLLECTION.to_string(),
                        document: document.pack(),
                    },
                )),
            }
        }

        pub fn delete<D: Pack>(id: Vec<u8>) -> Self {
            Self {
                operation: Some(operation::Operation::DeleteDocument(
                    operation::DeleteDocument {
                        collection: D::COLLECTION.to_string(),
                        primary_key: Some(bytes::Bytes::from(id).into()),
                    },
                )),
            }
        }

        pub fn new_index<D: Pack>(path: Vec<String>) -> Self {
            Self {
                operation: Some(operation::Operation::InsertIndex(operation::InsertIndex {
                    collection: D::COLLECTION.to_string(),
                    path: path.join("."),
                })),
            }
        }

        pub fn new_collection(
            name: String,
            descriptor_name: String,
            index_metadata: Vec<IndexMetadata>,
        ) -> Self {
            Self {
                operation: Some(operation::Operation::InsertCollection(CollectionMetadata {
                    name,
                    descriptor_name,
                    file_descriptor_set: Some(crate::sdk::FILE_DESCRIPTOR_SET.clone()),
                    primary_key_path: "id".to_string(),
                    index_metadata,
                })),
            }
        }
    }

    impl Pack for RoleBinding {
        const COLLECTION: Collection = Collection::RoleBindings;
        fn set_id(&mut self, id: Vec<u8>) {
            self.id = bytes::Bytes::from(id);
        }
        fn id(&self) -> &[u8] {
            &self.id
        }
    }

    impl Pack for Role {
        const COLLECTION: Collection = Collection::Roles;
        fn set_id(&mut self, id: Vec<u8>) {
            self.id = bytes::Bytes::from(id);
        }
        fn id(&self) -> &[u8] {
            &self.id
        }
    }
}

pub mod health {
    include_proto!("grpc.health.v1");
}

pub mod metadata;
mod pack;

/// Re-export of prost
pub mod prost {
    pub use prost::*;
    pub use prost_types::*;
}
pub use metadata::*;
pub use pack::{Collection, Pack};

use prost_types::Any;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeAs, SerializeAs};

pub struct AnySerDeCompat;

#[derive(Serialize)]
struct AnySerializeWrapper<'a> {
    pub type_url: &'a str,
    pub value: &'a [u8],
}

#[derive(Deserialize)]
struct AnyDeserializeWrapper {
    pub type_url: String,
    pub value: Vec<u8>,
}

impl SerializeAs<Any> for AnySerDeCompat {
    fn serialize_as<S>(source: &Any, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        AnySerializeWrapper {
            type_url: &source.type_url,
            value: &source.value,
        }
        .serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, Any> for AnySerDeCompat {
    fn deserialize_as<D>(deserializer: D) -> Result<Any, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let AnyDeserializeWrapper { type_url, value } =
            AnyDeserializeWrapper::deserialize(deserializer)?;
        Ok(Any { type_url, value })
    }
}
