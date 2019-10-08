use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// # [4.3.1.2. EXT-X-VERSION]
///
/// The [`ExtXVersion`] tag indicates the compatibility version of the
/// [`Master Playlist`] or [`Media Playlist`] file.
/// It applies to the entire Playlist.
///
/// # Examples
/// Parsing from a [`str`]:
/// ```
/// # use failure::Error;
/// # use hls_m3u8::tags::ExtXVersion;
/// #
/// # fn main() -> Result<(), Error> {
/// use hls_m3u8::types::ProtocolVersion;
///
/// assert_eq!(
///     "#EXT-X-VERSION:5".parse::<ExtXVersion>()?,
///     ExtXVersion::new(ProtocolVersion::V5)
/// );
/// #
/// # Ok(())
/// # }
/// ```
/// Converting to a [`str`]:
/// ```
/// # use hls_m3u8::tags::ExtXVersion;
/// #
/// use hls_m3u8::types::ProtocolVersion;
///
/// assert_eq!(
///     "#EXT-X-VERSION:5".to_string(),
///     ExtXVersion::new(ProtocolVersion::V5).to_string()
/// );
/// ```
///
/// [`Media Playlist`]: crate::MediaPlaylist
/// [`Master Playlist`]: crate::MasterPlaylist
/// [4.3.1.2. EXT-X-VERSION]: https://tools.ietf.org/html/rfc8216#section-4.3.1.2
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtXVersion(ProtocolVersion);

impl ExtXVersion {
    pub(crate) const PREFIX: &'static str = "#EXT-X-VERSION:";

    /// Makes a new [`ExtXVersion`] tag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXVersion;
    /// use hls_m3u8::types::ProtocolVersion;
    ///
    /// let version = ExtXVersion::new(ProtocolVersion::V2);
    /// ```
    pub const fn new(version: ProtocolVersion) -> Self { Self(version) }

    /// Returns the [`ProtocolVersion`] of the playlist, containing this tag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXVersion;
    /// use hls_m3u8::types::ProtocolVersion;
    ///
    /// assert_eq!(
    ///     ExtXVersion::new(ProtocolVersion::V6).version(),
    ///     ProtocolVersion::V6
    /// );
    /// ```
    pub const fn version(self) -> ProtocolVersion { self.0 }
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXVersion {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}{}", Self::PREFIX, self.0) }
}

impl Default for ExtXVersion {
    fn default() -> Self { Self(ProtocolVersion::V1) }
}

impl From<ProtocolVersion> for ExtXVersion {
    fn from(value: ProtocolVersion) -> Self { Self(value) }
}

impl FromStr for ExtXVersion {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
            "#EXT-X-VERSION:6".parse::<ExtXVersion>().unwrap(),
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
