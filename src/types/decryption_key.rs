use crate::attribute::AttributePairs;
use crate::types::{EncryptionMethod, InitializationVector, ProtocolVersion, QuotedString};
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::{self, FromStr};

/// Decryption key.
///
/// See: [4.3.2.4. EXT-X-KEY]
///
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecryptionKey {
    pub method: EncryptionMethod,
    pub uri: QuotedString,
    pub iv: Option<InitializationVector>,
    pub key_format: Option<QuotedString>,
    pub key_format_versions: Option<QuotedString>,
}

impl DecryptionKey {
    pub(crate) fn requires_version(&self) -> ProtocolVersion {
        if self.key_format.is_some() | self.key_format_versions.is_some() {
            ProtocolVersion::V5
        } else if self.iv.is_some() {
            ProtocolVersion::V2
        } else {
            ProtocolVersion::V1
        }
    }
}

impl fmt::Display for DecryptionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "METHOD={}", self.method)?;
        write!(f, ",URI={}", self.uri)?;
        if let Some(ref x) = self.iv {
            write!(f, ",IV={}", x)?;
        }
        if let Some(ref x) = self.key_format {
            write!(f, ",KEYFORMAT={}", x)?;
        }
        if let Some(ref x) = self.key_format_versions {
            write!(f, ",KEYFORMATVERSIONS={}", x)?;
        }
        Ok(())
    }
}

impl FromStr for DecryptionKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut method = None;
        let mut uri = None;
        let mut iv = None;
        let mut key_format = None;
        let mut key_format_versions = None;
        let attrs = AttributePairs::parse(s);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "METHOD" => method = Some(track!(value.parse())?),
                "URI" => uri = Some(track!(value.parse())?),
                "IV" => iv = Some(track!(value.parse())?),
                "KEYFORMAT" => key_format = Some(track!(value.parse())?),
                "KEYFORMATVERSIONS" => key_format_versions = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }
        let method = track_assert_some!(method, ErrorKind::InvalidInput);
        let uri = track_assert_some!(uri, ErrorKind::InvalidInput);
        Ok(DecryptionKey {
            method,
            uri,
            iv,
            key_format,
            key_format_versions,
        })
    }
}
