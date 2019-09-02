use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;

use crate::attribute::AttributePairs;
use crate::error::{Error, ErrorKind};
use crate::types::{InStreamId, MediaType, ProtocolVersion};
use crate::utils::parse_yes_or_no;
use crate::utils::{quote, unquote};

/// [4.3.4.1. EXT-X-MEDIA]
///
/// [4.3.4.1. EXT-X-MEDIA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.1
#[derive(Builder, Debug, Clone, PartialEq, Eq, Hash)]
#[builder(setter(into, strip_option))]
pub struct ExtXMedia {
    /// Sets the media type of the rendition.
    media_type: MediaType,
    /// The URI that identifies the media playlist.
    uri: Option<String>,
    /// Sets the identifier that specifies the group to which the rendition belongs.
    group_id: String,
    /// Sets the name of the primary language used in the rendition.
    language: Option<String>,
    /// Sets the name of a language associated with the rendition.
    assoc_language: Option<String>,
    /// Sets a human-readable description of the rendition.
    name: String,
    /// Sets the value of the `default` flag.
    // has been changed, from `default` to `is_default`, because it caused a naming conflict
    // with the trait implementation of `Default`.
    is_default: bool,
    /// Sets the value of the `autoselect` flag.
    autoselect: bool,
    /// Sets the value of the `forced` flag.
    forced: bool,
    /// Sets the identifier that specifies a rendition within the segments in the media playlist.
    instream_id: Option<InStreamId>,
    /// Sets the string that represents uniform type identifiers (UTI).
    characteristics: Option<String>,
    /// Sets the string that represents the parameters of the rendition.
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
            is_default: false,
            autoselect: false,
            forced: false,
            instream_id: None,
            characteristics: None,
            channels: None,
        }
    }

    /// Create a builder to configure a new `ExtXMedia`-struct.
    pub fn builder() -> ExtXMediaBuilder {
        ExtXMediaBuilder::default()
    }

    /// Returns the type of the media associated with this tag.
    pub fn media_type(&self) -> MediaType {
        self.media_type
    }

    /// Returns the identifier that specifies the group to which the rendition belongs.
    pub fn group_id(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.group_id)
    }

    /// Returns a human-readable description of the rendition.
    pub fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }

    /// Returns the URI that identifies the media playlist.
    pub fn uri(&self) -> Option<Cow<'_, str>> {
        // TODO! Uri
        match &self.uri {
            Some(value) => Some(Cow::Borrowed(&value)),
            None => None,
        }
    }

    /// Returns the name of the primary language used in the rendition.
    // TODO: look in spec if this can be an enum?
    pub fn language(&self) -> Option<Cow<'_, str>> {
        match &self.language {
            Some(value) => Some(Cow::Borrowed(&value)),
            None => None,
        }
    }

    /// Returns the name of a language associated with the rendition.
    pub fn assoc_language(&self) -> Option<Cow<'_, str>> {
        match &self.assoc_language {
            Some(value) => Some(Cow::Borrowed(&value)),
            None => None,
        }
    }

    /// Returns whether this is the default rendition.
    pub fn is_default(&self) -> bool {
        self.is_default
    }

    /// Returns whether the client may choose to
    /// play this rendition in the absence of explicit user preference.
    pub fn is_autoselect(&self) -> bool {
        self.autoselect
    }

    /// Returns whether the rendition contains content that is considered essential to play.
    pub fn is_forced(&self) -> bool {
        self.forced
    }

    /// Returns the identifier that specifies a rendition within the segments in the media playlist.
    pub fn instream_id(&self) -> Option<InStreamId> {
        self.instream_id
    }

    /// Returns a string that represents uniform type identifiers (UTI).
    ///
    /// Each UTI indicates an individual characteristic of the rendition.
    pub fn characteristics(&self) -> Option<Cow<'_, str>> {
        match &self.characteristics {
            Some(value) => Some(Cow::Borrowed(&value)),
            None => None,
        }
    }

    /// Returns a string that represents the parameters of the rendition.
    pub fn channels(&self) -> Option<Cow<'_, str>> {
        match &self.channels {
            Some(value) => Some(Cow::Borrowed(&value)),
            None => None,
        }
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn required_version(&self) -> ProtocolVersion {
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

        if let Some(value) = &self.uri {
            write!(f, ",URI={}", quote(value))?;
        }

        write!(f, ",GROUP-ID={}", self.group_id)?;

        if let Some(value) = &self.language {
            write!(f, ",LANGUAGE={}", value)?;
        }

        if let Some(value) = &self.assoc_language {
            write!(f, ",ASSOC-LANGUAGE={}", value)?;
        }

        write!(f, ",NAME={}", self.name)?;

        if self.is_default {
            write!(f, ",DEFAULT=YES")?;
        }

        if self.autoselect {
            write!(f, ",AUTOSELECT=YES")?;
        }

        if self.forced {
            write!(f, ",FORCED=YES")?;
        }

        if let Some(value) = &self.instream_id {
            write!(f, ",INSTREAM-ID={}", quote(value))?;
        }

        if let Some(value) = &self.characteristics {
            write!(f, ",CHARACTERISTICS={}", value)?;
        }

        if let Some(value) = &self.channels {
            write!(f, ",CHANNELS={}", value)?;
        }

        Ok(())
    }
}

impl FromStr for ExtXMedia {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: ErrorKind::InvalidPrefix(what_this_line_is_instead)
        if !s.starts_with(Self::PREFIX) {
            Err(ErrorKind::InvalidInput)?;
        }

        let mut builder = ExtXMediaBuilder::default();

        let attrs = track!((s.split_at(Self::PREFIX.len()).1).parse::<AttributePairs>())?;

        for (key, value) in attrs {
            match key.as_str() {
                "TYPE" => {
                    builder.media_type(value.parse::<MediaType>()?);
                }
                "URI" => {
                    builder.uri(unquote(value));
                }
                "GROUP-ID" => {
                    builder.group_id(value);
                }
                "LANGUAGE" => {
                    builder.language(value);
                }
                "ASSOC-LANGUAGE" => {
                    builder.assoc_language(value);
                }
                "NAME" => {
                    builder.name(value);
                }
                "DEFAULT" => {
                    builder.is_default(track!(parse_yes_or_no(value))?);
                }
                "AUTOSELECT" => {
                    builder.autoselect(track!(parse_yes_or_no(value))?);
                }
                "FORCED" => {
                    builder.forced(track!(parse_yes_or_no(value))?);
                }
                "INSTREAM-ID" => {
                    builder.instream_id(unquote(value).parse::<InStreamId>()?);
                }
                "CHARACTERISTICS" => {
                    builder.characteristics(value);
                }
                "CHANNELS" => {
                    builder.channels(value);
                }
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        Ok(builder.build().map_err(|x| ErrorKind::BuilderError(x))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn ext_x_media() {
        let tag = ExtXMedia::new(MediaType::Audio, "foo".to_string(), "bar".to_string());
        let text = r#"#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID="foo",NAME="bar""#;
        assert_eq!(Some(text.parse().unwrap()), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.required_version(), ProtocolVersion::V1);
    }
}
