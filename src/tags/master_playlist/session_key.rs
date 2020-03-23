use core::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use derive_more::{AsMut, AsRef, From};

use crate::tags::ExtXKey;
use crate::types::{DecryptionKey, ProtocolVersion};
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// The [`ExtXSessionKey`] tag allows encryption keys from [`MediaPlaylist`]s
/// to be specified in a [`MasterPlaylist`]. This allows the client to
/// preload these keys without having to read the [`MediaPlaylist`]s
/// first.
///
/// If an [`ExtXSessionKey`] is used, the values of [`ExtXKey::method`],
/// [`ExtXKey::key_format`] and [`ExtXKey::key_format_versions`] must match any
/// [`ExtXKey`] with the same uri field.
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
/// [`MasterPlaylist`]: crate::MasterPlaylist
#[derive(AsRef, AsMut, From, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct ExtXSessionKey(pub DecryptionKey);

impl ExtXSessionKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-KEY:";

    /// Makes a new [`ExtXSessionKey`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::{DecryptionKey, EncryptionMethod};
    ///
    /// let session_key = ExtXSessionKey::new(DecryptionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com/",
    /// ));
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(inner: DecryptionKey) -> Self { Self(inner) }
}

impl TryFrom<ExtXKey> for ExtXSessionKey {
    type Error = Error;

    fn try_from(value: ExtXKey) -> Result<Self, Self::Error> {
        if let ExtXKey(Some(inner)) = value {
            Ok(Self(inner))
        } else {
            Err(Error::custom("missing decryption key"))
        }
    }
}

/// This tag requires the same [`ProtocolVersion`] that is returned by
/// `DecryptionKey::required_version`.
impl RequiredVersion for ExtXSessionKey {
    fn required_version(&self) -> ProtocolVersion { self.0.required_version() }
}

impl fmt::Display for ExtXSessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0.to_string())
    }
}

impl FromStr for ExtXSessionKey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(DecryptionKey::from_str(tag(input, Self::PREFIX)?)?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{EncryptionMethod, KeyFormat};
    use pretty_assertions::assert_eq;

    macro_rules! generate_tests {
        ( $( { $struct:expr, $str:expr } ),+ $(,)* ) => {
            #[test]
            fn test_display() {
                $(
                    assert_eq!($struct.to_string(), $str.to_string());
                )+
            }

            #[test]
            fn test_parser() {
                $(
                    assert_eq!($struct, $str.parse().unwrap());
                )+
            }
        }
    }

    generate_tests! {
        {
            ExtXSessionKey::new(
                DecryptionKey::builder()
                    .method(EncryptionMethod::Aes128)
                    .uri("https://www.example.com/hls-key/key.bin")
                    .iv([
                        16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
                    ])
                    .build()
                    .unwrap(),
            ),
            concat!(
                "#EXT-X-SESSION-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0x10ef8f758ca555115584bb5b3c687f52"
            )
        },
        {
            ExtXSessionKey::new(
                DecryptionKey::builder()
                    .method(EncryptionMethod::Aes128)
                    .uri("https://www.example.com/hls-key/key.bin")
                    .iv([
                        16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
                    ])
                    .format(KeyFormat::Identity)
                    .build()
                    .unwrap(),
            ),
            concat!(
                "#EXT-X-SESSION-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0x10ef8f758ca555115584bb5b3c687f52,",
                "KEYFORMAT=\"identity\"",
            )
        }
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXSessionKey::new(DecryptionKey::new(
                EncryptionMethod::Aes128,
                "https://www.example.com/"
            ))
            .required_version(),
            ProtocolVersion::V1
        );
    }
}
