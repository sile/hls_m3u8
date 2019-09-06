use crate::types::{DecryptionKey, ProtocolVersion};
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;


/// [4.3.4.5. EXT-X-SESSION-KEY]
///
/// [4.3.4.5. EXT-X-SESSION-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.4.5
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXSessionKey {
    key: DecryptionKey,
}

impl ExtXSessionKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-KEY:";

    /// Makes a new `ExtXSessionKey` tag.
    pub fn new(key: DecryptionKey) -> Self {
        ExtXSessionKey { key }
    }

    /// Returns a decryption key for the playlist.
    pub fn key(&self) -> &DecryptionKey {
        &self.key
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        self.key.requires_version()
    }
}

impl fmt::Display for ExtXSessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.key)
    }
}

impl FromStr for ExtXSessionKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let suffix = s.split_at(Self::PREFIX.len()).1;
        let key = track!(suffix.parse())?;
        Ok(ExtXSessionKey { key })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{EncryptionMethod, InitializationVector, QuotedString};

    #[test]
    fn ext_x_session_key() {
        let tag = ExtXSessionKey::new(DecryptionKey {
            method: EncryptionMethod::Aes128,
            uri: quoted_string("foo"),
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
        assert_eq!(tag.requires_version(), ProtocolVersion::V2);
    }

    fn quoted_string(s: &str) -> QuotedString {
        QuotedString::new(s).unwrap()
    }
}
