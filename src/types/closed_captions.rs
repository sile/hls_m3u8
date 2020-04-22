use core::convert::{Infallible, TryFrom};
use std::borrow::Cow;
use std::fmt;

use crate::utils::{quote, unquote};

/// The identifier of a closed captions group or its absence.
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClosedCaptions<'a> {
    /// It indicates the set of closed-caption renditions that can be used when
    /// playing the presentation.
    ///
    /// The [`String`] must match [`ExtXMedia::group_id`] elsewhere in the
    /// Playlist and it's [`ExtXMedia::media_type`] must be
    /// [`MediaType::ClosedCaptions`].
    ///
    /// [`ExtXMedia::group_id`]: crate::tags::ExtXMedia::group_id
    /// [`ExtXMedia::media_type`]: crate::tags::ExtXMedia::media_type
    /// [`MediaType::ClosedCaptions`]: crate::types::MediaType::ClosedCaptions
    GroupId(Cow<'a, str>),
    /// This variant indicates that there are no closed captions in
    /// any [`VariantStream`] in the [`MasterPlaylist`], therefore all
    /// [`VariantStream::ExtXStreamInf`] tags must have this attribute with a
    /// value of [`ClosedCaptions::None`].
    ///
    /// Having [`ClosedCaptions`] in one [`VariantStream`] but not in another
    /// can trigger playback inconsistencies.
    ///
    /// [`MasterPlaylist`]: crate::MasterPlaylist
    /// [`VariantStream`]: crate::tags::VariantStream
    /// [`VariantStream::ExtXStreamInf`]:
    /// crate::tags::VariantStream::ExtXStreamInf
    None,
}

impl<'a> ClosedCaptions<'a> {
    /// Creates a [`ClosedCaptions::GroupId`] with the provided [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use hls_m3u8::types::ClosedCaptions;
    ///
    /// assert_eq!(
    ///     ClosedCaptions::group_id("vg1"),
    ///     ClosedCaptions::GroupId("vg1".into())
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn group_id<I: Into<Cow<'a, str>>>(value: I) -> Self {
        //
        Self::GroupId(value.into())
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> ClosedCaptions<'static> {
        match self {
            Self::GroupId(id) => ClosedCaptions::GroupId(Cow::Owned(id.into_owned())),
            Self::None => ClosedCaptions::None,
        }
    }
}

impl<'a, T: PartialEq<str>> PartialEq<T> for ClosedCaptions<'a> {
    fn eq(&self, other: &T) -> bool {
        match &self {
            Self::GroupId(value) => other.eq(value),
            Self::None => other.eq("NONE"),
        }
    }
}

impl<'a> fmt::Display for ClosedCaptions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::GroupId(value) => write!(f, "{}", quote(value)),
            Self::None => write!(f, "NONE"),
        }
    }
}

impl<'a> TryFrom<&'a str> for ClosedCaptions<'a> {
    type Error = Infallible;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        if input.trim() == "NONE" {
            Ok(Self::None)
        } else {
            Ok(Self::GroupId(unquote(input)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(ClosedCaptions::None.to_string(), "NONE".to_string());

        assert_eq!(
            ClosedCaptions::GroupId("value".into()).to_string(),
            "\"value\"".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ClosedCaptions::None,
            ClosedCaptions::try_from("NONE").unwrap()
        );

        assert_eq!(
            ClosedCaptions::GroupId("value".into()),
            ClosedCaptions::try_from("\"value\"").unwrap()
        );
    }
}
