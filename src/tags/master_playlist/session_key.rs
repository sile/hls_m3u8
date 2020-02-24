use core::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use derive_more::{Deref, DerefMut};

use crate::tags::ExtXKey;
use crate::types::{EncryptionMethod, ProtocolVersion};
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// # [4.3.4.5. EXT-X-SESSION-KEY]
///
/// The [`ExtXSessionKey`] tag allows encryption keys from [`MediaPlaylist`]s
/// to be specified in a [`MasterPlaylist`]. This allows the client to
/// preload these keys without having to read the [`MediaPlaylist`]s
/// first.
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
/// [`MasterPlaylist`]: crate::MasterPlaylist
/// [4.3.4.5. EXT-X-SESSION-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.4.5
#[derive(Deref, DerefMut, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXSessionKey(ExtXKey);

impl ExtXSessionKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-KEY:";

    /// Makes a new [`ExtXSessionKey`] tag.
    ///
    /// # Panic
    ///
    /// An [`ExtXSessionKey`] should only be used,
    /// if the segments of the stream are encrypted.
    /// Therefore this function will panic,
    /// if the `method` is [`EncryptionMethod::None`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::{ExtXSessionKey, ExtXKey};
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// ExtXSessionKey::new(ExtXKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com/",
    /// ));
    /// ```
    #[must_use]
    pub fn new(inner: ExtXKey) -> Self {
        if inner.method() == EncryptionMethod::None {
            panic!("the encryption method should never be `None`");
        }

        Self(inner)
    }
}

impl TryFrom<ExtXKey> for ExtXSessionKey {
    type Error = Error;

    fn try_from(value: ExtXKey) -> Result<Self, Self::Error> {
        if value.method() == EncryptionMethod::None {
            return Err(Error::custom(
                "the encryption method should never be `None`",
            ));
        }

        Ok(Self(value))
    }
}

/// This tag requires the same [`ProtocolVersion`] that is returned by
/// `DecryptionKey::required_version`.
impl RequiredVersion for ExtXSessionKey {
    fn required_version(&self) -> ProtocolVersion { self.0.required_version() }
}

impl fmt::Display for ExtXSessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: this is not the most elegant solution
        write!(
            f,
            "{}{}",
            Self::PREFIX,
            self.0.to_string().replacen(ExtXKey::PREFIX, "", 1)
        )
    }
}

impl FromStr for ExtXSessionKey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(ExtXKey::parse_from_str(tag(input, Self::PREFIX)?)?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{EncryptionMethod, KeyFormat};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        let mut key = ExtXSessionKey::new(ExtXKey::new(
            EncryptionMethod::Aes128,
            "https://www.example.com/hls-key/key.bin",
        ));

        key.set_iv(Some([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]));

        assert_eq!(
            key.to_string(),
            concat!(
                "#EXT-X-SESSION-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0x10ef8f758ca555115584bb5b3c687f52"
            )
            .to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            concat!(
                "#EXT-X-SESSION-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://priv.example.com/key.php?r=52\""
            )
            .parse::<ExtXSessionKey>()
            .unwrap(),
            ExtXSessionKey::new(ExtXKey::new(
                EncryptionMethod::Aes128,
                "https://priv.example.com/key.php?r=52"
            ))
        );

        let mut key = ExtXSessionKey::new(ExtXKey::new(
            EncryptionMethod::Aes128,
            "https://www.example.com/hls-key/key.bin",
        ));
        key.set_iv(Some([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]));

        assert_eq!(
            concat!(
                "#EXT-X-SESSION-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0X10ef8f758ca555115584bb5b3c687f52"
            )
            .parse::<ExtXSessionKey>()
            .unwrap(),
            key
        );

        key.set_key_format(Some(KeyFormat::Identity));

        assert_eq!(
            concat!(
                "#EXT-X-SESSION-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0x10ef8f758ca555115584bb5b3c687f52,",
                "KEYFORMAT=\"identity\"",
            )
            .parse::<ExtXSessionKey>()
            .unwrap(),
            key
        )
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXSessionKey::new(ExtXKey::new(
                EncryptionMethod::Aes128,
                "https://www.example.com/"
            ))
            .required_version(),
            ProtocolVersion::V1
        );
    }

    // ExtXSessionKey::new should panic, if the provided
    // EncryptionMethod is None!
    #[test]
    #[should_panic = "the encryption method should never be `None`"]
    fn test_new_panic() { let _ = ExtXSessionKey::new(ExtXKey::new(EncryptionMethod::None, "")); }

    #[test]
    fn test_deref() {
        let key = ExtXSessionKey::new(ExtXKey::new(
            EncryptionMethod::Aes128,
            "https://www.example.com/",
        ));

        assert_eq!(key.method(), EncryptionMethod::Aes128);
        assert_eq!(key.uri(), Some(&"https://www.example.com/".into()));
    }

    #[test]
    fn test_deref_mut() {
        let mut key = ExtXSessionKey::new(ExtXKey::new(
            EncryptionMethod::Aes128,
            "https://www.example.com/",
        ));

        key.set_method(EncryptionMethod::None);
        assert_eq!(key.method(), EncryptionMethod::None);
        key.set_uri(Some("https://www.github.com/"));
        assert_eq!(key.uri(), Some(&"https://www.github.com/".into()));
    }
}
