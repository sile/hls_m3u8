use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::error::{Error, ErrorKind};
use crate::types::{
    ClosedCaptions, DecimalFloatingPoint, DecimalResolution, HdcpLevel, ProtocolVersion,
};
use crate::utils::parse_u64;
use crate::utils::{quote, unquote};

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
    pub fn new(uri: String, bandwidth: u64) -> Self {
        ExtXStreamInf {
            uri,
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
    pub fn uri(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.uri)
    }

    /// Returns the peak segment bit rate of the variant stream.
    pub fn bandwidth(&self) -> u64 {
        self.bandwidth
    }

    /// Returns the average segment bit rate of the variant stream.
    pub fn average_bandwidth(&self) -> Option<u64> {
        self.average_bandwidth
    }

    /// Returns a string that represents the list of codec types contained by the stream variant.
    pub fn codecs(&self) -> Option<Cow<'_, str>> {
        match &self.codecs {
            Some(value) => Some(Cow::Borrowed(&value)),
            None => None,
        }
    }

    /// Returns the optimal pixel resolution at which to display all the video in the variant
    /// stream.
    pub fn resolution(&self) -> Option<(usize, usize)> {
        match self.resolution {
            Some(value) => Some((value.width(), value.height())),
            None => None,
        }
    }

    /// Returns the maximum frame rate for all the video in the variant stream.
    pub fn frame_rate(&self) -> Option<f64> {
        match &self.frame_rate {
            Some(value) => Some(value.as_f64()),
            None => None,
        }
    }

    /// Returns the HDCP level of the variant stream.
    pub fn hdcp_level(&self) -> Option<HdcpLevel> {
        self.hdcp_level
    }

    /// Returns the group identifier for the audio in the variant stream.
    pub fn audio(&self) -> Option<Cow<'_, str>> {
        match &self.audio {
            Some(value) => Some(Cow::Borrowed(&value)),
            None => None,
        }
    }

    /// Returns the group identifier for the video in the variant stream.
    pub fn video(&self) -> Option<Cow<'_, str>> {
        match &self.video {
            Some(value) => Some(Cow::Borrowed(&value)),
            None => None,
        }
    }

    /// Returns the group identifier for the subtitles in the variant stream.
    pub fn subtitles(&self) -> Option<Cow<'_, str>> {
        match &self.subtitles {
            Some(value) => Some(Cow::Borrowed(&value)),
            None => None,
        }
    }

    /// Returns the value of `CLOSED-CAPTIONS` attribute.
    pub fn closed_captions(&self) -> Option<Cow<'_, ClosedCaptions>> {
        match &self.closed_captions {
            Some(value) => Some(Cow::Borrowed(&value)),
            None => None,
        }
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn required_version(&self) -> ProtocolVersion {
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

        if let Some(value) = &self.codecs {
            write!(f, ",CODECS={}", quote(value))?;
        }

        if let Some(value) = &self.resolution {
            write!(f, ",RESOLUTION={}", value)?;
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.splitn(2, '\n');
        let first_line = lines.next().expect("Never fails").trim_end_matches('\r');
        let second_line = track_assert_some!(lines.next(), ErrorKind::InvalidInput);

        track_assert!(
            first_line.starts_with(Self::PREFIX),
            ErrorKind::InvalidInput
        );

        let uri = second_line.to_string();
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

        let attrs = track!((first_line.split_at(Self::PREFIX.len()).1).parse::<AttributePairs>())?;

        for (key, value) in attrs {
            match key.as_str() {
                "BANDWIDTH" => bandwidth = Some(track!(parse_u64(value))?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some(track!(parse_u64(value))?),
                "CODECS" => codecs = Some(unquote(value)),
                "RESOLUTION" => resolution = Some(track!(value.parse())?),
                "FRAME-RATE" => frame_rate = Some(track!(value.parse())?),
                "HDCP-LEVEL" => hdcp_level = Some(track!(value.parse())?),
                "AUDIO" => audio = Some(unquote(value)),
                "VIDEO" => video = Some(unquote(value)),
                "SUBTITLES" => subtitles = Some(unquote(value)),
                "CLOSED-CAPTIONS" => closed_captions = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let bandwidth = track_assert_some!(bandwidth, ErrorKind::InvalidInput);
        Ok(ExtXStreamInf {
            uri,
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
mod tests {
    use super::*;

    #[test]
    fn ext_x_stream_inf() {
        let tag = ExtXStreamInf::new(String::from("foo"), 1000);
        let text = "#EXT-X-STREAM-INF:BANDWIDTH=1000\nfoo";
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.required_version(), ProtocolVersion::V1);
    }
}
