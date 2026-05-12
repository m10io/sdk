use std::error::Error as StdError;
#[cfg(feature = "service")]
use std::io::ErrorKind;

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
    #[error(transparent)]
    WsError(#[from] tokio_tungstenite::tungstenite::error::Error),
}

pub type M10Result<T> = Result<T, M10Error>;

fn message_or_summary(summary: &str, detail: &str) -> String {
    let detail = detail.trim();
    if detail.is_empty() {
        summary.to_string()
    } else {
        detail.to_string()
    }
}

fn status_summary(code: tonic::Code) -> &'static str {
    match code {
        tonic::Code::Ok => "ok",
        tonic::Code::Cancelled => "request cancelled",
        tonic::Code::Unknown => "unknown error",
        tonic::Code::InvalidArgument => "bad request",
        tonic::Code::DeadlineExceeded => "deadline exceeded",
        tonic::Code::NotFound => "not found",
        tonic::Code::AlreadyExists => "already exists",
        tonic::Code::PermissionDenied => "unauthorized",
        tonic::Code::ResourceExhausted => "resource exhausted",
        tonic::Code::FailedPrecondition => "failed precondition",
        tonic::Code::Aborted => "request aborted",
        tonic::Code::OutOfRange => "value out of range",
        tonic::Code::Unimplemented => "unimplemented",
        tonic::Code::Internal => "internal error",
        tonic::Code::Unavailable => "service unavailable",
        tonic::Code::DataLoss => "data loss",
        tonic::Code::Unauthenticated => "unauthorized",
    }
}

impl M10Error {
    pub fn get_message(&self) -> String {
        match self {
            M10Error::Status(status) => transport_status_message(status).unwrap_or_else(|| {
                message_or_summary(status_summary(status.code()), status.message())
            }),
            M10Error::Transaction(error) => error.user_message(),
            _ => self.to_string(),
        }
    }
}

#[cfg(feature = "service")]
fn transport_status_message(status: &Status) -> Option<String> {
    let mut source = status.source();

    while let Some(error) = source {
        if let Some(hyper_error) = error.downcast_ref::<hyper::Error>() {
            if hyper_error.is_timeout() {
                return Some("connection to server timed out".to_string());
            }

            if hyper_error.is_closed() || hyper_error.is_incomplete_message() {
                return Some("connection to server was interrupted".to_string());
            }
        }

        if let Some(io_error) = error.downcast_ref::<std::io::Error>() {
            if matches!(
                io_error.kind(),
                ErrorKind::BrokenPipe
                    | ErrorKind::ConnectionAborted
                    | ErrorKind::ConnectionReset
                    | ErrorKind::NotConnected
                    | ErrorKind::UnexpectedEof
            ) {
                return Some("connection to server was interrupted".to_string());
            }
        }

        if let Some(h2_error) = error.downcast_ref::<h2::Error>() {
            if h2_error.is_io() || h2_error.reason().is_some() {
                return Some("connection to server was interrupted".to_string());
            }
        }

        source = error.source();
    }

    None
}

#[cfg(not(feature = "service"))]
fn transport_status_message(_status: &Status) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use m10_protos::sdk::transaction_error::Code;
    use std::{fmt, sync::Arc};

    #[derive(Debug)]
    struct WrappedError<E>(E);

    impl<E> fmt::Display for WrappedError<E>
    where
        E: fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<E> StdError for WrappedError<E>
    where
        E: StdError + Send + Sync + 'static,
    {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            Some(&self.0)
        }
    }

    #[test]
    fn blank_transaction_errors_fall_back_to_human_summary() {
        let err = M10Error::Transaction(TransactionError {
            code: Code::Unauthorized as i32,
            message: String::new(),
        });

        assert_eq!(err.get_message(), "unauthorized");
    }

    #[test]
    fn explicit_transaction_messages_are_not_prefixed() {
        let err = M10Error::Transaction(TransactionError {
            code: Code::DisplayCodeConflict as i32,
            message: "currency code already in use".to_string(),
        });

        assert_eq!(err.get_message(), "currency code already in use");
    }

    #[test]
    fn blank_status_errors_fall_back_to_human_summary() {
        let err = M10Error::Status(Status::permission_denied(""));

        assert_eq!(err.get_message(), "unauthorized");
    }

    #[test]
    fn explicit_status_messages_are_not_prefixed() {
        let err = M10Error::Status(Status::permission_denied(
            "cannot create a root ledger account",
        ));

        assert_eq!(err.get_message(), "cannot create a root ledger account");
    }

    #[cfg(feature = "service")]
    #[test]
    fn wrapped_h2_disconnects_are_humanized() {
        let mut status =
            Status::internal("h2 protocol error: error reading a body from connection");
        status.set_source(Arc::new(WrappedError(h2::Error::from(
            h2::Reason::NO_ERROR,
        ))));

        let err = M10Error::Status(status);

        assert_eq!(err.get_message(), "connection to server was interrupted");
    }

    #[cfg(feature = "service")]
    #[test]
    fn wrapped_io_disconnects_are_humanized() {
        let io_error = std::io::Error::new(ErrorKind::ConnectionReset, "connection reset by peer");
        let mut status = Status::internal("transport failure");
        status.set_source(Arc::new(WrappedError(io_error)));

        let err = M10Error::Status(status);

        assert_eq!(err.get_message(), "connection to server was interrupted");
    }
}
