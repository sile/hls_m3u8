use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;

use crate::attribute::AttributePairs;
use crate::types::{InStreamId, MediaType, ProtocolVersion, RequiredVersion};
use crate::utils::{parse_yes_or_no, quote, tag, unquote};
use crate::Error;

/// # [4.4.4.1. EXT-X-MEDIA]
/// The [ExtXMedia] tag is used to relate [Media Playlist]s that contain
/// alternative Renditions of the same content. For
/// example, three [ExtXMedia] tags can be used to identify audio-only
/// [Media Playlist]s, that contain English, French, and Spanish Renditions
/// of the same presentation. Or, two [ExtXMedia] tags can be used to
/// identify video-only [Media Playlist]s that show two different camera
/// angles.
///
/// Its format is:
/// ```text
/// #EXT-X-MEDIA:<attribute-list>
/// ```
///
/// [Media Playlist]: crate::MediaPlaylist
/// [4.4.4.1. EXT-X-MEDIA]:
/// https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-04#section-4.4.4.1
#[derive(Builder, Debug, Clone, PartialEq, Eq, Hash)]
#[builder(setter(into))]
#[builder(build_fn(validate = "Self::validate"))]
pub struct ExtXMedia {
    /// Sets the media type of the rendition.
    media_type: MediaType,
    #[builder(setter(strip_option, into), default)]
    /// Sets the URI that identifies the media playlist.
    uri: Option<String>,
    /// Sets the identifier that specifies the group to which the rendition belongs.
    group_id: String,
    /// Sets the name of the primary language used in the rendition.
    #[builder(setter(strip_option, into), default)]
    language: Option<String>,
    /// Sets the name of a language associated with the rendition.
    #[builder(setter(strip_option, into), default)]
    assoc_language: Option<String>,
    /// Sets a human-readable description of the rendition.
    name: String,
    /// Sets the value of the `default` flag.
    #[builder(default)]
    is_default: bool,
    /// Sets the value of the `autoselect` flag.
    #[builder(default)]
    is_autoselect: bool,
    /// Sets the value of the `forced` flag.
    #[builder(default)]
    is_forced: bool,
    /// Sets the identifier that specifies a rendition within the segments in the media playlist.
    #[builder(setter(strip_option, into), default)]
    instream_id: Option<InStreamId>,
    /// Sets the string that represents uniform type identifiers (UTI).
    #[builder(setter(strip_option, into), default)]
    characteristics: Option<String>,
    /// Sets the string that represents the parameters of the rendition.
    #[builder(setter(strip_option, into), default)]
    channels: Option<String>,
}

impl ExtXMediaBuilder {
    fn validate(&self) -> Result<(), String> {
        let media_type = self
            .media_type
            .ok_or_else(|| Error::missing_attribute("MEDIA-TYPE").to_string())?;

        if MediaType::ClosedCaptions == media_type {
            if self.uri.is_some() {
                return Err(Error::custom(
                    "Unexpected attribute: \"URL\" for MediaType::ClosedCaptions!",
                )
                .to_string());
            }
            self.instream_id
                .ok_or_else(|| Error::missing_attribute("INSTREAM-ID").to_string())?;
        } else if self.instream_id.is_some() {
            return Err(Error::custom("Unexpected attribute: \"INSTREAM-ID\"!").to_string());
        }

        if self.is_default.unwrap_or(false) && !self.is_autoselect.unwrap_or(false) {
            return Err(
                Error::custom("If `DEFAULT` is true, `AUTOSELECT` has to be true too!").to_string(),
            );
        }

        if MediaType::Subtitles != media_type && self.is_forced.is_some() {
            return Err(Error::invalid_input().to_string());
        }

        Ok(())
    }
}

impl ExtXMedia {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MEDIA:";

    /// Makes a new [ExtXMedia] tag.
    pub fn new<T: ToString>(media_type: MediaType, group_id: T, name: T) -> Self {
        Self {
            media_type,
            uri: None,
            group_id: group_id.to_string(),
            language: None,
            assoc_language: None,
            name: name.to_string(),
            is_default: false,
            is_autoselect: false,
            is_forced: false,
            instream_id: None,
            characteristics: None,
            channels: None,
        }
    }

    /// Returns a builder for [ExtXMedia].
    pub fn builder() -> ExtXMediaBuilder {
        ExtXMediaBuilder::default()
    }

