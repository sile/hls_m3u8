use core::fmt;
use core::str::FromStr;

use derive_more::{AsMut, AsRef, Deref, DerefMut};

use crate::Error;

/// A list of formats, where each format specifies a media sample type that is
/// present in one or more renditions specified by the [`VariantStream`].
///
/// Valid format identifiers are those in the ISO Base Media File Format Name
/// Space defined by "The 'Codecs' and 'Profiles' Parameters for "Bucket" Media
/// Types" ([RFC6381]).
///
/// For example, a stream containing AAC low complexity (AAC-LC) audio and H.264
/// Main Profile Level 3.0 video would be
///
/// ```
/// # use hls_m3u8::types::Codecs;
/// let codecs = Codecs::from(&["mp4a.40.2", "avc1.4d401e"]);
/// ```
///
/// [RFC6381]: https://tools.ietf.org/html/rfc6381
/// [`VariantStream`]: crate::tags::VariantStream
#[derive(
    AsMut, AsRef, Deref, DerefMut, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub struct Codecs {
    list: Vec<String>,
}

impl Codecs {
    /// Makes a new (empty) [`Codecs`] struct.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::Codecs;
    /// let codecs = Codecs::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self { list: Vec::new() } }
}

impl fmt::Display for Codecs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(codec) = self.list.iter().next() {
            write!(f, "{}", codec)?;

            for codec in self.list.iter().skip(1) {
                write!(f, ",{}", codec)?;
            }
        }

        Ok(())
    }
}
impl FromStr for Codecs {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            list: input.split(',').map(|s| s.into()).collect(),
        })
    }
}

impl<T: AsRef<str>, I: IntoIterator<Item = T>> From<I> for Codecs {
    fn from(value: I) -> Self {
        Self {
            list: value.into_iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(Codecs::from(Vec::<&str>::new()), Codecs::new());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            Codecs::from(vec!["mp4a.40.2", "avc1.4d401e"]).to_string(),
            "mp4a.40.2,avc1.4d401e".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            Codecs::from_str("mp4a.40.2,avc1.4d401e").unwrap(),
            Codecs::from(vec!["mp4a.40.2", "avc1.4d401e"])
        );
    }
}
