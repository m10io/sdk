use crate::account::AccountIdError;
use m10_protos::sdk::TransactionError;
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
    #[error("Invalid transaction")]
    InvalidTransaction,
}

pub type M10Result<T> = Result<T, M10Error>;
