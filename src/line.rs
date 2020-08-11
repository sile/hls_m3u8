use core::convert::TryFrom;
use core::iter::FusedIterator;

use derive_more::Display;

use crate::tags;
use crate::types::PlaylistType;
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
                tags::VariantStream::try_from(format!("{}\n{}", line, uri).as_str())
                    .map(tags::VariantStream::into_owned)
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

impl<'a> FusedIterator for Lines<'a> {}

impl<'a> From<&'a str> for Lines<'a> {
    fn from(buffer: &'a str) -> Self {
        Self {
            lines: buffer
                .lines()
                .filter_map(|line| Some(line.trim()).filter(|v| !v.is_empty())),
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
#[derive(Debug, Clone, PartialEq, Display)]
#[display(fmt = "{}")]
pub(crate) enum Tag<'a> {
    ExtXVersion(tags::ExtXVersion),
    ExtInf(tags::ExtInf<'a>),
    ExtXByteRange(tags::ExtXByteRange),
    ExtXDiscontinuity(tags::ExtXDiscontinuity),
    ExtXKey(tags::ExtXKey<'a>),
    ExtXMap(tags::ExtXMap<'a>),
    ExtXProgramDateTime(tags::ExtXProgramDateTime<'a>),
    ExtXDateRange(tags::ExtXDateRange<'a>),
    ExtXTargetDuration(tags::ExtXTargetDuration),
    ExtXMediaSequence(tags::ExtXMediaSequence),
    ExtXDiscontinuitySequence(tags::ExtXDiscontinuitySequence),
    ExtXEndList(tags::ExtXEndList),
    PlaylistType(PlaylistType),
    ExtXIFramesOnly(tags::ExtXIFramesOnly),
    ExtXMedia(tags::ExtXMedia<'a>),
    ExtXSessionData(tags::ExtXSessionData<'a>),
    ExtXSessionKey(tags::ExtXSessionKey<'a>),
    ExtXIndependentSegments(tags::ExtXIndependentSegments),
    ExtXStart(tags::ExtXStart),
    VariantStream(tags::VariantStream<'a>),
    Unknown(&'a str),
}

impl<'a> TryFrom<&'a str> for Tag<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        if input.starts_with(tags::ExtXVersion::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXVersion)
        } else if input.starts_with(tags::ExtInf::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtInf)
        } else if input.starts_with(tags::ExtXByteRange::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXByteRange)
        } else if input.starts_with(tags::ExtXDiscontinuitySequence::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXDiscontinuitySequence)
        } else if input.starts_with(tags::ExtXDiscontinuity::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXDiscontinuity)
        } else if input.starts_with(tags::ExtXKey::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXKey)
        } else if input.starts_with(tags::ExtXMap::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXMap)
        } else if input.starts_with(tags::ExtXProgramDateTime::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXProgramDateTime)
        } else if input.starts_with(tags::ExtXTargetDuration::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXTargetDuration)
        } else if input.starts_with(tags::ExtXDateRange::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXDateRange)
        } else if input.starts_with(tags::ExtXMediaSequence::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXMediaSequence)
        } else if input.starts_with(tags::ExtXEndList::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXEndList)
        } else if input.starts_with(PlaylistType::PREFIX) {
            TryFrom::try_from(input).map(Self::PlaylistType)
        } else if input.starts_with(tags::ExtXIFramesOnly::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXIFramesOnly)
        } else if input.starts_with(tags::ExtXMedia::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXMedia)
        } else if input.starts_with(tags::VariantStream::PREFIX_EXTXIFRAME)
            || input.starts_with(tags::VariantStream::PREFIX_EXTXSTREAMINF)
        {
            TryFrom::try_from(input).map(Self::VariantStream)
        } else if input.starts_with(tags::ExtXSessionData::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXSessionData)
        } else if input.starts_with(tags::ExtXSessionKey::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXSessionKey)
        } else if input.starts_with(tags::ExtXIndependentSegments::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXIndependentSegments)
        } else if input.starts_with(tags::ExtXStart::PREFIX) {
            TryFrom::try_from(input).map(Self::ExtXStart)
        } else {
            Ok(Self::Unknown(input))
        }
    }
}
