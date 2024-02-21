mod document_id;

#[cfg(feature = "account")]
pub mod account;
#[cfg(feature = "collections")]
mod collections;
#[cfg(feature = "contract")]
pub mod contract;
#[cfg(feature = "client")]
pub mod error;
#[cfg(feature = "client")]
mod grpc_client;
#[cfg(feature = "client")]
mod http_client;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "client")]
mod m10_core_client;
#[cfg(feature = "service")]
pub mod transaction_ext;
#[cfg(feature = "service")]
pub mod transfer_ext;

#[cfg(feature = "client")]
pub use builders::*;

#[cfg(feature = "client")]
pub use types::*;

#[cfg(feature = "client")]
mod builders;

#[cfg(feature = "client")]
mod types;

#[cfg(feature = "client")]
pub mod ws;

#[cfg(feature = "collections")]
pub use collections::DocumentUpdate;

#[cfg(feature = "service")]
pub use m10_signing::*;
#[cfg(feature = "service")]
pub use transfer_ext::*;

pub use document_id::*;

/// Models and requests for the M10 Ledger
pub use m10_protos::sdk as ledger;
/// A re-export of `ledger` as [`sdk`] for backwards compatability
pub use m10_protos::sdk;

/// Models and requests for the M10 Directory
pub use m10_protos::directory;

/// A re-export of the [prost](https://github.com/tokio-rs/prost) crate.
pub use m10_protos::prost;

pub use m10_protos::metadata::{self, *};
pub use m10_protos::{Collection, Pack};

pub use bytes;

#[cfg(feature = "service")]
pub use tonic;

#[cfg(feature = "image")]
pub use image::ImageClient;

#[cfg(feature = "client")]
pub use grpc_client::GrpcClient;

#[cfg(feature = "client")]
pub use http_client::HttpClient;

#[cfg(feature = "client")]
pub use m10_core_client::*;

#[cfg(feature = "service")]
pub use transaction_ext::TransactionExt;
