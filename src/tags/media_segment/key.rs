use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;
use shorthand::ShortHand;

use crate::attribute::AttributePairs;
use crate::types::{EncryptionMethod, KeyFormat, KeyFormatVersions, ProtocolVersion};
use crate::utils::{parse_iv_from_str, quote, tag, unquote};
use crate::{Error, RequiredVersion};

/// # [4.3.2.4. EXT-X-KEY]
///
/// [`Media Segment`]s may be encrypted. The [`ExtXKey`] tag specifies how to
/// decrypt them. It applies to every [`Media Segment`] and to every Media
/// Initialization Section declared by an [`ExtXMap`] tag, that appears
/// between it and the next [`ExtXKey`] tag in the Playlist file with the
/// same [`KeyFormat`] attribute (or the end of the Playlist file).
///
/// # Note
///
/// In case of an empty key ([`EncryptionMethod::None`]),
/// all attributes will be ignored.
///
/// [`KeyFormat`]: crate::types::KeyFormat
/// [`ExtXMap`]: crate::tags::ExtXMap
/// [`Media Segment`]: crate::MediaSegment
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[derive(ShortHand, Builder, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
#[shorthand(enable(must_use, into))]
pub struct ExtXKey {
    /// HLS supports multiple [`EncryptionMethod`]s for a [`MediaSegment`].
    ///
    /// For example `AES-128`.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_method(EncryptionMethod::SampleAes);
    ///
    /// assert_eq!(key.method(), EncryptionMethod::SampleAes);
    /// ```
    ///
    /// # Note
    ///
    /// This attribute is required.
    #[shorthand(enable(copy))]
    pub(crate) method: EncryptionMethod,
    /// An `URI` that specifies how to obtain the key.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_uri(Some("http://www.google.com/"));
    ///
    /// assert_eq!(key.uri(), Some(&"http://www.google.com/".to_string()));
    /// ```
    ///
    /// # Note
    ///
    /// This attribute is required, if the [`EncryptionMethod`] is not `None`.
    #[builder(setter(into, strip_option), default)]
    pub(crate) uri: Option<String>,
    /// An IV (initialization vector) is used to prevent repetitions between
    /// segments of encrypted data.
    ///
    /// <https://en.wikipedia.org/wiki/Initialization_vector>
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    /// # assert_eq!(key.iv(), None);
    ///
    /// key.set_iv(Some([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7]));
    ///
    /// assert_eq!(
    ///     key.iv(),
    ///     Some([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7])
    /// );
    /// ```
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(setter(into, strip_option), default)]
    // TODO: workaround for https://github.com/Luro02/shorthand/issues/20
    #[shorthand(enable(copy), disable(option_as_ref))]
    pub(crate) iv: Option<[u8; 16]>,
    /// The [`KeyFormat`] specifies how the key is
    /// represented in the resource identified by the `URI`.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::{EncryptionMethod, KeyFormat};
    ///
    /// let mut key = ExtXKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_key_format(Some(KeyFormat::Identity));
    ///
    /// assert_eq!(key.key_format(), Some(KeyFormat::Identity));
    /// ```
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(setter(into, strip_option), default)]
    #[shorthand(enable(copy))]
    pub(crate) key_format: Option<KeyFormat>,
    /// The [`KeyFormatVersions`] attribute.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::{EncryptionMethod, KeyFormatVersions};
    ///
    /// let mut key = ExtXKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_key_format_versions(Some(vec![1, 2, 3, 4, 5]));
    ///
    /// assert_eq!(
    ///     key.key_format_versions(),
    ///     Some(&KeyFormatVersions::from(vec![1, 2, 3, 4, 5]))
    /// );
    /// ```
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(setter(into, strip_option), default)]
    pub(crate) key_format_versions: Option<KeyFormatVersions>,
}

impl ExtXKeyBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.method != Some(EncryptionMethod::None) && self.uri.is_none() {
            return Err(Error::custom("missing URL").to_string());
        }
        Ok(())
    }
}

