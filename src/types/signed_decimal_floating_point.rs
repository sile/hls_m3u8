use std::fmt;
use std::str::FromStr;

use crate::Error;

/// Signed decimal floating-point number.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct SignedDecimalFloatingPoint(f64);

impl SignedDecimalFloatingPoint {
    /// Makes a new [SignedDecimalFloatingPoint] instance.
    ///
    /// # Panics
    /// The given value must be finite, otherwise this function will panic!
    pub fn new(n: f64) -> Self {
        if n.is_infinite() {
            panic!("Floating point value must be finite!");
        }
        Self(n)
    }

    /// Converts [DecimalFloatingPoint] to [f64].
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

impl From<i32> for SignedDecimalFloatingPoint {
    fn from(f: i32) -> Self {
        Self(f64::from(f))
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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(input.parse().map_err(Error::parse_float_error)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            SignedDecimalFloatingPoint::new(1.0).to_string(),
            1.0f64.to_string()
        );
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        SignedDecimalFloatingPoint::new(::std::f64::INFINITY);
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            SignedDecimalFloatingPoint::new(1.0),
            "1.0".parse::<SignedDecimalFloatingPoint>().unwrap()
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(
            SignedDecimalFloatingPoint::from(1i32),
            SignedDecimalFloatingPoint::new(1.0)
        );
    }
}
