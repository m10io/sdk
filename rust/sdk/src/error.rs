use crate::account::AccountIdError;
use m10_protos::{prost, sdk::TransactionError};
use m10_signing::SigningError;
use tonic::Status;

#[derive(thiserror::Error, Debug)]
pub enum M10Error {
    #[error(transparent)]
    Signing(#[from] SigningError),
    #[error(transparent)]
    Status(#[from] Status),
    #[error(transparent)]
    Transaction(#[from] TransactionError),
    #[error(transparent)]
    InvalidAccountId(#[from] AccountIdError),
    #[error(transparent)]
    Transport(#[from] tonic::transport::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Encoding(#[from] prost::EncodeError),
    #[error(transparent)]
    Decoding(#[from] prost::DecodeError),
    #[error(transparent)]
    SysTime(#[from] std::time::SystemTimeError),
    #[error("Invalid transaction")]
    InvalidTransaction,
    #[error("Signer required")]
    NoSigner,
}

pub type M10Result<T> = Result<T, M10Error>;
