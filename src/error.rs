use std::fmt;

use thiserror::Error;

use crate::types::ProtocolVersion;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

/// The [`ErrorKind`].
#[derive(Debug, Error, Clone, PartialEq)]
enum ErrorKind {
    /// A required value is missing.
    #[error("A value is missing for the attribute {}", _0)]
    MissingValue(String),

    /// Error for anything.
    #[error("Invalid Input")]
    InvalidInput,

    #[error("{}", _0)]
    /// Failed to parse a String to int.
    ParseIntError(::std::num::ParseIntError),

    #[error("{}", _0)]
    /// Failed to parse a String to float.
    ParseFloatError(::std::num::ParseFloatError),

    /// A tag is missing, that is required at the start of the input.
    #[error("Expected `{}` at the start of {:?}", tag, input)]
    MissingTag {
        /// The required tag.
        tag: String,
        /// The unparsed input data.
        input: String,
    },

    #[error("{}", _0)]
    /// A custom error.
    Custom(String),

    /// Unmatched Group
    #[error("Unmatched Group: {:?}", _0)]
    UnmatchedGroup(String),

    /// Unknown m3u8 version. This library supports up to ProtocolVersion 7.
    #[error("Unknown protocol version {:?}", _0)]
    UnknownProtocolVersion(String),

    /// Some io error
    #[error("{}", _0)]
    Io(String),

    /// This error occurs, if there is a ProtocolVersion mismatch.
    #[error("required_version: {:?}, specified_version: {:?}", _0, _1)]
    VersionError(ProtocolVersion, ProtocolVersion),

    /// An attribute is missing.
    #[error("Missing Attribute: {}", _0)]
    MissingAttribute(String),

    /// An unexpected value.
    #[error("Unexpected Attribute: {:?}", _0)]
    UnexpectedAttribute(String),

    /// An unexpected tag.
    #[error("Unexpected Tag: {:?}", _0)]
    UnexpectedTag(String),

    /// An error from the [`chrono`] crate.
    #[error("{}", _0)]
    ChronoParseError(chrono::ParseError),

    /// An error from a Builder.
    #[error("BuilderError: {}", _0)]
    Builder(String),

    #[error("{}", _0)]
    Hex(hex::FromHexError),

    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    #[error("Invalid error")]
    __Nonexhaustive,
}

/// The Error type of this library.
#[derive(Debug)]
pub struct Error {
    inner: ErrorKind,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.inner.fmt(f) }
}

impl Error {
    fn new(inner: ErrorKind) -> Self { Self { inner } }

    pub(crate) fn custom<T: fmt::Display>(value: T) -> Self {
        Self::new(ErrorKind::Custom(value.to_string()))
    }

    pub(crate) fn missing_value<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::MissingValue(value.to_string()))
    }

    pub(crate) fn unexpected_attribute<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::UnexpectedAttribute(value.to_string()))
    }

    pub(crate) fn unexpected_tag<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::UnexpectedTag(value.to_string()))
    }

    pub(crate) fn invalid_input() -> Self { Self::new(ErrorKind::InvalidInput) }

    pub(crate) fn parse_int(value: ::std::num::ParseIntError) -> Self {
        Self::new(ErrorKind::ParseIntError(value))
    }

    pub(crate) fn parse_float(value: ::std::num::ParseFloatError) -> Self {
        Self::new(ErrorKind::ParseFloatError(value))
    }

    pub(crate) fn missing_tag<T, U>(tag: T, input: U) -> Self
    where
        T: ToString,
        U: ToString,
    {
        Self::new(ErrorKind::MissingTag {
            tag: tag.to_string(),
            input: input.to_string(),
        })
    }

    pub(crate) fn unmatched_group<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::UnmatchedGroup(value.to_string()))
    }

    pub(crate) fn unknown_protocol_version<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::UnknownProtocolVersion(value.to_string()))
    }

    pub(crate) fn io<T: ToString>(value: T) -> Self { Self::new(ErrorKind::Io(value.to_string())) }

    pub(crate) fn builder<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::Builder(value.to_string()))
    }

    pub(crate) fn missing_attribute<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::MissingAttribute(value.to_string()))
    }

    // third party crates:
    pub(crate) fn chrono(value: chrono::format::ParseError) -> Self {
        Self::new(ErrorKind::ChronoParseError(value))
    }

    pub(crate) fn hex(value: hex::FromHexError) -> Self { Self::new(ErrorKind::Hex(value)) }

    pub(crate) fn strum(value: strum::ParseError) -> Self {
        Self::new(ErrorKind::Custom(value.to_string()))
    }
}

#[doc(hidden)]
impl From<::std::num::ParseIntError> for Error {
    fn from(value: ::std::num::ParseIntError) -> Self { Self::parse_int(value) }
}

#[doc(hidden)]
impl From<::std::num::ParseFloatError> for Error {
    fn from(value: ::std::num::ParseFloatError) -> Self { Self::parse_float(value) }
}

#[doc(hidden)]
impl From<::strum::ParseError> for Error {
    fn from(value: ::strum::ParseError) -> Self { Self::strum(value) }
}
