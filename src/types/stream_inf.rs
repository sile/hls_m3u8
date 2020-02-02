use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;
use shorthand::ShortHand;

use crate::attribute::AttributePairs;
use crate::types::{HdcpLevel, Resolution};
use crate::utils::{quote, unquote};
use crate::Error;

/// # [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[derive(ShortHand, Builder, PartialOrd, Debug, Clone, PartialEq, Eq, Hash, Ord)]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug, PartialEq))]
#[shorthand(enable(must_use, into))]
pub struct StreamInf {
    /// The peak segment bit rate of the variant stream.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_bandwidth(5);
    /// assert_eq!(stream.bandwidth(), 5);
    /// ```
    ///
    /// # Note
    ///
    /// This field is required.
    #[shorthand(disable(into))]
    bandwidth: u64,
    /// The average bandwidth of the stream.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_average_bandwidth(Some(300));
    /// assert_eq!(stream.average_bandwidth(), Some(300));
    /// ```
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    #[shorthand(enable(copy), disable(into, option_as_ref))]
    average_bandwidth: Option<u64>,
    /// A string that represents the list of codec types contained the variant
    /// stream.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_codecs(Some("mp4a.40.2,avc1.4d401e"));
    /// assert_eq!(stream.codecs(), Some(&"mp4a.40.2,avc1.4d401e".to_string()));
    /// ```
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    codecs: Option<String>,
    /// The resolution of the stream.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// use hls_m3u8::types::Resolution;
    ///
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_resolution(Some((1920, 1080)));
    /// assert_eq!(stream.resolution(), Some(Resolution::new(1920, 1080)));
    /// # stream.set_resolution(Some((1280, 10)));
    /// # assert_eq!(stream.resolution(), Some(Resolution::new(1280, 10)));
    /// ```
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    #[shorthand(enable(copy))]
    resolution: Option<Resolution>,
    /// High-bandwidth Digital Content Protection level of the variant stream.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::{HdcpLevel, StreamInf};
    /// #
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_hdcp_level(Some(HdcpLevel::None));
    /// assert_eq!(stream.hdcp_level(), Some(HdcpLevel::None));
    /// ```
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    #[shorthand(enable(copy), disable(into))]
    hdcp_level: Option<HdcpLevel>,
    /// It indicates the set of video renditions, that should be used when
    /// playing the presentation.
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    video: Option<String>,
}

impl StreamInf {
    /// Creates a new [`StreamInf`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let stream = StreamInf::new(20);
    /// ```
    pub const fn new(bandwidth: u64) -> Self {
        Self {
            bandwidth,
            average_bandwidth: None,
            codecs: None,
            resolution: None,
            hdcp_level: None,
            video: None,
        }
    }
}

impl fmt::Display for StreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        if let Some(value) = &self.hdcp_level {
            write!(f, ",HDCP-LEVEL={}", value)?;
        }
        if let Some(value) = &self.video {
            write!(f, ",VIDEO={}", quote(value))?;
        }
        Ok(())
    }
}

impl FromStr for StreamInf {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut bandwidth = None;
        let mut average_bandwidth = None;
        let mut codecs = None;
        let mut resolution = None;
        let mut hdcp_level = None;
        let mut video = None;

        for (key, value) in AttributePairs::new(input) {
            match key {
                "BANDWIDTH" => bandwidth = Some(value.parse::<u64>().map_err(Error::parse_int)?),
                "AVERAGE-BANDWIDTH" => {
                    average_bandwidth = Some(value.parse::<u64>().map_err(Error::parse_int)?)
                }
                "CODECS" => codecs = Some(unquote(value)),
                "RESOLUTION" => resolution = Some(value.parse()?),
                "HDCP-LEVEL" => {
                    hdcp_level = Some(value.parse::<HdcpLevel>().map_err(Error::strum)?)
                }
                "VIDEO" => video = Some(unquote(value)),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized
                    // AttributeName.
                }
            }
        }

        let bandwidth = bandwidth.ok_or_else(|| Error::missing_value("BANDWIDTH"))?;

        Ok(Self {
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
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        let mut stream_inf = StreamInf::new(200);
        stream_inf.set_average_bandwidth(Some(15));
        stream_inf.set_codecs(Some("mp4a.40.2,avc1.4d401e"));
        stream_inf.set_resolution(Some((1920, 1080)));
        stream_inf.set_hdcp_level(Some(HdcpLevel::Type0));
        stream_inf.set_video(Some("video"));

        assert_eq!(
            stream_inf.to_string(),
            "BANDWIDTH=200,\
             AVERAGE-BANDWIDTH=15,\
             CODECS=\"mp4a.40.2,avc1.4d401e\",\
             RESOLUTION=1920x1080,\
             HDCP-LEVEL=TYPE-0,\
             VIDEO=\"video\""
                .to_string()
        );
    }

    #[test]
    fn test_parser() {
        let mut stream_inf = StreamInf::new(200);
        stream_inf.set_average_bandwidth(Some(15));
        stream_inf.set_codecs(Some("mp4a.40.2,avc1.4d401e"));
        stream_inf.set_resolution(Some((1920, 1080)));
        stream_inf.set_hdcp_level(Some(HdcpLevel::Type0));
        stream_inf.set_video(Some("video"));

        assert_eq!(
            stream_inf,
            "BANDWIDTH=200,\
             AVERAGE-BANDWIDTH=15,\
             CODECS=\"mp4a.40.2,avc1.4d401e\",\
             RESOLUTION=1920x1080,\
             HDCP-LEVEL=TYPE-0,\
             VIDEO=\"video\""
                .parse()
                .unwrap()
        );

        assert_eq!(
            stream_inf,
            "BANDWIDTH=200,\
             AVERAGE-BANDWIDTH=15,\
             CODECS=\"mp4a.40.2,avc1.4d401e\",\
             RESOLUTION=1920x1080,\
             HDCP-LEVEL=TYPE-0,\
             VIDEO=\"video\",\
             UNKNOWN=\"value\""
                .parse()
                .unwrap()
        );

        assert!("garbage".parse::<StreamInf>().is_err());
    }
}
