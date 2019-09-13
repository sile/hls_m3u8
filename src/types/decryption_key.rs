use std::fmt;
use std::str::{self, FromStr};

use crate::attribute::AttributePairs;
use crate::types::{EncryptionMethod, InitializationVector, ProtocolVersion};
use crate::utils::{quote, unquote};
use crate::Error;

/// Decryption key.
///
/// See: [4.3.2.4. EXT-X-KEY]
///
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecryptionKey {
    pub method: EncryptionMethod,
    pub uri: String,
    pub iv: Option<InitializationVector>,
    pub key_format: Option<String>,
    pub key_format_versions: Option<String>,
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
        write!(f, ",URI={}", quote(&self.uri))?;
        if let Some(ref x) = self.iv {
            write!(f, ",IV={}", x)?;
        }
        if let Some(ref x) = self.key_format {
            write!(f, ",KEYFORMAT={}", quote(x))?;
        }
        if let Some(ref x) = self.key_format_versions {
            write!(f, ",KEYFORMATVERSIONS={}", quote(x))?;
        }
        Ok(())
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

        let attrs = AttributePairs::parse(input);
        for attr in attrs {
            let (key, value) = (attr)?;
            match key {
                "METHOD" => method = Some((value.parse())?),
                "URI" => uri = Some(unquote(value)),
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
        let uri = uri.ok_or(Error::missing_value("EXT-X-URI"))?;

        Ok(DecryptionKey {
            method,
            uri,
            iv,
            key_format,
            key_format_versions,
        })
    }
}
