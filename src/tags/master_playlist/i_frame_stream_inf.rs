use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::types::{HdcpLevel, ProtocolVersion, StreamInf, StreamInfBuilder};
use crate::utils::{quote, tag, unquote};
use crate::{Error, RequiredVersion};

/// # [4.3.5.3. EXT-X-I-FRAME-STREAM-INF]
///
/// The [`ExtXIFrameStreamInf`] tag identifies a [`Media Playlist`] file,
/// containing the I-frames of a multimedia presentation.
///
/// I-frames are encoded video frames, whose decoding
/// does not depend on any other frame.
///
/// [`Master Playlist`]: crate::MasterPlaylist
/// [`Media Playlist`]: crate::MediaPlaylist
/// [4.3.5.3. EXT-X-I-FRAME-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.5
#[derive(PartialOrd, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXIFrameStreamInf {
    uri: String,
    stream_inf: StreamInf,
}

#[derive(Default, Debug, Clone, PartialEq)]
/// Builder for [`ExtXIFrameStreamInf`].
pub struct ExtXIFrameStreamInfBuilder {
    uri: Option<String>,
    stream_inf: StreamInfBuilder,
}

impl ExtXIFrameStreamInfBuilder {
    /// An `URI` to the [`MediaPlaylist`] file.
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    pub fn uri<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.uri = Some(value.into());
        self
    }

    /// The maximum bandwidth of the stream.
    pub fn bandwidth(&mut self, value: u64) -> &mut Self {
        self.stream_inf.bandwidth(value);
        self
    }

    /// The average bandwidth of the stream.
    pub fn average_bandwidth(&mut self, value: u64) -> &mut Self {
        self.stream_inf.average_bandwidth(value);
        self
    }

    /// Every media format in any of the renditions specified by the Variant
    /// Stream.
    pub fn codecs<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.stream_inf.codecs(value);
        self
    }

    /// The resolution of the stream.
    pub fn resolution(&mut self, value: (usize, usize)) -> &mut Self {
        self.stream_inf.resolution(value);
        self
    }

    /// High-bandwidth Digital Content Protection
    pub fn hdcp_level(&mut self, value: HdcpLevel) -> &mut Self {
        self.stream_inf.hdcp_level(value);
        self
    }

    /// It indicates the set of video renditions, that should be used when
    /// playing the presentation.
    pub fn video<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.stream_inf.video(value);
        self
    }

    /// Build an [`ExtXIFrameStreamInf`].
    pub fn build(&self) -> crate::Result<ExtXIFrameStreamInf> {
        Ok(ExtXIFrameStreamInf {
            uri: self
                .uri
                .clone()
                .ok_or_else(|| Error::missing_value("frame rate"))?,
            stream_inf: self.stream_inf.build().map_err(Error::builder)?,
        })
    }
}

impl ExtXIFrameStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-I-FRAME-STREAM-INF:";

    /// Makes a new [`ExtXIFrameStreamInf`] tag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// let stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    /// ```
    pub fn new<T: ToString>(uri: T, bandwidth: u64) -> Self {
        Self {
            uri: uri.to_string(),
            stream_inf: StreamInf::new(bandwidth),
        }
    }

    /// Returns a builder for [`ExtXIFrameStreamInf`].
    pub fn builder() -> ExtXIFrameStreamInfBuilder { ExtXIFrameStreamInfBuilder::default() }

    /// Returns the `URI`, that identifies the associated [`media playlist`].
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXIFrameStreamInf;
    /// let stream = ExtXIFrameStreamInf::new("https://www.example.com", 20);
    /// assert_eq!(stream.uri(), &"https://www.example.com".to_string());
    /// ```
    ///
    /// [`media playlist`]: crate::MediaPlaylist
    pub const fn uri(&self) -> &String { &self.uri }

    /// Sets the `URI`, that identifies the associated [`media playlist`].
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
    ///
    /// [`media playlist`]: crate::MediaPlaylist
    pub fn set_uri<T: ToString>(&mut self, value: T) -> &mut Self {
        self.uri = value.to_string();
        self
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXIFrameStreamInf {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXIFrameStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "URI={},{}", quote(&self.uri), self.stream_inf)?;
        Ok(())
    }
}

impl FromStr for ExtXIFrameStreamInf {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let mut uri = None;

        for (key, value) in input.parse::<AttributePairs>()? {
            if let "URI" = key.as_str() {
                uri = Some(unquote(value));
            }
        }

        let uri = uri.ok_or_else(|| Error::missing_value("URI"))?;

        Ok(Self {
            uri,
            stream_inf: input.parse()?,
        })
    }
}

impl Deref for ExtXIFrameStreamInf {
    type Target = StreamInf;

    fn deref(&self) -> &Self::Target { &self.stream_inf }
}

impl DerefMut for ExtXIFrameStreamInf {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.stream_inf }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_builder() {
        let mut i_frame_stream_inf =
            ExtXIFrameStreamInf::new("http://example.com/audio-only.m3u8", 200_000);

        i_frame_stream_inf
            .set_average_bandwidth(Some(100_000))
            .set_codecs(Some("mp4a.40.5"))
            .set_resolution(1920, 1080)
            .set_hdcp_level(Some(HdcpLevel::None))
            .set_video(Some("video"));

        assert_eq!(
            ExtXIFrameStreamInf::builder()
                .uri("http://example.com/audio-only.m3u8")
                .bandwidth(200_000)
                .average_bandwidth(100_000)
                .codecs("mp4a.40.5")
                .resolution((1920, 1080))
                .hdcp_level(HdcpLevel::None)
                .video("video")
                .build()
                .unwrap(),
            i_frame_stream_inf
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXIFrameStreamInf::new("foo", 1000).to_string(),
            "#EXT-X-I-FRAME-STREAM-INF:URI=\"foo\",BANDWIDTH=1000".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            "#EXT-X-I-FRAME-STREAM-INF:URI=\"foo\",BANDWIDTH=1000"
                .parse::<ExtXIFrameStreamInf>()
                .unwrap(),
            ExtXIFrameStreamInf::new("foo", 1000)
        );

        assert!("garbage".parse::<ExtXIFrameStreamInf>().is_err());
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXIFrameStreamInf::new("foo", 1000).required_version(),
            ProtocolVersion::V1
        );
    }

    #[test]
    fn test_deref() {
        assert_eq!(
            ExtXIFrameStreamInf::new("https://www.example.com", 20).average_bandwidth(),
            None
        )
    }

    #[test]
    fn test_deref_mut() {
        assert_eq!(
            ExtXIFrameStreamInf::new("https://www.example.com", 20)
                .set_average_bandwidth(Some(4))
                .average_bandwidth(),
            Some(4)
        )
    }
}
