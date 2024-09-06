use std::convert::TryFrom;
use std::fmt;

use core::ops::{Add, AddAssign, Sub, SubAssign};

use derive_more::{AsMut, AsRef, Deref, DerefMut, From};

use crate::types::{ByteRange, ProtocolVersion};
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Indicates that a [`MediaSegment`] is a sub-range of the resource identified
/// by its `URI`.
///
/// # Example
///
/// Constructing an [`ExtXByteRange`]:
///
/// ```
/// # use hls_m3u8::tags::ExtXByteRange;
/// assert_eq!(ExtXByteRange::from(22..55), ExtXByteRange::from(22..=54));
/// ```
///
/// It is also possible to omit the start, in which case it assumes that the
/// [`ExtXByteRange`] starts at the byte after the end of the previous
/// [`ExtXByteRange`] or 0 if there is no previous one.
///
/// ```
/// # use hls_m3u8::tags::ExtXByteRange;
/// assert_eq!(ExtXByteRange::from(..55), ExtXByteRange::from(..=54));
/// ```
///
/// [`MediaSegment`]: crate::MediaSegment
#[derive(
    AsRef, AsMut, From, Deref, DerefMut, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
#[from(forward)]
pub struct ExtXByteRange(ByteRange);

impl ExtXByteRange {
    pub(crate) const PREFIX: &'static str = "#EXT-X-BYTERANGE:";

    /// Adds `num` to the `start` and `end` of the range.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXByteRange;
    /// let range = ExtXByteRange::from(10..22);
    /// let nrange = range.saturating_add(5);
    ///
    /// assert_eq!(nrange.len(), range.len());
    /// assert_eq!(nrange.start(), range.start().map(|c| c + 5));
    /// ```
    ///
    /// # Overflow
    ///
    /// If the range is saturated it will not overflow and instead
    /// stay at it's current value.
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXByteRange;
    /// let range = ExtXByteRange::from(5..usize::max_value());
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
    #[inline]
    #[must_use]
    pub fn saturating_add(self, num: usize) -> Self { Self(self.0.saturating_add(num)) }

    /// Subtracts `num` from the `start` and `end` of the range.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXByteRange;
    /// let range = ExtXByteRange::from(10..22);
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
    /// # use hls_m3u8::tags::ExtXByteRange;
    /// let range = ExtXByteRange::from(0..10);
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
    #[inline]
    #[must_use]
    pub fn saturating_sub(self, num: usize) -> Self { Self(self.0.saturating_sub(num)) }

    /// Returns a shared reference to the underlying [`ByteRange`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXByteRange;
    /// use hls_m3u8::types::ByteRange;
    ///
    /// assert_eq!(
    ///     ExtXByteRange::from(2..11).as_byte_range(),
    ///     &ByteRange::from(2..11)
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_byte_range(&self) -> &ByteRange { &self.0 }
}

/// This tag requires [`ProtocolVersion::V4`].
impl RequiredVersion for ExtXByteRange {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V4 }
}

#[allow(clippy::from_over_into)] // Some magic `From` blanket impl is going on that means this can't be done.
impl Into<ByteRange> for ExtXByteRange {
    fn into(self) -> ByteRange { self.0 }
}

impl<T> Sub<T> for ExtXByteRange
where
    ByteRange: Sub<T, Output = ByteRange>,
{
    type Output = Self;

    #[must_use]
    #[inline]
    fn sub(self, rhs: T) -> Self::Output { Self(self.0.sub(rhs)) }
}

impl<T> SubAssign<T> for ExtXByteRange
where
    ByteRange: SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, other: T) { self.0.sub_assign(other); }
}

impl<T> Add<T> for ExtXByteRange
where
    ByteRange: Add<T, Output = ByteRange>,
{
    type Output = Self;

    #[must_use]
    #[inline]
    fn add(self, rhs: T) -> Self::Output { Self(self.0.add(rhs)) }
}

impl<T> AddAssign<T> for ExtXByteRange
where
    ByteRange: AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, other: T) { self.0.add_assign(other); }
}

impl fmt::Display for ExtXByteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl TryFrom<&str> for ExtXByteRange {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = tag(input, Self::PREFIX)?;

        Ok(Self(ByteRange::try_from(input)?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXByteRange::from(2..15).to_string(),
            "#EXT-X-BYTERANGE:13@2".to_string()
        );

        assert_eq!(
            ExtXByteRange::from(..22).to_string(),
            "#EXT-X-BYTERANGE:22".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXByteRange::from(2..15),
            ExtXByteRange::try_from("#EXT-X-BYTERANGE:13@2").unwrap()
        );

        assert_eq!(
            ExtXByteRange::from(..22),
            ExtXByteRange::try_from("#EXT-X-BYTERANGE:22").unwrap()
        );
    }

    #[test]
    fn test_deref() {
        let byte_range = ExtXByteRange::from(0..22);

        assert_eq!(byte_range.len(), 22);
        assert_eq!(byte_range.start(), Some(0));
    }

    #[test]
    fn test_deref_mut() {
        let mut byte_range = ExtXByteRange::from(10..110);

        byte_range.set_start(Some(50));

        assert_eq!(byte_range.len(), 60);
        assert_eq!(byte_range.start(), Some(50));
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXByteRange::from(5..20).required_version(),
            ProtocolVersion::V4
        );
    }
}
