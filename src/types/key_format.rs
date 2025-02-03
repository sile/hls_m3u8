use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::{quote, unquote};
use crate::{Error, RequiredVersion};

const IDENTITY: &str = "identity";
const FAIRPLAY: &str = "com.apple.streamingkeydelivery";
const WIDEVINE: &str = "urn:uuid:edef8ba9-79d6-4ace-a3c8-27dcd51d21ed";
const PLAYREADY: &str = "com.microsoft.playready";

/// Specifies how the key is represented in the resource identified by the
/// `URI`.
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum KeyFormat<'a> {
    /// An [`EncryptionMethod::Aes128`] uses 16-octet (16 byte/128 bit) keys. If
    /// the format is [`KeyFormat::Identity`], the key file is a single packed
    /// array of 16 octets (16 byte/128 bit) in binary format.
    ///
    /// [`EncryptionMethod::Aes128`]: crate::types::EncryptionMethod::Aes128
    Identity,
    /// The keyformat used by FairPlay.
    FairPlay,
    /// The keyformat used by Widevine.
    Widevine,
    /// The keyformat used by PlayReady.
    PlayReady,
    /// An unspecified key format.
    Other(Cow<'a, str>),
}

impl KeyFormat<'_> {
    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> KeyFormat<'static> {
        match self {
            KeyFormat::Identity => KeyFormat::Identity,
            KeyFormat::FairPlay => KeyFormat::FairPlay,
            KeyFormat::Widevine => KeyFormat::Widevine,
            KeyFormat::PlayReady => KeyFormat::PlayReady,
            KeyFormat::Other(cow) => KeyFormat::Other(Cow::Owned(cow.into_owned())),
        }
    }
}

impl Default for KeyFormat<'_> {
    fn default() -> Self { Self::Identity }
}

impl FromStr for KeyFormat<'static> {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_key_format(input).into_owned())
    }
}

impl<'a> From<&'a str> for KeyFormat<'a> {
    fn from(input: &'a str) -> Self {
        parse_key_format(input)
    }
}

impl fmt::Display for KeyFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyFormat::Identity => write!(f, "{}", quote(IDENTITY)),
            KeyFormat::FairPlay => write!(f, "{}", quote(FAIRPLAY)),
            KeyFormat::Widevine => write!(f, "{}", quote(WIDEVINE)),
            KeyFormat::PlayReady => write!(f, "{}", quote(PLAYREADY)),
            KeyFormat::Other(value) => write!(f, "{}", quote(value)),
        }
    }
}

/// This tag requires [`ProtocolVersion::V5`].
impl RequiredVersion for KeyFormat<'_> {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V5 }
}

fn parse_key_format(input: &str) -> KeyFormat<'_> {
    let format = unquote(input);
    match format.as_ref() {
        IDENTITY => KeyFormat::Identity,
        FAIRPLAY => KeyFormat::FairPlay,
        WIDEVINE => KeyFormat::Widevine,
        PLAYREADY => KeyFormat::PlayReady,
        _ => KeyFormat::Other(format),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(KeyFormat::Identity.to_string(), quote("identity"));

        assert_eq!(
            KeyFormat::FairPlay.to_string(),
            quote("com.apple.streamingkeydelivery")
        );
        assert_eq!(
            KeyFormat::Widevine.to_string(),
            quote("urn:uuid:edef8ba9-79d6-4ace-a3c8-27dcd51d21ed")
        );
        assert_eq!(
            KeyFormat::PlayReady.to_string(),
            quote("com.microsoft.playready")
        );

        assert_eq!(KeyFormat::Other("other".into()).to_string(), quote("other"));
    }

    #[test]
    fn test_parser() {
        assert_eq!(KeyFormat::Identity, quote("identity").parse().unwrap());

        assert_eq!(KeyFormat::Identity, "identity".parse().unwrap());

        assert_eq!(
            KeyFormat::FairPlay,
            quote("com.apple.streamingkeydelivery").parse().unwrap()
        );
        assert_eq!(
            KeyFormat::Widevine,
            quote("urn:uuid:edef8ba9-79d6-4ace-a3c8-27dcd51d21ed")
                .parse()
                .unwrap()
        );
        assert_eq!(
            KeyFormat::PlayReady,
            quote("com.microsoft.playready").parse().unwrap()
        );

        assert_eq!(
            KeyFormat::Other(Cow::Borrowed("other")),
            quote("other").parse().unwrap()
        );
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
