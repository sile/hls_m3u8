use crate::attribute::AttributePairs;
use crate::types::{DecryptionKey, ProtocolVersion};
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// [4.3.2.4. EXT-X-KEY]
///
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXKey {
    key: Option<DecryptionKey>,
}

impl ExtXKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-KEY:";

    /// Makes a new `ExtXKey` tag.
    pub const fn new(key: DecryptionKey) -> Self {
        ExtXKey { key: Some(key) }
    }

    /// Makes a new `ExtXKey` tag without a decryption key.
    ///
    /// This tag has the `METHDO=NONE` attribute.
    pub const fn new_without_key() -> Self {
        ExtXKey { key: None }
    }

    /// Returns the decryption key for the following media segments and media initialization sections.
    pub fn key(&self) -> Option<&DecryptionKey> {
        self.key.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        self.key
            .as_ref()
            .map_or(ProtocolVersion::V1, |k| k.requires_version())
    }
}

impl fmt::Display for ExtXKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        if let Some(ref key) = self.key {
            write!(f, "{}", key)?;
        } else {
            write!(f, "METHOD=NONE")?;
        }
        Ok(())
    }
}

impl FromStr for ExtXKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let suffix = s.split_at(Self::PREFIX.len()).1;

        if AttributePairs::parse(suffix).any(|a| a.as_ref().ok() == Some(&("METHOD", "NONE"))) {
            for attr in AttributePairs::parse(suffix) {
                let (key, _) = track!(attr)?;
                track_assert_ne!(key, "URI", ErrorKind::InvalidInput);
                track_assert_ne!(key, "IV", ErrorKind::InvalidInput);
                track_assert_ne!(key, "KEYFORMAT", ErrorKind::InvalidInput);
                track_assert_ne!(key, "KEYFORMATVERSIONS", ErrorKind::InvalidInput);
            }
            Ok(ExtXKey { key: None })
        } else {
            let key = track!(suffix.parse())?;
            Ok(ExtXKey { key: Some(key) })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{EncryptionMethod, InitializationVector};

    #[test]
    fn ext_x_key() {
        let tag = ExtXKey::new_without_key();
        let text = "#EXT-X-KEY:METHOD=NONE";
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);

        let tag = ExtXKey::new(DecryptionKey {
            method: EncryptionMethod::Aes128,
            uri: "foo".to_string(),
            iv: None,
            key_format: None,
            key_format_versions: None,
        });
        let text = r#"#EXT-X-KEY:METHOD=AES-128,URI="foo""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);

        let tag = ExtXKey::new(DecryptionKey {
            method: EncryptionMethod::Aes128,
            uri: "foo".to_string(),
            iv: Some(InitializationVector([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            ])),
            key_format: None,
            key_format_versions: None,
        });
        let text = r#"#EXT-X-KEY:METHOD=AES-128,URI="foo",IV=0x000102030405060708090a0b0c0d0e0f"#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V2);

        let tag = ExtXKey::new(DecryptionKey {
            method: EncryptionMethod::Aes128,
            uri: "foo".to_string(),
            iv: Some(InitializationVector([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            ])),
            key_format: Some("baz".to_string()),
            key_format_versions: None,
        });
        let text = r#"#EXT-X-KEY:METHOD=AES-128,URI="foo",IV=0x000102030405060708090a0b0c0d0e0f,KEYFORMAT="baz""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V5);
    }
}
