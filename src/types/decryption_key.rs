use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;

use crate::attribute::AttributePairs;
use crate::types::{
    EncryptionMethod, InitializationVector, KeyFormat, KeyFormatVersions, ProtocolVersion,
};
use crate::utils::{quote, unquote};
use crate::{Error, RequiredVersion};

#[derive(Builder, Debug, Clone, PartialEq, Eq, Hash)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
/// [`DecryptionKey`] contains data, that is shared between [`ExtXSessionKey`]
/// and [`ExtXKey`].
///
/// [`ExtXSessionKey`]: crate::tags::ExtXSessionKey
/// [`ExtXKey`]: crate::tags::ExtXKey
pub struct DecryptionKey {
    /// The [EncryptionMethod].
    pub(crate) method: EncryptionMethod,
    #[builder(setter(into, strip_option), default)]
    /// An `URI`, that specifies how to obtain the key.
    pub(crate) uri: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// The IV (Initialization Vector) attribute.
    pub(crate) iv: Option<InitializationVector>,
    #[builder(setter(into, strip_option), default)]
    /// A string that specifies how the key is
    /// represented in the resource identified by the `URI`.
    pub(crate) key_format: Option<KeyFormat>,
    #[builder(setter(into, strip_option), default)]
    /// The [KeyFormatVersions] attribute.
    pub(crate) key_format_versions: Option<KeyFormatVersions>,
}

impl DecryptionKeyBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.method != Some(EncryptionMethod::None) && self.uri.is_none() {
            return Err(Error::custom("Missing URL").to_string());
        }
        Ok(())
    }
}

impl DecryptionKey {
    /// Makes a new [`DecryptionKey`].
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    /// ```
    pub fn new<T: ToString>(method: EncryptionMethod, uri: T) -> Self {
        Self {
            method,
            uri: Some(uri.to_string()),
            iv: None,
            key_format: None,
            key_format_versions: None,
        }
    }

    /// Returns the [`EncryptionMethod`].
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// assert_eq!(key.method(), EncryptionMethod::Aes128);
    /// ```
    pub const fn method(&self) -> EncryptionMethod { self.method }

    /// Returns a Builder to build a [DecryptionKey].
    pub fn builder() -> DecryptionKeyBuilder { DecryptionKeyBuilder::default() }

    /// Sets the [`EncryptionMethod`].
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_method(EncryptionMethod::SampleAes);
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "METHOD=SAMPLE-AES,URI=\"https://www.example.com/\"".to_string()
    /// );
    /// ```
    pub fn set_method(&mut self, value: EncryptionMethod) -> &mut Self {
        self.method = value;
        self
    }

    /// Returns an `URI`, that specifies how to obtain the key.
    ///
    /// # Note
    /// This attribute is required, if the [EncryptionMethod] is not `None`.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// assert_eq!(key.uri(), &Some("https://www.example.com/".to_string()));
    /// ```
    pub const fn uri(&self) -> &Option<String> { &self.uri }

    /// Sets the `URI` attribute.
    ///
    /// # Note
    /// This attribute is required, if the [`EncryptionMethod`] is not `None`.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_uri(Some("http://www.google.com/"));
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "METHOD=AES-128,URI=\"http://www.google.com/\"".to_string()
    /// );
    /// ```
    pub fn set_uri<T: ToString>(&mut self, value: Option<T>) -> &mut Self {
        self.uri = value.map(|v| v.to_string());
        self
    }

    /// Returns the IV (Initialization Vector) attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// # assert_eq!(key.iv(), None);
    /// key.set_iv(Some([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7]));
    ///
    /// assert_eq!(
    ///     key.iv(),
    ///     Some([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7])
    /// );
    /// ```
    pub fn iv(&self) -> Option<[u8; 16]> {
        if let Some(iv) = &self.iv {
            Some(iv.to_slice())
        } else {
            None
        }
    }

    /// Sets the `IV` attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_iv(Some([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7]));
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "METHOD=AES-128,URI=\"https://www.example.com/\",IV=0x01020304050607080901020304050607"
    ///         .to_string()
    /// );
    /// ```
    pub fn set_iv<T>(&mut self, value: Option<T>) -> &mut Self
    where
        T: Into<[u8; 16]>,
    {
        self.iv = value.map(|v| InitializationVector(v.into()));
        self
    }

    /// Returns a string that specifies how the key is
    /// represented in the resource identified by the `URI`.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::{EncryptionMethod, KeyFormat};
    ///
    /// let mut key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_key_format(Some(KeyFormat::Identity));
    ///
    /// assert_eq!(key.key_format(), Some(KeyFormat::Identity));
    /// ```
    pub const fn key_format(&self) -> Option<KeyFormat> { self.key_format }

    /// Sets the [`KeyFormat`] attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::{EncryptionMethod, KeyFormat};
    ///
    /// let mut key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_key_format(Some(KeyFormat::Identity));
    ///
    /// assert_eq!(key.key_format(), Some(KeyFormat::Identity));
    /// ```
    pub fn set_key_format<T: Into<KeyFormat>>(&mut self, value: Option<T>) -> &mut Self {
        self.key_format = value.map(|v| v.into());
        self
    }

    /// Returns the [`KeyFormatVersions`] attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::{EncryptionMethod, KeyFormatVersions};
    ///
    /// let mut key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_key_format_versions(Some(vec![1, 2, 3, 4, 5]));
    ///
    /// assert_eq!(
    ///     key.key_format_versions(),
    ///     &Some(KeyFormatVersions::from(vec![1, 2, 3, 4, 5]))
    /// );
    /// ```
    pub const fn key_format_versions(&self) -> &Option<KeyFormatVersions> {
        &self.key_format_versions
    }

