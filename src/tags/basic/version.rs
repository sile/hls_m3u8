use std::convert::TryFrom;
use std::fmt;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// The compatibility version of a playlist.
///
/// It applies to the entire [`MasterPlaylist`] or [`MediaPlaylist`].
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
/// [`MasterPlaylist`]: crate::MasterPlaylist
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ExtXVersion(ProtocolVersion);

impl ExtXVersion {
    pub(crate) const PREFIX: &'static str = "#EXT-X-VERSION:";

    /// Makes a new [`ExtXVersion`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXVersion;
    /// use hls_m3u8::types::ProtocolVersion;
    ///
    /// let version = ExtXVersion::new(ProtocolVersion::V2);
    /// ```
    #[must_use]
    pub const fn new(version: ProtocolVersion) -> Self { Self(version) }

    /// Returns the underlying [`ProtocolVersion`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXVersion;
    /// use hls_m3u8::types::ProtocolVersion;
    ///
    /// assert_eq!(
    ///     ExtXVersion::new(ProtocolVersion::V6).version(),
    ///     ProtocolVersion::V6
    /// );
    /// ```
    #[must_use]
    pub const fn version(self) -> ProtocolVersion { self.0 }
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXVersion {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl From<ProtocolVersion> for ExtXVersion {
    fn from(value: ProtocolVersion) -> Self { Self(value) }
}

impl TryFrom<&str> for ExtXVersion {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let version = tag(input, Self::PREFIX)?.parse()?;
        Ok(Self::new(version))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXVersion::new(ProtocolVersion::V6).to_string(),
            "#EXT-X-VERSION:6"
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXVersion::try_from("#EXT-X-VERSION:6").unwrap(),
            ExtXVersion::new(ProtocolVersion::V6)
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXVersion::new(ProtocolVersion::V6).required_version(),
            ProtocolVersion::V1
        );
    }

    #[test]
    fn test_default_and_from() {
        assert_eq!(
            ExtXVersion::default(),
            ExtXVersion::from(ProtocolVersion::V1)
        );
    }
}
