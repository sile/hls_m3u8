use core::convert::TryInto;
use core::fmt;
use core::ops::{
    Add, AddAssign, Bound, Range, RangeBounds, RangeInclusive, RangeTo, RangeToInclusive, Sub,
    SubAssign,
};
use core::str::FromStr;

use shorthand::ShortHand;

use crate::Error;

/// A range of bytes, which can be seen as either `..end` or `start..end`.
///
/// It can be constructed from `..end` and `start..end`:
///
/// ```
/// use hls_m3u8::types::ByteRange;
///
/// let range = ByteRange::from(10..20);
/// let range = ByteRange::from(..20);
/// ```
#[derive(ShortHand, Copy, Hash, Eq, Ord, Debug, PartialEq, Clone, PartialOrd)]
#[shorthand(enable(must_use, copy), disable(option_as_ref, set))]
pub struct ByteRange {
    /// Returns the `start` of the [`ByteRange`], if there is one.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// assert_eq!(ByteRange::from(0..5).start(), Some(0));
    /// assert_eq!(ByteRange::from(..5).start(), None);
    /// ```
    start: Option<usize>,
    /// Returns the `end` of the [`ByteRange`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// assert_eq!(ByteRange::from(0..5).end(), 5);
    /// assert_eq!(ByteRange::from(..=5).end(), 6);
    /// ```
    end: usize,
}

impl ByteRange {
    /// Changes the length of the [`ByteRange`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// let mut range = ByteRange::from(0..5);
    /// range.set_len(2);
    ///
    /// assert_eq!(range, ByteRange::from(0..2));
    ///
    /// range.set_len(200);
    /// assert_eq!(range, ByteRange::from(0..200));
    /// ```
    ///
    /// # Note
    ///
    /// The `start` will not be changed.
    pub fn set_len(&mut self, new_len: usize) {
        // the new_len can be either greater or smaller than `self.len()`.
        // if new_len is larger `checked_sub` will return `None`
        if let Some(value) = self.len().checked_sub(new_len) {
            self.end -= value;
        } else {
            self.end += new_len.saturating_sub(self.len());
        }
    }

    /// Sets the `start` of the [`ByteRange`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// assert_eq!(ByteRange::from(0..5).set_start(Some(5)).start(), Some(5));
    /// assert_eq!(ByteRange::from(..5).set_start(Some(2)).start(), Some(2));
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic, if the `new_start` is larger, than the
    /// [`end`](ByteRange::end).
    pub fn set_start(&mut self, new_start: Option<usize>) -> &mut Self {
        if new_start.map_or(false, |s| s > self.end) {
            panic!(
                "attempt to make the start ({}) larger than the end ({})",
                new_start.unwrap(),
                self.end
            );
        }

        self.start = new_start;

        self
    }

    /// Adds `num` to the `start` and `end` of the range.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// let range = ByteRange::from(10..22);
    /// let nrange = range.saturating_add(5);
    ///
    /// assert_eq!(nrange.len(), range.len());
    /// assert_eq!(nrange.start(), range.start().map(|c| c + 5));
    /// ```
    ///
    /// # Overflow
    ///
    /// If the range is saturated it will not overflow and instead stay
    /// at it's current value.
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// let range = ByteRange::from(5..usize::max_value());
    ///
    /// // this would cause the end to overflow
    /// let nrange = range.saturating_add(1);
    ///
    /// // but the range remains unchanged
    /// assert_eq!(range, nrange);
    /// ```
    ///
    /// # Note
    ///
    /// The length of the range will remain unchanged,
    /// if the `start` is `Some`.
    #[must_use]
    pub fn saturating_add(mut self, num: usize) -> Self {
        if let Some(start) = self.start {
            // add the number to the start
            if let (Some(start), Some(end)) = (start.checked_add(num), self.end.checked_add(num)) {
                self.start = Some(start);
                self.end = end;
            } else {
                // it is ensured at construction that the start will never be larger than the
                // end. This clause can therefore be only reached if the end overflowed.
                // -> It is only possible to add `usize::max_value() - end` to the start.
                if let Some(start) = start.checked_add(usize::max_value() - self.end) {
                    self.start = Some(start);
                    self.end = usize::max_value();
                } else {
                    // both end + start overflowed -> do not change anything
                }
            }
        } else {
            self.end = self.end.saturating_add(num);
        }

        self
    }

