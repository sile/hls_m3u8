use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::ops::Deref;
use std::str::{self, FromStr};

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
    /// # Errors
    ///
    /// If the given string contains any control characters or double-quote character,
    /// this function will return an error which has the kind `ErrorKind::InvalidInput`.
    pub fn new<T: Into<String>>(s: T) -> Result<Self> {
        let s = s.into();
        track_assert!(
            !s.chars().any(|c| c.is_control() || c == '"'),
            ErrorKind::InvalidInput
        );
        Ok(QuotedString(s))
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
        write!(f, "{:?}", self.0)
    }
}

impl FromStr for QuotedString {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let len = s.len();
        let bytes = s.as_bytes();
        track_assert!(len >= 2, ErrorKind::InvalidInput);
        track_assert_eq!(bytes[0], b'"', ErrorKind::InvalidInput);
        track_assert_eq!(bytes[len - 1], b'"', ErrorKind::InvalidInput);

        let s = unsafe { str::from_utf8_unchecked(&bytes[1..len - 1]) };
        track!(QuotedString::new(s))
    }
}
