use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;
use shorthand::ShortHand;

use crate::attribute::AttributePairs;
use crate::types::{
    EncryptionMethod, InitializationVector, KeyFormat, KeyFormatVersions, ProtocolVersion,
};
use crate::utils::{quote, unquote};
use crate::{Error, RequiredVersion};

/// Specifies how to decrypt encrypted data from the server.
#[derive(ShortHand, Builder, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
#[shorthand(enable(skip, must_use, into))]
#[non_exhaustive]
pub struct DecryptionKey {
    /// The encryption method, which has been used to encrypt the data.
    ///
    /// An [`EncryptionMethod::Aes128`] signals that the data is encrypted using
    /// the Advanced Encryption Standard (AES) with a 128-bit key, Cipher Block
    /// Chaining (CBC), and Public-Key Cryptography Standards #7 (PKCS7)
    /// padding. CBC is restarted on each segment boundary, using either the
    /// [`DecryptionKey::iv`] field or the [`MediaSegment::number`] as the IV.
    ///
    /// An [`EncryptionMethod::SampleAes`] means that the [`MediaSegment`]s
    /// contain media samples, such as audio or video, that are encrypted using
    /// the Advanced Encryption Standard (Aes128). How these media streams are
    /// encrypted and encapsulated in a segment depends on the media encoding
    /// and the media format of the segment.
    ///
    /// ## Note
    ///
    /// This field is required.
    ///
    /// [`MediaSegment::number`]: crate::MediaSegment::number
    /// [`MediaSegment`]: crate::MediaSegment
    pub method: EncryptionMethod,
    /// This uri points to a key file, which contains the cipher key.
    ///
    /// ## Note
    ///
    /// This field is required.
    #[builder(setter(into, strip_option), default)]
    #[shorthand(disable(skip))]
    pub(crate) uri: String,
    /// An initialization vector (IV) is a fixed size input that can be used
    /// along with a secret key for data encryption.
    ///
    /// ## Note
    ///
    /// This field is optional and an absent value indicates that
    /// [`MediaSegment::number`] should be used instead.
    ///
    /// [`MediaSegment::number`]: crate::MediaSegment::number
    #[builder(setter(into, strip_option), default)]
    pub iv: InitializationVector,
    /// A server may offer multiple ways to retrieve a key by providing multiple
    /// [`DecryptionKey`]s with different [`KeyFormat`] values.
    ///
    /// An [`EncryptionMethod::Aes128`] uses 16-octet (16 byte/128 bit) keys. If
    /// the format is [`KeyFormat::Identity`], the key file is a single packed
    /// array of 16 octets (16 byte/128 bit) in binary format.
    ///
    /// ## Note
    ///
    /// This field is optional.
    #[builder(setter(into, strip_option), default)]
    pub format: Option<KeyFormat>,
    /// A list of numbers that can be used to indicate which version(s)
    /// this instance complies with, if more than one version of a particular
    /// [`KeyFormat`] is defined.
    ///
    /// ## Note
    ///
    /// This field is optional.
    #[builder(setter(into, strip_option), default)]
    pub versions: Option<KeyFormatVersions>,
}

impl DecryptionKey {
    #[must_use]
    #[inline]
    pub fn new<I: Into<String>>(method: EncryptionMethod, uri: I) -> Self {
        Self {
            method,
            uri: uri.into(),
            iv: InitializationVector::default(),
            format: None,
            versions: None,
        }
    }

    #[must_use]
    #[inline]
    pub fn builder() -> DecryptionKeyBuilder { DecryptionKeyBuilder::default() }
}

/// This tag requires [`ProtocolVersion::V5`], if [`KeyFormat`] or
/// [`KeyFormatVersions`] is specified and [`ProtocolVersion::V2`] if an iv is
/// specified.
///
/// Otherwise [`ProtocolVersion::V1`] is required.
impl RequiredVersion for DecryptionKey {
    fn required_version(&self) -> ProtocolVersion {
        if self.format.is_some() || self.versions.is_some() {
            ProtocolVersion::V5
        } else if self.iv.is_some() {
            ProtocolVersion::V2
        } else {
            ProtocolVersion::V1
        }
    }
}

