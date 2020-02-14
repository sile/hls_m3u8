use core::convert::TryFrom;
use core::str::FromStr;

use derive_more::{Deref, Display};

use crate::Error;

/// This is a wrapper type around an [`f32`] that can not be constructed
/// with a negative float (ex. `-1.1`), [`NaN`], [`INFINITY`] or
/// [`NEG_INFINITY`].
///
/// [`NaN`]: core::f32::NAN
/// [`INFINITY`]: core::f32::INFINITY
/// [`NEG_INFINITY`]: core::f32::NEG_INFINITY
#[derive(Deref, Default, Debug, Copy, Clone, PartialEq, PartialOrd, Display)]
pub struct UFloat(f32);

impl UFloat {
    /// Makes a new [`UFloat`] from an [`f32`].
    ///
    /// # Panics
    ///
    /// If the given float is negative, infinite or [`NaN`].
    ///
    /// # Examples
    ///
    /// ```
    /// use hls_m3u8::types::UFloat;
    ///
    /// let float = UFloat::new(1.0);
    /// ```
    ///
    /// This would panic:
    ///
    /// ```should_panic
    /// use hls_m3u8::types::UFloat;
    ///
    /// let float = UFloat::new(-1.0);
    /// ```
    ///
    /// [`NaN`]: core::f32::NAN
    pub fn new(float: f32) -> Self {
        if float.is_infinite() {
            panic!("float must be finite: `{}`", float);
        }

        if float.is_nan() {
            panic!("float must not be `NaN`");
        }

        if float.is_sign_negative() {
            panic!("float must be positive: `{}`", float);
        }

        Self(float)
    }

    /// Returns the underlying [`f32`].
    pub const fn as_f32(self) -> f32 { self.0 }
}

impl FromStr for UFloat {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let float = f32::from_str(input).map_err(|e| Error::parse_float(input, e))?;
        Self::try_from(float)
    }
}

impl TryFrom<f32> for UFloat {
    type Error = Error;

    fn try_from(float: f32) -> Result<Self, Self::Error> {
        if float.is_infinite() {
            return Err(Error::custom(format!("float must be finite: `{}`", float)));
        }

        if float.is_nan() {
            return Err(Error::custom("float must not be `NaN`"));
        }

        if float.is_sign_negative() {
            return Err(Error::custom(format!(
                "float must be positive: `{}`",
                float
            )));
        }

        Ok(Self(float))
    }
}

macro_rules! implement_from {
    ( $( $type:tt ),+ ) => {
        $(
            impl ::core::convert::From<$type> for UFloat {
                fn from(value: $type) -> Self {
                    Self(value as f32)
                }
            }
        )+
    }
}

implement_from!(u16, u8);

// convenience implementation to compare f32 with a Float.
impl PartialEq<f32> for UFloat {
    fn eq(&self, other: &f32) -> bool { &self.0 == other }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(UFloat::new(22.0).to_string(), "22".to_string());
        assert_eq!(
            UFloat::new(3.14159265359).to_string(),
            "3.1415927".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(UFloat::new(22.0), UFloat::from_str("22").unwrap());
        assert_eq!(
            UFloat::new(3.14159265359),
            UFloat::from_str("3.14159265359").unwrap()
        );
        assert!(UFloat::from_str("1#").is_err());
        assert!(UFloat::from_str("-1.0").is_err());
        assert!(UFloat::from_str("NaN").is_err());
        assert!(UFloat::from_str("inf").is_err());
        assert!(UFloat::from_str("-inf").is_err());
    }

    #[test]
    #[should_panic = "float must be positive: `-1.1`"]
    fn test_new_negative() { UFloat::new(-1.1); }

    #[test]
    #[should_panic = "float must be finite: `inf`"]
    fn test_new_infinite() { UFloat::new(::core::f32::INFINITY); }

    #[test]
    #[should_panic = "float must be finite: `-inf`"]
    fn test_new_neg_infinite() { UFloat::new(::core::f32::NEG_INFINITY); }

    #[test]
    #[should_panic = "float must not be `NaN`"]
    fn test_new_nan() { UFloat::new(::core::f32::NAN); }

    #[test]
    fn test_partial_eq() {
        assert_eq!(UFloat::new(1.1), 1.1);
    }

    #[test]
    fn test_as_f32() {
        assert_eq!(UFloat::new(1.1).as_f32(), 1.1_f32);
    }

    #[test]
    fn test_from() {
        assert_eq!(UFloat::from(1_u8), UFloat::new(1.0));
        assert_eq!(UFloat::from(1_u16), UFloat::new(1.0));
    }

    #[test]
    fn test_try_from() {
        assert_eq!(UFloat::try_from(1.1_f32).unwrap(), UFloat::new(1.1));

        assert!(UFloat::try_from(-1.1_f32).is_err());
        assert!(UFloat::try_from(::core::f32::INFINITY).is_err());
        assert!(UFloat::try_from(::core::f32::NAN).is_err());
        assert!(UFloat::try_from(::core::f32::NEG_INFINITY).is_err());
    }
}
