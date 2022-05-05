mod document_id;

#[cfg(feature = "account")]
pub mod account;
#[cfg(feature = "service")]
pub mod client;
#[cfg(feature = "collections")]
mod collections;
#[cfg(feature = "contract")]
pub mod contract;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "service")]
mod transfer_ext;

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

#[cfg(feature = "image")]
pub use image::ImageClient;

#[cfg(feature = "service")]
pub use client::LedgerClient;
