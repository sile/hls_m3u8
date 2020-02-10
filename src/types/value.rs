use std::fmt;
use std::str::FromStr;

use hex;

use crate::utils::{quote, unquote};
use crate::Error;

/// A [`Value`].
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    /// A [`String`].
    String(String),
    /// A sequence of bytes.
    Hex(Vec<u8>),
    /// A floating point number, that's neither NaN nor infinite!
    Float(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::String(value) => write!(f, "{}", quote(value)),
            Self::Hex(value) => write!(f, "0x{}", hex::encode_upper(value)),
            Self::Float(value) => write!(f, "{}", value),
        }
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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

impl From<f64> for Value {
    fn from(value: f64) -> Self { Self::Float(value) }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self { Self::Hex(value) }
}

impl From<String> for Value {
    fn from(value: String) -> Self { Self::String(unquote(value)) }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self { Self::String(unquote(value)) }
}

// impl<T: AsRef<[u8]>> From<T> for Value {
//     fn from(value: T) -> Self { Self::Hex(value.as_ref().into()) }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(Value::Float(1.1).to_string(), "1.1".to_string());
        assert_eq!(
            Value::String("&str".to_string()).to_string(),
            "\"&str\"".to_string()
        );
        assert_eq!(
            Value::Hex(vec![1, 2, 3]).to_string(),
            "0x010203".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(Value::Float(1.1), "1.1".parse().unwrap());
        assert_eq!(
            Value::String("&str".to_string()),
            "\"&str\"".parse().unwrap()
        );
        assert_eq!(Value::Hex(vec![1, 2, 3]), "0x010203".parse().unwrap());
        assert_eq!(Value::Hex(vec![1, 2, 3]), "0X010203".parse().unwrap());
        assert!("0x010203Z".parse::<Value>().is_err());
    }

    #[test]
    fn test_from() {
        assert_eq!(Value::from(1.0_f64), Value::Float(1.0));
        assert_eq!(Value::from("\"&str\""), Value::String("&str".to_string()));
        assert_eq!(
            Value::from("&str".to_string()),
            Value::String("&str".to_string())
        );
        assert_eq!(Value::from(vec![1, 2, 3]), Value::Hex(vec![1, 2, 3]));
    }
}