    /// Subtracts `num` from the `start` and `end` of the range.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// let range = ByteRange::from(10..22);
    /// let nrange = range.saturating_sub(5);
    ///
    /// assert_eq!(nrange.len(), range.len());
    /// assert_eq!(nrange.start(), range.start().map(|c| c - 5));
    /// ```
    ///
    /// # Underflow
    ///
    /// If the range is saturated it will not underflow and instead stay
    /// at it's current value.
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// let range = ByteRange::from(0..10);
    ///
    /// // this would cause the start to underflow
    /// let nrange = range.saturating_sub(1);
    ///
    /// // but the range remains unchanged
    /// assert_eq!(range, nrange);
    /// ```
    ///
    /// # Note
    ///
    /// The length of the range will remain unchanged,
    /// if the `start` is `Some`.
    #[must_use]
    pub fn saturating_sub(mut self, num: usize) -> Self {
        if let Some(start) = self.start {
            // subtract the number from the start
            if let (Some(start), Some(end)) = (start.checked_sub(num), self.end.checked_sub(num)) {
                self.start = Some(start);
                self.end = end;
            } else {
                // it is ensured at construction that the start will never be larger, than the
                // end so this clause will only be reached, if the start underflowed.
                // -> can at most subtract `start` from `end`
                if let Some(end) = self.end.checked_sub(start) {
                    self.start = Some(0);
                    self.end = end;
                } else {
                    // both end + start underflowed
                    // -> do not change anything
                }
            }
        } else {
            self.end = self.end.saturating_sub(num);
        }

        self
    }

    /// Returns the length, which is calculated by subtracting the `end` from
    /// the `start`. If the `start` is `None` a 0 is assumed.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// let range = ByteRange::from(1..16);
    ///
    /// assert_eq!(range.len(), 15);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.end.saturating_sub(self.start.unwrap_or(0)) }

    /// Returns `true` if the length is zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// let range = ByteRange::from(12..12);
    ///
    /// assert_eq!(range.is_empty(), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

impl Sub<usize> for ByteRange {
    type Output = Self;

    #[must_use]
    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            start: self.start.map(|lhs| lhs - rhs),
            end: self.end - rhs,
        }
    }
}

impl SubAssign<usize> for ByteRange {
    #[inline]
    fn sub_assign(&mut self, other: usize) { *self = <Self as Sub<usize>>::sub(*self, other); }
}

impl Add<usize> for ByteRange {
    type Output = Self;

    #[must_use]
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        Self {
            start: self.start.map(|lhs| lhs + rhs),
            end: self.end + rhs,
        }
    }
}

impl AddAssign<usize> for ByteRange {
    #[inline]
    fn add_assign(&mut self, other: usize) { *self = <Self as Add<usize>>::add(*self, other); }
}

macro_rules! impl_from_ranges {
    ( $( $type:tt ),* ) => {
        $(
            #[allow(trivial_numeric_casts, clippy::fallible_impl_from)]
            impl From<Range<$type>> for ByteRange {
                fn from(range: Range<$type>) -> Self {
                    if range.start > range.end {
                        panic!(
                            "the range start ({}) must be smaller than the end ({})",
                            range.start, range.end
                        );
                    }

                    Self {
                        start: Some(range.start as usize),
                        end: range.end as usize,
                    }
                }
            }

            #[allow(trivial_numeric_casts, clippy::fallible_impl_from)]
            impl From<RangeInclusive<$type>> for ByteRange {
                fn from(range: RangeInclusive<$type>) -> Self {
                    let (start, end) = range.into_inner();

                    if start > end {
                        panic!(
                            "the range start ({}) must be smaller than the end ({}+1)",
                            start, end
                        );
                    }

                    Self {
                        start: Some(start as usize),
                        end: (end as usize).saturating_add(1),
                    }
                }
            }

            #[allow(trivial_numeric_casts, clippy::fallible_impl_from)]
            impl From<RangeTo<$type>> for ByteRange {
                fn from(range: RangeTo<$type>) -> Self {
                    Self {
                        start: None,
                        end: range.end as usize,
                    }
                }
            }

            #[allow(trivial_numeric_casts, clippy::fallible_impl_from)]
            impl From<RangeToInclusive<$type>> for ByteRange {
                fn from(range: RangeToInclusive<$type>) -> Self {
                    Self {
                        start: None,
                        end: (range.end as usize).saturating_add(1),
                    }
                }
            }
        )*
    }
}

// TODO: replace with generics as soon as overlapping trait implementations are
// stable (`Into<i64> for usize` is reserved for upstream crates ._.)
impl_from_ranges![u64, u32, u16, u8, usize, i32];

#[must_use]
impl RangeBounds<usize> for ByteRange {
    fn start_bound(&self) -> Bound<&usize> {
        if let Some(start) = &self.start {
            Bound::Included(start)
        } else {
            Bound::Unbounded
        }
    }

    #[inline]
    fn end_bound(&self) -> Bound<&usize> { Bound::Excluded(&self.end) }
}

