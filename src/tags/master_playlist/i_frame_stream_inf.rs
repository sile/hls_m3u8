use std::fmt;
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::types::{DecimalResolution, HdcpLevel, ProtocolVersion};
use crate::utils::parse_u64;
use crate::utils::{quote, tag, unquote};
use crate::Error;

/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]
///
/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.3
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXIFrameStreamInf {
    uri: String,
    bandwidth: u64,
    average_bandwidth: Option<u64>,
    codecs: Option<String>,
    resolution: Option<DecimalResolution>,
    hdcp_level: Option<HdcpLevel>,
    video: Option<String>,
}

impl ExtXIFrameStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-I-FRAME-STREAM-INF:";

    /// Makes a new `ExtXIFrameStreamInf` tag.
    pub fn new<T: ToString>(uri: T, bandwidth: u64) -> Self {
        ExtXIFrameStreamInf {
            uri: uri.to_string(),
            bandwidth,
            average_bandwidth: None,
            codecs: None,
            resolution: None,
            hdcp_level: None,
            video: None,
        }
    }

    /// Returns the URI, that identifies the associated media playlist.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    /// assert_eq!(stream.uri(), &"https://www.example.com".to_string());
    /// ```
    pub const fn uri(&self) -> &String {
        &self.uri
    }

    /// Sets the URI, that identifies the associated media playlist.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let mut stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    ///
    /// stream.set_uri("../new/uri");
    /// assert_eq!(stream.uri(), &"../new/uri".to_string());
    /// ```
    pub fn set_uri<T: ToString>(&mut self, value: T) -> &mut Self {
        self.uri = value.to_string();
        self
    }

    /// Returns the peak segment bit rate of the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    /// assert_eq!(stream.bandwidth(), 20);
    /// ```
    pub const fn bandwidth(&self) -> u64 {
        self.bandwidth
    }

    /// Sets the group identifier for the video in the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let mut stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    ///
    /// stream.set_video(Some("video"));
    /// assert_eq!(stream.video(), &Some("video".to_string()));
    /// ```
    pub fn set_video<T: ToString>(&mut self, value: Option<T>) -> &mut Self {
        self.video = value.map(|v| v.to_string());
        self
    }

    /// Returns the group identifier for the video in the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    /// assert_eq!(stream.video(), &None);
    /// ```
    pub const fn video(&self) -> &Option<String> {
        &self.video
    }

    /// Sets the peak segment bit rate of the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let mut stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    ///
    /// stream.set_bandwidth(5);
    /// assert_eq!(stream.bandwidth(), 5);
    /// ```
    pub fn set_bandwidth(&mut self, value: u64) -> &mut Self {
        self.bandwidth = value;
        self
    }

    /// Returns the average segment bit rate of the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    /// assert_eq!(stream.average_bandwidth(), None);
    /// ```
    pub const fn average_bandwidth(&self) -> Option<u64> {
        self.average_bandwidth
    }

    /// Sets the average segment bit rate of the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let mut stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    ///
    /// stream.set_average_bandwidth(Some(300));
    /// assert_eq!(stream.average_bandwidth(), Some(300));
    /// ```
    pub fn set_average_bandwidth(&mut self, value: Option<u64>) -> &mut Self {
        self.average_bandwidth = value;
        self
    }

    /// A string that represents the list of codec types contained the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    /// assert_eq!(stream.codecs(), &None);
    /// ```
    pub const fn codecs(&self) -> &Option<String> {
        &self.codecs
    }

    /// A string that represents the list of codec types contained the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let mut stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    ///
    /// stream.set_uri("../new/uri");
    /// assert_eq!(stream.uri(), &"../new/uri".to_string());
    /// ```
    pub fn set_codecs<T: ToString>(&mut self, value: Option<T>) -> &mut Self {
        self.codecs = value.map(|v| v.to_string());
        self
    }

    /// Returns the resolution of the stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    /// assert_eq!(stream.resolution(), None);
    /// ```
    pub fn resolution(&self) -> Option<(usize, usize)> {
        if let Some(res) = &self.resolution {
            Some((res.width(), res.height()))
        } else {
            None
        }
    }

    /// Sets the resolution of the stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let mut stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    ///
    /// stream.set_resolution(1920, 1080);
    /// assert_eq!(stream.resolution(), Some((1920, 1080)));
    /// ```
    pub fn set_resolution(&mut self, width: usize, height: usize) -> &mut Self {
        if let Some(res) = &mut self.resolution {
            res.set_width(width);
            res.set_height(height);
        } else {
            self.resolution = Some(DecimalResolution::new(width, height));
        }
        self
    }

    /// The HDCP level of the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    /// assert_eq!(stream.uri(), &"https://www.example.com".to_string());
    /// ```
    pub const fn hdcp_level(&self) -> Option<HdcpLevel> {
        self.hdcp_level
    }

    /// The HDCP level of the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// #
    /// let mut stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    ///
    /// stream.set_uri("../new/uri");
    /// assert_eq!(stream.uri(), &"../new/uri".to_string());
    /// ```
    pub fn set_hdcp_level<T: Into<HdcpLevel>>(&mut self, value: Option<T>) -> &mut Self {
        self.hdcp_level = value.map(|v| v.into());
        self
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXIFrameStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "URI={}", quote(&self.uri))?;
        write!(f, ",BANDWIDTH={}", self.bandwidth)?;

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

impl FromStr for ExtXIFrameStreamInf {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let mut uri = None;
        let mut bandwidth = None;
        let mut average_bandwidth = None;
        let mut codecs = None;
        let mut resolution = None;
        let mut hdcp_level = None;
        let mut video = None;

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "URI" => uri = Some(unquote(value)),
                "BANDWIDTH" => bandwidth = Some(parse_u64(value)?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some(parse_u64(value)?),
                "CODECS" => codecs = Some(unquote(value)),
                "RESOLUTION" => resolution = Some(value.parse()?),
                "HDCP-LEVEL" => hdcp_level = Some(value.parse()?),
                "VIDEO" => video = Some(unquote(value)),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let uri = uri.ok_or(Error::missing_value("URI"))?;
        let bandwidth = bandwidth.ok_or(Error::missing_value("BANDWIDTH"))?;

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
    fn test_display() {
        let text = r#"#EXT-X-I-FRAME-STREAM-INF:URI="foo",BANDWIDTH=1000"#;
        assert_eq!(ExtXIFrameStreamInf::new("foo", 1000).to_string(), text);
    }

    #[test]
    fn test_parser() {
        let text = r#"#EXT-X-I-FRAME-STREAM-INF:URI="foo",BANDWIDTH=1000"#;
        let i_frame_stream_inf = ExtXIFrameStreamInf::new("foo", 1000);
        assert_eq!(
            text.parse::<ExtXIFrameStreamInf>().unwrap(),
            i_frame_stream_inf.clone()
        );

        assert_eq!(i_frame_stream_inf.uri(), "foo");
        assert_eq!(i_frame_stream_inf.bandwidth(), 1000);
        // TODO: test all the optional fields
    }

    #[test]
    fn test_requires_version() {
        assert_eq!(
            ExtXIFrameStreamInf::new("foo", 1000).requires_version(),
            ProtocolVersion::V1
        );
    }
}
