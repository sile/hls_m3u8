use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::types::{ProtocolVersion, StreamInf};
use crate::utils::{quote, tag, unquote};
use crate::Error;

/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]
///
/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.3
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXIFrameStreamInf {
    uri: String,
    stream_inf: StreamInf,
}

impl ExtXIFrameStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-I-FRAME-STREAM-INF:";

    /// Makes a new `ExtXIFrameStreamInf` tag.
    pub fn new<T: ToString>(uri: T, bandwidth: u64) -> Self {
        ExtXIFrameStreamInf {
            uri: uri.to_string(),
            stream_inf: StreamInf::new(bandwidth),
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

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
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
            match key.as_str() {
                "URI" => uri = Some(unquote(value)),
                _ => {}
            }
        }

        let uri = uri.ok_or(Error::missing_value("URI"))?;

        Ok(Self {
            uri,
            stream_inf: input.parse()?,
        })
    }
}

impl Deref for ExtXIFrameStreamInf {
    type Target = StreamInf;

    fn deref(&self) -> &Self::Target {
        &self.stream_inf
    }
}

impl DerefMut for ExtXIFrameStreamInf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stream_inf
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
