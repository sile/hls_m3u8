use std::fmt;

#[cfg(feature = "backtrace")]
use backtrace::Backtrace;
use thiserror::Error;

//use crate::types::ProtocolVersion;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Clone, PartialEq)]
#[non_exhaustive]
enum ErrorKind {
    #[error("a value is missing for the attribute {value}")]
    MissingValue { value: String },

    #[error("invalid input")]
    InvalidInput,

    #[error("{source}: {input:?}")]
    ParseIntError {
        input: String,
        source: ::std::num::ParseIntError,
    },

    #[error("{source}: {input:?}")]
    ParseFloatError {
        input: String,
        source: ::std::num::ParseFloatError,
    },

    #[error("expected `{tag}` at the start of {input:?}")]
    MissingTag {
        /// The required tag.
        tag: String,
        /// The unparsed input data.
        input: String,
    },

    #[error("{0}")]
    Custom(String),

    #[error("unmatched group: {0:?}")]
    UnmatchedGroup(String),

    #[error("unknown protocol version {0:?}")]
    UnknownProtocolVersion(String),

    // #[error("required_version: {:?}, specified_version: {:?}", _0, _1)]
    // VersionError(ProtocolVersion, ProtocolVersion),
    #[error("missing attribute: {attribute:?}")]
    MissingAttribute { attribute: String },

    #[error("unexpected attribute: {attribute:?}")]
    UnexpectedAttribute { attribute: String },

    #[error("unexpected tag: {tag:?}")]
    UnexpectedTag { tag: String },

    #[error("{source}")]
    #[cfg(feature = "chrono")]
    Chrono { source: chrono::ParseError },

    #[error("builder error: {message}")]
    Builder { message: String },

    #[error("{source}")]
    Hex { source: hex::FromHexError },
}

/// The Error type of this library.
#[derive(Debug)]
pub struct Error {
    inner: ErrorKind,
    #[cfg(feature = "backtrace")]
    backtrace: Backtrace,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool { self.inner == other.inner }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.inner.fmt(f) }
}

#[allow(clippy::needless_pass_by_value)]
impl Error {
    fn new(inner: ErrorKind) -> Self {
        Self {
            inner,
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn custom<T: fmt::Display>(value: T) -> Self {
        Self::new(ErrorKind::Custom(value.to_string()))
    }

    pub(crate) fn missing_value<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::MissingValue {
            value: value.to_string(),
        })
    }

    pub(crate) fn missing_field<T: fmt::Display, D: fmt::Display>(strct: D, field: T) -> Self {
        Self::new(ErrorKind::Custom(format!(
            "the field `{}` is missing for `{}`",
            field, strct
        )))
    }

    pub(crate) fn unexpected_attribute<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::UnexpectedAttribute {
            attribute: value.to_string(),
        })
    }

    pub(crate) fn unexpected_tag<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::UnexpectedTag {
            tag: value.to_string(),
        })
    }

    pub(crate) fn invalid_input() -> Self { Self::new(ErrorKind::InvalidInput) }

    pub(crate) fn parse_int<T: fmt::Display>(input: T, source: ::std::num::ParseIntError) -> Self {
        Self::new(ErrorKind::ParseIntError {
            input: input.to_string(),
            source,
        })
    }

    pub(crate) fn parse_float<T: fmt::Display>(
        input: T,
        source: ::std::num::ParseFloatError,
    ) -> Self {
        Self::new(ErrorKind::ParseFloatError {
            input: input.to_string(),
            source,
        })
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
        Self::new(ErrorKind::Builder {
            message: value.to_string(),
        })
    }

    pub(crate) fn missing_attribute<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::MissingAttribute {
            attribute: value.to_string(),
        })
    }

    // third party crates:
    #[cfg(feature = "chrono")]
    pub(crate) fn chrono(source: chrono::format::ParseError) -> Self {
        Self::new(ErrorKind::Chrono { source })
    }

    pub(crate) fn hex(source: hex::FromHexError) -> Self {
        //
        Self::new(ErrorKind::Hex { source })
    }

    pub(crate) fn strum(value: strum::ParseError) -> Self {
        Self::new(ErrorKind::Custom(value.to_string()))
    }
}

#[doc(hidden)]
impl From<::strum::ParseError> for Error {
    fn from(value: ::strum::ParseError) -> Self { Self::strum(value) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_float_error() {
        assert_eq!(
            Error::parse_float(
                "1.x234",
                "1.x234"
                    .parse::<f32>()
                    .expect_err("this should not parse as a float!")
            )
            .to_string(),
            "invalid float literal: \"1.x234\"".to_string()
        );
    }

    #[test]
    fn test_parse_int_error() {
        assert_eq!(
            Error::parse_int(
                "1x",
                "1x".parse::<usize>()
                    .expect_err("this should not parse as an usize!")
            )
            .to_string(),
            "invalid digit found in string: \"1x\"".to_string()
        );
    }
}
