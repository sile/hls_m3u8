use std::convert::TryFrom;
use std::fmt;

use crate::types::{DecryptionKey, ProtocolVersion};
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Specifies how to decrypt encrypted data from the server.
///
/// An unencrypted segment should be marked with [`ExtXKey::empty`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ExtXKey<'a>(pub Option<DecryptionKey<'a>>);

impl<'a> ExtXKey<'a> {
    pub(crate) const PREFIX: &'static str = "#EXT-X-KEY:";

    /// Constructs an [`ExtXKey`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::{DecryptionKey, EncryptionMethod, KeyFormat};
    ///
    /// let key = ExtXKey::new(
    ///     DecryptionKey::builder()
    ///         .method(EncryptionMethod::Aes128)
    ///         .uri("https://www.example.com/")
    ///         .iv([
    ///             16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
    ///         ])
    ///         .format(KeyFormat::Identity)
    ///         .versions(vec![1, 2, 3, 4, 5])
    ///         .build()?,
    /// );
    /// # Ok::<(), String>(())
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(inner: DecryptionKey<'a>) -> Self { Self(Some(inner)) }

    /// Constructs an empty [`ExtXKey`], which signals that a segment is
    /// unencrypted.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// assert_eq!(ExtXKey::empty(), ExtXKey(None));
    /// ```
    #[must_use]
    #[inline]
    pub const fn empty() -> Self { Self(None) }

    /// Returns `true` if the key is not empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::{DecryptionKey, EncryptionMethod};
    ///
    /// let k = ExtXKey::new(DecryptionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.url",
    /// ));
    /// assert_eq!(k.is_some(), true);
    ///
    /// let k = ExtXKey::empty();
    /// assert_eq!(k.is_some(), false);
    /// ```
    #[must_use]
    #[inline]
    pub fn is_some(&self) -> bool { self.0.is_some() }

    /// Returns `true` if the key is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::{DecryptionKey, EncryptionMethod};
    ///
    /// let k = ExtXKey::new(DecryptionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.url",
    /// ));
    /// assert_eq!(k.is_none(), false);
    ///
    /// let k = ExtXKey::empty();
    /// assert_eq!(k.is_none(), true);
    /// ```
    #[must_use]
    #[inline]
    pub fn is_none(&self) -> bool { self.0.is_none() }

    /// Returns the underlying [`DecryptionKey`], if there is one.
    ///
    /// # Panics
    ///
    /// Panics if there is no underlying decryption key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::{DecryptionKey, EncryptionMethod};
    ///
    /// let k = ExtXKey::new(DecryptionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.url",
    /// ));
    ///
    /// assert_eq!(
    ///     k.unwrap(),
    ///     DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.url")
    /// );
    /// ```
    ///
    /// ```{.should_panic}
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::DecryptionKey;
    ///
    /// let decryption_key: DecryptionKey = ExtXKey::empty().unwrap(); // panics
    /// ```
    #[must_use]
    pub fn unwrap(self) -> DecryptionKey<'a> {
        match self.0 {
            Some(v) => v,
            None => panic!("called `ExtXKey::unwrap()` on an empty key"),
        }
    }

    /// Returns a reference to the underlying [`DecryptionKey`].
    #[must_use]
    #[inline]
    pub fn as_ref(&self) -> Option<&DecryptionKey<'a>> { self.0.as_ref() }

    /// Converts an [`ExtXKey`] into an `Option<DecryptionKey>`.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::{DecryptionKey, EncryptionMethod};
    ///
    /// assert_eq!(ExtXKey::empty().into_option(), None);
    ///
    /// assert_eq!(
    ///     ExtXKey::new(DecryptionKey::new(
    ///         EncryptionMethod::Aes128,
    ///         "https://www.example.url"
    ///     ))
    ///     .into_option(),
    ///     Some(DecryptionKey::new(
    ///         EncryptionMethod::Aes128,
    ///         "https://www.example.url"
    ///     ))
    /// );
    /// ```
    #[must_use]
    #[inline]
    pub fn into_option(self) -> Option<DecryptionKey<'a>> { self.0 }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    ///
    /// [`Cow`]: std::borrow::Cow
    #[must_use]
    #[inline]
    pub fn into_owned(self) -> ExtXKey<'static> { ExtXKey(self.0.map(|v| v.into_owned())) }
}

