use crate::{ErrorKind, Result};
use std::fmt;
use std::ops::Deref;

/// String that represents a single line in a playlist file.
///
/// See: [4.1. Definition of a Playlist]
///
/// [4.1. Definition of a Playlist]: https://tools.ietf.org/html/rfc8216#section-4.1
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SingleLineString(String);

impl SingleLineString {
    /// Makes a new `SingleLineString` instance.
    ///
    /// # Errors
    ///
    /// If the given string contains any control characters,
    /// this function will return an error which has the kind `ErrorKind::InvalidInput`.
    pub fn new<T: Into<String>>(s: T) -> Result<Self> {
        let s = s.into();
        track_assert!(!s.chars().any(|c| c.is_control()), ErrorKind::InvalidInput);
        Ok(SingleLineString(s))
    }
}

impl Deref for SingleLineString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for SingleLineString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SingleLineString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_line_string() {
        assert!(SingleLineString::new("foo").is_ok());
        assert!(SingleLineString::new("b\rar").is_err());
    }
}
