use core::cmp::Ordering;
use core::convert::TryFrom;
use core::str::FromStr;

use derive_more::{AsRef, Deref, Display};

use crate::Error;

/// A wrapper type around an [`f32`] that can not be constructed
/// with [`NaN`], [`INFINITY`] or [`NEG_INFINITY`].
///
/// [`NaN`]: core::f32::NAN
/// [`INFINITY`]: core::f32::INFINITY
/// [`NEG_INFINITY`]: core::f32::NEG_INFINITY
#[derive(AsRef, Deref, Default, Debug, Copy, Clone, Display)]
pub struct Float(f32);

impl Float {
    /// Makes a new [`Float`] from an [`f32`].
    ///
    /// # Panics
    ///
    /// If the given float is infinite or [`NaN`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use hls_m3u8::types::Float;
    /// let float = Float::new(1.0);
    /// ```
    ///
    /// This would panic:
    ///
    /// ```should_panic
    /// # use hls_m3u8::types::Float;
    /// use core::f32::NAN;
    ///
    /// let float = Float::new(NAN);
    /// ```
    ///
    /// [`NaN`]: core::f32::NAN
    #[must_use]
    pub fn new(float: f32) -> Self {
        if float.is_infinite() {
            panic!("float must be finite: `{}`", float);
        }

        if float.is_nan() {
            panic!("float must not be `NaN`");
        }

        Self(float)
    }

    /// Returns the underlying [`f32`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::Float;
    /// assert_eq!(Float::new(1.1_f32).as_f32(), 1.1_f32);
    /// ```
    #[must_use]
    pub const fn as_f32(self) -> f32 {
        self.0
    }
}

impl FromStr for Float {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let float = f32::from_str(input).map_err(|e| Error::parse_float(input, e))?;
        Self::try_from(float)
    }
}

impl TryFrom<f32> for Float {
    type Error = Error;

    fn try_from(float: f32) -> Result<Self, Self::Error> {
        if float.is_infinite() {
            return Err(Error::custom(format!("float must be finite: `{}`", float)));
        }

        if float.is_nan() {
            return Err(Error::custom("float must not be `NaN`"));
        }

        Ok(Self(float))
    }
}

macro_rules! implement_from {
    ( $( $type:tt ),+ ) => {
        $(
            impl ::core::convert::From<$type> for Float {
                fn from(value: $type) -> Self {
                    Self(value as f32)
                }
            }
        )+
    }
}

implement_from!(i16, u16, i8, u8);

impl PartialEq for Float {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// convenience implementation to compare f32 with a Float.
impl PartialEq<f32> for Float {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        &self.0 == other
    }
}

// In order to implement `Eq` a struct has to satisfy
// the following requirements:
// - reflexive: a == a;
// - symmetric: a == b implies b == a; and
// - transitive: a == b and b == c implies a == c.
//
// The symmetric and transitive parts are already satisfied
// through `PartialEq`. The reflexive part is not satisfied for f32,
// because `f32::NAN` never equals `f32::NAN`. (`assert!(f32::NAN, f32::NAN)`)
//
// It is ensured, that this struct can not be constructed
// with NaN so all of the above requirements are satisfied and therefore Eq can
// be soundly implemented.
impl Eq for Float {}

