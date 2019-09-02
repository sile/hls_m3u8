use trackable::error::{ErrorKind as TrackableErrorKind, TrackableError};

/// This crate specific `Error` type.
#[derive(Debug, Clone, TrackableError)]
pub struct Error(TrackableError<ErrorKind>);

/// Possible error kinds.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ErrorKind {
    InvalidInput,
    BuilderError(String),
}

impl TrackableErrorKind for ErrorKind {}
