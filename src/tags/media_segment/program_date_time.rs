#[cfg(not(feature = "chrono"))]
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;
use std::marker::PhantomData;

#[cfg(feature = "chrono")]
use chrono::{DateTime, FixedOffset, SecondsFormat};
#[cfg(feature = "chrono")]
use derive_more::{Deref, DerefMut};

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Associates the first sample of a [`MediaSegment`] with an absolute date
/// and/or time.
///
/// ## Features
///
/// By enabling the `chrono` feature the `date_time`-field will change from
/// `String` to `DateTime<FixedOffset>` and the traits
/// - `Deref<Target=DateTime<FixedOffset>>`,
/// - `DerefMut<Target=DateTime<FixedOffset>>`
/// - and `Copy`
///
/// will be derived.
///
/// [`MediaSegment`]: crate::MediaSegment
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "chrono", derive(Deref, DerefMut, Copy))]
#[non_exhaustive]
pub struct ExtXProgramDateTime<'a> {
    /// The date-time of the first sample of the associated media segment.
    #[cfg(feature = "chrono")]
    #[cfg_attr(feature = "chrono", deref_mut, deref)]
    pub date_time: DateTime<FixedOffset>,
    /// The date-time of the first sample of the associated media segment.
    #[cfg(not(feature = "chrono"))]
    pub date_time: Cow<'a, str>,
    _p: PhantomData<&'a str>,
}

impl<'a> ExtXProgramDateTime<'a> {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PROGRAM-DATE-TIME:";

    /// Makes a new [`ExtXProgramDateTime`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXProgramDateTime;
    /// use chrono::{FixedOffset, TimeZone};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let program_date_time = ExtXProgramDateTime::new(
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// ```
    #[must_use]
    #[cfg(feature = "chrono")]
    pub const fn new(date_time: DateTime<FixedOffset>) -> Self {
        Self {
            date_time,
            _p: PhantomData,
        }
    }

    /// Makes a new [`ExtXProgramDateTime`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXProgramDateTime;
    /// let program_date_time = ExtXProgramDateTime::new("2010-02-19T14:54:23.031+08:00");
    /// ```
    #[cfg(not(feature = "chrono"))]
    pub fn new<T: Into<Cow<'a, str>>>(date_time: T) -> Self {
        Self {
            date_time: date_time.into(),
            _p: PhantomData,
        }
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> ExtXProgramDateTime<'static> {
        ExtXProgramDateTime {
            #[cfg(not(feature = "chrono"))]
            date_time: Cow::Owned(self.date_time.into_owned()),
            #[cfg(feature = "chrono")]
            date_time: self.date_time,
            _p: PhantomData,
        }
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl<'a> RequiredVersion for ExtXProgramDateTime<'a> {
    fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl<'a> fmt::Display for ExtXProgramDateTime<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date_time = {
            #[cfg(feature = "chrono")]
            {
                self.date_time.to_rfc3339_opts(SecondsFormat::Millis, true)
            }
            #[cfg(not(feature = "chrono"))]
            {
                &self.date_time
            }
        };
        write!(f, "{}{}", Self::PREFIX, date_time)
    }
}

impl<'a> TryFrom<&'a str> for ExtXProgramDateTime<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let input = tag(input, Self::PREFIX)?;

        Ok(Self::new({
            #[cfg(feature = "chrono")]
            {
                DateTime::parse_from_rfc3339(input).map_err(Error::chrono)?
            }
            #[cfg(not(feature = "chrono"))]
            {
                input
            }
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(feature = "chrono")]
    use chrono::{Datelike, TimeZone};
    #[cfg(feature = "chrono")]
    use core::ops::DerefMut;
    use pretty_assertions::assert_eq;

    #[cfg(feature = "chrono")]
    const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXProgramDateTime::new({
                #[cfg(feature = "chrono")]
                {
                    FixedOffset::east(8 * HOURS_IN_SECS)
                        .ymd(2010, 2, 19)
                        .and_hms_milli(14, 54, 23, 31)
                }
                #[cfg(not(feature = "chrono"))]
                {
                    "2010-02-19T14:54:23.031+08:00"
                }
            })
            .to_string(),
            "#EXT-X-PROGRAM-DATE-TIME:2010-02-19T14:54:23.031+08:00".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXProgramDateTime::new({
                #[cfg(feature = "chrono")]
                {
                    FixedOffset::east(8 * HOURS_IN_SECS)
                        .ymd(2010, 2, 19)
                        .and_hms_milli(14, 54, 23, 31)
                }
                #[cfg(not(feature = "chrono"))]
                {
                    "2010-02-19T14:54:23.031+08:00"
                }
            }),
            ExtXProgramDateTime::try_from("#EXT-X-PROGRAM-DATE-TIME:2010-02-19T14:54:23.031+08:00")
                .unwrap()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXProgramDateTime::new({
                #[cfg(feature = "chrono")]
                {
                    FixedOffset::east(8 * HOURS_IN_SECS)
                        .ymd(2010, 2, 19)
                        .and_hms_milli(14, 54, 23, 31)
                }
                #[cfg(not(feature = "chrono"))]
                {
                    "2010-02-19T14:54:23.031+08:00"
                }
            })
            .required_version(),
            ProtocolVersion::V1
        );
    }

    #[test]
    #[cfg(feature = "chrono")]
    fn test_deref() {
        assert_eq!(
            ExtXProgramDateTime::new(
                FixedOffset::east(8 * HOURS_IN_SECS)
                    .ymd(2010, 2, 19)
                    .and_hms_milli(14, 54, 23, 31),
            )
            .year(),
            2010
        );
    }

    #[test]
    #[cfg(feature = "chrono")]
    fn test_deref_mut() {
        assert_eq!(
            ExtXProgramDateTime::new(
                FixedOffset::east(8 * HOURS_IN_SECS)
                    .ymd(2010, 2, 19)
                    .and_hms_milli(14, 54, 23, 31),
            )
            .deref_mut(),
            &mut FixedOffset::east(8 * HOURS_IN_SECS)
                .ymd(2010, 2, 19)
                .and_hms_milli(14, 54, 23, 31),
        );
    }
}
