use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::{quote, tag, unquote};
use crate::{Error, RequiredVersion};

/// Specifies how the key is represented in the resource identified by the
/// `URI`.
#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum KeyFormat {
    /// An [`EncryptionMethod::Aes128`] uses 16-octet (16 byte/128 bit) keys. If
    /// the format is [`KeyFormat::Identity`], the key file is a single packed
    /// array of 16 octets (16 byte/128 bit) in binary format.
    ///
    /// [`EncryptionMethod::Aes128`]: crate::types::EncryptionMethod::Aes128
    Identity,
}

impl Default for KeyFormat {
    fn default() -> Self { Self::Identity }
}

impl FromStr for KeyFormat {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        tag(&unquote(input), "identity")?; // currently only KeyFormat::Identity exists!

        Ok(Self::Identity)
    }
}

impl fmt::Display for KeyFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", quote(&"identity")) }
}

/// This tag requires [`ProtocolVersion::V5`].
impl RequiredVersion for KeyFormat {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V5 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(KeyFormat::Identity.to_string(), quote("identity"));
    }

    #[test]
    fn test_parser() {
        assert_eq!(KeyFormat::Identity, quote("identity").parse().unwrap());

        assert_eq!(KeyFormat::Identity, "identity".parse().unwrap());

        assert!("garbage".parse::<KeyFormat>().is_err());
    }

    #[test]
    fn test_required_version() {
        assert_eq!(KeyFormat::Identity.required_version(), ProtocolVersion::V5)
    }

    #[test]
    fn test_default() {
        assert_eq!(KeyFormat::Identity, KeyFormat::default());
    }
}
