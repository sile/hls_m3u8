use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use crate::Error;

/// Non-negative decimal floating-point number.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct DecimalFloatingPoint(f64);

impl DecimalFloatingPoint {
    /// Makes a new `DecimalFloatingPoint` instance.
    ///
    /// # Errors
    ///
    /// The given value must have a positive sign and be finite,
    /// otherwise this function will return an error that has the kind `ErrorKind::InvalidInput`.
    pub fn new(n: f64) -> crate::Result<Self> {
        if n.is_sign_negative() || n.is_infinite() {
            return Err(Error::invalid_input());
        }
        Ok(Self(n))
    }

    /// Converts `DecimalFloatingPoint` to `f64`.
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    pub(crate) fn to_duration(self) -> Duration {
        Duration::from_secs_f64(self.0)
    }

    pub(crate) fn from_duration(value: Duration) -> Self {
        Self::from(value)
    }
}

impl From<u32> for DecimalFloatingPoint {
    fn from(f: u32) -> Self {
        Self(f64::from(f))
    }
}

impl From<Duration> for DecimalFloatingPoint {
    fn from(value: Duration) -> Self {
        Self(value.as_secs_f64())
    }
}

impl Eq for DecimalFloatingPoint {}

impl fmt::Display for DecimalFloatingPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for DecimalFloatingPoint {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if !input.chars().all(|c| c.is_digit(10) || c == '.') {
            return Err(Error::invalid_input());
        }
        Self::new(input.parse()?)
    }
}

impl From<f64> for DecimalFloatingPoint {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<f32> for DecimalFloatingPoint {
    fn from(value: f32) -> Self {
        Self(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_display() {
        let decimal_floating_point = DecimalFloatingPoint::new(22.0).unwrap();
        assert_eq!(decimal_floating_point.to_string(), "22".to_string());

        let decimal_floating_point = DecimalFloatingPoint::new(4.1).unwrap();
        assert_eq!(decimal_floating_point.to_string(), "4.1".to_string());
    }

    #[test]
    pub fn test_parser() {
        let decimal_floating_point = DecimalFloatingPoint::new(22.0).unwrap();
        assert_eq!(
            decimal_floating_point,
            "22".parse::<DecimalFloatingPoint>().unwrap()
        );

        let decimal_floating_point = DecimalFloatingPoint::new(4.1).unwrap();
        assert_eq!(
            decimal_floating_point,
            "4.1".parse::<DecimalFloatingPoint>().unwrap()
        );

        assert!("1#".parse::<DecimalFloatingPoint>().is_err());
    }

    #[test]
    fn test_new() {
        assert!(DecimalFloatingPoint::new(::std::f64::INFINITY).is_err());
        assert!(DecimalFloatingPoint::new(-1.0).is_err());
    }

    #[test]
    fn test_as_f64() {
        assert_eq!(DecimalFloatingPoint::new(1.0).unwrap().as_f64(), 1.0);
    }

    #[test]
    fn test_from_duration() {
        assert_eq!(
            DecimalFloatingPoint::from_duration(Duration::from_nanos(11_234_500_112_345)),
            DecimalFloatingPoint::new(11234.500112345).unwrap()
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(
            DecimalFloatingPoint::from(1u32),
            DecimalFloatingPoint::new(1.0).unwrap()
        );

        assert_eq!(
            DecimalFloatingPoint::from(1 as f64),
            DecimalFloatingPoint::new(1.0).unwrap()
        );

        assert_eq!(
            DecimalFloatingPoint::from(1 as f32),
            DecimalFloatingPoint::new(1.0).unwrap()
        );
    }
}