    /// Sets the [`KeyFormatVersions`] attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::DecryptionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = DecryptionKey::new(EncryptionMethod::Aes128, "https://www.example.com/");
    ///
    /// key.set_key_format_versions(Some(vec![1, 2, 3, 4, 5]));
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "METHOD=AES-128,URI=\"https://www.example.com/\",KEYFORMATVERSIONS=\"1/2/3/4/5\""
    ///         .to_string()
    /// );
    /// ```
    pub fn set_key_format_versions<T: Into<KeyFormatVersions>>(
        &mut self,
        value: Option<T>,
    ) -> &mut Self {
        self.key_format_versions = value.map(|v| v.into());
        self
    }
}

impl RequiredVersion for DecryptionKey {
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

impl FromStr for DecryptionKey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut method = None;
        let mut uri = None;
        let mut iv = None;
        let mut key_format = None;
        let mut key_format_versions = None;

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "METHOD" => method = Some(value.parse()?),
                "URI" => uri = Some(unquote(value)),
                "IV" => iv = Some(value.parse()?),
                "KEYFORMAT" => key_format = Some(value.parse()?),
                "KEYFORMATVERSIONS" => key_format_versions = Some(value.parse()?),
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

impl fmt::Display for DecryptionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "METHOD={}", self.method)?;

        if self.method == EncryptionMethod::None {
            return Ok(());
        }
        if let Some(uri) = &self.uri {
            write!(f, ",URI={}", quote(uri))?;
        }
        if let Some(value) = &self.iv {
            write!(f, ",IV={}", value)?;
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
    use crate::types::EncryptionMethod;

    #[test]
    fn test_builder() {
        let key = DecryptionKey::builder()
            .method(EncryptionMethod::Aes128)
            .uri("https://www.example.com/")
            .iv([
                16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
            ])
            .key_format(KeyFormat::Identity)
            .key_format_versions(vec![1, 2, 3, 4, 5])
            .build()
            .unwrap();

        assert_eq!(
            key.to_string(),
            "METHOD=AES-128,\
             URI=\"https://www.example.com/\",\
             IV=0x10ef8f758ca555115584bb5b3c687f52,\
             KEYFORMAT=\"identity\",\
             KEYFORMATVERSIONS=\"1/2/3/4/5\"\
             "
            .to_string()
        );

        assert!(DecryptionKey::builder().build().is_err());
        assert!(DecryptionKey::builder()
            .method(EncryptionMethod::Aes128)
            .build()
            .is_err());
    }

    #[test]
    fn test_display() {
        let mut key = DecryptionKey::new(
            EncryptionMethod::Aes128,
            "https://www.example.com/hls-key/key.bin",
        );
        key.set_iv(Some([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]));

        assert_eq!(
            key.to_string(),
            "METHOD=AES-128,\
             URI=\"https://www.example.com/hls-key/key.bin\",\
             IV=0x10ef8f758ca555115584bb5b3c687f52"
                .to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            "METHOD=AES-128,\
             URI=\"https://priv.example.com/key.php?r=52\""
                .parse::<DecryptionKey>()
                .unwrap(),
            DecryptionKey::new(
                EncryptionMethod::Aes128,
                "https://priv.example.com/key.php?r=52"
            )
        );

        let mut key = DecryptionKey::new(
            EncryptionMethod::Aes128,
            "https://www.example.com/hls-key/key.bin",
        );
        key.set_iv(Some([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]));

        assert_eq!(
            "METHOD=AES-128,\
             URI=\"https://www.example.com/hls-key/key.bin\",\
             IV=0X10ef8f758ca555115584bb5b3c687f52"
                .parse::<DecryptionKey>()
                .unwrap(),
            key
        );

        let mut key = DecryptionKey::new(EncryptionMethod::Aes128, "http://www.example.com");
        key.set_iv(Some([
            16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82,
        ]));
        key.set_key_format(Some(KeyFormat::Identity));

        assert_eq!(
            "METHOD=AES-128,\
             URI=\"http://www.example.com\",\
             IV=0x10ef8f758ca555115584bb5b3c687f52,\
             KEYFORMAT=\"identity\""
                .parse::<DecryptionKey>()
                .unwrap(),
            key
        );

        key.set_key_format_versions(Some(vec![1, 2, 3]));
        assert_eq!(
            "METHOD=AES-128,\
             URI=\"http://www.example.com\",\
             IV=0x10ef8f758ca555115584bb5b3c687f52,\
             KEYFORMAT=\"identity\",\
             KEYFORMATVERSIONS=\"1/2/3\""
                .parse::<DecryptionKey>()
                .unwrap(),
            key
        );

        assert_eq!(
            "METHOD=AES-128,\
             URI=\"http://www.example.com\",\
             UNKNOWNTAG=abcd"
                .parse::<DecryptionKey>()
                .unwrap(),
            DecryptionKey::new(EncryptionMethod::Aes128, "http://www.example.com")
        );
        assert!("METHOD=AES-128,URI=".parse::<DecryptionKey>().is_err());
        assert!("garbage".parse::<DecryptionKey>().is_err());
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
                .key_format(KeyFormat::Identity)
                .key_format_versions(vec![1, 2, 3])
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
