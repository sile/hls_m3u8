use std::fmt;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
enum ErrorKind {
    MissingValue {
        value: String,
    },
    InvalidInput,
    ParseIntError {
        input: String,
        source: std::num::ParseIntError,
    },
    ParseFloatError {
        input: String,
        source: std::num::ParseFloatError,
    },
    MissingTag {
        tag: String,
        input: String,
    },
    Custom(String),
    UnmatchedGroup(String),
    UnknownProtocolVersion(String),
    MissingAttribute {
        attribute: String,
    },
    UnexpectedAttribute {
        attribute: String,
    },
    UnexpectedTag {
        tag: String,
    },
    #[cfg(feature = "chrono")]
    Chrono {
        source: chrono::ParseError,
    },
    InvalidHex(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingValue { value } => {
                write!(f, "a value is missing for the attribute {value}")
            }
            Self::InvalidInput => write!(f, "invalid input"),
            Self::ParseIntError { input, source } => write!(f, "{source}: {input:?}"),
            Self::ParseFloatError { input, source } => write!(f, "{source}: {input:?}"),
            Self::MissingTag { tag, input } => {
                write!(f, "expected `{tag}` at the start of {input:?}")
            }
            Self::Custom(message) => write!(f, "{message}"),
            Self::UnmatchedGroup(group) => write!(f, "unmatched group: {group:?}"),
            Self::UnknownProtocolVersion(version) => {
                write!(f, "unknown protocol version {version:?}")
            }
            Self::MissingAttribute { attribute } => write!(f, "missing attribute: {attribute:?}"),
            Self::UnexpectedAttribute { attribute } => {
                write!(f, "unexpected attribute: {attribute:?}")
            }
            Self::UnexpectedTag { tag } => write!(f, "unexpected tag: {tag:?}"),
            #[cfg(feature = "chrono")]
            Self::Chrono { source } => write!(f, "{source}"),
            Self::InvalidHex(reason) => write!(f, "invalid hex: {reason}"),
        }
    }
}

impl std::error::Error for ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ParseIntError { source, .. } => Some(source),
            Self::ParseFloatError { source, .. } => Some(source),
            #[cfg(feature = "chrono")]
            Self::Chrono { source } => Some(source),
            _ => None,
        }
    }
}

/// The Error type of this library.
#[derive(Debug)]
pub struct Error {
    inner: ErrorKind,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Error {
    fn new(inner: ErrorKind) -> Self {
        Self { inner }
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

    pub(crate) fn invalid_input() -> Self {
        Self::new(ErrorKind::InvalidInput)
    }

    pub(crate) fn parse_int<T: fmt::Display>(input: T, source: std::num::ParseIntError) -> Self {
        Self::new(ErrorKind::ParseIntError {
            input: input.to_string(),
            source,
        })
    }

    pub(crate) fn parse_float<T: fmt::Display>(
        input: T,
        source: std::num::ParseFloatError,
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

    pub(crate) fn missing_attribute<T: ToString>(value: T) -> Self {
        Self::new(ErrorKind::MissingAttribute {
            attribute: value.to_string(),
        })
    }

    pub(crate) fn unexpected_data(value: &str) -> Self {
        Self::custom(format!("Unexpected data in the line: {:?}", value))
    }

    // third party crates:
    #[cfg(feature = "chrono")]
    pub(crate) fn chrono(source: chrono::format::ParseError) -> Self {
        Self::new(ErrorKind::Chrono { source })
    }

    pub(crate) fn invalid_hex<T: fmt::Display>(reason: T) -> Self {
        Self::new(ErrorKind::InvalidHex(reason.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
