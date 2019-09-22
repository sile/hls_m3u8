use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::types::{ProtocolVersion, RequiredVersion, StreamInf};
use crate::utils::{quote, tag, unquote};
use crate::Error;

/// # [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]
/// The [ExtXIFrameStreamInf] tag identifies a [Media Playlist] file
/// containing the I-frames of a multimedia presentation. It stands
/// alone, in that it does not apply to a particular `URI` in the [Master Playlist].
///
/// Its format is:
///
/// ```text
/// #EXT-X-I-FRAME-STREAM-INF:<attribute-list>
/// ```
///
/// [Master Playlist]: crate::MasterPlaylist
/// [Media Playlist]: crate::MediaPlaylist
/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.3
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXIFrameStreamInf {
    uri: String,
    stream_inf: StreamInf,
}

impl ExtXIFrameStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-I-FRAME-STREAM-INF:";

    /// Makes a new [ExtXIFrameStreamInf] tag.
    pub fn new<T: ToString>(uri: T, bandwidth: u64) -> Self {
        ExtXIFrameStreamInf {
            uri: uri.to_string(),
            stream_inf: StreamInf::new(bandwidth),
        }
    }

    /// Returns the `URI`, that identifies the associated media playlist.
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

    /// Sets the `URI`, that identifies the associated media playlist.
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
}

impl RequiredVersion for ExtXIFrameStreamInf {
    fn required_version(&self) -> ProtocolVersion {
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
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXIFrameStreamInf::new("foo", 1000).required_version(),
            ProtocolVersion::V1
        );
    }
}
