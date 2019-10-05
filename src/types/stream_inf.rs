use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;

use crate::attribute::AttributePairs;
use crate::types::{DecimalResolution, HdcpLevel};
use crate::utils::{quote, unquote};
use crate::Error;

/// # [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[derive(Builder, PartialOrd, Debug, Clone, PartialEq, Eq, Hash)]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug, PartialEq))]
pub struct StreamInf {
    /// The maximum bandwidth of the stream.
    bandwidth: u64,
    #[builder(default)]
    /// The average bandwidth of the stream.
    average_bandwidth: Option<u64>,
    #[builder(default)]
    /// Every media format in any of the renditions specified by the Variant
    /// Stream.
    codecs: Option<String>,
    #[builder(default)]
    /// The resolution of the stream.
    resolution: Option<DecimalResolution>,
    #[builder(default)]
    /// High-bandwidth Digital Content Protection
    hdcp_level: Option<HdcpLevel>,
    #[builder(default)]
    /// It indicates the set of video renditions, that should be used when
    /// playing the presentation.
    video: Option<String>,
}

impl StreamInf {
    /// Creates a new [`StreamInf`].
    ///
    /// # Examples
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

    /// Returns the peak segment bit rate of the variant stream.
    ///
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let stream = StreamInf::new(20);
    /// assert_eq!(stream.bandwidth(), 20);
    /// ```
    pub const fn bandwidth(&self) -> u64 { self.bandwidth }

    /// Sets the peak segment bit rate of the variant stream.
    ///
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_bandwidth(5);
    /// assert_eq!(stream.bandwidth(), 5);
    /// ```
    pub fn set_bandwidth(&mut self, value: u64) -> &mut Self {
        self.bandwidth = value;
        self
    }

    /// Returns the group identifier for the video in the variant stream.
    ///
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let stream = StreamInf::new(20);
    /// assert_eq!(stream.video(), &None);
    /// ```
    pub const fn video(&self) -> &Option<String> { &self.video }

    /// Sets the group identifier for the video in the variant stream.
    ///
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_video(Some("video"));
    /// assert_eq!(stream.video(), &Some("video".to_string()));
    /// ```
    pub fn set_video<T: ToString>(&mut self, value: Option<T>) -> &mut Self {
        self.video = value.map(|v| v.to_string());
        self
    }

    /// Returns the average segment bit rate of the variant stream.
    ///
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let stream = StreamInf::new(20);
    /// assert_eq!(stream.average_bandwidth(), None);
    /// ```
    pub const fn average_bandwidth(&self) -> Option<u64> { self.average_bandwidth }

    /// Sets the average segment bit rate of the variant stream.
    ///
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_average_bandwidth(Some(300));
    /// assert_eq!(stream.average_bandwidth(), Some(300));
    /// ```
    pub fn set_average_bandwidth(&mut self, value: Option<u64>) -> &mut Self {
        self.average_bandwidth = value;
        self
    }

    /// A string that represents the list of codec types contained the variant
    /// stream.
    ///
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let stream = StreamInf::new(20);
    /// assert_eq!(stream.codecs(), &None);
    /// ```
    pub const fn codecs(&self) -> &Option<String> { &self.codecs }

    /// A string that represents the list of codec types contained the variant
    /// stream.
    ///
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_codecs(Some("mp4a.40.2,avc1.4d401e"));
    /// assert_eq!(stream.codecs(), &Some("mp4a.40.2,avc1.4d401e".to_string()));
    /// ```
    pub fn set_codecs<T: ToString>(&mut self, value: Option<T>) -> &mut Self {
        self.codecs = value.map(|v| v.to_string());
        self
    }

    /// Returns the resolution of the stream.
    ///
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let stream = StreamInf::new(20);
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
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_resolution(1920, 1080);
    /// assert_eq!(stream.resolution(), Some((1920, 1080)));
    /// # stream.set_resolution(1280, 10);
    /// # assert_eq!(stream.resolution(), Some((1280, 10)));
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
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::StreamInf;
    /// #
    /// let stream = StreamInf::new(20);
    /// assert_eq!(stream.hdcp_level(), None);
    /// ```
    pub const fn hdcp_level(&self) -> Option<HdcpLevel> { self.hdcp_level }

    /// The HDCP level of the variant stream.
    ///
    /// # Examples
    /// ```
    /// # use hls_m3u8::types::{HdcpLevel, StreamInf};
    /// #
    /// let mut stream = StreamInf::new(20);
    ///
    /// stream.set_hdcp_level(Some(HdcpLevel::None));
    /// assert_eq!(stream.hdcp_level(), Some(HdcpLevel::None));
    /// ```
    pub fn set_hdcp_level<T: Into<HdcpLevel>>(&mut self, value: Option<T>) -> &mut Self {
        self.hdcp_level = value.map(|v| v.into());
        self
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

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "BANDWIDTH" => bandwidth = Some(value.parse::<u64>()?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some(value.parse::<u64>()?),
                "CODECS" => codecs = Some(unquote(value)),
                "RESOLUTION" => resolution = Some(value.parse()?),
                "HDCP-LEVEL" => hdcp_level = Some(value.parse()?),
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

    #[test]
    fn test_display() {
        let mut stream_inf = StreamInf::new(200);
        stream_inf.set_average_bandwidth(Some(15));
        stream_inf.set_codecs(Some("mp4a.40.2,avc1.4d401e"));
        stream_inf.set_resolution(1920, 1080);
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
        stream_inf.set_resolution(1920, 1080);
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
