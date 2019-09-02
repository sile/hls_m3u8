use crate::error::{Error, ErrorKind};
use std::fmt;
use std::str::FromStr;
/// Encryption method.
///
/// See: [4.3.2.4. EXT-X-KEY]
///
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptionMethod {
    Aes128,
    SampleAes,
}

impl fmt::Display for EncryptionMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncryptionMethod::Aes128 => "AES-128".fmt(f),
            EncryptionMethod::SampleAes => "SAMPLE-AES".fmt(f),
        }
    }
}

impl FromStr for EncryptionMethod {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AES-128" => Ok(EncryptionMethod::Aes128),
            "SAMPLE-AES" => Ok(EncryptionMethod::SampleAes),
            _ => track_panic!(
                ErrorKind::InvalidInput,
                "Unknown encryption method: {:?}",
                s
            ),
        }
    }
}
