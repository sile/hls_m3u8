use core::convert::TryFrom;
use core::fmt;
use std::borrow::Cow;

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
pub struct Codecs<'a> {
    list: Vec<Cow<'a, str>>,
}

impl<'a> Codecs<'a> {
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

// TODO: this should be implemented with const generics in the future!
macro_rules! implement_from {
    ($($size:expr),*) => {
        $(
            #[allow(clippy::reversed_empty_ranges)]
            impl<'a> From<[&'a str; $size]> for Codecs<'a> {
                fn from(value: [&'a str; $size]) -> Self {
                    Self {
                        list: {
                            let mut result = Vec::with_capacity($size);

                            for i in 0..$size {
                                result.push(Cow::Borrowed(value[i]))
                            }

                            result
                        },
                    }
                }
            }

            #[allow(clippy::reversed_empty_ranges)]
            impl<'a> From<&[&'a str; $size]> for Codecs<'a> {
                fn from(value: &[&'a str; $size]) -> Self {
                    Self {
                        list: {
                            let mut result = Vec::with_capacity($size);

                            for i in 0..$size {
                                result.push(Cow::Borrowed(value[i]))
                            }

                            result
                        },
                    }
                }
            }
        )*
    };
}

implement_from!(
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    0x20
);

impl<'a> fmt::Display for Codecs<'a> {
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
