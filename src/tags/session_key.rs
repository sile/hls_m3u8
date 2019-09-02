use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use crate::error::{Error, ErrorKind};
use crate::types::{DecryptionKey, ProtocolVersion};

/// [4.3.4.5. EXT-X-SESSION-KEY]
///
/// [4.3.4.5. EXT-X-SESSION-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.4.5
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXSessionKey(DecryptionKey);

impl ExtXSessionKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-KEY:";

    /// Makes a new `ExtXSessionKey` tag.
    pub const fn new(key: DecryptionKey) -> Self {
        Self(key)
    }

    /// Returns a decryption key for the playlist.
    pub const fn key(&self) -> Cow<'_, DecryptionKey> {
        Cow::Borrowed(&self.0)
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn required_version(&self) -> ProtocolVersion {
        self.0.required_version()
    }
}

impl fmt::Display for ExtXSessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, &self.0)
    }
}

impl FromStr for ExtXSessionKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let suffix = s.split_at(Self::PREFIX.len()).1;
        let key = track!(suffix.parse())?;

        Ok(Self(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{EncryptionMethod, InitializationVector};

    #[test]
    fn ext_x_session_key() {
        let tag = ExtXSessionKey::new(DecryptionKey {
            method: EncryptionMethod::Aes128,
            uri: "foo".to_string(),
            iv: Some(InitializationVector([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            ])),
            key_format: None,
            key_format_versions: None,
        });

        let text =
            r#"#EXT-X-SESSION-KEY:METHOD=AES-128,URI="foo",IV=0x000102030405060708090a0b0c0d0e0f"#;

        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.required_version(), ProtocolVersion::V2);
    }
}