impl PartialOrd for Float {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Float {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

/// The output of Hash cannot be relied upon to be stable. The same version of
/// rust can return different values in different architectures. This is not a
/// property of the Hasher that you’re using but instead of the way Hash happens
/// to be implemented for the type you’re using (e.g., the current
/// implementation of Hash for slices of integers returns different values in
/// big and little-endian architectures).
///
/// See <https://internals.rust-lang.org/t/f32-f64-should-implement-hash/5436/33>
#[doc(hidden)]
impl ::core::hash::Hash for Float {
    fn hash<H>(&self, state: &mut H)
    where
        H: ::core::hash::Hasher,
    {
        // this implementation assumes, that the internal float is:
        // - not NaN
        // - neither negative nor positive infinity

        // to validate those assumptions debug_assertions are here
        // (those will be removed in a release build)
        debug_assert!(self.0.is_finite());
        debug_assert!(!self.0.is_nan());

        // this implementation is based on
        // https://internals.rust-lang.org/t/f32-f64-should-implement-hash/5436/33
        //
        // The important points are:
        // - NaN == NaN (Float does not allow NaN, so this should be satisfied)
        // - +0 == -0

        if self.0 == 0.0 || self.0 == -0.0 {
            state.write(&0.0_f32.to_be_bytes());
        } else {
            // I do not think it matters to differentiate between architectures, that use
            // big endian by default and those, that use little endian.
            state.write(&self.to_be_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::hash::{Hash, Hasher};
    use pretty_assertions::assert_eq;

    #[allow(clippy::all, clippy::unreadable_literal)]
    const PI: f32 = 3.14159265359;

    #[test]
    fn test_ord() {
        assert_eq!(Float::new(1.1).cmp(&Float::new(1.1)), Ordering::Equal);
        assert_eq!(Float::new(1.1).cmp(&Float::new(2.1)), Ordering::Less);
        assert_eq!(Float::new(1.1).cmp(&Float::new(0.1)), Ordering::Greater);
    }

    #[test]
    fn test_partial_ord() {
        assert_eq!(
            Float::new(1.1).partial_cmp(&Float::new(1.1)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Float::new(1.1).partial_cmp(&Float::new(2.1)),
            Some(Ordering::Less)
        );
        assert_eq!(
            Float::new(1.1).partial_cmp(&Float::new(0.1)),
            Some(Ordering::Greater)
        );
    }

    #[test]
    #[allow(clippy::unit_cmp)] // fucked test
    fn test_hash() {
        let mut hasher_left = std::collections::hash_map::DefaultHasher::new();
        let mut hasher_right = std::collections::hash_map::DefaultHasher::new();

        assert_eq!(
            Float::new(0.0).hash(&mut hasher_left),
            Float::new(-0.0).hash(&mut hasher_right)
        );

        assert_eq!(hasher_left.finish(), hasher_right.finish());

        let mut hasher_left = std::collections::hash_map::DefaultHasher::new();
        let mut hasher_right = std::collections::hash_map::DefaultHasher::new();

        assert_eq!(
            Float::new(1.0).hash(&mut hasher_left),
            Float::new(1.0).hash(&mut hasher_right)
        );

        assert_eq!(hasher_left.finish(), hasher_right.finish());
    }

    #[test]
    const fn test_eq() {
        struct _AssertEq
        where
            Float: Eq;
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(Float::new(1.0), Float::new(1.0));
        assert_ne!(Float::new(1.0), Float::new(33.3));
        assert_eq!(Float::new(1.1), 1.1);
    }

    #[test]
    fn test_display() {
        assert_eq!(Float::new(22.0).to_string(), "22".to_string());
        assert_eq!(Float::new(PI).to_string(), "3.1415927".to_string());
        assert_eq!(Float::new(-PI).to_string(), "-3.1415927".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(Float::new(22.0), Float::from_str("22").unwrap());
        assert_eq!(Float::new(-22.0), Float::from_str("-22").unwrap());
        assert_eq!(Float::new(PI), Float::from_str("3.14159265359").unwrap());
        assert!(Float::from_str("1#").is_err());
        assert!(Float::from_str("NaN").is_err());
        assert!(Float::from_str("inf").is_err());
        assert!(Float::from_str("-inf").is_err());
    }

    #[test]
    #[should_panic = "float must be finite: `inf`"]
    fn test_new_infinite() {
        let _ = Float::new(f32::INFINITY);
    }

    #[test]
    #[should_panic = "float must be finite: `-inf`"]
    fn test_new_neg_infinite() {
        let _ = Float::new(f32::NEG_INFINITY);
    }

    #[test]
    #[should_panic = "float must not be `NaN`"]
    fn test_new_nan() {
        let _ = Float::new(f32::NAN);
    }

    #[test]
    fn test_as_f32() {
        assert_eq!(Float::new(1.1).as_f32(), 1.1_f32);
    }

    #[test]
    fn test_from() {
        assert_eq!(Float::from(-1_i8), Float::new(-1.0));
        assert_eq!(Float::from(1_u8), Float::new(1.0));
        assert_eq!(Float::from(-1_i16), Float::new(-1.0));
        assert_eq!(Float::from(1_u16), Float::new(1.0));
    }

    #[test]
    fn test_try_from() {
        assert_eq!(Float::try_from(1.1_f32).unwrap(), Float::new(1.1));
        assert_eq!(Float::try_from(-1.1_f32).unwrap(), Float::new(-1.1));

        assert!(Float::try_from(f32::INFINITY).is_err());
        assert!(Float::try_from(f32::NAN).is_err());
        assert!(Float::try_from(f32::NEG_INFINITY).is_err());
    }
}
