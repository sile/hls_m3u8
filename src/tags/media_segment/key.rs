use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::types::{DecryptionKey, EncryptionMethod};
use crate::utils::tag;
use crate::Error;

/// [4.3.2.4. EXT-X-KEY]
///
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
/// # Note
/// In case of an empty key (`EncryptionMethod::None`), all attributes will be ignored.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXKey(DecryptionKey);

impl ExtXKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-KEY:";

    /// Makes a new `ExtXKey` tag.
    /// # Examples
    /// ```
    /// use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = ExtXKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com/"
    /// );
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "#EXT-X-KEY:METHOD=AES-128,URI=\"https://www.example.com/\""
    /// );
    /// ```
    pub fn new<T: ToString>(method: EncryptionMethod, uri: T) -> Self {
        Self(DecryptionKey::new(method, uri))
    }

    /// Makes a new `ExtXKey` tag without a decryption key.
    /// # Examples
    /// ```
    /// use hls_m3u8::tags::ExtXKey;
    ///
    /// let key = ExtXKey::empty();
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "#EXT-X-KEY:METHOD=NONE"
    /// );
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

    /// Returns whether the [EncryptionMethod] is [None](EncryptionMethod::None).
    /// # Examples
    /// ```
    /// use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = ExtXKey::empty();
    ///
    /// assert_eq!(
    ///     key.method() == EncryptionMethod::None,
    ///     key.is_empty()
    /// );
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.method() == EncryptionMethod::None
    }
}

impl FromStr for ExtXKey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;
        Ok(Self(input.parse()?))
    }
}

impl fmt::Display for ExtXKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl Deref for ExtXKey {
    type Target = DecryptionKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ExtXKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::EncryptionMethod;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXKey::empty().to_string(),
            "#EXT-X-KEY:METHOD=NONE".to_string()
        );

        let mut key = ExtXKey::empty();
        // it is expected, that all attributes will be ignored in an empty key!
        key.set_key_format("hi");
        key.set_iv([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]);
        key.set_uri(Some("https://www.example.com"));
        key.set_key_format_versions("1/2/3");

        assert_eq!(key.to_string(), "#EXT-X-KEY:METHOD=NONE".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            r#"#EXT-X-KEY:METHOD=AES-128,URI="https://priv.example.com/key.php?r=52""#
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
        key.set_iv([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]);
    }
}
