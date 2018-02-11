use trackable::error::{ErrorKind as TrackableErrorKind, TrackableError};

#[derive(Debug, Clone)]
pub struct Error(TrackableError<ErrorKind>);
derive_traits_for_trackable_error_newtype!(Error, ErrorKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidInput,
    Other,
}
impl TrackableErrorKind for ErrorKind {}
