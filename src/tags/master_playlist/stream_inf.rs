use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::types::{
    ClosedCaptions, DecimalFloatingPoint, HdcpLevel, ProtocolVersion, StreamInf, StreamInfBuilder,
};
use crate::utils::{quote, tag, unquote};
use crate::{Error, RequiredVersion};

/// # [4.3.4.2. EXT-X-STREAM-INF]
///
/// The [`ExtXStreamInf`] tag specifies a Variant Stream, which is a set
/// of Renditions that can be combined to play the presentation.  The
/// attributes of the tag provide information about the Variant Stream.
///
/// The URI line that follows the [`ExtXStreamInf`] tag specifies a Media
/// Playlist that carries a rendition of the Variant Stream.  The URI
/// line is REQUIRED.  Clients that do not support multiple video
/// Renditions SHOULD play this Rendition.
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[derive(PartialOrd, Debug, Clone, PartialEq)]
pub struct ExtXStreamInf {
    uri: String,
    frame_rate: Option<DecimalFloatingPoint>,
    audio: Option<String>,
    subtitles: Option<String>,
    closed_captions: Option<ClosedCaptions>,
    stream_inf: StreamInf,
}

#[derive(Default, Debug, Clone)]
/// Builder for [`ExtXStreamInf`].
pub struct ExtXStreamInfBuilder {
    uri: Option<String>,
    frame_rate: Option<DecimalFloatingPoint>,
    audio: Option<String>,
    subtitles: Option<String>,
    closed_captions: Option<ClosedCaptions>,
    stream_inf: StreamInfBuilder,
}

impl ExtXStreamInfBuilder {
    /// An `URI` to the [`MediaPlaylist`] file.
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    pub fn uri<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.uri = Some(value.into());
        self
    }

    /// Maximum frame rate for all the video in the variant stream.
    pub fn frame_rate(&mut self, value: f64) -> &mut Self {
        self.frame_rate = Some(value.into());
        self
    }

    /// The group identifier for the audio in the variant stream.
    pub fn audio<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.audio = Some(value.into());
        self
    }

    /// The group identifier for the subtitles in the variant stream.
    pub fn subtitles<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.subtitles = Some(value.into());
        self
    }

    /// The value of [`ClosedCaptions`] attribute.
    pub fn closed_captions<T: Into<ClosedCaptions>>(&mut self, value: T) -> &mut Self {
        self.closed_captions = Some(value.into());
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

    /// Build an [`ExtXStreamInf`].
    pub fn build(&self) -> crate::Result<ExtXStreamInf> {
        Ok(ExtXStreamInf {
            uri: self
                .uri
                .clone()
                .ok_or_else(|| Error::missing_value("frame rate"))?,
            frame_rate: self.frame_rate,
            audio: self.audio.clone(),
            subtitles: self.subtitles.clone(),
            closed_captions: self.closed_captions.clone(),
            stream_inf: self.stream_inf.build().map_err(Error::builder)?,
        })
    }
}

