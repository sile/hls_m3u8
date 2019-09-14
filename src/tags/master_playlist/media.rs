use std::fmt;
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::types::{InStreamId, MediaType, ProtocolVersion};
use crate::utils::{parse_yes_or_no, quote, tag, unquote};
use crate::Error;

/// `ExtXMedia` builder.
#[derive(Debug, Clone)]
pub struct ExtXMediaBuilder {
    media_type: Option<MediaType>,
    uri: Option<String>,
    group_id: Option<String>,
    language: Option<String>,
    assoc_language: Option<String>,
    name: Option<String>,
    default: bool,
    autoselect: Option<bool>,
    forced: Option<bool>,
    instream_id: Option<InStreamId>,
    characteristics: Option<String>,
    channels: Option<String>,
}

impl ExtXMediaBuilder {
    /// Makes a `ExtXMediaBuilder` instance.
    pub const fn new() -> Self {
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
    pub fn group_id<T: ToString>(&mut self, group_id: T) -> &mut Self {
        self.group_id = Some(group_id.to_string());
        self
    }

    /// Sets a human-readable description of the rendition.
    pub fn name<T: ToString>(&mut self, name: T) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets the URI that identifies the media playlist.
    pub fn uri<T: ToString>(&mut self, uri: T) -> &mut Self {
        self.uri = Some(uri.to_string());
        self
    }

    /// Sets the name of the primary language used in the rendition.
    pub fn language<T: ToString>(&mut self, language: T) -> &mut Self {
        self.language = Some(language.to_string());
        self
    }

    /// Sets the name of a language associated with the rendition.
    pub fn assoc_language<T: ToString>(&mut self, language: T) -> &mut Self {
        self.assoc_language = Some(language.to_string());
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
    pub fn characteristics<T: ToString>(&mut self, characteristics: T) -> &mut Self {
        self.characteristics = Some(characteristics.to_string());
        self
    }

    /// Sets the string that represents the parameters of the rendition.
    pub fn channels<T: ToString>(&mut self, channels: T) -> &mut Self {
        self.channels = Some(channels.to_string());
        self
    }

    /// Builds a `ExtXMedia` instance.
    pub fn finish(self) -> crate::Result<ExtXMedia> {
        let media_type = self
            .media_type
            .ok_or(Error::missing_value("self.media_type"))?;
        let group_id = self.group_id.ok_or(Error::missing_value("self.group_id"))?;
        let name = self.name.ok_or(Error::missing_value("self.name"))?;

        if MediaType::ClosedCaptions == media_type {
            if let None = self.uri {
                return Err(Error::missing_value("self.uri"));
            }
            self.instream_id
                .ok_or(Error::missing_value("self.instream_id"))?;
        } else {
            if let Some(_) = &self.instream_id {
                Err(Error::invalid_input())?;
            }
        }

        if self.default && self.autoselect.is_some() {
            if let Some(value) = &self.autoselect {
                if *value {
                    Err(Error::invalid_input())?;
                }
            }
        }

        if MediaType::Subtitles != media_type {
            if self.forced.is_some() {
                Err(Error::invalid_input())?;
            }
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
    uri: Option<String>,
    group_id: String,
    language: Option<String>,
    assoc_language: Option<String>,
    name: String,
    default: bool,
    autoselect: bool,
    forced: bool,
    instream_id: Option<InStreamId>,
    characteristics: Option<String>,
    channels: Option<String>,
}

impl ExtXMedia {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MEDIA:";

    /// Makes a new `ExtXMedia` tag.
    pub fn new<T: ToString>(media_type: MediaType, group_id: T, name: T) -> Self {
        ExtXMedia {
            media_type,
            uri: None,
            group_id: group_id.to_string(),
            language: None,
            assoc_language: None,
            name: name.to_string(),
            default: false,
            autoselect: false,
            forced: false,
            instream_id: None,
            characteristics: None,
            channels: None,
        }
    }

    /// Returns the type of the media associated with this tag.
    pub const fn media_type(&self) -> MediaType {
        self.media_type
    }

    /// Returns the identifier that specifies the group to which the rendition belongs.
    pub const fn group_id(&self) -> &String {
        &self.group_id
    }

    /// Returns a human-readable description of the rendition.
    pub const fn name(&self) -> &String {
        &self.name
    }

    /// Returns the URI that identifies the media playlist.
    pub fn uri(&self) -> Option<&String> {
        self.uri.as_ref()
    }

    /// Returns the name of the primary language used in the rendition.
    pub fn language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    /// Returns the name of a language associated with the rendition.
    pub fn assoc_language(&self) -> Option<&String> {
        self.assoc_language.as_ref()
    }

    /// Returns whether this is the default rendition.
    pub const fn default(&self) -> bool {
        self.default
    }

    /// Returns whether the client may choose to
    /// play this rendition in the absence of explicit user preference.
    pub const fn autoselect(&self) -> bool {
        self.autoselect
    }

    /// Returns whether the rendition contains content that is considered essential to play.
    pub const fn forced(&self) -> bool {
        self.forced
    }

    /// Returns the identifier that specifies a rendition within the segments in the media playlist.
    pub const fn instream_id(&self) -> Option<InStreamId> {
        self.instream_id
    }

    /// Returns a string that represents uniform type identifiers (UTI).
    ///
    /// Each UTI indicates an individual characteristic of the rendition.
    pub fn characteristics(&self) -> Option<&String> {
        self.characteristics.as_ref()
    }

    /// Returns a string that represents the parameters of the rendition.
    pub fn channels(&self) -> Option<&String> {
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
            write!(f, ",URI={}", quote(x))?;
        }
        write!(f, ",GROUP-ID={}", quote(&self.group_id))?;
        if let Some(ref x) = self.language {
            write!(f, ",LANGUAGE={}", quote(x))?;
        }
        if let Some(ref x) = self.assoc_language {
            write!(f, ",ASSOC-LANGUAGE={}", quote(x))?;
        }
        write!(f, ",NAME={}", quote(&self.name))?;
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
            write!(f, ",INSTREAM-ID={}", quote(x))?;
        }
        if let Some(ref x) = self.characteristics {
            write!(f, ",CHARACTERISTICS={}", quote(x))?;
        }
        if let Some(ref x) = self.channels {
            write!(f, ",CHANNELS={}", quote(x))?;
        }
        Ok(())
    }
}

impl FromStr for ExtXMedia {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let mut builder = ExtXMediaBuilder::new();

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "TYPE" => {
                    builder.media_type(value.parse()?);
                }
                "URI" => {
                    builder.uri(unquote(value));
                }
                "GROUP-ID" => {
                    builder.group_id(unquote(value));
                }
                "LANGUAGE" => {
                    builder.language(unquote(value));
                }
                "ASSOC-LANGUAGE" => {
                    builder.assoc_language(unquote(value));
                }
                "NAME" => {
                    builder.name(unquote(value));
                }
                "DEFAULT" => {
                    builder.default((parse_yes_or_no(value))?);
                }
                "AUTOSELECT" => {
                    builder.autoselect((parse_yes_or_no(value))?);
                }
                "FORCED" => {
                    builder.forced((parse_yes_or_no(value))?);
                }
                "INSTREAM-ID" => {
                    builder.instream_id(unquote(value).parse()?);
                }
                "CHARACTERISTICS" => {
                    builder.characteristics(unquote(value));
                }
                "CHANNELS" => {
                    builder.channels(unquote(value));
                }
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }
        (builder.finish())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_media() {
        let tag = ExtXMedia::new(MediaType::Audio, "foo", "bar");
        let text = r#"#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID="foo",NAME="bar""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
