use std::error;
use std::fmt;

use failure::{Backtrace, Context, Fail};

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

/// The ErrorKind.
#[derive(Debug, Fail, Clone)]
pub enum ErrorKind {
    #[fail(display = "UnknownError: {}", _0)]
    /// An unknown error occured.
    UnknownError(String),

    #[fail(display = "A value is missing for the attribute {}", _0)]
    /// A required value is missing.
    MissingValue(String),

    #[fail(display = "Invalid Input")]
    /// Error for anything.
    InvalidInput,

    #[fail(display = "ParseIntError: {}", _0)]
    /// Failed to parse a String to int.
    ParseIntError(String),

    #[fail(display = "ParseFloatError: {}", _0)]
    /// Failed to parse a String to float.
    ParseFloatError(String),

    #[fail(display = "MissingTag: Expected {} at the start of {:?}", tag, input)]
    /// A tag is missing, that is required at the start of the input.
    MissingTag {
        /// The required tag.
        tag: String,
        /// The unparsed input data.
        input: String,
    },

    #[fail(display = "CustomError: {}", _0)]
    /// A custom error.
    Custom(String),

    #[fail(display = "Unmatched Group: {:?}", _0)]
    /// Unmatched Group
    UnmatchedGroup(String),

    #[fail(display = "Unknown Protocol version: {:?}", _0)]
    /// Unknown m3u8 version. This library supports up to ProtocolVersion 7.
    UnknownProtocolVersion(String),

    #[fail(display = "IoError: {}", _0)]
    /// Some io error
    Io(String),

    #[fail(
        display = "VersionError: required_version: {:?}, specified_version: {:?}",
        _0, _1
    )]
    /// This error occurs, if there is a ProtocolVersion mismatch.
    VersionError(String, String),

    #[fail(display = "BuilderError: {}", _0)]
    /// An Error from a Builder.
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
/// The Error type of this library.
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

impl Error {
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

impl From<::std::num::ParseIntError> for Error {
    fn from(value: ::std::num::ParseIntError) -> Self {
        Error::parse_int_error(value)
    }
}

impl From<::std::num::ParseFloatError> for Error {
    fn from(value: ::std::num::ParseFloatError) -> Self {
        Error::parse_float_error(value)
    }
}

impl From<::std::io::Error> for Error {
    fn from(value: ::std::io::Error) -> Self {
        Error::io(value)
    }
}
