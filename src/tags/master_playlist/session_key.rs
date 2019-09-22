use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::types::{DecryptionKey, EncryptionMethod, ProtocolVersion, RequiredVersion};
use crate::utils::tag;
use crate::Error;

/// [4.3.4.5. EXT-X-SESSION-KEY]
///
/// [4.3.4.5. EXT-X-SESSION-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.4.5
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXSessionKey(DecryptionKey);

impl ExtXSessionKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-KEY:";

    /// Makes a new [ExtXSessionKey] tag.
    /// # Panic
    /// This method will panic, if the [EncryptionMethod] is None.
    pub fn new<T: ToString>(method: EncryptionMethod, uri: T) -> Self {
        if method == EncryptionMethod::None {
            panic!("The EncryptionMethod is not allowed to be None");
        }

        Self(DecryptionKey::new(method, uri))
    }
}

impl RequiredVersion for ExtXSessionKey {
    fn required_version(&self) -> ProtocolVersion {
        if self.0.key_format.is_some() | self.0.key_format_versions.is_some() {
            ProtocolVersion::V5
        } else if self.0.iv.is_some() {
            ProtocolVersion::V2
        } else {
            ProtocolVersion::V1
        }
    }
}

impl fmt::Display for ExtXSessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl FromStr for ExtXSessionKey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;
        Ok(Self(input.parse()?))
    }
}

impl Deref for ExtXSessionKey {
    type Target = DecryptionKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ExtXSessionKey {
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
        let mut key = ExtXSessionKey::new(
            EncryptionMethod::Aes128,
            "https://www.example.com/hls-key/key.bin",
        );
        key.set_iv([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]);

        assert_eq!(
            key.to_string(),
            "#EXT-X-SESSION-KEY:METHOD=AES-128,\
             URI=\"https://www.example.com/hls-key/key.bin\",\
             IV=0x10ef8f758ca555115584bb5b3c687f52"
                .to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            r#"#EXT-X-SESSION-KEY:METHOD=AES-128,URI="https://priv.example.com/key.php?r=52""#
                .parse::<ExtXSessionKey>()
                .unwrap(),
            ExtXSessionKey::new(
                EncryptionMethod::Aes128,
                "https://priv.example.com/key.php?r=52"
            )
        );

        let mut key = ExtXSessionKey::new(
            EncryptionMethod::Aes128,
            "https://www.example.com/hls-key/key.bin",
        );
        key.set_iv([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]);

        assert_eq!(
            "#EXT-X-SESSION-KEY:METHOD=AES-128,\
             URI=\"https://www.example.com/hls-key/key.bin\",\
             IV=0X10ef8f758ca555115584bb5b3c687f52"
                .parse::<ExtXSessionKey>()
                .unwrap(),
            key
        );

        key.set_key_format("baz");

        assert_eq!(
            r#"#EXT-X-SESSION-KEY:METHOD=AES-128,URI="https://www.example.com/hls-key/key.bin",IV=0x10ef8f758ca555115584bb5b3c687f52,KEYFORMAT="baz""#
            .parse::<ExtXSessionKey>().unwrap(),
            key
        )
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXSessionKey::new(EncryptionMethod::Aes128, "https://www.example.com/")
                .required_version(),
            ProtocolVersion::V1
        );
    }
}
