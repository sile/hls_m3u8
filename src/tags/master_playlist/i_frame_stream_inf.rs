use crate::utils::parse_u64;
use crate::attribute::AttributePairs;
use crate::types::{DecimalResolution, HdcpLevel, ProtocolVersion, QuotedString};
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;


/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]
///
/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.3
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXIFrameStreamInf {
    uri: QuotedString,
    bandwidth: u64,
    average_bandwidth: Option<u64>,
    codecs: Option<QuotedString>,
    resolution: Option<DecimalResolution>,
    hdcp_level: Option<HdcpLevel>,
    video: Option<QuotedString>,
}

impl ExtXIFrameStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-I-FRAME-STREAM-INF:";

    /// Makes a new `ExtXIFrameStreamInf` tag.
    pub fn new(uri: QuotedString, bandwidth: u64) -> Self {
        ExtXIFrameStreamInf {
            uri,
            bandwidth,
            average_bandwidth: None,
            codecs: None,
            resolution: None,
            hdcp_level: None,
            video: None,
        }
    }

    /// Returns the URI that identifies the associated media playlist.
    pub fn uri(&self) -> &QuotedString {
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

    /// Returns the HDCP level of the variant stream.
    pub fn hdcp_level(&self) -> Option<HdcpLevel> {
        self.hdcp_level
    }

    /// Returns the group identifier for the video in the variant stream.
    pub fn video(&self) -> Option<&QuotedString> {
        self.video.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXIFrameStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "URI={}", self.uri)?;
        write!(f, ",BANDWIDTH={}", self.bandwidth)?;
        if let Some(ref x) = self.average_bandwidth {
            write!(f, ",AVERAGE-BANDWIDTH={}", x)?;
        }
        if let Some(ref x) = self.codecs {
            write!(f, ",CODECS={}", x)?;
        }
        if let Some(ref x) = self.resolution {
            write!(f, ",RESOLUTION={}", x)?;
        }
        if let Some(ref x) = self.hdcp_level {
            write!(f, ",HDCP-LEVEL={}", x)?;
        }
        if let Some(ref x) = self.video {
            write!(f, ",VIDEO={}", x)?;
        }
        Ok(())
    }
}

impl FromStr for ExtXIFrameStreamInf {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut uri = None;
        let mut bandwidth = None;
        let mut average_bandwidth = None;
        let mut codecs = None;
        let mut resolution = None;
        let mut hdcp_level = None;
        let mut video = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "URI" => uri = Some(track!(value.parse())?),
                "BANDWIDTH" => bandwidth = Some(track!(parse_u64(value))?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some(track!(parse_u64(value))?),
                "CODECS" => codecs = Some(track!(value.parse())?),
                "RESOLUTION" => resolution = Some(track!(value.parse())?),
                "HDCP-LEVEL" => hdcp_level = Some(track!(value.parse())?),
                "VIDEO" => video = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let uri = track_assert_some!(uri, ErrorKind::InvalidInput);
        let bandwidth = track_assert_some!(bandwidth, ErrorKind::InvalidInput);
        Ok(ExtXIFrameStreamInf {
            uri,
            bandwidth,
            average_bandwidth,
            codecs,
            resolution,
            hdcp_level,
            video,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_i_frame_stream_inf() {
        let tag = ExtXIFrameStreamInf::new(quoted_string("foo"), 1000);
        let text = r#"#EXT-X-I-FRAME-STREAM-INF:URI="foo",BANDWIDTH=1000"#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }


    fn quoted_string(s: &str) -> QuotedString {
        QuotedString::new(s).unwrap()
    }
}
