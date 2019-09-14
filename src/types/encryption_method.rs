use std::fmt;
use std::str::FromStr;

use crate::Error;

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
        match &self {
            EncryptionMethod::Aes128 => "AES-128".fmt(f),
            EncryptionMethod::SampleAes => "SAMPLE-AES".fmt(f),
        }
    }
}

impl FromStr for EncryptionMethod {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "AES-128" => Ok(EncryptionMethod::Aes128),
            "SAMPLE-AES" => Ok(EncryptionMethod::SampleAes),
            _ => Err(Error::custom(format!(
                "Unknown encryption method: {:?}",
                input
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let encryption_method = EncryptionMethod::Aes128;
        assert_eq!(encryption_method.to_string(), "AES-128".to_string());

        let encryption_method = EncryptionMethod::SampleAes;
        assert_eq!(encryption_method.to_string(), "SAMPLE-AES".to_string());
    }

    #[test]
    fn test_parse() {
        let encryption_method = EncryptionMethod::Aes128;
        assert_eq!(
            encryption_method,
            "AES-128".parse::<EncryptionMethod>().unwrap()
        );

        let encryption_method = EncryptionMethod::SampleAes;
        assert_eq!(
            encryption_method,
            "SAMPLE-AES".parse::<EncryptionMethod>().unwrap()
        );
    }
}