    /// Returns the type of the media, associated with this tag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// assert_eq!(
    ///     ExtXMedia::new(MediaType::Audio, "audio", "name").media_type(),
    ///     MediaType::Audio
    /// );
    /// ```
    pub const fn media_type(&self) -> MediaType {
        self.media_type
    }

    /// Sets the type of the media, associated with this tag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    ///
    /// media.set_media_type(MediaType::Video);
    ///
    /// assert_eq!(
    ///     media.media_type(),
    ///     MediaType::Video
    /// );
    /// ```
    pub fn set_media_type(&mut self, value: MediaType) -> &mut Self {
        self.media_type = value;
        self
    }

    /// Returns the identifier that specifies the group to which the rendition belongs.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// assert_eq!(
    ///     ExtXMedia::new(MediaType::Audio, "audio", "name").group_id(),
    ///     &"audio".to_string()
    /// );
    /// ```
    pub const fn group_id(&self) -> &String {
        &self.group_id
    }

    /// Sets the identifier that specifies the group, to which the rendition belongs.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    ///
    /// media.set_group_id("video");
    ///
    /// assert_eq!(
    ///     media.group_id(),
    ///     &"video".to_string()
    /// );
    /// ```
    pub fn set_group_id<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.group_id = value.into();
        self
    }

    /// Returns a human-readable description of the rendition.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// assert_eq!(
    ///     ExtXMedia::new(MediaType::Audio, "audio", "name").name(),
    ///     &"name".to_string()
    /// );
    /// ```
    pub const fn name(&self) -> &String {
        &self.name
    }

    /// Sets a human-readable description of the rendition.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    ///
    /// media.set_name("new_name");
    ///
    /// assert_eq!(
    ///     media.name(),
    ///     &"new_name".to_string()
    /// );
    /// ```
    pub fn set_name<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.name = value.into();
        self
    }

    /// Returns the `URI`, that identifies the [MediaPlaylist].
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.uri(), &None);
    ///
    /// media.set_uri(Some("https://www.example.com/"));
    ///
    /// assert_eq!(
    ///     media.uri(),
    ///     &Some("https://www.example.com/".into())
    /// );
    /// ```
    pub const fn uri(&self) -> &Option<String> {
        &self.uri
    }

    /// Sets the `URI`, that identifies the [MediaPlaylist].
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.uri(), &None);
    ///
    /// media.set_uri(Some("https://www.example.com/"));
    ///
    /// assert_eq!(
    ///     media.uri(),
    ///     &Some("https://www.example.com/".into())
    /// );
    /// ```
    pub fn set_uri<T: Into<String>>(&mut self, value: Option<T>) -> &mut Self {
        self.uri = value.map(|v| v.into());
        self
    }

    /// Returns the name of the primary language used in the rendition.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.language(), &None);
    ///
    /// media.set_language(Some("english"));
    ///
    /// assert_eq!(
    ///     media.language(),
    ///     &Some("english".into())
    /// );
    /// ```
    pub const fn language(&self) -> &Option<String> {
        &self.language
    }

    /// Sets the name of the primary language used in the rendition.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.language(), &None);
    ///
    /// media.set_language(Some("english"));
    ///
    /// assert_eq!(
    ///     media.language(),
    ///     &Some("english".into())
    /// );
    /// ```
    pub fn set_language<T: Into<String>>(&mut self, value: Option<T>) -> &mut Self {
        self.language = value.map(|v| v.into());
        self
    }

    /// Returns the name of a language associated with the rendition.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.assoc_language(), &None);
    ///
    /// media.set_assoc_language(Some("spanish"));
    ///
    /// assert_eq!(
    ///     media.assoc_language(),
    ///     &Some("spanish".into())
    /// );
    /// ```
    pub const fn assoc_language(&self) -> &Option<String> {
        &self.assoc_language
    }

    /// Sets the name of a language associated with the rendition.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.assoc_language(), &None);
    ///
    /// media.set_assoc_language(Some("spanish"));
    ///
    /// assert_eq!(
    ///     media.assoc_language(),
    ///     &Some("spanish".into())
    /// );
    /// ```
    pub fn set_assoc_language<T: Into<String>>(&mut self, value: Option<T>) -> &mut Self {
        self.assoc_language = value.map(|v| v.into());
        self
    }

    /// Returns whether this is the default rendition.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.is_default(), false);
    ///
    /// media.set_default(true);
    ///
    /// assert_eq!(
    ///     media.is_default(),
    ///     true
    /// );
    /// ```
    pub const fn is_default(&self) -> bool {
        self.is_default
    }

    /// Sets the `default` flag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.is_default(), false);
    ///
    /// media.set_default(true);
    ///
    /// assert_eq!(media.is_default(), true);
    /// ```
    pub fn set_default(&mut self, value: bool) -> &mut Self {
        self.is_default = value;
        self
    }

    /// Returns whether the client may choose to
    /// play this rendition in the absence of explicit user preference.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.is_autoselect(), false);
    ///
    /// media.set_autoselect(true);
    ///
    /// assert_eq!(media.is_autoselect(), true);
    /// ```
    pub const fn is_autoselect(&self) -> bool {
        self.is_autoselect
    }

    /// Sets the `autoselect` flag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.is_autoselect(), false);
    ///
    /// media.set_autoselect(true);
    ///
    /// assert_eq!(media.is_autoselect(), true);
    /// ```
    pub fn set_autoselect(&mut self, value: bool) -> &mut Self {
        self.is_autoselect = value;
        self
    }

    /// Returns whether the rendition contains content that is considered essential to play.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.is_forced(), false);
    ///
    /// media.set_forced(true);
    ///
    /// assert_eq!(media.is_forced(), true);
    /// ```
    pub const fn is_forced(&self) -> bool {
        self.is_forced
    }

    /// Sets the `forced` flag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.is_forced(), false);
    ///
    /// media.set_forced(true);
    ///
    /// assert_eq!(media.is_forced(), true);
    /// ```
    pub fn set_forced(&mut self, value: bool) -> &mut Self {
        self.is_forced = value;
        self
    }

    /// Returns the identifier that specifies a rendition within the segments in the
    /// [MediaPlaylist].
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::{InStreamId, MediaType};
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.instream_id(), None);
    ///
    /// media.set_instream_id(Some(InStreamId::Cc1));
    ///
    /// assert_eq!(media.instream_id(), Some(InStreamId::Cc1));
    /// ```
    pub const fn instream_id(&self) -> Option<InStreamId> {
        self.instream_id
    }

    /// Sets the [InStreamId].
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::{InStreamId, MediaType};
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.instream_id(), None);
    ///
    /// media.set_instream_id(Some(InStreamId::Cc1));
    ///
    /// assert_eq!(media.instream_id(), Some(InStreamId::Cc1));
    /// ```
    pub fn set_instream_id(&mut self, value: Option<InStreamId>) -> &mut Self {
        self.instream_id = value;
        self
    }

    /// Returns a string that represents uniform type identifiers (UTI).
    ///
    /// Each UTI indicates an individual characteristic of the rendition.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.characteristics(), &None);
    ///
    /// media.set_characteristics(Some("characteristic"));
    ///
    /// assert_eq!(media.characteristics(), &Some("characteristic".into()));
    /// ```
    pub const fn characteristics(&self) -> &Option<String> {
        &self.characteristics
    }

    /// Sets the characteristics.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.characteristics(), &None);
    ///
    /// media.set_characteristics(Some("characteristic"));
    ///
    /// assert_eq!(media.characteristics(), &Some("characteristic".into()));
    /// ```
    pub fn set_characteristics<T: Into<String>>(&mut self, value: Option<T>) -> &mut Self {
        self.characteristics = value.map(|v| v.into());
        self
    }

    /// Returns a [String] that represents the parameters of the rendition.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.channels(), &None);
    ///
    /// media.set_channels(Some("channel"));
    ///
    /// assert_eq!(media.channels(), &Some("channel".into()));
    /// ```
    pub const fn channels(&self) -> &Option<String> {
        &self.channels
    }

    /// Sets the channels.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let mut media = ExtXMedia::new(MediaType::Audio, "audio", "name");
    /// # assert_eq!(media.characteristics(), &None);
    ///
    /// media.set_characteristics(Some("characteristic"));
    ///
    /// assert_eq!(media.characteristics(), &Some("characteristic".into()));
    /// ```
    pub fn set_channels<T: Into<String>>(&mut self, value: Option<T>) -> &mut Self {
        self.channels = value.map(|v| v.into());
        self
    }
}

