use std::fmt;
use std::str::FromStr;

use shorthand::ShortHand;

use crate::attribute::AttributePairs;
use crate::tags::ExtXKey;
use crate::types::{ByteRange, ProtocolVersion};
use crate::utils::{quote, tag, unquote};
use crate::{Encrypted, Error, RequiredVersion};

/// The [`ExtXMap`] tag specifies how to obtain the [Media Initialization
/// Section], required to parse the applicable [`MediaSegment`]s.
///
/// It applies to every [`MediaSegment`] that appears after it in the playlist
/// until the next [`ExtXMap`] tag or until the end of the playlist.
///
/// An [`ExtXMap`] tag should be supplied for [`MediaSegment`]s in playlists
/// with the [`ExtXIFramesOnly`] tag when the first [`MediaSegment`] (i.e.,
/// I-frame) in the playlist (or the first segment following an
/// [`ExtXDiscontinuity`] tag) does not immediately follow the Media
/// Initialization Section at the beginning of its resource.
///
/// If the Media Initialization Section declared by an [`ExtXMap`] tag is
/// encrypted with [`EncryptionMethod::Aes128`], the IV attribute of
/// the [`ExtXKey`] tag that applies to the [`ExtXMap`] is required.
///
/// [Media Initialization Section]: https://tools.ietf.org/html/rfc8216#section-3
/// [`MediaSegment`]: crate::MediaSegment
/// [`ExtXIFramesOnly`]: crate::tags::ExtXIFramesOnly
/// [`ExtXDiscontinuity`]: crate::tags::ExtXDiscontinuity
/// [`EncryptionMethod::Aes128`]: crate::types::EncryptionMethod::Aes128
/// [`MediaPlaylist`]: crate::MediaPlaylist
#[derive(ShortHand, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[shorthand(enable(must_use, into))]
pub struct ExtXMap {
    /// The `URI` that identifies a resource, that contains the media
    /// initialization section.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXMap;
    /// let mut map = ExtXMap::new("https://prod.mediaspace.com/init.bin");
    /// # assert_eq!(
    /// #     map.uri(),
    /// #     &"https://prod.mediaspace.com/init.bin".to_string()
    /// # );
    /// map.set_uri("https://www.google.com/init.bin");
    ///
    /// assert_eq!(map.uri(), &"https://www.google.com/init.bin".to_string());
    /// ```
    uri: String,
    /// The range of the media initialization section.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXMap;
    /// use hls_m3u8::types::ByteRange;
    ///
    /// let mut map = ExtXMap::with_range("https://prod.mediaspace.com/init.bin", ..9);
    ///
    /// map.set_range(Some(2..5));
    /// assert_eq!(map.range(), Some(ByteRange::from(2..5)));
    /// ```
    #[shorthand(enable(copy))]
    range: Option<ByteRange>,
    #[shorthand(enable(skip))]
    keys: Vec<ExtXKey>,
}

impl ExtXMap {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MAP:";

    /// Makes a new [`ExtXMap`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXMap;
    /// let map = ExtXMap::new("https://prod.mediaspace.com/init.bin");
    /// ```
    pub fn new<T: Into<String>>(uri: T) -> Self {
        Self {
            uri: uri.into(),
            range: None,
            keys: vec![],
        }
    }

    /// Makes a new [`ExtXMap`] tag with the given range.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXMap;
    /// use hls_m3u8::types::ByteRange;
    ///
    /// ExtXMap::with_range("https://prod.mediaspace.com/init.bin", 2..11);
    /// ```
    pub fn with_range<I: Into<String>, B: Into<ByteRange>>(uri: I, range: B) -> Self {
        Self {
            uri: uri.into(),
            range: Some(range.into()),
            keys: vec![],
        }
    }
}

impl Encrypted for ExtXMap {
    fn keys(&self) -> &Vec<ExtXKey> { &self.keys }

    fn keys_mut(&mut self) -> &mut Vec<ExtXKey> { &mut self.keys }
}

/// Use of the [`ExtXMap`] tag in a [`MediaPlaylist`] that contains the
/// [`ExtXIFramesOnly`] tag requires [`ProtocolVersion::V5`] or
/// greater. Use of the [`ExtXMap`] tag in a [`MediaPlaylist`] that does not
/// contain the [`ExtXIFramesOnly`] tag requires [`ProtocolVersion::V6`] or
/// greater.
///
/// [`ExtXIFramesOnly`]: crate::tags::ExtXIFramesOnly
/// [`MediaPlaylist`]: crate::MediaPlaylist
impl RequiredVersion for ExtXMap {
    // this should return ProtocolVersion::V5, if it does not contain an
    // EXT-X-I-FRAMES-ONLY!
    // http://alexzambelli.com/blog/2016/05/04/understanding-hls-versions-and-client-compatibility/
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V6 }

    fn introduced_version(&self) -> ProtocolVersion { ProtocolVersion::V5 }
}

impl fmt::Display for ExtXMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "URI={}", quote(&self.uri))?;

        if let Some(value) = &self.range {
            write!(f, ",BYTERANGE={}", quote(value))?;
        }

        Ok(())
    }
}

impl FromStr for ExtXMap {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let mut uri = None;
        let mut range = None;

        for (key, value) in AttributePairs::new(input) {
            match key {
                "URI" => uri = Some(unquote(value)),
                "BYTERANGE" => {
                    range = Some(unquote(value).parse()?);
                }
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized
                    // AttributeName.
                }
            }
        }

        let uri = uri.ok_or_else(|| Error::missing_value("URI"))?;

        Ok(Self {
            uri,
            range,
            keys: vec![],
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXMap::new("foo").to_string(),
            "#EXT-X-MAP:URI=\"foo\"".to_string(),
        );

        assert_eq!(
            ExtXMap::with_range("foo", ByteRange::from(2..11)).to_string(),
            "#EXT-X-MAP:URI=\"foo\",BYTERANGE=\"9@2\"".to_string(),
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXMap::new("foo"),
            "#EXT-X-MAP:URI=\"foo\"".parse().unwrap()
        );

        assert_eq!(
            ExtXMap::with_range("foo", ByteRange::from(2..11)),
            "#EXT-X-MAP:URI=\"foo\",BYTERANGE=\"9@2\"".parse().unwrap()
        );
        assert_eq!(
            ExtXMap::with_range("foo", ByteRange::from(2..11)),
            "#EXT-X-MAP:URI=\"foo\",BYTERANGE=\"9@2\",UNKNOWN=IGNORED"
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(ExtXMap::new("foo").required_version(), ProtocolVersion::V6);
        assert_eq!(
            ExtXMap::with_range("foo", ByteRange::from(2..11)).required_version(),
            ProtocolVersion::V6
        );
    }

    #[test]
    fn test_encrypted() {
        assert_eq!(ExtXMap::new("foo").keys(), &vec![]);
        assert_eq!(ExtXMap::new("foo").keys_mut(), &mut vec![]);
    }
}