impl ExtXStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-STREAM-INF:";

    /// Creates a new [`ExtXStreamInf`] tag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// let stream = ExtXStreamInf::new("https://www.example.com/", 20);
    /// ```
    pub fn new<T: ToString>(uri: T, bandwidth: u64) -> Self {
        Self {
            uri: uri.to_string(),
            frame_rate: None,
            audio: None,
            subtitles: None,
            closed_captions: None,
            stream_inf: StreamInf::new(bandwidth),
        }
    }

    /// Returns a builder for [`ExtXStreamInf`].
    pub fn builder() -> ExtXStreamInfBuilder { ExtXStreamInfBuilder::default() }

    /// Returns the `URI` that identifies the associated media playlist.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// let stream = ExtXStreamInf::new("https://www.example.com/", 20);
    ///
    /// assert_eq!(stream.uri(), &"https://www.example.com/".to_string());
    /// ```
    pub const fn uri(&self) -> &String { &self.uri }

    /// Sets the `URI` that identifies the associated media playlist.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// let mut stream = ExtXStreamInf::new("https://www.example.com/", 20);
    ///
    /// stream.set_uri("https://www.google.com/");
    /// assert_eq!(stream.uri(), &"https://www.google.com/".to_string());
    /// ```
    pub fn set_uri<T: ToString>(&mut self, value: T) -> &mut Self {
        self.uri = value.to_string();
        self
    }

    /// Sets the maximum frame rate for all the video in the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// let mut stream = ExtXStreamInf::new("https://www.example.com/", 20);
    /// # assert_eq!(stream.frame_rate(), None);
    ///
    /// stream.set_frame_rate(Some(59.9));
    /// assert_eq!(stream.frame_rate(), Some(59.9));
    /// ```
    pub fn set_frame_rate(&mut self, value: Option<f64>) -> &mut Self {
        self.frame_rate = value.map(Into::into);
        self
    }

    /// Returns the maximum frame rate for all the video in the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// let mut stream = ExtXStreamInf::new("https://www.example.com/", 20);
    /// # assert_eq!(stream.frame_rate(), None);
    ///
    /// stream.set_frame_rate(Some(59.9));
    /// assert_eq!(stream.frame_rate(), Some(59.9));
    /// ```
    pub fn frame_rate(&self) -> Option<f64> { self.frame_rate.map(|v| v.as_f64()) }

    /// Returns the group identifier for the audio in the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// let mut stream = ExtXStreamInf::new("https://www.example.com/", 20);
    /// # assert_eq!(stream.audio(), &None);
    ///
    /// stream.set_audio(Some("audio"));
    /// assert_eq!(stream.audio(), &Some("audio".to_string()));
    /// ```
    pub const fn audio(&self) -> &Option<String> { &self.audio }

    /// Sets the group identifier for the audio in the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// let mut stream = ExtXStreamInf::new("https://www.example.com/", 20);
    /// # assert_eq!(stream.audio(), &None);
    ///
    /// stream.set_audio(Some("audio"));
    /// assert_eq!(stream.audio(), &Some("audio".to_string()));
    /// ```
    pub fn set_audio<T: Into<String>>(&mut self, value: Option<T>) -> &mut Self {
        self.audio = value.map(Into::into);
        self
    }

    /// Returns the group identifier for the subtitles in the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// let mut stream = ExtXStreamInf::new("https://www.example.com/", 20);
    /// # assert_eq!(stream.subtitles(), &None);
    ///
    /// stream.set_subtitles(Some("subs"));
    /// assert_eq!(stream.subtitles(), &Some("subs".to_string()));
    /// ```
    pub const fn subtitles(&self) -> &Option<String> { &self.subtitles }

    /// Sets the group identifier for the subtitles in the variant stream.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// let mut stream = ExtXStreamInf::new("https://www.example.com/", 20);
    /// # assert_eq!(stream.subtitles(), &None);
    ///
    /// stream.set_subtitles(Some("subs"));
    /// assert_eq!(stream.subtitles(), &Some("subs".to_string()));
    /// ```
    pub fn set_subtitles<T: Into<String>>(&mut self, value: Option<T>) -> &mut Self {
        self.subtitles = value.map(Into::into);
        self
    }

    /// Returns the value of [`ClosedCaptions`] attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// use hls_m3u8::types::ClosedCaptions;
    ///
    /// let mut stream = ExtXStreamInf::new("https://www.example.com/", 20);
    /// # assert_eq!(stream.closed_captions(), &None);
    ///
    /// stream.set_closed_captions(Some(ClosedCaptions::None));
    /// assert_eq!(stream.closed_captions(), &Some(ClosedCaptions::None));
    /// ```
    pub const fn closed_captions(&self) -> &Option<ClosedCaptions> { &self.closed_captions }

    /// Sets the value of [`ClosedCaptions`] attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXStreamInf;
    /// use hls_m3u8::types::ClosedCaptions;
    ///
    /// let mut stream = ExtXStreamInf::new("https://www.example.com/", 20);
    /// # assert_eq!(stream.closed_captions(), &None);
    ///
    /// stream.set_closed_captions(Some(ClosedCaptions::None));
    /// assert_eq!(stream.closed_captions(), &Some(ClosedCaptions::None));
    /// ```
    pub fn set_closed_captions(&mut self, value: Option<ClosedCaptions>) -> &mut Self {
        self.closed_captions = value;
        self
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXStreamInf {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.stream_inf)?;
        if let Some(value) = &self.frame_rate {
            write!(f, ",FRAME-RATE={:.3}", value.as_f64())?;
        }
        if let Some(value) = &self.audio {
            write!(f, ",AUDIO={}", quote(value))?;
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
        let first_line = lines
            .next()
            .ok_or_else(|| Error::missing_value("first_line"))?;
        let uri = lines.next().ok_or_else(|| Error::missing_value("URI"))?;

        let input = tag(first_line, Self::PREFIX)?;

        let mut frame_rate = None;
        let mut audio = None;
        let mut subtitles = None;
        let mut closed_captions = None;

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "FRAME-RATE" => frame_rate = Some((value.parse())?),
                "AUDIO" => audio = Some(unquote(value)),
                "SUBTITLES" => subtitles = Some(unquote(value)),
                "CLOSED-CAPTIONS" => closed_captions = Some(value.parse().unwrap()),
                _ => {}
            }
        }

        Ok(Self {
            uri: uri.to_string(),
            frame_rate,
            audio,
            subtitles,
            closed_captions,
            stream_inf: input.parse()?,
        })
    }
}

impl Deref for ExtXStreamInf {
    type Target = StreamInf;

    fn deref(&self) -> &Self::Target { &self.stream_inf }
}

impl DerefMut for ExtXStreamInf {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.stream_inf }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

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
    fn test_display() {
        assert_eq!(
            ExtXStreamInf::new("http://www.example.com/", 1000).to_string(),
            "#EXT-X-STREAM-INF:BANDWIDTH=1000\nhttp://www.example.com/".to_string()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ProtocolVersion::V1,
            ExtXStreamInf::new("http://www.example.com", 1000).required_version()
        );
    }

    #[test]
    fn test_deref() {
        assert_eq!(
            ExtXStreamInf::new("http://www.example.com", 1000).bandwidth(),
            1000
        );
    }

    #[test]
    fn test_deref_mut() {
        assert_eq!(
            ExtXStreamInf::new("http://www.example.com", 1000)
                .set_bandwidth(1)
                .bandwidth(),
            1
        );
    }
}
