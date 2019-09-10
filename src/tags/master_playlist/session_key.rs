use std::fmt;
use std::str::FromStr;

use crate::types::{DecryptionKey, ProtocolVersion};
use crate::utils::tag;

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
    pub const fn new(key: DecryptionKey) -> Self {
        ExtXSessionKey { key }
    }

    /// Returns a decryption key for the playlist.
    pub const fn key(&self) -> &DecryptionKey {
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
    type Err = crate::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let key = tag(input, Self::PREFIX)?.parse()?;
        Ok(Self::new(key))
    }
}

#[cfg(test)]
mod test {
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
        assert_eq!(tag.requires_version(), ProtocolVersion::V2);
    }
}
