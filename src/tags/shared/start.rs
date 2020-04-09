use std::fmt;
use std::str::FromStr;

use shorthand::ShortHand;

use crate::attribute::AttributePairs;
use crate::types::{Float, ProtocolVersion};
use crate::utils::{parse_yes_or_no, tag};
use crate::{Error, RequiredVersion};

/// This tag indicates a preferred point at which to start
/// playing a Playlist.
///
/// By default, clients should start playback at this point when beginning a
/// playback session.
#[derive(ShortHand, PartialOrd, Debug, Clone, Copy, PartialEq, Eq, Ord, Hash)]
#[shorthand(enable(must_use))]
pub struct ExtXStart {
    /// The time offset of the [`MediaSegment`]s in the playlist.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXStart;
    /// use hls_m3u8::types::Float;
    ///
    /// let mut start = ExtXStart::new(Float::new(20.123456));
    /// # assert_eq!(start.time_offset(), Float::new(20.123456));
    ///
    /// start.set_time_offset(Float::new(1.0));
    /// assert_eq!(start.time_offset(), Float::new(1.0));
    /// ```
    ///
    /// [`MediaSegment`]: crate::MediaSegment
    #[shorthand(enable(copy))]
    time_offset: Float,
    /// Whether clients should not render media stream whose presentation times
    /// are prior to the specified time offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXStart;
    /// use hls_m3u8::types::Float;
    ///
    /// let mut start = ExtXStart::new(Float::new(20.123456));
    /// # assert_eq!(start.is_precise(), false);
    /// start.set_is_precise(true);
    ///
    /// assert_eq!(start.is_precise(), true);
    /// ```
    is_precise: bool,
}

impl ExtXStart {
    pub(crate) const PREFIX: &'static str = "#EXT-X-START:";

    /// Makes a new [`ExtXStart`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXStart;
    /// use hls_m3u8::types::Float;
    ///
    /// let start = ExtXStart::new(Float::new(20.123456));
    /// ```
    #[must_use]
    pub const fn new(time_offset: Float) -> Self {
        Self {
            time_offset,
            is_precise: false,
        }
    }

    /// Makes a new [`ExtXStart`] tag with the given `precise` flag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXStart;
    /// use hls_m3u8::types::Float;
    ///
    /// let start = ExtXStart::with_precise(Float::new(20.123456), true);
    /// assert_eq!(start.is_precise(), true);
    /// ```
    #[must_use]
    pub const fn with_precise(time_offset: Float, is_precise: bool) -> Self {
        Self {
            time_offset,
            is_precise,
        }
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXStart {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "TIME-OFFSET={}", self.time_offset)?;

        if self.is_precise {
            write!(f, ",PRECISE=YES")?;
        }

        Ok(())
    }
}

impl FromStr for ExtXStart {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let mut time_offset = None;
        let mut is_precise = false;

        for (key, value) in AttributePairs::new(input) {
            match key {
                "TIME-OFFSET" => time_offset = Some(value.parse()?),
                "PRECISE" => is_precise = parse_yes_or_no(value)?,
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized
                    // AttributeName.
                }
            }
        }

        let time_offset = time_offset.ok_or_else(|| Error::missing_value("TIME-OFFSET"))?;

        Ok(Self {
            time_offset,
            is_precise,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXStart::new(Float::new(-1.23)).to_string(),
            "#EXT-X-START:TIME-OFFSET=-1.23".to_string(),
        );

        assert_eq!(
            ExtXStart::with_precise(Float::new(1.23), true).to_string(),
            "#EXT-X-START:TIME-OFFSET=1.23,PRECISE=YES".to_string(),
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXStart::new(Float::new(-1.23)).required_version(),
            ProtocolVersion::V1,
        );

        assert_eq!(
            ExtXStart::with_precise(Float::new(1.23), true).required_version(),
            ProtocolVersion::V1,
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXStart::new(Float::new(-1.23)),
            "#EXT-X-START:TIME-OFFSET=-1.23".parse().unwrap(),
        );

        assert_eq!(
            ExtXStart::with_precise(Float::new(1.23), true),
            "#EXT-X-START:TIME-OFFSET=1.23,PRECISE=YES".parse().unwrap(),
        );

        assert_eq!(
            ExtXStart::with_precise(Float::new(1.23), true),
            "#EXT-X-START:TIME-OFFSET=1.23,PRECISE=YES,UNKNOWN=TAG"
                .parse()
                .unwrap(),
        );
    }
}
