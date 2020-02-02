use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset, SecondsFormat};
use derive_more::{Deref, DerefMut};

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// # [4.3.2.6. EXT-X-PROGRAM-DATE-TIME]
///
/// The [`ExtXProgramDateTime`] tag associates the first sample of a
/// [`Media Segment`] with an absolute date and/or time.
///
/// [`Media Segment`]: crate::MediaSegment
/// [4.3.2.6. EXT-X-PROGRAM-DATE-TIME]:
/// https://tools.ietf.org/html/rfc8216#section-4.3.2.6
#[derive(Deref, DerefMut, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtXProgramDateTime(DateTime<FixedOffset>);

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
    pub const fn new(date_time: DateTime<FixedOffset>) -> Self { Self(date_time) }

    /// Returns the date-time of the first sample of the associated media
    /// segment.
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
    ///
    /// assert_eq!(
    ///     program_date_time.date_time(),
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31)
    /// );
    /// ```
    pub const fn date_time(&self) -> DateTime<FixedOffset> { self.0 }

    /// Sets the date-time of the first sample of the associated media segment.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXProgramDateTime;
    /// use chrono::{FixedOffset, TimeZone};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut program_date_time = ExtXProgramDateTime::new(
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    ///
    /// program_date_time.set_date_time(
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 10, 10)
    ///         .and_hms_milli(10, 10, 10, 10),
    /// );
    ///
    /// assert_eq!(
    ///     program_date_time.date_time(),
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 10, 10)
    ///         .and_hms_milli(10, 10, 10, 10)
    /// );
    /// ```
    pub fn set_date_time(&mut self, value: DateTime<FixedOffset>) -> &mut Self {
        self.0 = value;
        self
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXProgramDateTime {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXProgramDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let date_time = self.0.to_rfc3339_opts(SecondsFormat::Millis, true);
        write!(f, "{}{}", Self::PREFIX, date_time)
    }
}

impl FromStr for ExtXProgramDateTime {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let date_time = DateTime::parse_from_rfc3339(input).map_err(Error::chrono)?;
        Ok(Self::new(date_time))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Datelike, TimeZone};
    use core::ops::DerefMut;
    use pretty_assertions::assert_eq;

    const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXProgramDateTime::new(
                FixedOffset::east(8 * HOURS_IN_SECS)
                    .ymd(2010, 2, 19)
                    .and_hms_milli(14, 54, 23, 31)
            )
            .to_string(),
            "#EXT-X-PROGRAM-DATE-TIME:2010-02-19T14:54:23.031+08:00".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXProgramDateTime::new(
                FixedOffset::east(8 * HOURS_IN_SECS)
                    .ymd(2010, 2, 19)
                    .and_hms_milli(14, 54, 23, 31)
            ),
            "#EXT-X-PROGRAM-DATE-TIME:2010-02-19T14:54:23.031+08:00"
                .parse::<ExtXProgramDateTime>()
                .unwrap()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXProgramDateTime::new(
                FixedOffset::east(8 * HOURS_IN_SECS)
                    .ymd(2010, 2, 19)
                    .and_hms_milli(14, 54, 23, 31),
            )
            .required_version(),
            ProtocolVersion::V1
        );
    }

    #[test]
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
