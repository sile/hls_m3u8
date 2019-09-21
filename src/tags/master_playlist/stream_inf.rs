use std::fmt;
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::types::{
    ClosedCaptions, DecimalFloatingPoint, DecimalResolution, HdcpLevel, ProtocolVersion,
};
use crate::utils::{parse_u64, quote, tag, unquote};
use crate::Error;

/// [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXStreamInf {
    uri: String,
    bandwidth: u64,
    average_bandwidth: Option<u64>,
    codecs: Option<String>,
    resolution: Option<DecimalResolution>,
    frame_rate: Option<DecimalFloatingPoint>,
    hdcp_level: Option<HdcpLevel>,
    audio: Option<String>,
    video: Option<String>,
    subtitles: Option<String>,
    closed_captions: Option<ClosedCaptions>,
}

impl ExtXStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-STREAM-INF:";

    /// Makes a new `ExtXStreamInf` tag.
    pub fn new<T: ToString>(uri: T, bandwidth: u64) -> Self {
        ExtXStreamInf {
            uri: uri.to_string(),
            bandwidth,
            average_bandwidth: None,
            codecs: None,
            resolution: None,
            frame_rate: None,
            hdcp_level: None,
            audio: None,
            video: None,
            subtitles: None,
            closed_captions: None,
        }
    }

    /// Returns the URI that identifies the associated media playlist.
    pub const fn uri(&self) -> &String {
        &self.uri
    }

    /// Returns the peak segment bit rate of the variant stream.
    pub const fn bandwidth(&self) -> u64 {
        self.bandwidth
    }

    /// Returns the average segment bit rate of the variant stream.
    pub const fn average_bandwidth(&self) -> Option<u64> {
        self.average_bandwidth
    }

    /// Returns a string that represents the list of codec types contained the variant stream.
    pub fn codecs(&self) -> Option<&String> {
        self.codecs.as_ref()
    }

    /// Returns the optimal pixel resolution at which to display all the video in the variant
    /// stream.
    pub fn resolution(&self) -> Option<(usize, usize)> {
        if let Some(res) = &self.resolution {
            Some((res.width(), res.height()))
        } else {
            None
        }
    }

    /// Returns the maximum frame rate for all the video in the variant stream.
    pub fn frame_rate(&self) -> Option<f64> {
        self.frame_rate.map_or(None, |v| Some(v.as_f64()))
    }

    /// Returns the HDCP level of the variant stream.
    pub const fn hdcp_level(&self) -> Option<HdcpLevel> {
        self.hdcp_level
    }

    /// Returns the group identifier for the audio in the variant stream.
    pub fn audio(&self) -> Option<&String> {
        self.audio.as_ref()
    }

    /// Returns the group identifier for the video in the variant stream.
    pub fn video(&self) -> Option<&String> {
        self.video.as_ref()
    }

    /// Returns the group identifier for the subtitles in the variant stream.
    pub fn subtitles(&self) -> Option<&String> {
        self.subtitles.as_ref()
    }

    /// Returns the value of `CLOSED-CAPTIONS` attribute.
    pub fn closed_captions(&self) -> Option<&ClosedCaptions> {
        self.closed_captions.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "BANDWIDTH={}", self.bandwidth)?;
        if let Some(value) = &self.average_bandwidth {
            write!(f, ",AVERAGE-BANDWIDTH={}", value)?;
        }
        if let Some(value) = &self.resolution {
            write!(f, ",RESOLUTION={}", value)?;
        }
        if let Some(value) = &self.codecs {
            write!(f, ",CODECS={}", quote(value))?;
        }
        if let Some(value) = &self.frame_rate {
            write!(f, ",FRAME-RATE={:.3}", value.as_f64())?;
        }
        if let Some(value) = &self.hdcp_level {
            write!(f, ",HDCP-LEVEL={}", value)?;
        }
        if let Some(value) = &self.audio {
            write!(f, ",AUDIO={}", quote(value))?;
        }
        if let Some(value) = &self.video {
            write!(f, ",VIDEO={}", quote(value))?;
        }
        if let Some(value) = &self.subtitles {
            write!(f, ",SUBTITLES={}", quote(value))?;
        }
        if let Some(value) = &self.closed_captions {
            write!(f, ",CLOSED-CAPTIONS={}", value)?;
        }
        write!(f, "\n{}", self.uri)?;
        Ok(())
    }
}

impl FromStr for ExtXStreamInf {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        let first_line = lines.next().ok_or(Error::missing_value("first_line"))?;
        let uri = lines.next().ok_or(Error::missing_value("second_line"))?;

        let first_line = tag(first_line, Self::PREFIX)?;

        let mut bandwidth = None;
        let mut average_bandwidth = None;
        let mut codecs = None;
        let mut resolution = None;
        let mut frame_rate = None;
        let mut hdcp_level = None;
        let mut audio = None;
        let mut video = None;
        let mut subtitles = None;
        let mut closed_captions = None;

        for (key, value) in first_line.parse::<AttributePairs>()? {
            match key.as_str() {
                "BANDWIDTH" => bandwidth = Some((parse_u64(value))?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some((parse_u64(value))?),
                "CODECS" => codecs = Some(unquote(value)),
                "RESOLUTION" => resolution = Some((value.parse())?),
                "FRAME-RATE" => frame_rate = Some((value.parse())?),
                "HDCP-LEVEL" => hdcp_level = Some((value.parse())?),
                "AUDIO" => audio = Some(unquote(value)),
                "VIDEO" => video = Some(unquote(value)),
                "SUBTITLES" => subtitles = Some(unquote(value)),
                "CLOSED-CAPTIONS" => closed_captions = Some((value.parse())?),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let bandwidth = bandwidth.ok_or(Error::missing_value("EXT-X-BANDWIDTH"))?;

        Ok(ExtXStreamInf {
            uri: uri.to_string(),
            bandwidth,
            average_bandwidth,
            codecs,
            resolution,
            frame_rate,
            hdcp_level,
            audio,
            video,
            subtitles,
            closed_captions,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let stream_inf = "#EXT-X-STREAM-INF:BANDWIDTH=1000\nhttp://www.example.com"
            .parse::<ExtXStreamInf>()
            .unwrap();

        assert_eq!(
            stream_inf,
            ExtXStreamInf::new("http://www.example.com", 1000)
        );
    }

    #[test]
    fn test_requires_version() {
        assert_eq!(
            ProtocolVersion::V1,
            ExtXStreamInf::new("http://www.example.com", 1000).requires_version()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXStreamInf::new("http://www.example.com/", 1000).to_string(),
            "#EXT-X-STREAM-INF:BANDWIDTH=1000\nhttp://www.example.com/".to_string()
        );
    }
}
