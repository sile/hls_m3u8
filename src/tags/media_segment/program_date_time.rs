use std::fmt;
use std::str::FromStr;

#[cfg(feature = "chrono")]
use chrono::{DateTime, FixedOffset, SecondsFormat};
#[cfg(feature = "chrono")]
use derive_more::{Deref, DerefMut};

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// # [4.3.2.6. EXT-X-PROGRAM-DATE-TIME]
///
/// The [`ExtXProgramDateTime`] tag associates the first sample of a
/// [`MediaSegment`] with an absolute date and/or time.
///
/// [`MediaSegment`]: crate::MediaSegment
/// [4.3.2.6. EXT-X-PROGRAM-DATE-TIME]:
/// https://tools.ietf.org/html/rfc8216#section-4.3.2.6
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "chrono", derive(Deref, DerefMut, Copy))]
pub struct ExtXProgramDateTime {
    /// The date-time of the first sample of the associated media segment.
    #[cfg(feature = "chrono")]
    #[cfg_attr(feature = "chrono", deref_mut, deref)]
    pub date_time: DateTime<FixedOffset>,
    /// The date-time of the first sample of the associated media segment.
    #[cfg(not(feature = "chrono"))]
    pub date_time: String,
    __non_exhaustive: (),
}

impl ExtXProgramDateTime {
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
            __non_exhaustive: (),
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
    pub fn new<T: Into<String>>(date_time: T) -> Self {
        Self {
            date_time: date_time.into(),
            __non_exhaustive: (),
        }
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXProgramDateTime {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXProgramDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl FromStr for ExtXProgramDateTime {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let date_time = {
            #[cfg(feature = "chrono")]
            {
                DateTime::parse_from_rfc3339(input).map_err(Error::chrono)?
            }
            #[cfg(not(feature = "chrono"))]
            {
                input
            }
        };

        Ok(Self::new(date_time))
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
            "#EXT-X-PROGRAM-DATE-TIME:2010-02-19T14:54:23.031+08:00"
                .parse::<ExtXProgramDateTime>()
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
