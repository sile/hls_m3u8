use crate::attribute::AttributePairs;
use crate::types::{InStreamId, MediaType, ProtocolVersion, QuotedString};
use crate::utils::parse_yes_or_no;
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// `ExtXMedia` builder.
#[derive(Debug, Clone)]
pub struct ExtXMediaBuilder {
    media_type: Option<MediaType>,
    uri: Option<QuotedString>,
    group_id: Option<QuotedString>,
    language: Option<QuotedString>,
    assoc_language: Option<QuotedString>,
    name: Option<QuotedString>,
    default: bool,
    autoselect: Option<bool>,
    forced: Option<bool>,
    instream_id: Option<InStreamId>,
    characteristics: Option<QuotedString>,
    channels: Option<QuotedString>,
}

impl ExtXMediaBuilder {
    /// Makes a `ExtXMediaBuilder` instance.
    pub fn new() -> Self {
        ExtXMediaBuilder {
            media_type: None,
            uri: None,
            group_id: None,
            language: None,
            assoc_language: None,
            name: None,
            default: false,
            autoselect: None,
            forced: None,
            instream_id: None,
            characteristics: None,
            channels: None,
        }
    }

    /// Sets the media type of the rendition.
    pub fn media_type(&mut self, media_type: MediaType) -> &mut Self {
        self.media_type = Some(media_type);
        self
    }

    /// Sets the identifier that specifies the group to which the rendition belongs.
    pub fn group_id(&mut self, group_id: QuotedString) -> &mut Self {
        self.group_id = Some(group_id);
        self
    }

    /// Sets a human-readable description of the rendition.
    pub fn name(&mut self, name: QuotedString) -> &mut Self {
        self.name = Some(name);
        self
    }

    /// Sets the URI that identifies the media playlist.
    pub fn uri(&mut self, uri: QuotedString) -> &mut Self {
        self.uri = Some(uri);
        self
    }

    /// Sets the name of the primary language used in the rendition.
    pub fn language(&mut self, language: QuotedString) -> &mut Self {
        self.language = Some(language);
        self
    }

    /// Sets the name of a language associated with the rendition.
    pub fn assoc_language(&mut self, language: QuotedString) -> &mut Self {
        self.assoc_language = Some(language);
        self
    }

    /// Sets the value of the `default` flag.
    pub fn default(&mut self, b: bool) -> &mut Self {
        self.default = b;
        self
    }

    /// Sets the value of the `autoselect` flag.
    pub fn autoselect(&mut self, b: bool) -> &mut Self {
        self.autoselect = Some(b);
        self
    }

    /// Sets the value of the `forced` flag.
    pub fn forced(&mut self, b: bool) -> &mut Self {
        self.forced = Some(b);
        self
    }

    /// Sets the identifier that specifies a rendition within the segments in the media playlist.
    pub fn instream_id(&mut self, id: InStreamId) -> &mut Self {
        self.instream_id = Some(id);
        self
    }

    /// Sets the string that represents uniform type identifiers (UTI).
    pub fn characteristics(&mut self, characteristics: QuotedString) -> &mut Self {
        self.characteristics = Some(characteristics);
        self
    }

    /// Sets the string that represents the parameters of the rendition.
    pub fn channels(&mut self, channels: QuotedString) -> &mut Self {
        self.channels = Some(channels);
        self
    }

    /// Builds a `ExtXMedia` instance.
    pub fn finish(self) -> Result<ExtXMedia> {
        let media_type = track_assert_some!(self.media_type, ErrorKind::InvalidInput);
        let group_id = track_assert_some!(self.group_id, ErrorKind::InvalidInput);
        let name = track_assert_some!(self.name, ErrorKind::InvalidInput);
        if MediaType::ClosedCaptions == media_type {
            track_assert_ne!(self.uri, None, ErrorKind::InvalidInput);
            track_assert!(self.instream_id.is_some(), ErrorKind::InvalidInput);
        } else {
            track_assert!(self.instream_id.is_none(), ErrorKind::InvalidInput);
        }
        if self.default && self.autoselect.is_some() {
            track_assert_eq!(self.autoselect, Some(true), ErrorKind::InvalidInput);
        }
        if MediaType::Subtitles != media_type {
            track_assert_eq!(self.forced, None, ErrorKind::InvalidInput);
        }
        Ok(ExtXMedia {
            media_type,
            uri: self.uri,
            group_id,
            language: self.language,
            assoc_language: self.assoc_language,
            name,
            default: self.default,
            autoselect: self.autoselect.unwrap_or(false),
            forced: self.forced.unwrap_or(false),
            instream_id: self.instream_id,
            characteristics: self.characteristics,
            channels: self.channels,
        })
    }
}

impl Default for ExtXMediaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// [4.3.4.1. EXT-X-MEDIA]
///
/// [4.3.4.1. EXT-X-MEDIA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.1
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXMedia {
    media_type: MediaType,
    uri: Option<QuotedString>,
    group_id: QuotedString,
    language: Option<QuotedString>,
    assoc_language: Option<QuotedString>,
    name: QuotedString,
    default: bool,
    autoselect: bool,
    forced: bool,
    instream_id: Option<InStreamId>,
    characteristics: Option<QuotedString>,
    channels: Option<QuotedString>,
}

impl ExtXMedia {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MEDIA:";

