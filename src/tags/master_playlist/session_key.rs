use core::convert::TryFrom;
use std::fmt;

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
/// If an [`ExtXSessionKey`] is used, the values of [`DecryptionKey::method`],
/// [`DecryptionKey::format`] and [`DecryptionKey::versions`] must match any
/// [`ExtXKey`] with the same uri field.
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
/// [`MasterPlaylist`]: crate::MasterPlaylist
/// [`ExtXKey`]: crate::tags::ExtXKey
#[derive(AsRef, AsMut, From, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtXSessionKey<'a>(pub DecryptionKey<'a>);

impl<'a> ExtXSessionKey<'a> {
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
    pub const fn new(inner: DecryptionKey<'a>) -> Self { Self(inner) }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    ///
    /// [`Cow`]: std::borrow::Cow
    #[must_use]
    pub fn into_owned(self) -> ExtXSessionKey<'static> { ExtXSessionKey(self.0.into_owned()) }
}

impl<'a> TryFrom<ExtXKey<'a>> for ExtXSessionKey<'a> {
    type Error = Error;

    fn try_from(value: ExtXKey<'a>) -> Result<Self, Self::Error> {
        if let ExtXKey(Some(inner)) = value {
            Ok(Self(inner))
        } else {
            Err(Error::custom("missing decryption key"))
        }
    }
}

/// This tag requires the same [`ProtocolVersion`] that is returned by
/// `DecryptionKey::required_version`.
impl<'a> RequiredVersion for ExtXSessionKey<'a> {
    fn required_version(&self) -> ProtocolVersion { self.0.required_version() }
}

impl<'a> fmt::Display for ExtXSessionKey<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0.to_string())
    }
}

impl<'a> TryFrom<&'a str> for ExtXSessionKey<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Ok(Self(DecryptionKey::try_from(tag(input, Self::PREFIX)?)?))
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
                    assert_eq!($struct, TryFrom::try_from($str).unwrap());
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