impl FromStr for DecryptionKey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut method = None;
        let mut uri = None;
        let mut iv = None;
        let mut format = None;
        let mut versions = None;

        for (key, value) in AttributePairs::new(input) {
            match key {
                "METHOD" => method = Some(value.parse().map_err(Error::strum)?),
                "URI" => {
                    let unquoted_uri = unquote(value);

                    if !unquoted_uri.trim().is_empty() {
                        uri = Some(unquoted_uri);
                    }
                }
                "IV" => iv = Some(value.parse()?),
                "KEYFORMAT" => format = Some(value.parse()?),
                "KEYFORMATVERSIONS" => versions = Some(value.parse()?),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized
                    // AttributeName.
                }
            }
        }

        let method = method.ok_or_else(|| Error::missing_value("METHOD"))?;
        let uri = uri.ok_or_else(|| Error::missing_value("URI"))?;
        let iv = iv.unwrap_or_default();

        Ok(Self {
            method,
            uri,
            iv,
            format,
            versions,
        })
    }
}

impl fmt::Display for DecryptionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "METHOD={},URI={}", self.method, quote(&self.uri))?;

        if let InitializationVector::Aes128(_) = &self.iv {
            write!(f, ",IV={}", &self.iv)?;
        }

        if let Some(value) = &self.format {
            write!(f, ",KEYFORMAT={}", quote(value))?;
        }

        if let Some(value) = &self.versions {
            if !value.is_default() {
                write!(f, ",KEYFORMATVERSIONS={}", value)?;
            }
        }

        Ok(())
    }
}

impl DecryptionKeyBuilder {
    fn validate(&self) -> Result<(), String> {
        // a decryption key must contain a uri and a method
        if self.method.is_none() {
            return Err(Error::missing_field("DecryptionKey", "method").to_string());
        } else if self.uri.is_none() {
            return Err(Error::missing_field("DecryptionKey", "uri").to_string());
        }

        Ok(())
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

                assert_eq!(
                    DecryptionKey::new(EncryptionMethod::Aes128, "http://www.example.com"),
                    concat!(
                        "METHOD=AES-128,",
                        "URI=\"http://www.example.com\",",
                        "UNKNOWNTAG=abcd"
                    ).parse().unwrap(),
                );
                assert!("METHOD=AES-128,URI=".parse::<DecryptionKey>().is_err());
                assert!("garbage".parse::<DecryptionKey>().is_err());
            }
        }
    }

    #[test]
    fn test_builder() {
        let mut key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
        key.iv = [
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]
        .into();
        key.format = Some(KeyFormat::Identity);
        key.versions = Some(vec![1, 2, 3, 4, 5].into());

        assert_eq!(
            DecryptionKey::builder()
                .method(EncryptionMethod::Aes128)
                .uri("https://www.example.com/")
                .iv([16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82])
                .format(KeyFormat::Identity)
                .versions(vec![1, 2, 3, 4, 5])
                .build()
                .unwrap(),
            key
        );

        assert!(DecryptionKey::builder().build().is_err());
        assert!(DecryptionKey::builder()
            .method(EncryptionMethod::Aes128)
            .build()
            .is_err());
    }

    generate_tests! {
        {
            DecryptionKey::new(
                EncryptionMethod::Aes128,
                "https://priv.example.com/key.php?r=52"
            ),
            concat!(
                "METHOD=AES-128,",
                "URI=\"https://priv.example.com/key.php?r=52\""
            )
        },
        {
            DecryptionKey::builder()
                .method(EncryptionMethod::Aes128)
                .uri("https://www.example.com/hls-key/key.bin")
                .iv([16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82])
                .build()
                .unwrap(),
            concat!(
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0x10ef8f758ca555115584bb5b3c687f52"
            )
        },
        {
            DecryptionKey::builder()
                .method(EncryptionMethod::Aes128)
                .uri("https://www.example.com/hls-key/key.bin")
                .iv([16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82])
                .format(KeyFormat::Identity)
                .versions(vec![1, 2, 3])
                .build()
                .unwrap(),
            concat!(
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
            DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/")
                .required_version(),
            ProtocolVersion::V1
        );

        assert_eq!(
            DecryptionKey::builder()
                .method(EncryptionMethod::Aes128)
                .uri("https://www.example.com/")
                .format(KeyFormat::Identity)
                .versions(vec![1, 2, 3])
                .build()
                .unwrap()
                .required_version(),
            ProtocolVersion::V5
        );

        assert_eq!(
            DecryptionKey::builder()
                .method(EncryptionMethod::Aes128)
                .uri("https://www.example.com/")
                .iv([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7])
                .build()
                .unwrap()
                .required_version(),
            ProtocolVersion::V2
        );
    }
}