/// This tag requires [`ProtocolVersion::V5`], if [`KeyFormat`] or
/// [`KeyFormatVersions`] is specified and [`ProtocolVersion::V2`] if an iv is
/// specified.
///
/// Otherwise [`ProtocolVersion::V1`] is required.
impl<'a> RequiredVersion for ExtXKey<'a> {
    fn required_version(&self) -> ProtocolVersion {
        self.0
            .as_ref()
            .map_or(ProtocolVersion::V1, |i| i.required_version())
    }
}

impl<'a> TryFrom<&'a str> for ExtXKey<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let input = tag(input, Self::PREFIX)?;

        if input.trim() == "METHOD=NONE" {
            Ok(Self(None))
        } else {
            Ok(DecryptionKey::try_from(input)?.into())
        }
    }
}

impl<'a> From<Option<DecryptionKey<'a>>> for ExtXKey<'a> {
    fn from(value: Option<DecryptionKey<'a>>) -> Self { Self(value) }
}

impl<'a> From<DecryptionKey<'a>> for ExtXKey<'a> {
    fn from(value: DecryptionKey<'a>) -> Self { Self(Some(value)) }
}

impl<'a> From<crate::tags::ExtXSessionKey<'a>> for ExtXKey<'a> {
    fn from(value: crate::tags::ExtXSessionKey<'a>) -> Self { Self(Some(value.0)) }
}

impl<'a> fmt::Display for ExtXKey<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;

        if let Some(value) = &self.0 {
            write!(f, "{}", value)
        } else {
            write!(f, "METHOD=NONE")
        }
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

                assert_eq!(
                    ExtXKey::new(
                        DecryptionKey::new(
                            EncryptionMethod::Aes128,
                            "http://www.example.com"
                        )
                    ),
                    ExtXKey::try_from(concat!(
                        "#EXT-X-KEY:",
                        "METHOD=AES-128,",
                        "URI=\"http://www.example.com\",",
                        "UNKNOWNTAG=abcd"
                    )).unwrap(),
                );
                assert!(ExtXKey::try_from("#EXT-X-KEY:METHOD=AES-128,URI=").is_err());
                assert!(ExtXKey::try_from("garbage").is_err());
            }
        }
    }

    generate_tests! {
        {
            ExtXKey::empty(),
            "#EXT-X-KEY:METHOD=NONE"
        },
        {
            ExtXKey::new(DecryptionKey::new(
                EncryptionMethod::Aes128,
                "https://priv.example.com/key.php?r=52"
            )),
            concat!(
                "#EXT-X-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://priv.example.com/key.php?r=52\""
            )
        },
        {
            ExtXKey::new(
                DecryptionKey::builder()
                    .method(EncryptionMethod::Aes128)
                    .uri("https://www.example.com/hls-key/key.bin")
                    .iv([16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82])
                    .build()
                    .unwrap()
            ),
            concat!(
                "#EXT-X-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0x10ef8f758ca555115584bb5b3c687f52"
            )
        },
        {
            ExtXKey::new(
                DecryptionKey::builder()
                    .method(EncryptionMethod::Aes128)
                    .uri("https://www.example.com/hls-key/key.bin")
                    .iv([16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82])
                    .format(KeyFormat::Identity)
                    .versions(vec![1, 2, 3])
                    .build()
                    .unwrap()
            ),
            concat!(
                "#EXT-X-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0x10ef8f758ca555115584bb5b3c687f52,",
                "KEYFORMAT=\"identity\",",
                "KEYFORMATVERSIONS=\"1/2/3\""
            )
        },
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXKey::new(DecryptionKey::new(
                EncryptionMethod::Aes128,
                "https://www.example.com/"
            ))
            .required_version(),
            ProtocolVersion::V1
        );

        assert_eq!(
            ExtXKey::new(
                DecryptionKey::builder()
                    .method(EncryptionMethod::Aes128)
                    .uri("https://www.example.com/")
                    .format(KeyFormat::Identity)
                    .versions(vec![1, 2, 3])
                    .build()
                    .unwrap()
            )
            .required_version(),
            ProtocolVersion::V5
        );

        assert_eq!(
            ExtXKey::new(
                DecryptionKey::builder()
                    .method(EncryptionMethod::Aes128)
                    .uri("https://www.example.com/")
                    .iv([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7])
                    .build()
                    .unwrap()
            )
            .required_version(),
            ProtocolVersion::V2
        );
    }
}
