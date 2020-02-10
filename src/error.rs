use std::fmt;

use thiserror::Error;
//use crate::types::ProtocolVersion;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Clone, PartialEq)]
#[non_exhaustive]
enum ErrorKind {
    #[error("a value is missing for the attribute {}", _0)]
    MissingValue(String),

    #[error("invalid input")]
    InvalidInput,

    #[error("{}", _0)]
    ParseIntError(::std::num::ParseIntError),

    #[error("{}", _0)]
    ParseFloatError(::std::num::ParseFloatError),

    #[error("expected `{}` at the start of {:?}", tag, input)]
    MissingTag {
        /// The required tag.
        tag: String,
        /// The unparsed input data.
        input: String,
    },

    #[error("{}", _0)]
    Custom(String),

    #[error("unmatched group: {:?}", _0)]
    UnmatchedGroup(String),

    #[error("unknown protocol version {:?}", _0)]
    UnknownProtocolVersion(String),

    // #[error("required_version: {:?}, specified_version: {:?}", _0, _1)]
    // VersionError(ProtocolVersion, ProtocolVersion),
    #[error("missing attribute: {}", _0)]
    MissingAttribute(String),

    #[error("unexpected attribute: {:?}", _0)]
    UnexpectedAttribute(String),

    #[error("unexpected tag: {:?}", _0)]
    UnexpectedTag(String),

    #[error("{}", _0)]
    ChronoParseError(chrono::ParseError),

    #[error("builder error: {}", _0)]
    Builder(String),

    #[doc(hidden)]
    #[error("{}", _0)]
    Hex(hex::FromHexError),
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

#[allow(clippy::needless_pass_by_value)]
impl Error {
    const fn new(inner: ErrorKind) -> Self { Self { inner } }

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

    pub(crate) const fn invalid_input() -> Self { Self::new(ErrorKind::InvalidInput) }

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