impl RequiredVersion for ExtXMedia {
    fn required_version(&self) -> ProtocolVersion {
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
        write!(f, ",GROUP-ID={}", quote(&self.group_id))?;
        if let Some(value) = &self.language {
            write!(f, ",LANGUAGE={}", quote(value))?;
        }
        if let Some(value) = &self.assoc_language {
            write!(f, ",ASSOC-LANGUAGE={}", quote(value))?;
        }
        write!(f, ",NAME={}", quote(&self.name))?;
        if self.is_default {
            write!(f, ",DEFAULT=YES")?;
        }
        if self.is_autoselect {
            write!(f, ",AUTOSELECT=YES")?;
        }
        if self.is_forced {
            write!(f, ",FORCED=YES")?;
        }
        if let Some(value) = &self.instream_id {
            write!(f, ",INSTREAM-ID={}", quote(value))?;
        }
        if let Some(value) = &self.characteristics {
            write!(f, ",CHARACTERISTICS={}", quote(value))?;
        }
        if let Some(value) = &self.channels {
            write!(f, ",CHANNELS={}", quote(value))?;
        }
        Ok(())
    }
}

impl FromStr for ExtXMedia {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let mut builder = Self::builder();

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "TYPE" => {
                    builder.media_type(value.parse::<MediaType>()?);
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
                    builder.is_default(parse_yes_or_no(value)?);
                }
                "AUTOSELECT" => {
                    builder.is_autoselect(parse_yes_or_no(value)?);
                }
                "FORCED" => {
                    builder.is_forced(parse_yes_or_no(value)?);
                }
                "INSTREAM-ID" => {
                    builder.instream_id(unquote(value).parse::<InStreamId>()?);
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
        builder.build().map_err(Error::builder_error)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        // TODO: https://developer.apple.com/documentation/http_live_streaming/example_playlists_for_http_live_streaming/adding_alternate_media_to_a_playlist
        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio")
                .language("eng")
                .name("English")
                .is_autoselect(true)
                .is_default(true)
                .uri("eng/prog_index.m3u8")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"eng/prog_index.m3u8\",\
             GROUP-ID=\"audio\",\
             LANGUAGE=\"eng\",\
             NAME=\"English\",\
             DEFAULT=YES,\
             AUTOSELECT=YES"
                .to_string()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio")
                .language("fre")
                .name("Français")
                .is_autoselect(true)
                .is_default(false)
                .uri("fre/prog_index.m3u8")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"fre/prog_index.m3u8\",\
             GROUP-ID=\"audio\",\
             LANGUAGE=\"fre\",\
             NAME=\"Français\",\
             AUTOSELECT=YES"
                .to_string()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio")
                .language("sp")
                .name("Espanol")
                .is_autoselect(true)
                .is_default(false)
                .uri("sp/prog_index.m3u8")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"sp/prog_index.m3u8\",\
             GROUP-ID=\"audio\",\
             LANGUAGE=\"sp\",\
             NAME=\"Espanol\",\
             AUTOSELECT=YES"
                .to_string()
        );
        // ----
        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-lo")
                .language("eng")
                .name("English")
                .is_autoselect(true)
                .is_default(true)
                .uri("englo/prog_index.m3u8")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"englo/prog_index.m3u8\",\
             GROUP-ID=\"audio-lo\",\
             LANGUAGE=\"eng\",\
             NAME=\"English\",\
             DEFAULT=YES,\
             AUTOSELECT=YES"
                .to_string()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-lo")
                .language("fre")
                .name("Français")
                .is_autoselect(true)
                .is_default(false)
                .uri("frelo/prog_index.m3u8")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"frelo/prog_index.m3u8\",\
             GROUP-ID=\"audio-lo\",\
             LANGUAGE=\"fre\",\
             NAME=\"Français\",\
             AUTOSELECT=YES"
                .to_string()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-lo")
                .language("es")
                .name("Espanol")
                .is_autoselect(true)
                .is_default(false)
                .uri("splo/prog_index.m3u8")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"splo/prog_index.m3u8\",\
             GROUP-ID=\"audio-lo\",\
             LANGUAGE=\"es\",\
             NAME=\"Espanol\",\
             AUTOSELECT=YES"
                .to_string()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-hi")
                .language("eng")
                .name("English")
                .is_autoselect(true)
                .is_default(true)
                .uri("eng/prog_index.m3u8")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"eng/prog_index.m3u8\",\
             GROUP-ID=\"audio-hi\",\
             LANGUAGE=\"eng\",\
             NAME=\"English\",\
             DEFAULT=YES,\
             AUTOSELECT=YES"
                .to_string()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-hi")
                .language("fre")
                .name("Français")
                .is_autoselect(true)
                .is_default(false)
                .uri("fre/prog_index.m3u8")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"fre/prog_index.m3u8\",\
             GROUP-ID=\"audio-hi\",\
             LANGUAGE=\"fre\",\
             NAME=\"Français\",\
             AUTOSELECT=YES"
                .to_string()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-hi")
                .language("es")
                .name("Espanol")
                .is_autoselect(true)
                .is_default(false)
                .uri("sp/prog_index.m3u8")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"sp/prog_index.m3u8\",\
             GROUP-ID=\"audio-hi\",\
             LANGUAGE=\"es\",\
             NAME=\"Espanol\",\
             AUTOSELECT=YES"
                .to_string()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-aacl-312")
                .language("en")
                .name("English")
                .is_autoselect(true)
                .is_default(true)
                .channels("2")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             GROUP-ID=\"audio-aacl-312\",\
             LANGUAGE=\"en\",\
             NAME=\"English\",\
             DEFAULT=YES,\
             AUTOSELECT=YES,\
             CHANNELS=\"2\""
                .to_string()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Subtitles)
                .uri("french/ed.ttml")
                .group_id("subs")
                .language("fra")
                .assoc_language("fra")
                .name("French")
                .is_autoselect(true)
                .is_forced(true)
                .characteristics("public.accessibility.transcribes-spoken-dialog,public.accessibility.describes-music-and-sound")
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
            TYPE=SUBTITLES,\
            URI=\"french/ed.ttml\",\
            GROUP-ID=\"subs\",\
            LANGUAGE=\"fra\",\
            ASSOC-LANGUAGE=\"fra\",\
            NAME=\"French\",\
            AUTOSELECT=YES,\
            FORCED=YES,\
            CHARACTERISTICS=\"public.accessibility.transcribes-spoken-dialog,public.accessibility.describes-music-and-sound\"".to_string()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::ClosedCaptions)
                .group_id("cc")
                .language("sp")
                .name("CC2")
                .instream_id(InStreamId::Cc2)
                .is_autoselect(true)
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-MEDIA:\
             TYPE=CLOSED-CAPTIONS,\
             GROUP-ID=\"cc\",\
             LANGUAGE=\"sp\",\
             NAME=\"CC2\",\
             AUTOSELECT=YES,\
             INSTREAM-ID=\"CC2\""
                .to_string()
        );

        // ----
        assert_eq!(
            ExtXMedia::new(MediaType::Audio, "foo", "bar").to_string(),
            "#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"foo\",NAME=\"bar\"".to_string()
        )
    }

    #[test]
    fn test_parser() {
        // TODO: https://developer.apple.com/documentation/http_live_streaming/example_playlists_for_http_live_streaming/adding_alternate_media_to_a_playlist
        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio")
                .language("eng")
                .name("English")
                .is_autoselect(true)
                .is_default(true)
                .uri("eng/prog_index.m3u8")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"eng/prog_index.m3u8\",\
             GROUP-ID=\"audio\",\
             LANGUAGE=\"eng\",\
             NAME=\"English\",\
             DEFAULT=YES,\
             AUTOSELECT=YES"
                .parse()
                .unwrap()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio")
                .language("fre")
                .name("Français")
                .is_autoselect(true)
                .is_default(false)
                .uri("fre/prog_index.m3u8")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"fre/prog_index.m3u8\",\
             GROUP-ID=\"audio\",\
             LANGUAGE=\"fre\",\
             NAME=\"Français\",\
             AUTOSELECT=YES"
                .parse()
                .unwrap()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio")
                .language("sp")
                .name("Espanol")
                .is_autoselect(true)
                .is_default(false)
                .uri("sp/prog_index.m3u8")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"sp/prog_index.m3u8\",\
             GROUP-ID=\"audio\",\
             LANGUAGE=\"sp\",\
             NAME=\"Espanol\",\
             AUTOSELECT=YES"
                .parse()
                .unwrap()
        );
        // ----
        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-lo")
                .language("eng")
                .name("English")
                .is_autoselect(true)
                .is_default(true)
                .uri("englo/prog_index.m3u8")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"englo/prog_index.m3u8\",\
             GROUP-ID=\"audio-lo\",\
             LANGUAGE=\"eng\",\
             NAME=\"English\",\
             DEFAULT=YES,\
             AUTOSELECT=YES"
                .parse()
                .unwrap()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-lo")
                .language("fre")
                .name("Français")
                .is_autoselect(true)
                .is_default(false)
                .uri("frelo/prog_index.m3u8")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"frelo/prog_index.m3u8\",\
             GROUP-ID=\"audio-lo\",\
             LANGUAGE=\"fre\",\
             NAME=\"Français\",\
             AUTOSELECT=YES"
                .parse()
                .unwrap()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-lo")
                .language("es")
                .name("Espanol")
                .is_autoselect(true)
                .is_default(false)
                .uri("splo/prog_index.m3u8")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"splo/prog_index.m3u8\",\
             GROUP-ID=\"audio-lo\",\
             LANGUAGE=\"es\",\
             NAME=\"Espanol\",\
             AUTOSELECT=YES"
                .parse()
                .unwrap()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-hi")
                .language("eng")
                .name("English")
                .is_autoselect(true)
                .is_default(true)
                .uri("eng/prog_index.m3u8")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"eng/prog_index.m3u8\",\
             GROUP-ID=\"audio-hi\",\
             LANGUAGE=\"eng\",\
             NAME=\"English\",\
             DEFAULT=YES,\
             AUTOSELECT=YES"
                .parse()
                .unwrap()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-hi")
                .language("fre")
                .name("Français")
                .is_autoselect(true)
                .is_default(false)
                .uri("fre/prog_index.m3u8")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"fre/prog_index.m3u8\",\
             GROUP-ID=\"audio-hi\",\
             LANGUAGE=\"fre\",\
             NAME=\"Français\",\
             AUTOSELECT=YES"
                .parse()
                .unwrap()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-hi")
                .language("es")
                .name("Espanol")
                .is_autoselect(true)
                .is_default(false)
                .uri("sp/prog_index.m3u8")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             URI=\"sp/prog_index.m3u8\",\
             GROUP-ID=\"audio-hi\",\
             LANGUAGE=\"es\",\
             NAME=\"Espanol\",\
             AUTOSELECT=YES"
                .parse()
                .unwrap()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-aacl-312")
                .language("en")
                .name("English")
                .is_autoselect(true)
                .is_default(true)
                .channels("2")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=AUDIO,\
             GROUP-ID=\"audio-aacl-312\",\
             LANGUAGE=\"en\",\
             NAME=\"English\",\
             DEFAULT=YES,\
             AUTOSELECT=YES,\
             CHANNELS=\"2\""
                .parse()
                .unwrap()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Subtitles)
                .uri("french/ed.ttml")
                .group_id("subs")
                .language("fra")
                .assoc_language("fra")
                .name("French")
                .is_autoselect(true)
                .characteristics("public.accessibility.transcribes-spoken-dialog,public.accessibility.describes-music-and-sound")
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
            URI=\"french/ed.ttml\",\
            TYPE=SUBTITLES,\
            GROUP-ID=\"subs\",\
            LANGUAGE=\"fra\",\
            ASSOC-LANGUAGE=\"fra\",\
            NAME=\"French\",\
            AUTOSELECT=YES,\
            FORCED=NO,\
            CHARACTERISTICS=\"public.accessibility.transcribes-spoken-dialog,public.accessibility.describes-music-and-sound\"".parse().unwrap()
        );

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::ClosedCaptions)
                .group_id("cc")
                .language("sp")
                .name("CC2")
                .instream_id(InStreamId::Cc2)
                .is_autoselect(true)
                .build()
                .unwrap(),
            "#EXT-X-MEDIA:\
             TYPE=CLOSED-CAPTIONS,\
             GROUP-ID=\"cc\",\
             LANGUAGE=\"sp\",\
             NAME=\"CC2\",\
             AUTOSELECT=YES,\
             INSTREAM-ID=\"CC2\""
                .parse()
                .unwrap()
        );

        // ----
        assert_eq!(
            ExtXMedia::new(MediaType::Audio, "foo", "bar"),
            "#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"foo\",NAME=\"bar\""
                .parse()
                .unwrap()
        )
    }

    #[test]
    fn test_required_version() {
        macro_rules! gen_required_version {
            ( $( $id:expr => $output:expr, )* ) => {
                $(
                    assert_eq!(
                        ExtXMedia::builder()
                            .media_type(MediaType::ClosedCaptions)
                            .group_id("audio")
                            .name("English")
                            .instream_id($id)
                            .build()
                            .unwrap()
                            .required_version(),
                        $output
                    );
                )*
            }
        }

        gen_required_version![
            InStreamId::Cc1 => ProtocolVersion::V1,
            InStreamId::Cc2 => ProtocolVersion::V1,
            InStreamId::Cc3 => ProtocolVersion::V1,
            InStreamId::Cc4 => ProtocolVersion::V1,
            InStreamId::Service1 => ProtocolVersion::V7,
        ];

        assert_eq!(
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio")
                .name("English")
                .build()
                .unwrap()
                .required_version(),
            ProtocolVersion::V1
        );
    }
}