impl ExtXKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-KEY:";

    /// Makes a new [`ExtXKey`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = ExtXKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "#EXT-X-KEY:METHOD=AES-128,URI=\"https://www.example.com/\""
    /// );
    /// ```
    pub fn new<T: Into<String>>(method: EncryptionMethod, uri: T) -> Self {
        Self {
            method,
            uri: Some(uri.into()),
            iv: None,
            key_format: None,
            key_format_versions: None,
        }
    }

    /// Returns a Builder to build an [`ExtXKey`].
    ///
    /// # Example
    ///
    /// ```
    /// use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::{EncryptionMethod, KeyFormat};
    ///
    /// ExtXKey::builder()
    ///     .method(EncryptionMethod::Aes128)
    ///     .uri("https://www.example.com/")
    ///     .iv([
    ///         16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
    ///     ])
    ///     .key_format(KeyFormat::Identity)
    ///     .key_format_versions(vec![1, 2, 3, 4, 5])
    ///     .build()?;
    /// # Ok::<(), Box<dyn ::std::error::Error>>(())
    /// ```
    pub fn builder() -> ExtXKeyBuilder { ExtXKeyBuilder::default() }

    /// Makes a new [`ExtXKey`] tag without a decryption key.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// let key = ExtXKey::empty();
    ///
    /// assert_eq!(key.to_string(), "#EXT-X-KEY:METHOD=NONE");
    /// ```
    pub const fn empty() -> Self {
        Self {
            method: EncryptionMethod::None,
            uri: None,
            iv: None,
            key_format: None,
            key_format_versions: None,
        }
    }

    /// Returns whether the [`EncryptionMethod`] is
    /// [`None`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = ExtXKey::empty();
    ///
    /// assert_eq!(key.method() == EncryptionMethod::None, key.is_empty());
    /// ```
    ///
    /// [`None`]: EncryptionMethod::None
    pub fn is_empty(&self) -> bool { self.method() == EncryptionMethod::None }
}

/// This tag requires [`ProtocolVersion::V5`], if [`KeyFormat`] or
/// [`KeyFormatVersions`] is specified and [`ProtocolVersion::V2`] if an iv is
/// specified.
///
/// Otherwise [`ProtocolVersion::V1`] is required.
impl RequiredVersion for ExtXKey {
    fn required_version(&self) -> ProtocolVersion {
        if self.key_format.is_some() || self.key_format_versions.is_some() {
            ProtocolVersion::V5
        } else if self.iv.is_some() {
            ProtocolVersion::V2
        } else {
            ProtocolVersion::V1
        }
    }
}

impl ExtXKey {
    /// Parses a String without verifying the starting tag
    pub(crate) fn parse_from_str(input: &str) -> crate::Result<Self> {
        let mut method = None;
        let mut uri = None;
        let mut iv = None;
        let mut key_format = None;
        let mut key_format_versions = None;

        for (key, value) in AttributePairs::new(input) {
            match key {
                "METHOD" => method = Some(value.parse().map_err(Error::strum)?),
                "URI" => {
                    let unquoted_uri = unquote(value);

                    if unquoted_uri.trim().is_empty() {
                        uri = None;
                    } else {
                        uri = Some(unquoted_uri);
                    }
                }
                "IV" => iv = Some(parse_iv_from_str(value)?),
                "KEYFORMAT" => key_format = Some(value.parse()?),
                "KEYFORMATVERSIONS" => key_format_versions = Some(value.parse().unwrap()),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized
                    // AttributeName.
                }
            }
        }

        let method = method.ok_or_else(|| Error::missing_value("METHOD"))?;
        if method != EncryptionMethod::None && uri.is_none() {
            return Err(Error::missing_value("URI"));
        }

        Ok(Self {
            method,
            uri,
            iv,
            key_format,
            key_format_versions,
        })
    }
}

impl FromStr for ExtXKey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;
        Self::parse_from_str(input)
    }
}