/// This conversion will fail if the start of the [`ByteRange`] is `Some`.
impl TryInto<RangeTo<usize>> for ByteRange {
    type Error = Error;

    fn try_into(self) -> Result<RangeTo<usize>, Self::Error> {
        if self.start.is_some() {
            return Err(Error::custom("a `RangeTo` (`..end`) does not have a start"));
        }

        Ok(RangeTo { end: self.end })
    }
}

/// This conversion will fail if the start of the [`ByteRange`] is `None`.
impl TryInto<Range<usize>> for ByteRange {
    type Error = Error;

    fn try_into(self) -> Result<Range<usize>, Self::Error> {
        if self.start.is_none() {
            return Err(Error::custom(
                "a `Range` (`start..end`) has to have a start.",
            ));
        }

        Ok(Range {
            start: self.start.unwrap(),
            end: self.end,
        })
    }
}

impl fmt::Display for ByteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.len())?;

        if let Some(value) = self.start {
            write!(f, "@{}", value)?;
        }

        Ok(())
    }
}

impl FromStr for ByteRange {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.splitn(2, '@');

        let length = input.next().unwrap();
        let length = length
            .parse::<usize>()
            .map_err(|e| Error::parse_int(length, e))?;

        let start = input
            .next()
            .map(|v| v.parse::<usize>().map_err(|e| Error::parse_int(v, e)))
            .transpose()?;

