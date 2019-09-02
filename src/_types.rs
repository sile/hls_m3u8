//! Miscellaneous types.
use crate::attribute::AttributePairs;
use crate::{Error, ErrorKind};
use std::fmt;
use std::ops::Deref;
use std::str::{self, FromStr};
use std::time::Duration;
use trackable::error::ErrorKindExt


/// Quoted string.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuotedString(String);

impl QuotedString {
    /// Makes a new `QuotedString` instance.
    ///
    /// # Note
    /// This function will silently remove the following characters, which must not appear in a
    /// quoted-string:
    ///
    /// - line feed (`"\n"`)
    /// - carriage return (`"\r"`),
    /// - double quote (`"`)
    ///
    /// [Reference](https://tools.ietf.org/html/rfc8216#section-4.2)
    pub fn new<T: ToString>(value: T) -> Self {
        let result = format!(
            "\"{}\"",
            value
                .to_string()
                // silently remove forbidden characters
                .replace("\n", "")
                .replace("\r", "")
                .replace("\"", "")
        );
        Self(result)
    }

    /// Converts a `QuotedString` to a `String` (removes the quotes).
    pub fn unquote(&self) -> String {
        self.0.clone().replace("\"", "")
    }
}

impl Deref for QuotedString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for QuotedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for QuotedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for QuotedString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true)
    }
}
