use core::str::FromStr;

use derive_more::{Deref, Display};

use crate::Error;

/// Non-negative decimal floating-point number.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]:
/// https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-05#section-4.2
#[derive(Deref, Default, Debug, Clone, Copy, PartialEq, PartialOrd, Display)]
pub(crate) struct DecimalFloatingPoint(f64);

impl DecimalFloatingPoint {
    /// Makes a new [`DecimalFloatingPoint`] instance.
    ///
    /// # Errors
    ///
    /// The given value must have a positive sign and be finite,
    /// otherwise this function will return an error that has the kind
    /// `ErrorKind::InvalidInput`.
    pub fn new(value: f64) -> crate::Result<Self> {
        if value.is_sign_negative() || value.is_infinite() || value.is_nan() {
            return Err(Error::invalid_input());
        }
        Ok(Self(value))
    }

    pub(crate) const fn from_f64_unchecked(value: f64) -> Self { Self(value) }

    /// Converts [`DecimalFloatingPoint`] to [`f64`].
    pub const fn as_f64(self) -> f64 { self.0 }
}

// this trait is implemented manually, so it doesn't construct a
// [`DecimalFloatingPoint`], with a negative value.
impl FromStr for DecimalFloatingPoint {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input.parse().map_err(Error::parse_float)?)
    }
}

#[doc(hidden)]
impl From<f64> for DecimalFloatingPoint {
    fn from(value: f64) -> Self {
        let mut result = value;

        // guard against the unlikely case of an infinite value...
        if result.is_infinite() || result.is_nan() {
            result = 0.0;
        }

        Self(result.abs())
    }
}

#[doc(hidden)]
impl From<f32> for DecimalFloatingPoint {
    fn from(value: f32) -> Self { f64::from(value).into() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! test_from {
        ( $($input:expr),* ) => {
            use ::core::convert::From;

            #[test]
            fn test_from() {
                $(
                    assert_eq!(
                        DecimalFloatingPoint::from($input),
                        DecimalFloatingPoint::new(1.0).unwrap(),
                    );
                )*
            }
        }
    }

    test_from![1_u8, 1_u16, 1_u32, 1.0_f32, -1.0_f32, 1.0_f64, -1.0_f64];

    #[test]
    pub fn test_display() {
        let decimal_floating_point = DecimalFloatingPoint::new(22.0).unwrap();
        assert_eq!(decimal_floating_point.to_string(), "22".to_string());

        let decimal_floating_point = DecimalFloatingPoint::new(4.1).unwrap();
        assert_eq!(decimal_floating_point.to_string(), "4.1".to_string());
    }

    #[test]
    pub fn test_parser() {
        assert_eq!(
            DecimalFloatingPoint::new(22.0).unwrap(),
            "22".parse::<DecimalFloatingPoint>().unwrap()
        );

        assert_eq!(
            DecimalFloatingPoint::new(4.1).unwrap(),
            "4.1".parse::<DecimalFloatingPoint>().unwrap()
        );

        assert!("1#".parse::<DecimalFloatingPoint>().is_err());
        assert!("-1.0".parse::<DecimalFloatingPoint>().is_err());
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
    fn test_from_inf() {
        assert_eq!(
            DecimalFloatingPoint::from(::std::f64::INFINITY),
            DecimalFloatingPoint::new(0.0).unwrap()
        );
    }

    #[test]
    fn test_deref() {
        assert_eq!(DecimalFloatingPoint::from(0.1).floor(), 0.0);
    }
}