impl fmt::Display for ExtXKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;

        write!(f, "METHOD={}", self.method)?;

        if self.method == EncryptionMethod::None {
            return Ok(());
        }

        if let Some(uri) = &self.uri {
            write!(f, ",URI={}", quote(uri))?;
        }

        if let Some(value) = &self.iv {
            // TODO: use hex::encode_to_slice
            write!(f, ",IV=0x{}", hex::encode(&value))?;
        }

        if let Some(value) = &self.key_format {
            write!(f, ",KEYFORMAT={}", quote(value))?;
        }

        if let Some(key_format_versions) = &self.key_format_versions {
            if !key_format_versions.is_default() {
                write!(f, ",KEYFORMATVERSIONS={}", key_format_versions)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{EncryptionMethod, KeyFormat};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_builder() {
        assert_eq!(
            ExtXKey::builder()
                .method(EncryptionMethod::Aes128)
                .uri("https://www.example.com/")
                .iv([16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,])
                .key_format(KeyFormat::Identity)
                .key_format_versions(vec![1, 2, 3, 4, 5])
                .build()
                .unwrap()
                .to_string(),
            concat!(
                "#EXT-X-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/\",",
                "IV=0x10ef8f758ca555115584bb5b3c687f52,",
                "KEYFORMAT=\"identity\",",
                "KEYFORMATVERSIONS=\"1/2/3/4/5\"",
            )
            .to_string()
        );

        assert!(ExtXKey::builder().build().is_err());
        assert!(ExtXKey::builder()
            .method(EncryptionMethod::Aes128)
            .build()
            .is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXKey::empty().to_string(),
            "#EXT-X-KEY:METHOD=NONE".to_string()
        );

        let mut key = ExtXKey::empty();
        // it is expected, that all attributes will be ignored for an empty key!
        key.set_key_format(Some(KeyFormat::Identity));
        key.set_iv(Some([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]));
        key.set_uri(Some("https://www.example.com"));
        key.set_key_format_versions(Some(vec![1, 2, 3]));

        assert_eq!(key.to_string(), "#EXT-X-KEY:METHOD=NONE".to_string());

        assert_eq!(
            ExtXKey::builder()
                .method(EncryptionMethod::Aes128)
                .uri("https://www.example.com/hls-key/key.bin")
                .iv([16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82])
                .build()
                .unwrap()
                .to_string(),
            concat!(
                "#EXT-X-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0x10ef8f758ca555115584bb5b3c687f52"
            )
            .to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            concat!(
                "#EXT-X-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://priv.example.com/key.php?r=52\""
            )
            .parse::<ExtXKey>()
            .unwrap(),
            ExtXKey::new(
                EncryptionMethod::Aes128,
                "https://priv.example.com/key.php?r=52"
            )
        );

        assert_eq!(
            concat!(
                "#EXT-X-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0X10ef8f758ca555115584bb5b3c687f52"
            )
            .parse::<ExtXKey>()
            .unwrap(),
            ExtXKey::builder()
                .method(EncryptionMethod::Aes128)
                .uri("https://www.example.com/hls-key/key.bin")
                .iv([16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82])
                .build()
                .unwrap()
        );

        assert_eq!(
            concat!(
                "#EXT-X-KEY:",
                "METHOD=AES-128,",
                "URI=\"https://www.example.com/hls-key/key.bin\",",
                "IV=0X10ef8f758ca555115584bb5b3c687f52,",
                "KEYFORMAT=\"identity\",",
                "KEYFORMATVERSIONS=\"1/2/3\""
            )
            .parse::<ExtXKey>()
            .unwrap(),
            ExtXKey::builder()
                .method(EncryptionMethod::Aes128)
                .uri("https://www.example.com/hls-key/key.bin")
                .iv([16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82])
                .key_format(KeyFormat::Identity)
                .key_format_versions(vec![1, 2, 3])
                .build()
                .unwrap()
        );

        assert_eq!(
            concat!(
                "#EXT-X-KEY:",
                "METHOD=AES-128,",
                "URI=\"http://www.example.com\",",
                "UNKNOWNTAG=abcd"
            )
            .parse::<ExtXKey>()
            .unwrap(),
            ExtXKey::new(EncryptionMethod::Aes128, "http://www.example.com")
        );
        assert!("#EXT-X-KEY:METHOD=AES-128,URI=".parse::<ExtXKey>().is_err());
        assert!("garbage".parse::<ExtXKey>().is_err());
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXKey::new(EncryptionMethod::Aes128, "https://www.example.com/").required_version(),
            ProtocolVersion::V1
        );

        assert_eq!(
            ExtXKey::builder()
                .method(EncryptionMethod::Aes128)
                .uri("https://www.example.com/")
                .key_format(KeyFormat::Identity)
                .key_format_versions(vec![1, 2, 3])
                .build()
                .unwrap()
                .required_version(),
            ProtocolVersion::V5
        );

        assert_eq!(
            ExtXKey::builder()
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
