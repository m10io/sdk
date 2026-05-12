#![allow(clippy::derive_partial_eq_without_eq)]

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
    use sha2::Digest as _;

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

    pub use crate::Collection;
    use crate::{sdk, Pack};

    impl Eq for RedeemableToken {}

    impl PartialOrd for RedeemableToken {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for RedeemableToken {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match (self.data.as_ref(), other.data.as_ref()) {
                (None, None) => std::cmp::Ordering::Equal,
                (Some(_), None) => std::cmp::Ordering::Greater,
                (None, Some(_)) => std::cmp::Ordering::Less,
                (Some(s), Some(o)) => s.id.cmp(&o.id),
            }
        }
    }

    impl Pack for AccountSet {
        const COLLECTION: Collection = Collection::AccountSets;
        fn set_id(&mut self, id: Vec<u8>) {
            self.id = id;
        }
        fn id(&self) -> &[u8] {
            &self.id
        }
    }

    impl Pack for AccountMetadata {
        const COLLECTION: Collection = Collection::AccountMetadata;
        fn set_id(&mut self, id: Vec<u8>) {
            self.id = id;
        }
        fn id(&self) -> &[u8] {
            &self.id
        }
    }

    impl Pack for Bank {
        const COLLECTION: Collection = Collection::Banks;
        fn set_id(&mut self, id: Vec<u8>) {
            self.id = id;
        }
        fn id(&self) -> &[u8] {
            &self.id
        }
    }

    impl Pack for LockDocument {
        const COLLECTION: Collection = Collection::Locks;
        fn set_id(&mut self, id: Vec<u8>) {
            self.lock_id = id;
        }
        fn id(&self) -> &[u8] {
            self.lock_id.as_ref()
        }
    }

    impl Pack for SettlementCycle {
        const COLLECTION: Collection = Collection::SettlementCycles;
        fn set_id(&mut self, id: Vec<u8>) {
            self.cycle_id = id;
        }
        fn id(&self) -> &[u8] {
            self.cycle_id.as_ref()
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

    impl From<SetInstrument> for Data {
        fn from(request: SetInstrument) -> Self {
            Self::SetInstrument(request)
        }
    }

    impl From<SetBalanceLimit> for Data {
        fn from(request: SetBalanceLimit) -> Self {
            Self::SetBalanceLimit(request)
        }
    }

    impl From<SetIssuanceLimit> for Data {
        fn from(request: SetIssuanceLimit) -> Self {
            Self::SetIssuanceLimit(request)
        }
    }

    impl From<SetDisplayCode> for Data {
        fn from(request: SetDisplayCode) -> Self {
            Self::SetDisplayCode(request)
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

    impl From<CreateToken> for Data {
        fn from(request: CreateToken) -> Self {
            Self::CreateToken(request)
        }
    }

    impl From<RedeemToken> for Data {
        fn from(request: RedeemToken) -> Self {
            Self::RedeemToken(request)
        }
    }

    impl From<CreateLock> for Data {
        fn from(request: CreateLock) -> Self {
            Self::CreateLock(request)
        }
    }

    impl From<ReleaseLock> for Data {
        fn from(request: ReleaseLock) -> Self {
            Self::ReleaseLock(request)
        }
    }

    impl From<RedeemLocksForCycle> for Data {
        fn from(request: RedeemLocksForCycle) -> Self {
            Self::RedeemLocksForCycle(request)
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

    fn invalid_signature(message: &'static str) -> TransactionError {
        TransactionError::with_message(transaction_error::Code::InvalidSignature, message)
    }

    impl transaction_error::Code {
        pub fn summary(self) -> &'static str {
            match self {
                transaction_error::Code::Unknown => "unknown error",
                transaction_error::Code::Unimplemented => "unimplemented",
                transaction_error::Code::NotFound => "not found",
                transaction_error::Code::AlreadyExists => "already exists",
                transaction_error::Code::Unauthorized => "unauthorized",
                transaction_error::Code::BadRequest => "bad request",
                transaction_error::Code::InvalidRequestType => "invalid request type",
                transaction_error::Code::InvalidAccountId => "invalid account id",
                transaction_error::Code::InvalidTransfer => "invalid transfer",
                transaction_error::Code::MessageTooLarge => "request is too large",
                transaction_error::Code::InvalidSignature => "invalid request signature",
                transaction_error::Code::VerificationFailed => "verification failed",
                transaction_error::Code::ReplayProtection => {
                    "request rejected by replay protection"
                }
                transaction_error::Code::InvalidExpression => "invalid expression",
                transaction_error::Code::IncorrectType => "incorrect value type",
                transaction_error::Code::AccountFrozen => "account is frozen",
                transaction_error::Code::UnmodifiedState => "request would not change state",
                transaction_error::Code::InsufficientBalance => "insufficient balance",
                transaction_error::Code::BalanceOverflow => "balance overflow",
                transaction_error::Code::AccountDepthExceeded => "account depth limit exceeded",
                transaction_error::Code::HoldingLimitExceeded => "holding balance limit exceeded",
                transaction_error::Code::IssuanceLimitExceeded => "issuance limit exceeded",
                transaction_error::Code::InvalidTarget => "invalid target",
                transaction_error::Code::DisplayCodeConflict => "display code already in use",
                transaction_error::Code::InvalidDisplayCode => "invalid display code",
                transaction_error::Code::InsufficientAvailableBalance => {
                    "insufficient available balance"
                }
                transaction_error::Code::LockNotFound => "lock not found",
                transaction_error::Code::InvalidLockState => "invalid lock state",
                transaction_error::Code::DuplicateLockId => "duplicate lock id",
            }
        }
    }

    impl TransactionError {
        pub fn with_message(code: transaction_error::Code, message: impl Into<String>) -> Self {
            Self {
                code: code.into(),
                message: message.into(),
            }
        }

        pub fn user_message(&self) -> String {
            let message = self.message.trim();
            if message.is_empty() {
                self.code().summary().to_string()
            } else {
                message.to_string()
            }
        }
    }

    impl fmt::Display for TransactionError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.user_message())
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
        fn default_primary_key_path(descriptor_name: &str) -> &'static str {
            let descriptor_name = descriptor_name.trim_start_matches('.');
            match descriptor_name {
                "m10.sdk.transaction.LockDocument" => "lock_id",
                "m10.sdk.transaction.SettlementCycle" => "cycle_id",
                _ => "id",
            }
        }

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
            let primary_key_path = Self::default_primary_key_path(&descriptor_name).to_string();
            Self {
                operation: Some(operation::Operation::InsertCollection(CollectionMetadata {
                    name,
                    descriptor_name,
                    file_descriptor_set: Some(crate::sdk::FILE_DESCRIPTOR_SET.clone()),
                    primary_key_path,
                    index_metadata,
                })),
            }
        }
    }

    impl Signature {
        pub fn verify(&self, message: &[u8]) -> Result<(), TransactionError> {
            let Signature {
                signature,
                public_key,
                algorithm,
            } = self;

            let alg = signature::Algorithm::try_from(*algorithm).map_err(|_| TransactionError {
                code: transaction_error::Code::BadRequest.into(),
                message: "unsupported request signature algorithm".to_owned(),
            })?;

            match alg {
                signature::Algorithm::P256Sha256Asn1 => {
                    ring::signature::UnparsedPublicKey::new(
                        &ring::signature::ECDSA_P256_SHA256_ASN1,
                        public_key,
                    )
                    .verify(message, signature)
                    .map_err(|_| invalid_signature("signature verification failed"))?;
                }
                signature::Algorithm::Ed25519 => {
                    ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key)
                        .verify(message, signature)
                        .map_err(|_| invalid_signature("signature verification failed"))?;
                }
                signature::Algorithm::Ed25519PhSha512 => {
                    let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(
                        public_key
                            .as_slice()
                            .try_into()
                            .map_err(|_| invalid_signature("public key has an invalid length"))?,
                    )
                    .map_err(|_| invalid_signature("public key is invalid"))?;
                    let sig = ed25519_dalek::Signature::from_slice(signature)
                        .map_err(|_| invalid_signature("signature format is invalid"))?;
                    verifying_key
                        .verify_prehashed_strict(
                            sha2::Sha512::new_with_prefix(sha2::Sha512::digest(message)),
                            None,
                            &sig,
                        )
                        .map_err(|_| invalid_signature("signature verification failed"))?;
                }
            }

            Ok(())
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn transaction_error_uses_human_summary_when_message_is_empty() {
            let err = TransactionError::with_message(transaction_error::Code::Unauthorized, "");

            assert_eq!(err.user_message(), "unauthorized");
            assert_eq!(err.to_string(), "unauthorized");
        }

        #[test]
        fn transaction_error_preserves_explicit_message() {
            let err = TransactionError::with_message(
                transaction_error::Code::Unauthorized,
                "cannot create a root ledger account",
            );

            assert_eq!(err.user_message(), "cannot create a root ledger account");
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
