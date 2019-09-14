use std::error;
use std::fmt;

use failure::{Backtrace, Context, Fail};

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail, Clone)]
pub enum AttributeError {
    #[fail(display = "The attribute has an invalid name; {:?}", _0)]
    InvalidAttribute(String),
    #[fail(display = "A value is missing for the attribute: {}", _0)]
    MissingValue(String),
}

#[derive(Debug, Fail, Clone)]
pub enum ErrorKind {
    #[fail(display = "AttributeError: {}", _0)]
    AttributeError(AttributeError),

    #[fail(display = "UnknownError: {}", _0)]
    UnknownError(String),

    #[fail(display = "A value is missing for the attribute {}", _0)]
    MissingValue(String),

    #[fail(display = "Invalid Input")]
    InvalidInput,

    #[fail(display = "ParseIntError: {}", _0)]
    ParseIntError(String),

    #[fail(display = "ParseFloatError: {}", _0)]
    ParseFloatError(String),

    #[fail(display = "MissingTag: Expected {} at the start of {:?}", tag, input)]
    MissingTag { tag: String, input: String },

    #[fail(display = "CustomError: {}", _0)]
    Custom(String),

    #[fail(display = "Unmatched Group: {:?}", _0)]
    UnmatchedGroup(String),

    #[fail(display = "Unknown Protocol version: {:?}", _0)]
    UnknownProtocolVersion(String),

    #[fail(display = "IoError: {}", _0)]
    Io(String),

    #[fail(
        display = "VersionError: required_version: {:?}, specified_version: {:?}",
        _0, _1
    )]
    VersionError(String, String),

    #[fail(display = "BuilderError: {}", _0)]
    BuilderError(String),

    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    #[fail(display = "Invalid error")]
    __Nonexhaustive,
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

macro_rules! from_error {
    ( $( $f:tt ),* ) => {
        $(
            impl From<$f> for ErrorKind {
                fn from(value: $f) -> Self {
                    Self::$f(value)
                }
            }
        )*
    }
}

from_error!(AttributeError);

impl Error {
    pub(crate) fn invalid_attribute<T: ToString>(value: T) -> Self {
        Self::from(ErrorKind::from(AttributeError::InvalidAttribute(
            value.to_string(),
        )))
    }

    pub(crate) fn missing_attribute_value<T: ToString>(value: T) -> Self {
        Self::from(ErrorKind::from(AttributeError::MissingValue(
            value.to_string(),
        )))
    }

    pub(crate) fn unknown<T>(value: T) -> Self
    where
        T: error::Error,
    {
        Self::from(ErrorKind::UnknownError(value.to_string()))
    }

    pub(crate) fn missing_value<T: ToString>(value: T) -> Self {
        Self::from(ErrorKind::MissingValue(value.to_string()))
    }

    pub(crate) fn invalid_input() -> Self {
        Self::from(ErrorKind::InvalidInput)
    }

    pub(crate) fn parse_int_error<T: ToString>(value: T) -> Self {
        Self::from(ErrorKind::ParseIntError(value.to_string()))
    }

    pub(crate) fn parse_float_error<T: ToString>(value: T) -> Self {
        Self::from(ErrorKind::ParseFloatError(value.to_string()))
    }

    pub(crate) fn missing_tag<T, U>(tag: T, input: U) -> Self
    where
        T: ToString,
        U: ToString,
    {
        Self::from(ErrorKind::MissingTag {
            tag: tag.to_string(),
            input: input.to_string(),
        })
    }

    pub(crate) fn unmatched_group<T: ToString>(value: T) -> Self {
        Self::from(ErrorKind::UnmatchedGroup(value.to_string()))
    }

    pub(crate) fn custom<T>(value: T) -> Self
    where
        T: fmt::Display,
    {
        Self::from(ErrorKind::Custom(value.to_string()))
    }

    pub(crate) fn unknown_protocol_version<T: ToString>(value: T) -> Self {
        Self::from(ErrorKind::UnknownProtocolVersion(value.to_string()))
    }

    pub(crate) fn io<T: ToString>(value: T) -> Self {
        Self::from(ErrorKind::Io(value.to_string()))
    }

    pub(crate) fn required_version<T, U>(required_version: T, specified_version: U) -> Self
    where
        T: ToString,
        U: ToString,
    {
        Self::from(ErrorKind::VersionError(
            required_version.to_string(),
            specified_version.to_string(),
        ))
    }

    pub(crate) fn builder_error<T: ToString>(value: T) -> Self {
        Self::from(ErrorKind::BuilderError(value.to_string()))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: ::std::num::ParseIntError) -> Self {
        Error::parse_int_error(value)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(value: ::std::num::ParseFloatError) -> Self {
        Error::parse_float_error(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: ::std::io::Error) -> Self {
        Error::io(value)
    }
}
