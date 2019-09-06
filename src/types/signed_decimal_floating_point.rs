use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::{self, FromStr};
use trackable::error::ErrorKindExt;

/// Signed decimal floating-point number.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SignedDecimalFloatingPoint(f64);

impl SignedDecimalFloatingPoint {
    /// Makes a new `SignedDecimalFloatingPoint` instance.
    ///
    /// # Errors
    ///
    /// The given value must be finite,
    /// otherwise this function will return an error that has the kind `ErrorKind::InvalidInput`.
    pub fn new(n: f64) -> Result<Self> {
        track_assert!(n.is_finite(), ErrorKind::InvalidInput);
        Ok(SignedDecimalFloatingPoint(n))
    }

    /// Converts `DecimalFloatingPoint` to `f64`.
    pub fn as_f64(self) -> f64 {
        self.0
    }
}

impl From<i32> for SignedDecimalFloatingPoint {
    fn from(f: i32) -> Self {
        SignedDecimalFloatingPoint(f64::from(f))
    }
}

impl Eq for SignedDecimalFloatingPoint {}

impl fmt::Display for SignedDecimalFloatingPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for SignedDecimalFloatingPoint {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(
            s.chars().all(|c| c.is_digit(10) || c == '.' || c == '-'),
            ErrorKind::InvalidInput
        );
        let n = track!(s.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
        Ok(SignedDecimalFloatingPoint(n))
    }
}
