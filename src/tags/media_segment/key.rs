use std::fmt;
use std::str::FromStr;

use derive_more::{Deref, DerefMut};

use crate::types::{DecryptionKey, EncryptionMethod, ProtocolVersion};
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// # [4.3.2.4. EXT-X-KEY]
///
/// [`Media Segment`]s may be encrypted. The [`ExtXKey`] tag specifies how to
/// decrypt them. It applies to every [`Media Segment`] and to every Media
/// Initialization Section declared by an [`ExtXMap`] tag, that appears
/// between it and the next [`ExtXKey`] tag in the Playlist file with the
/// same [`KeyFormat`] attribute (or the end of the Playlist file).
///
/// # Note
///
/// In case of an empty key ([`EncryptionMethod::None`]),
/// all attributes will be ignored.
///
/// [`KeyFormat`]: crate::types::KeyFormat
/// [`ExtXMap`]: crate::tags::ExtXMap
/// [`Media Segment`]: crate::MediaSegment
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[derive(Deref, DerefMut, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtXKey(DecryptionKey);

impl ExtXKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-KEY:";

    /// Makes a new [`ExtXKey`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = ExtXKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "#EXT-X-KEY:METHOD=AES-128,URI=\"https://www.example.com/\""
    /// );
    /// ```
    pub fn new<T: ToString>(method: EncryptionMethod, uri: T) -> Self {
        Self(DecryptionKey::new(method, uri))
    }

    /// Makes a new [`ExtXKey`] tag without a decryption key.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// let key = ExtXKey::empty();
    ///
    /// assert_eq!(key.to_string(), "#EXT-X-KEY:METHOD=NONE");
    /// ```
    pub const fn empty() -> Self {
        Self(DecryptionKey {
            method: EncryptionMethod::None,
            uri: None,
            iv: None,
            key_format: None,
            key_format_versions: None,
        })
    }

    /// Returns whether the [`EncryptionMethod`] is
    /// [`None`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = ExtXKey::empty();
    ///
    /// assert_eq!(key.method() == EncryptionMethod::None, key.is_empty());
    /// ```
    ///
    /// [`None`]: EncryptionMethod::None
    pub fn is_empty(&self) -> bool { self.0.method() == EncryptionMethod::None }
}

/// This tag requires the [`ProtocolVersion`] returned by
/// [`DecryptionKey::required_version`].
impl RequiredVersion for ExtXKey {
    fn required_version(&self) -> ProtocolVersion { self.0.required_version() }
}

impl FromStr for ExtXKey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;
        Ok(Self(input.parse()?))
    }
}

impl fmt::Display for ExtXKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}{}", Self::PREFIX, self.0) }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{EncryptionMethod, KeyFormat};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXKey::empty().to_string(),
            "#EXT-X-KEY:METHOD=NONE".to_string()
        );

        let mut key = ExtXKey::empty();
        // it is expected, that all attributes will be ignored for an empty key!
        key.set_key_format(Some(KeyFormat::Identity));
        key.set_iv(Some([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]));
        key.set_uri(Some("https://www.example.com"));
        key.set_key_format_versions(Some(vec![1, 2, 3]));

        assert_eq!(key.to_string(), "#EXT-X-KEY:METHOD=NONE".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            "#EXT-X-KEY:\
             METHOD=AES-128,\
             URI=\"https://priv.example.com/key.php?r=52\""
                .parse::<ExtXKey>()
                .unwrap(),
            ExtXKey::new(
                EncryptionMethod::Aes128,
                "https://priv.example.com/key.php?r=52"
            )
        );

        let mut key = ExtXKey::new(
            EncryptionMethod::Aes128,
            "https://www.example.com/hls-key/key.bin",
        );
        key.set_iv(Some([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]));
    }
}
