use core::convert::TryFrom;
use core::fmt;
use core::str::FromStr;

use crate::tags;
use crate::Error;

#[derive(Debug, Clone)]
pub(crate) struct Lines<'a> {
    lines: ::core::iter::FilterMap<::core::str::Lines<'a>, fn(&'a str) -> Option<&'a str>>,
}

impl<'a> Iterator for Lines<'a> {
    type Item = crate::Result<Line<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;

        if line.starts_with(tags::VariantStream::PREFIX_EXTXSTREAMINF) {
            let uri = self.lines.next()?;

            Some(
                tags::VariantStream::from_str(&format!("{}\n{}", line, uri))
                    .map(|v| Line::Tag(Tag::VariantStream(v))),
            )
        } else if line.starts_with("#EXT") {
            Some(Tag::try_from(line).map(Line::Tag))
        } else if line.starts_with('#') {
            Some(Ok(Line::Comment(line)))
        } else {
            Some(Ok(Line::Uri(line)))
        }
    }
}

impl<'a> From<&'a str> for Lines<'a> {
    fn from(buffer: &'a str) -> Self {
        Self {
            lines: buffer.lines().filter_map(|line| {
                if line.trim().is_empty() {
                    None
                } else {
                    Some(line.trim())
                }
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Line<'a> {
    Tag(Tag<'a>),
    Comment(&'a str),
    Uri(&'a str),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Tag<'a> {
    ExtXVersion(tags::ExtXVersion),
    ExtInf(tags::ExtInf),
    ExtXByteRange(tags::ExtXByteRange),
    ExtXDiscontinuity(tags::ExtXDiscontinuity),
    ExtXKey(tags::ExtXKey),
    ExtXMap(tags::ExtXMap),
    ExtXProgramDateTime(tags::ExtXProgramDateTime),
    ExtXDateRange(tags::ExtXDateRange),
    ExtXTargetDuration(tags::ExtXTargetDuration),
    ExtXMediaSequence(tags::ExtXMediaSequence),
    ExtXDiscontinuitySequence(tags::ExtXDiscontinuitySequence),
    ExtXEndList(tags::ExtXEndList),
    ExtXPlaylistType(tags::ExtXPlaylistType),
    ExtXIFramesOnly(tags::ExtXIFramesOnly),
    ExtXMedia(tags::ExtXMedia),
    ExtXSessionData(tags::ExtXSessionData),
    ExtXSessionKey(tags::ExtXSessionKey),
    ExtXIndependentSegments(tags::ExtXIndependentSegments),
    ExtXStart(tags::ExtXStart),
    VariantStream(tags::VariantStream),
    Unknown(&'a str),
}

impl<'a> fmt::Display for Tag<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::ExtXVersion(value) => value.fmt(f),
            Self::ExtInf(value) => value.fmt(f),
            Self::ExtXByteRange(value) => value.fmt(f),
            Self::ExtXDiscontinuity(value) => value.fmt(f),
            Self::ExtXKey(value) => value.fmt(f),
            Self::ExtXMap(value) => value.fmt(f),
            Self::ExtXProgramDateTime(value) => value.fmt(f),
            Self::ExtXDateRange(value) => value.fmt(f),
            Self::ExtXTargetDuration(value) => value.fmt(f),
            Self::ExtXMediaSequence(value) => value.fmt(f),
            Self::ExtXDiscontinuitySequence(value) => value.fmt(f),
            Self::ExtXEndList(value) => value.fmt(f),
            Self::ExtXPlaylistType(value) => value.fmt(f),
            Self::ExtXIFramesOnly(value) => value.fmt(f),
            Self::ExtXMedia(value) => value.fmt(f),
            Self::VariantStream(value) => value.fmt(f),
            Self::ExtXSessionData(value) => value.fmt(f),
            Self::ExtXSessionKey(value) => value.fmt(f),
            Self::ExtXIndependentSegments(value) => value.fmt(f),
            Self::ExtXStart(value) => value.fmt(f),
            Self::Unknown(value) => value.fmt(f),
        }
    }
}

impl<'a> TryFrom<&'a str> for Tag<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        if input.starts_with(tags::ExtXVersion::PREFIX) {
            input.parse().map(Self::ExtXVersion)
        } else if input.starts_with(tags::ExtInf::PREFIX) {
            input.parse().map(Self::ExtInf)
        } else if input.starts_with(tags::ExtXByteRange::PREFIX) {
            input.parse().map(Self::ExtXByteRange)
        } else if input.starts_with(tags::ExtXDiscontinuity::PREFIX) {
            input.parse().map(Self::ExtXDiscontinuity)
        } else if input.starts_with(tags::ExtXKey::PREFIX) {
            input.parse().map(Self::ExtXKey)
        } else if input.starts_with(tags::ExtXMap::PREFIX) {
            input.parse().map(Self::ExtXMap)
        } else if input.starts_with(tags::ExtXProgramDateTime::PREFIX) {
            input.parse().map(Self::ExtXProgramDateTime)
        } else if input.starts_with(tags::ExtXTargetDuration::PREFIX) {
            input.parse().map(Self::ExtXTargetDuration)
        } else if input.starts_with(tags::ExtXDateRange::PREFIX) {
            input.parse().map(Self::ExtXDateRange)
        } else if input.starts_with(tags::ExtXMediaSequence::PREFIX) {
            input.parse().map(Self::ExtXMediaSequence)
        } else if input.starts_with(tags::ExtXDiscontinuitySequence::PREFIX) {
            input.parse().map(Self::ExtXDiscontinuitySequence)
        } else if input.starts_with(tags::ExtXEndList::PREFIX) {
            input.parse().map(Self::ExtXEndList)
        } else if input.starts_with(tags::ExtXPlaylistType::PREFIX) {
            input.parse().map(Self::ExtXPlaylistType)
        } else if input.starts_with(tags::ExtXIFramesOnly::PREFIX) {
            input.parse().map(Self::ExtXIFramesOnly)
        } else if input.starts_with(tags::ExtXMedia::PREFIX) {
            input.parse().map(Self::ExtXMedia).map_err(Error::custom)
        } else if input.starts_with(tags::VariantStream::PREFIX_EXTXIFRAME)
            || input.starts_with(tags::VariantStream::PREFIX_EXTXSTREAMINF)
        {
            input
                .parse()
                .map(Self::VariantStream)
                .map_err(Error::custom)
        } else if input.starts_with(tags::ExtXSessionData::PREFIX) {
            input.parse().map(Self::ExtXSessionData)
        } else if input.starts_with(tags::ExtXSessionKey::PREFIX) {
            input.parse().map(Self::ExtXSessionKey)
        } else if input.starts_with(tags::ExtXIndependentSegments::PREFIX) {
            input.parse().map(Self::ExtXIndependentSegments)
        } else if input.starts_with(tags::ExtXStart::PREFIX) {
            input.parse().map(Self::ExtXStart)
        } else {
            Ok(Self::Unknown(input))
        }
    }
}
