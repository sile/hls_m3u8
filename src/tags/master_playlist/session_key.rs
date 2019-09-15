use std::fmt;
use std::str::FromStr;

use url::Url;

use crate::attribute::AttributePairs;
use crate::types::{EncryptionMethod, InitializationVector, ProtocolVersion};
use crate::utils::{quote, tag, unquote};
use crate::Error;

/// [4.3.4.5. EXT-X-SESSION-KEY]
///
/// [4.3.4.5. EXT-X-SESSION-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.4.5
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXSessionKey {
    method: EncryptionMethod,
    uri: Option<Url>,
    iv: Option<InitializationVector>,
    key_format: Option<String>,
    key_format_versions: Option<String>,
}

impl ExtXSessionKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-KEY:";

    /// Makes a new [ExtXSessionKey] tag.
    /// # Panic
    /// This method will panic, if the [EncryptionMethod] is None.
    pub fn new(method: EncryptionMethod, uri: Url) -> Self {
        if method == EncryptionMethod::None {
            panic!("The EncryptionMethod is not allowed to be None");
        }

        Self {
            method,
            uri: Some(uri),
            iv: None,
            key_format: None,
            key_format_versions: None,
        }
    }

    /// Returns the [EncryptionMethod].
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     key.method(),
    ///     EncryptionMethod::Aes128
    /// );
    /// ```
    pub const fn method(&self) -> EncryptionMethod {
        self.method
    }

    /// Sets the [EncryptionMethod].
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// key.set_method(EncryptionMethod::SampleAes);
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "#EXT-X-SESSION-KEY:METHOD=SAMPLE-AES,URI=\"https://www.example.com/\"".to_string()
    /// );
    /// ```
    pub fn set_method(&mut self, value: EncryptionMethod) -> &mut Self {
        self.method = value;
        self
    }

    /// Returns an `URI` that specifies how to obtain the key.
    ///
    /// This attribute is required, if the [EncryptionMethod] is not None.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     key.uri(),
    ///     &Some("https://www.example.com".parse().unwrap())
    /// );
    /// ```
    pub const fn uri(&self) -> &Option<Url> {
        &self.uri
    }

    /// Sets the `URI` attribute.
    ///
    /// This attribute is required, if the [EncryptionMethod] is not None.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// key.set_uri("http://www.google.com".parse().unwrap());
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "#EXT-X-SESSION-KEY:METHOD=AES-128,URI=\"http://www.google.com/\"".to_string()
    /// );
    /// ```
    pub fn set_uri(&mut self, value: Url) -> &mut Self {
        self.uri = Some(value);
        self
    }

    /// Returns the IV (Initialization Vector) attribute.
    ///
    /// This attribute is optional.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// key.set_iv([
    ///    1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7
    /// ]);
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
    /// This attribute is optional.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// key.set_iv([
    ///    1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7
    /// ]);
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "#EXT-X-SESSION-KEY:METHOD=AES-128,URI=\"https://www.example.com/\",IV=0x01020304050607080901020304050607".to_string()
    /// );
    /// ```
    pub fn set_iv<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<[u8; 16]>,
    {
        self.iv = Some(InitializationVector(value.into()));
        self
    }

    /// Returns a string that specifies how the key is
    /// represented in the resource identified by the URI.
    ///
    //// This attribute is optional.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// key.set_key_format("key_format_attribute");
    ///
    /// assert_eq!(
    ///     key.key_format(),
    ///     &Some("key_format_attribute".to_string())
    /// );
    /// ```
    pub const fn key_format(&self) -> &Option<String> {
        &self.key_format
    }

    /// Sets the `KEYFORMAT` attribute.
    ///
    /// This attribute is optional.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// key.set_key_format("key_format_attribute");
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "#EXT-X-SESSION-KEY:METHOD=AES-128,URI=\"https://www.example.com/\",KEYFORMAT=\"key_format_attribute\"".to_string()
    /// );
    /// ```
    pub fn set_key_format<T: ToString>(&mut self, value: T) -> &mut Self {
        self.key_format = Some(value.to_string());
        self
    }

    /// Returns a string containing one or more positive
    /// integers separated by the "/" character (for example, "1", "1/2",
    /// or "1/2/5").  If more than one version of a particular `KEYFORMAT`
    /// is defined, this attribute can be used to indicate which
    /// version(s) this instance complies with.
    ///
    /// This attribute is optional.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// key.set_key_format_versions("1/2/3/4/5");
    ///
    /// assert_eq!(
    ///     key.key_format_versions(),
    ///     &Some("1/2/3/4/5".to_string())
    /// );
    /// ```
    pub const fn key_format_versions(&self) -> &Option<String> {
        &self.key_format_versions
    }

    /// Sets the `KEYFORMATVERSIONS` attribute.
    ///
    /// This attribute is optional.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::EncryptionMethod;
    ///
    /// let mut key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// key.set_key_format_versions("1/2/3/4/5");
    ///
    /// assert_eq!(
    ///     key.to_string(),
    ///     "#EXT-X-SESSION-KEY:METHOD=AES-128,URI=\"https://www.example.com/\",KEYFORMATVERSIONS=\"1/2/3/4/5\"".to_string()
    /// );
    /// ```
    pub fn set_key_format_versions<T: ToString>(&mut self, value: T) -> &mut Self {
        self.key_format_versions = Some(value.to_string());
        self
    }

    /// Returns the protocol compatibility version that this tag requires.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXSessionKey;
    /// use hls_m3u8::types::{EncryptionMethod, ProtocolVersion};
    ///
    /// let mut key = ExtXSessionKey::new(
    ///     EncryptionMethod::Aes128,
    ///     "https://www.example.com".parse().unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     key.requires_version(),
    ///     ProtocolVersion::V1
    /// );
    /// ```
    pub fn requires_version(&self) -> ProtocolVersion {
        if self.key_format.is_some() | self.key_format_versions.is_some() {
            ProtocolVersion::V5
        } else if self.iv.is_some() {
            ProtocolVersion::V2
        } else {
            ProtocolVersion::V1
        }
    }
}

