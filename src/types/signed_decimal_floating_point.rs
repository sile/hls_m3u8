use core::ops::Deref;
use derive_more::{Display, FromStr};

/// Signed decimal floating-point number.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Display, FromStr)]
pub(crate) struct SignedDecimalFloatingPoint(f64);

impl SignedDecimalFloatingPoint {
    /// Makes a new [SignedDecimalFloatingPoint] instance.
    ///
    /// # Panics
    /// The given value must be finite, otherwise this function will panic!
    pub fn new(value: f64) -> Self {
        if value.is_infinite() {
            panic!("Floating point value must be finite!");
        }
        Self(value)
    }

    pub(crate) const fn from_f64_unchecked(value: f64) -> Self { Self(value) }

    /// Converts [DecimalFloatingPoint] to [f64].
    pub const fn as_f64(self) -> f64 { self.0 }
}

impl Deref for SignedDecimalFloatingPoint {
    type Target = f64;

    fn deref(&self) -> &Self::Target { &self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_from {
        ( $( $input:expr => $output:expr ),* ) => {
            use ::core::convert::From;

            #[test]
            fn test_from() {
                $(
                    assert_eq!(
                        $input,
                        $output,
                    );
                )*
            }
        }
    }

    test_from![
        SignedDecimalFloatingPoint::from(1u8) => SignedDecimalFloatingPoint::new(1.0),
        SignedDecimalFloatingPoint::from(1i8) => SignedDecimalFloatingPoint::new(1.0),
        SignedDecimalFloatingPoint::from(1u16) => SignedDecimalFloatingPoint::new(1.0),
        SignedDecimalFloatingPoint::from(1i16) => SignedDecimalFloatingPoint::new(1.0),
        SignedDecimalFloatingPoint::from(1u32) => SignedDecimalFloatingPoint::new(1.0),
        SignedDecimalFloatingPoint::from(1i32) => SignedDecimalFloatingPoint::new(1.0),
        SignedDecimalFloatingPoint::from(1.0f32) => SignedDecimalFloatingPoint::new(1.0),
        SignedDecimalFloatingPoint::from(1.0f64) => SignedDecimalFloatingPoint::new(1.0)
    ];

    #[test]
    fn test_display() {
        assert_eq!(
            SignedDecimalFloatingPoint::new(1.0).to_string(),
            1.0f64.to_string()
        );
    }

    #[test]
    #[should_panic]
    fn test_new_panic() { SignedDecimalFloatingPoint::new(::std::f64::INFINITY); }

    #[test]
    fn test_parser() {
        assert_eq!(
            SignedDecimalFloatingPoint::new(1.0),
            "1.0".parse::<SignedDecimalFloatingPoint>().unwrap()
        );

        assert!("garbage".parse::<SignedDecimalFloatingPoint>().is_err());
    }

    #[test]
    fn test_as_f64() {
        assert_eq!(SignedDecimalFloatingPoint::new(1.0).as_f64(), 1.0);
    }

    #[test]
    fn test_deref() {
        assert_eq!(SignedDecimalFloatingPoint::from(0.1).floor(), 0.0);
    }
}
