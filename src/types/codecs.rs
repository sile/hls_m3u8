use core::convert::TryFrom;
use core::fmt;
use core::ops::{Deref, DerefMut};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Codecs<'a> {
    list: Vec<Cow<'a, str>>,
}

impl<'a> AsRef<Vec<Cow<'a, str>>> for Codecs<'a> {
    fn as_ref(&self) -> &Vec<Cow<'a, str>> {
        &self.list
    }
}

impl<'a> AsMut<Vec<Cow<'a, str>>> for Codecs<'a> {
    fn as_mut(&mut self) -> &mut Vec<Cow<'a, str>> {
        &mut self.list
    }
}

impl<'a> Deref for Codecs<'a> {
    type Target = Vec<Cow<'a, str>>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl DerefMut for Codecs<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}

impl Codecs<'_> {
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
    pub const fn new() -> Self {
        Self { list: Vec::new() }
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> Codecs<'static> {
        Codecs {
            list: self
                .list
                .into_iter()
                .map(|v| Cow::Owned(v.into_owned()))
                .collect(),
        }
    }
}

impl<'a, T> From<Vec<T>> for Codecs<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: Vec<T>) -> Self {
        Self {
            list: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a, const N: usize> From<[&'a str; N]> for Codecs<'a> {
    fn from(value: [&'a str; N]) -> Self {
        Self {
            list: value.iter().map(|s| Cow::Borrowed(*s)).collect(),
        }
    }
}

impl<'a, const N: usize> From<&[&'a str; N]> for Codecs<'a> {
    fn from(value: &[&'a str; N]) -> Self {
        Self {
            list: value.iter().map(|s| Cow::Borrowed(*s)).collect(),
        }
    }
}

impl fmt::Display for Codecs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(codec) = self.list.first() {
            write!(f, "{}", codec)?;

            for codec in self.list.iter().skip(1) {
                write!(f, ",{}", codec)?;
            }
        }

        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for Codecs<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            list: input.split(',').map(|s| s.into()).collect(),
        })
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Codecs<'a> {
    type Error = Error;

    fn try_from(input: Cow<'a, str>) -> Result<Self, Self::Error> {
        match input {
            Cow::Owned(o) => Ok(Codecs::try_from(o.as_str())?.into_owned()),
            Cow::Borrowed(b) => Self::try_from(b),
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
            Codecs::from(["mp4a.40.2", "avc1.4d401e"]).to_string(),
            "mp4a.40.2,avc1.4d401e".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            Codecs::try_from("mp4a.40.2,avc1.4d401e").unwrap(),
            Codecs::from(["mp4a.40.2", "avc1.4d401e"])
        );
    }
}