impl fmt::Display for ExtXSessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}METHOD={}", Self::PREFIX, self.method)?;
        if let Some(uri) = &self.uri {
            write!(f, ",URI={}", quote(uri))?;
        }
        if let Some(value) = &self.iv {
            write!(f, ",IV={}", value)?;
        }
        if let Some(value) = &self.key_format {
            write!(f, ",KEYFORMAT={}", quote(value))?;
        }
        if let Some(value) = &self.key_format_versions {
            write!(f, ",KEYFORMATVERSIONS={}", quote(value))?;
        }
        Ok(())
    }
}

impl FromStr for ExtXSessionKey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;
        let mut method = None;
        let mut uri = None;
        let mut iv = None;
        let mut key_format = None;
        let mut key_format_versions = None;

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "METHOD" => method = Some((value.parse())?),
                "URI" => uri = Some(unquote(value).parse()?),
                "IV" => iv = Some((value.parse())?),
                "KEYFORMAT" => key_format = Some(unquote(value)),
                "KEYFORMATVERSIONS" => key_format_versions = Some(unquote(value)),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let method = method.ok_or(Error::missing_value("EXT-X-METHOD"))?;
        if method == EncryptionMethod::None {
            return Err(Error::custom(
                "EXT-X-SESSION-KEY: Encryption Method must not be NONE",
            ));
        }
        Ok(ExtXSessionKey {
            method,
            uri,
            iv,
            key_format,
            key_format_versions,
        })
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
            "https://www.example.com/hls-key/key.bin".parse().unwrap(),
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
                "https://priv.example.com/key.php?r=52".parse().unwrap()
            )
        );

        let mut key = ExtXSessionKey::new(
            EncryptionMethod::Aes128,
            "https://www.example.com/hls-key/key.bin".parse().unwrap(),
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
}
