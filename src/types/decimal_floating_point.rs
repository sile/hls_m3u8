use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use crate::error::{Error, ErrorKind};
use crate::trackable::error::ErrorKindExt as _;

/// Non-negative decimal floating-point number.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct DecimalFloatingPoint(f64);

#[allow(dead_code)]
impl DecimalFloatingPoint {
    /// Makes a new `DecimalFloatingPoint` instance.
    ///
    /// # Errors
    ///
    /// The given value must have a positive sign and be finite,
    /// otherwise this function will return an error that has the kind `ErrorKind::InvalidInput`.
    pub fn new(n: f64) -> crate::Result<Self> {
        track_assert!(n.is_sign_positive(), ErrorKind::InvalidInput);
        track_assert!(n.is_finite(), ErrorKind::InvalidInput);
        Ok(DecimalFloatingPoint(n))
    }

    /// Converts `DecimalFloatingPoint` to `f64`.
    pub fn as_f64(self) -> f64 {
        self.0
    }

    // TODO: this should be default? Duration > DecimalFloatingPoint
    pub(crate) fn to_duration(self) -> Duration {
        let secs = self.0 as u64;
        let nanos = (self.0.fract() * 1_000_000_000.0) as u32;
        Duration::new(secs, nanos)
    }

    pub(crate) fn from_duration(duration: Duration) -> Self {
        let n =
            (duration.as_secs() as f64) + (f64::from(duration.subsec_nanos()) / 1_000_000_000.0);
        DecimalFloatingPoint(n)
    }
}

impl From<u32> for DecimalFloatingPoint {
    fn from(f: u32) -> Self {
        DecimalFloatingPoint(f64::from(f))
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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        track_assert!(
            s.chars().all(|c| c.is_digit(10) || c == '.'),
            ErrorKind::InvalidInput
        );
        let n = track!(s.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
        Ok(DecimalFloatingPoint(n))
    }
}
