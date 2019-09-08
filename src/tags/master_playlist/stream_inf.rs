use crate::attribute::AttributePairs;
use crate::types::{
    ClosedCaptions, DecimalFloatingPoint, DecimalResolution, HdcpLevel, ProtocolVersion,
    QuotedString, SingleLineString,
};
use crate::utils::parse_u64;
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXStreamInf {
    uri: SingleLineString,
    bandwidth: u64,
    average_bandwidth: Option<u64>,
    codecs: Option<QuotedString>,
    resolution: Option<DecimalResolution>,
    frame_rate: Option<DecimalFloatingPoint>,
    hdcp_level: Option<HdcpLevel>,
    audio: Option<QuotedString>,
    video: Option<QuotedString>,
    subtitles: Option<QuotedString>,
    closed_captions: Option<ClosedCaptions>,
}

impl ExtXStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-STREAM-INF:";

    /// Makes a new `ExtXStreamInf` tag.
    pub fn new(uri: SingleLineString, bandwidth: u64) -> Self {
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
    pub fn uri(&self) -> &SingleLineString {
        &self.uri
    }

    /// Returns the peak segment bit rate of the variant stream.
    pub fn bandwidth(&self) -> u64 {
        self.bandwidth
    }

    /// Returns the average segment bit rate of the variant stream.
    pub fn average_bandwidth(&self) -> Option<u64> {
        self.average_bandwidth
    }

    /// Returns a string that represents the list of codec types contained the variant stream.
    pub fn codecs(&self) -> Option<&QuotedString> {
        self.codecs.as_ref()
    }

    /// Returns the optimal pixel resolution at which to display all the video in the variant stream.
    pub fn resolution(&self) -> Option<DecimalResolution> {
        self.resolution
    }

    /// Returns the maximum frame rate for all the video in the variant stream.
    pub fn frame_rate(&self) -> Option<DecimalFloatingPoint> {
        self.frame_rate
    }

    /// Returns the HDCP level of the variant stream.
    pub fn hdcp_level(&self) -> Option<HdcpLevel> {
        self.hdcp_level
    }

    /// Returns the group identifier for the audio in the variant stream.
    pub fn audio(&self) -> Option<&QuotedString> {
        self.audio.as_ref()
    }

    /// Returns the group identifier for the video in the variant stream.
    pub fn video(&self) -> Option<&QuotedString> {
        self.video.as_ref()
    }

    /// Returns the group identifier for the subtitles in the variant stream.
    pub fn subtitles(&self) -> Option<&QuotedString> {
        self.subtitles.as_ref()
    }

    /// Returns the value of `CLOSED-CAPTIONS` attribute.
    pub fn closed_captions(&self) -> Option<&ClosedCaptions> {
        self.closed_captions.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "BANDWIDTH={}", self.bandwidth)?;
        if let Some(ref x) = self.average_bandwidth {
            write!(f, ",AVERAGE-BANDWIDTH={}", x)?;
        }
        if let Some(ref x) = self.codecs {
            write!(f, ",CODECS={}", x)?;
        }
        if let Some(ref x) = self.resolution {
            write!(f, ",RESOLUTION={}", x)?;
        }
        if let Some(ref x) = self.frame_rate {
            write!(f, ",FRAME-RATE={:.3}", x.as_f64())?;
        }
        if let Some(ref x) = self.hdcp_level {
            write!(f, ",HDCP-LEVEL={}", x)?;
        }
        if let Some(ref x) = self.audio {
            write!(f, ",AUDIO={}", x)?;
        }
        if let Some(ref x) = self.video {
            write!(f, ",VIDEO={}", x)?;
        }
        if let Some(ref x) = self.subtitles {
            write!(f, ",SUBTITLES={}", x)?;
        }
        if let Some(ref x) = self.closed_captions {
            write!(f, ",CLOSED-CAPTIONS={}", x)?;
        }
        write!(f, "\n{}", self.uri)?;
        Ok(())
    }
}

impl FromStr for ExtXStreamInf {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.splitn(2, '\n');
        let first_line = lines.next().expect("Never fails").trim_end_matches('\r');
        let second_line = track_assert_some!(lines.next(), ErrorKind::InvalidInput);

        track_assert!(
            first_line.starts_with(Self::PREFIX),
            ErrorKind::InvalidInput
        );
        let uri = track!(SingleLineString::new(second_line))?;
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
        let attrs = AttributePairs::parse(first_line.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "BANDWIDTH" => bandwidth = Some(track!(parse_u64(value))?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some(track!(parse_u64(value))?),
                "CODECS" => codecs = Some(track!(value.parse())?),
                "RESOLUTION" => resolution = Some(track!(value.parse())?),
                "FRAME-RATE" => frame_rate = Some(track!(value.parse())?),
                "HDCP-LEVEL" => hdcp_level = Some(track!(value.parse())?),
                "AUDIO" => audio = Some(track!(value.parse())?),
                "VIDEO" => video = Some(track!(value.parse())?),
                "SUBTITLES" => subtitles = Some(track!(value.parse())?),
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
mod test {
    use super::*;

    #[test]
    fn ext_x_stream_inf() {
        let tag = ExtXStreamInf::new(SingleLineString::new("foo").unwrap(), 1000);
        let text = "#EXT-X-STREAM-INF:BANDWIDTH=1000\nfoo";
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
