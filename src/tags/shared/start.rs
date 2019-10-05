use std::fmt;
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::types::{ProtocolVersion, SignedDecimalFloatingPoint};
use crate::utils::{parse_yes_or_no, tag};
use crate::{Error, RequiredVersion};

/// [4.3.5.2. EXT-X-START]
///
/// [4.3.5.2. EXT-X-START]: https://tools.ietf.org/html/rfc8216#section-4.3.5.2
#[derive(PartialOrd, Debug, Clone, Copy, PartialEq)]
pub struct ExtXStart {
    time_offset: SignedDecimalFloatingPoint,
    precise: bool,
}

impl ExtXStart {
    pub(crate) const PREFIX: &'static str = "#EXT-X-START:";

    /// Makes a new [`ExtXStart`] tag.
    ///
    /// # Panic
    /// Panics if the time_offset value is infinite.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStart;
    /// ExtXStart::new(20.123456);
    /// ```
    pub fn new(time_offset: f64) -> Self {
        Self {
            time_offset: SignedDecimalFloatingPoint::new(time_offset),
            precise: false,
        }
    }

    /// Makes a new [`ExtXStart`] tag with the given `precise` flag.
    ///
    /// # Panic
    /// Panics if the time_offset value is infinite.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStart;
    /// let start = ExtXStart::with_precise(20.123456, true);
    /// assert_eq!(start.precise(), true);
    /// ```
    pub fn with_precise(time_offset: f64, precise: bool) -> Self {
        Self {
            time_offset: SignedDecimalFloatingPoint::new(time_offset),
            precise,
        }
    }

    /// Returns the time offset of the media segments in the playlist.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStart;
    /// let start = ExtXStart::new(20.123456);
    /// assert_eq!(start.time_offset(), 20.123456);
    /// ```
    pub const fn time_offset(&self) -> f64 { self.time_offset.as_f64() }

    /// Sets the time offset of the media segments in the playlist.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStart;
    /// let mut start = ExtXStart::new(20.123456);
    /// # assert_eq!(start.time_offset(), 20.123456);
    ///
    /// start.set_time_offset(1.0);
    ///
    /// assert_eq!(start.time_offset(), 1.0);
    /// ```
    pub fn set_time_offset(&mut self, value: f64) -> &mut Self {
        self.time_offset = SignedDecimalFloatingPoint::new(value);
        self
    }

    /// Returns whether clients should not render media stream whose
    /// presentation times are prior to the specified time offset.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStart;
    /// let start = ExtXStart::with_precise(20.123456, true);
    /// assert_eq!(start.precise(), true);
    /// ```
    pub const fn precise(&self) -> bool { self.precise }

    /// Sets the `precise` flag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStart;
    /// let mut start = ExtXStart::new(20.123456);
    /// # assert_eq!(start.precise(), false);
    ///
    /// start.set_precise(true);
    ///
    /// assert_eq!(start.precise(), true);
    /// ```
    pub fn set_precise(&mut self, value: bool) -> &mut Self {
        self.precise = value;
        self
    }
}

impl RequiredVersion for ExtXStart {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXStart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "TIME-OFFSET={}", self.time_offset)?;
        if self.precise {
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
        let mut precise = false;

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "TIME-OFFSET" => time_offset = Some((value.parse())?),
                "PRECISE" => precise = (parse_yes_or_no(value))?,
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized
                    // AttributeName.
                }
            }
        }

        let time_offset = time_offset.ok_or_else(|| Error::missing_value("EXT-X-TIME-OFFSET"))?;

        Ok(Self {
            time_offset,
            precise,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXStart::new(-1.23).to_string(),
            "#EXT-X-START:TIME-OFFSET=-1.23".to_string(),
        );

        assert_eq!(
            ExtXStart::with_precise(1.23, true).to_string(),
            "#EXT-X-START:TIME-OFFSET=1.23,PRECISE=YES".to_string(),
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXStart::new(-1.23).required_version(),
            ProtocolVersion::V1,
        );

        assert_eq!(
            ExtXStart::with_precise(1.23, true).required_version(),
            ProtocolVersion::V1,
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXStart::new(-1.23),
            "#EXT-X-START:TIME-OFFSET=-1.23".parse().unwrap(),
        );

        assert_eq!(
            ExtXStart::with_precise(1.23, true),
            "#EXT-X-START:TIME-OFFSET=1.23,PRECISE=YES".parse().unwrap(),
        );

        assert_eq!(
            ExtXStart::with_precise(1.23, true),
            "#EXT-X-START:TIME-OFFSET=1.23,PRECISE=YES,UNKNOWN=TAG"
                .parse()
                .unwrap(),
        );
    }
}
