use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;

use crate::types::Float;
use crate::utils::{quote, unquote};
use crate::Error;

/// A `Value`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Value<'a> {
    /// A `String`.
    String(Cow<'a, str>),
    /// A sequence of bytes.
    Hex(Vec<u8>),
    /// A floating point number, that's neither NaN nor infinite.
    Float(Float),
}

impl<'a> Value<'a> {
    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> Value<'static> {
        match self {
            Self::String(value) => Value::String(Cow::Owned(value.into_owned())),
            Self::Hex(value) => Value::Hex(value),
            Self::Float(value) => Value::Float(value),
        }
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::String(value) => write!(f, "{}", quote(value)),
            Self::Hex(value) => write!(f, "0x{}", hex::encode_upper(value)),
            Self::Float(value) => write!(f, "{}", value),
        }
    }
}

impl<'a> TryFrom<&'a str> for Value<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        if input.starts_with("0x") || input.starts_with("0X") {
            Ok(Self::Hex(
                hex::decode(input.trim_start_matches("0x").trim_start_matches("0X"))
                    .map_err(Error::hex)?,
            ))
        } else {
            match input.parse() {
                Ok(value) => Ok(Self::Float(value)),
                Err(_) => Ok(Self::String(unquote(input))),
            }
        }
    }
}

impl<T: Into<Float>> From<T> for Value<'static> {
    fn from(value: T) -> Self {
        Self::Float(value.into())
    }
}

impl From<Vec<u8>> for Value<'static> {
    fn from(value: Vec<u8>) -> Self {
        Self::Hex(value)
    }
}

impl From<String> for Value<'static> {
    fn from(value: String) -> Self {
        Self::String(Cow::Owned(unquote(&value).into_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(Value::Float(Float::new(1.1)).to_string(), "1.1".to_string());
        assert_eq!(
            Value::String("&str".into()).to_string(),
            "\"&str\"".to_string()
        );
        assert_eq!(
            Value::Hex(vec![1, 2, 3]).to_string(),
            "0x010203".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            Value::Float(Float::new(1.1)),
            Value::try_from("1.1").unwrap()
        );
        assert_eq!(
            Value::String("&str".into()),
            Value::try_from("\"&str\"").unwrap()
        );
        assert_eq!(
            Value::Hex(vec![1, 2, 3]),
            Value::try_from("0x010203").unwrap()
        );
        assert_eq!(
            Value::Hex(vec![1, 2, 3]),
            Value::try_from("0X010203").unwrap()
        );
        assert!(Value::try_from("0x010203Z").is_err());
    }

    #[test]
    fn test_from() {
        assert_eq!(Value::from(1_u8), Value::Float(Float::new(1.0)));
        assert_eq!(
            Value::from("&str".to_string()),
            Value::String("&str".into())
        );
        assert_eq!(Value::from(vec![1, 2, 3]), Value::Hex(vec![1, 2, 3]));
    }
}