    /// Makes a new `ExtXMedia` tag.
    pub fn new(media_type: MediaType, group_id: QuotedString, name: QuotedString) -> Self {
        ExtXMedia {
            media_type,
            uri: None,
            group_id,
            language: None,
            assoc_language: None,
            name,
            default: false,
            autoselect: false,
            forced: false,
            instream_id: None,
            characteristics: None,
            channels: None,
        }
    }

    /// Returns the type of the media associated with this tag.
    pub fn media_type(&self) -> MediaType {
        self.media_type
    }

    /// Returns the identifier that specifies the group to which the rendition belongs.
    pub fn group_id(&self) -> &QuotedString {
        &self.group_id
    }

    /// Returns a human-readable description of the rendition.
    pub fn name(&self) -> &QuotedString {
        &self.name
    }

    /// Returns the URI that identifies the media playlist.
    pub fn uri(&self) -> Option<&QuotedString> {
        self.uri.as_ref()
    }

    /// Returns the name of the primary language used in the rendition.
    pub fn language(&self) -> Option<&QuotedString> {
        self.language.as_ref()
    }

    /// Returns the name of a language associated with the rendition.
    pub fn assoc_language(&self) -> Option<&QuotedString> {
        self.assoc_language.as_ref()
    }

    /// Returns whether this is the default rendition.
    pub fn default(&self) -> bool {
        self.default
    }

    /// Returns whether the client may choose to
    /// play this rendition in the absence of explicit user preference.
    pub fn autoselect(&self) -> bool {
        self.autoselect
    }

    /// Returns whether the rendition contains content that is considered essential to play.
    pub fn forced(&self) -> bool {
        self.forced
    }

    /// Returns the identifier that specifies a rendition within the segments in the media playlist.
    pub fn instream_id(&self) -> Option<InStreamId> {
        self.instream_id
    }

    /// Returns a string that represents uniform type identifiers (UTI).
    ///
    /// Each UTI indicates an individual characteristic of the rendition.
    pub fn characteristics(&self) -> Option<&QuotedString> {
        self.characteristics.as_ref()
    }

    /// Returns a string that represents the parameters of the rendition.
    pub fn channels(&self) -> Option<&QuotedString> {
        self.channels.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        match self.instream_id {
            None
            | Some(InStreamId::Cc1)
            | Some(InStreamId::Cc2)
            | Some(InStreamId::Cc3)
            | Some(InStreamId::Cc4) => ProtocolVersion::V1,
            _ => ProtocolVersion::V7,
        }
    }
}

impl fmt::Display for ExtXMedia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "TYPE={}", self.media_type)?;
        if let Some(ref x) = self.uri {
            write!(f, ",URI={}", x)?;
        }
        write!(f, ",GROUP-ID={}", self.group_id)?;
        if let Some(ref x) = self.language {
            write!(f, ",LANGUAGE={}", x)?;
        }
        if let Some(ref x) = self.assoc_language {
            write!(f, ",ASSOC-LANGUAGE={}", x)?;
        }
        write!(f, ",NAME={}", self.name)?;
        if self.default {
            write!(f, ",DEFAULT=YES")?;
        }
        if self.autoselect {
            write!(f, ",AUTOSELECT=YES")?;
        }
        if self.forced {
            write!(f, ",FORCED=YES")?;
        }
        if let Some(ref x) = self.instream_id {
            write!(f, ",INSTREAM-ID=\"{}\"", x)?;
        }
        if let Some(ref x) = self.characteristics {
            write!(f, ",CHARACTERISTICS={}", x)?;
        }
        if let Some(ref x) = self.channels {
            write!(f, ",CHANNELS={}", x)?;
        }
        Ok(())
    }
}

impl FromStr for ExtXMedia {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut builder = ExtXMediaBuilder::new();
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "TYPE" => {
                    builder.media_type(track!(value.parse())?);
                }
                "URI" => {
                    builder.uri(track!(value.parse())?);
                }
                "GROUP-ID" => {
                    builder.group_id(track!(value.parse())?);
                }
                "LANGUAGE" => {
                    builder.language(track!(value.parse())?);
                }
                "ASSOC-LANGUAGE" => {
                    builder.assoc_language(track!(value.parse())?);
                }
                "NAME" => {
                    builder.name(track!(value.parse())?);
                }
                "DEFAULT" => {
                    builder.default(track!(parse_yes_or_no(value))?);
                }
                "AUTOSELECT" => {
                    builder.autoselect(track!(parse_yes_or_no(value))?);
                }
                "FORCED" => {
                    builder.forced(track!(parse_yes_or_no(value))?);
                }
                "INSTREAM-ID" => {
                    let s: QuotedString = track!(value.parse())?;
                    builder.instream_id(track!(s.parse())?);
                }
                "CHARACTERISTICS" => {
                    builder.characteristics(track!(value.parse())?);
                }
                "CHANNELS" => {
                    builder.channels(track!(value.parse())?);
                }
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }
        track!(builder.finish())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_media() {
        let tag = ExtXMedia::new(MediaType::Audio, quoted_string("foo"), quoted_string("bar"));
        let text = r#"#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID="foo",NAME="bar""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    fn quoted_string(s: &str) -> QuotedString {
        QuotedString::new(s).unwrap()
    }
}