        Ok(Self {
            start,
            end: start.unwrap_or(0) + length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic = "the range start (6) must be smaller than the end (0)"]
    fn test_from_range_panic() { let _ = ByteRange::from(6..0); }

    #[test]
    #[should_panic = "the range start (6) must be smaller than the end (0+1)"]
    fn test_from_range_inclusive_panic() { let _ = ByteRange::from(6..=0); }

    #[test]
    fn test_from_ranges() {
        assert_eq!(ByteRange::from(1..10), ByteRange::from(1..=9));
        assert_eq!(ByteRange::from(..10), ByteRange::from(..=9));
    }

    #[test]
    fn test_range_bounds() {
        assert_eq!(ByteRange::from(0..10).start_bound(), Bound::Included(&0));
        assert_eq!(ByteRange::from(..10).start_bound(), Bound::Unbounded);

        assert_eq!(ByteRange::from(0..10).end_bound(), Bound::Excluded(&10));
        assert_eq!(ByteRange::from(..10).end_bound(), Bound::Excluded(&10));
    }

    #[test]
    fn test_try_into() {
        assert_eq!(ByteRange::from(1..4).try_into(), Ok(1..4));
        assert_eq!(ByteRange::from(..4).try_into(), Ok(..4));

        assert!(TryInto::<RangeTo<usize>>::try_into(ByteRange::from(1..4)).is_err());
        assert!(TryInto::<Range<usize>>::try_into(ByteRange::from(..4)).is_err());
    }

    #[test]
    fn test_add_assign() {
        let mut range = ByteRange::from(5..10);
        range += 5;

        assert_eq!(range, ByteRange::from(10..15));
    }

    #[test]
    #[should_panic = "attempt to add with overflow"]
    fn test_add_assign_panic() {
        let mut range = ByteRange::from(4..usize::max_value());
        range += 5;

        unreachable!();
    }

    #[test]
    fn test_sub_assign() {
        let mut range = ByteRange::from(10..20);
        range -= 5;

        assert_eq!(range, ByteRange::from(5..15));
    }

    #[test]
    #[should_panic = "attempt to subtract with overflow"]
    fn test_sub_assign_panic() {
        let mut range = ByteRange::from(4..10);
        range -= 5;

        unreachable!();
    }

    #[test]
    #[should_panic = "attempt to make the start (11) larger than the end (10)"]
    fn test_set_start() { let _ = ByteRange::from(4..10).set_start(Some(11)); }

    #[test]
    fn test_add() {
        // normal addition
        assert_eq!(ByteRange::from(5..10) + 5, ByteRange::from(10..15));
        assert_eq!(ByteRange::from(..10) + 5, ByteRange::from(..15));

        // adding 0
        assert_eq!(ByteRange::from(5..10) + 0, ByteRange::from(5..10));
        assert_eq!(ByteRange::from(..10) + 0, ByteRange::from(..10));
    }

    #[test]
    #[should_panic = "attempt to add with overflow"]
    fn test_add_panic() { let _ = ByteRange::from(usize::max_value()..usize::max_value()) + 1; }

    #[test]
    fn test_sub() {
        // normal subtraction
        assert_eq!(ByteRange::from(5..10) - 4, ByteRange::from(1..6));
        assert_eq!(ByteRange::from(..10) - 4, ByteRange::from(..6));

        // subtracting 0
        assert_eq!(ByteRange::from(0..0) - 0, ByteRange::from(0..0));
        assert_eq!(ByteRange::from(2..3) - 0, ByteRange::from(2..3));

        assert_eq!(ByteRange::from(..0) - 0, ByteRange::from(..0));
        assert_eq!(ByteRange::from(..3) - 0, ByteRange::from(..3));
    }

    #[test]
    #[should_panic = "attempt to subtract with overflow"]
    fn test_sub_panic() { let _ = ByteRange::from(0..0) - 1; }

    #[test]
    fn test_saturating_add() {
        // normal addition
        assert_eq!(
            ByteRange::from(5..10).saturating_add(5),
            ByteRange::from(10..15)
        );
        assert_eq!(
            ByteRange::from(..10).saturating_add(5),
            ByteRange::from(..15)
        );

        // adding 0
        assert_eq!(
            ByteRange::from(6..11).saturating_add(0),
            ByteRange::from(6..11)
        );
        assert_eq!(
            ByteRange::from(..11).saturating_add(0),
            ByteRange::from(..11)
        );

        assert_eq!(
            ByteRange::from(0..0).saturating_add(0),
            ByteRange::from(0..0)
        );
        assert_eq!(ByteRange::from(..0).saturating_add(0), ByteRange::from(..0));

        // overflow
        assert_eq!(
            ByteRange::from(usize::max_value()..usize::max_value()).saturating_add(1),
            ByteRange::from(usize::max_value()..usize::max_value())
        );
        assert_eq!(
            ByteRange::from(..usize::max_value()).saturating_add(1),
            ByteRange::from(..usize::max_value())
        );

        assert_eq!(
            ByteRange::from(usize::max_value() - 5..usize::max_value()).saturating_add(1),
            ByteRange::from(usize::max_value() - 5..usize::max_value())
        );

        // overflow, but something can be added to the range:
        assert_eq!(
            ByteRange::from(usize::max_value() - 5..usize::max_value() - 3).saturating_add(4),
            ByteRange::from(usize::max_value() - 2..usize::max_value())
        );

        assert_eq!(
            ByteRange::from(..usize::max_value() - 3).saturating_add(4),
            ByteRange::from(..usize::max_value())
        );
    }

    #[test]
    fn test_saturating_sub() {
        // normal subtraction
        assert_eq!(
            ByteRange::from(5..10).saturating_sub(4),
            ByteRange::from(1..6)
        );

        // subtracting 0
        assert_eq!(
            ByteRange::from(0..0).saturating_sub(0),
            ByteRange::from(0..0)
        );
        assert_eq!(
            ByteRange::from(2..3).saturating_sub(0),
            ByteRange::from(2..3)
        );

        // the start underflows
        assert_eq!(
            ByteRange::from(0..5).saturating_sub(4),
            ByteRange::from(0..5)
        );

        // the start underflows, but one can still subtract something from it
        assert_eq!(
            ByteRange::from(1..5).saturating_sub(2),
            ByteRange::from(0..4)
        );

        // both start and end underflow
        assert_eq!(
            ByteRange::from(1..3).saturating_sub(5),
            ByteRange::from(0..2)
        );

        // both start + end are 0 + underflow
        assert_eq!(
            ByteRange::from(0..0).saturating_sub(1),
            ByteRange::from(0..0)
        );

        // half open ranges:
        assert_eq!(ByteRange::from(..6).saturating_sub(2), ByteRange::from(..4));
        assert_eq!(ByteRange::from(..5).saturating_sub(0), ByteRange::from(..5));
        assert_eq!(ByteRange::from(..0).saturating_sub(0), ByteRange::from(..0));

        assert_eq!(ByteRange::from(..0).saturating_sub(1), ByteRange::from(..0));
    }

    #[test]
    fn test_display() {
        assert_eq!(ByteRange::from(0..5).to_string(), "5@0".to_string());

        assert_eq!(
            ByteRange::from(2..100001).to_string(),
            "99999@2".to_string()
        );

        assert_eq!(ByteRange::from(..99999).to_string(), "99999".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(ByteRange::from(2..22), "20@2".parse().unwrap());

        assert_eq!(ByteRange::from(..300), "300".parse().unwrap());

        assert_eq!(
            ByteRange::from_str("a"),
            Err(Error::parse_int("a", "a".parse::<usize>().unwrap_err()))
        );

        assert_eq!(
            ByteRange::from_str("1@a"),
            Err(Error::parse_int("a", "a".parse::<usize>().unwrap_err()))
        );

        assert!("".parse::<ByteRange>().is_err());
    }
}
