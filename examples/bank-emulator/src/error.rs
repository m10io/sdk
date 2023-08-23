use actix_web::ResponseError;
use eyre::eyre;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<eyre::Report>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "code")]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    AlreadyExists,
    BadRequest { errors: Vec<ValidationError> },
    Internal,
    Upstream,
    NotFound,
    Unauthorized,
    InsufficentFunds,
    AccountUnopened,
    SandboxOnly,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ValidationError {
    field: String,
    msg: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            msg: msg.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(source) = &self.source {
            write!(f, "{}: {}", self.kind, source)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::BadRequest { errors } => {
                write!(f, "bad request: ")?;
                f.debug_list().entries(errors.iter()).finish()
            }
            ErrorKind::Internal => write!(f, "internal error"),
            ErrorKind::Upstream => write!(f, "upstream"),
            ErrorKind::NotFound => write!(f, "not found"),
            ErrorKind::Unauthorized => write!(f, "unauthorized"),
            ErrorKind::InsufficentFunds => write!(f, "insufficent funds"),
            ErrorKind::AlreadyExists => write!(f, "already exists"),
            ErrorKind::AccountUnopened => write!(f, "account unopened"),
            ErrorKind::SandboxOnly => write!(f, "sandbox only - not allowed in prod"),
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, source: Option<eyre::Report>) -> Error {
        Error { kind, source }
    }

    pub fn internal(err: impl std::error::Error + Send + Sync + 'static) -> Error {
        Error {
            kind: ErrorKind::Internal,
            source: Some(eyre::Report::from(err)),
        }
    }

    pub fn internal_msg(msg: impl std::fmt::Display) -> Error {
        Error {
            kind: ErrorKind::Internal,
            source: Some(eyre!(format!("{}", msg))),
        }
    }

    pub fn upstream(err: impl std::error::Error + Send + Sync + 'static) -> Error {
        Error {
            kind: ErrorKind::Upstream,
            source: Some(eyre::Report::from(err)),
        }
    }

    pub fn not_found(msg: impl std::fmt::Display) -> Error {
        Error {
            kind: ErrorKind::NotFound,
            source: Some(eyre::anyhow!("{} not found", msg)),
        }
    }

    pub fn unauthorized() -> Error {
        Error {
            kind: ErrorKind::Unauthorized,
            source: None,
        }
    }

    pub fn insufficent_funds() -> Error {
        Error {
            kind: ErrorKind::InsufficentFunds,
            source: None,
        }
    }

    pub fn already_exists(msg: impl std::fmt::Display) -> Error {
        Error {
            kind: ErrorKind::AlreadyExists,
            source: Some(eyre::anyhow!("{} already exists", msg)),
        }
    }

    pub fn validation(field: impl Into<String>, msg: impl Into<String>) -> Error {
        ErrorKind::BadRequest {
            errors: vec![ValidationError {
                field: field.into(),
                msg: msg.into(),
            }],
        }
        .into()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error { kind, source: None }
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(err: E) -> Self {
        let source = eyre::Report::from(err);
        let kind = if let Some(status) = source.downcast_ref::<tonic::Status>() {
            match status.code() {
                tonic::Code::Ok => ErrorKind::Internal,
                tonic::Code::Cancelled => ErrorKind::Upstream,
                tonic::Code::Unknown => ErrorKind::Internal,
                tonic::Code::InvalidArgument => ErrorKind::Internal,
                tonic::Code::DeadlineExceeded => ErrorKind::Upstream,
                tonic::Code::NotFound => ErrorKind::NotFound,
                tonic::Code::AlreadyExists => ErrorKind::AlreadyExists,
                tonic::Code::PermissionDenied => ErrorKind::Unauthorized,
                tonic::Code::ResourceExhausted => ErrorKind::Internal,
                tonic::Code::FailedPrecondition => ErrorKind::Internal,
                tonic::Code::Aborted => ErrorKind::Upstream,
                tonic::Code::OutOfRange => ErrorKind::Internal,
                tonic::Code::Unimplemented => ErrorKind::Upstream,
                tonic::Code::Internal => ErrorKind::Upstream,
                tonic::Code::Unavailable => ErrorKind::Upstream,
                tonic::Code::DataLoss => ErrorKind::Upstream,
                tonic::Code::Unauthenticated => ErrorKind::Unauthorized,
            }
        } else if source.downcast_ref::<reqwest::Error>().is_some() {
            ErrorKind::Upstream
        } else {
            ErrorKind::Internal
        };
        Error {
            kind,
            source: Some(source),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is invalid - {}", self.field, self.msg)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(flatten)]
    kind: ErrorKind,
    msg: String,
}

impl ErrorResponse {
    fn new(err: &Error) -> Self {
        ErrorResponse {
            kind: err.kind.clone(),
            msg: format!("{}", err),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> hyper::StatusCode {
        match &self.kind {
            ErrorKind::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ErrorKind::Internal | ErrorKind::Upstream => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorKind::InsufficentFunds
            | ErrorKind::AlreadyExists
            | ErrorKind::AccountUnopened
            | ErrorKind::SandboxOnly => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let mut res = actix_web::HttpResponse::new(self.status_code());
        let resp = ErrorResponse::new(self);
        let buf = serde_json::to_vec(&resp).unwrap_or_else(|_| resp.msg.into_bytes());

        res.headers_mut().insert(
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::HeaderValue::from_static("application/json; charset=utf-8"),
        );

        res.set_body(actix_web::body::BoxBody::new(buf))
    }
}

pub trait ResultExt<T, E> {
    fn context<D>(self, kind: ErrorKind, msg: D) -> Result<T, Error>
    where
        D: std::fmt::Display + Send + Sync + 'static;

    fn internal_error<D>(self, msg: D) -> Result<T, Error>
    where
        D: std::fmt::Display + Send + Sync + 'static,
        Self: Sized,
    {
        self.context(ErrorKind::Internal, msg)
    }
}

impl<T> ResultExt<T, Error> for Result<T, Error> {
    fn context<D>(self, kind: ErrorKind, msg: D) -> Result<T, Error>
    where
        D: std::fmt::Display + Send + Sync + 'static,
    {
        match self {
            Ok(res) => Ok(res),
            Err(err) => Err(Error {
                kind,
                source: err.source.map(|err| err.wrap_err(msg)),
            }),
        }
    }
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
    T: 'static,
{
    fn context<D>(self, kind: ErrorKind, msg: D) -> Result<T, Error>
    where
        D: std::fmt::Display + Send + Sync + 'static,
    {
        match self {
            Ok(res) => Ok(res),
            Err(err) => Err(Error {
                kind,
                source: Some(eyre::Report::new(err).wrap_err(msg)),
            }),
        }
    }
}

impl From<Error> for eyre::Report {
    fn from(err: Error) -> eyre::Report {
        match err.source {
            Some(source) => source.wrap_err(err.kind),
            None => eyre!(err.kind),
        }
    }
}
