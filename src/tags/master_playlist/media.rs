use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;

use crate::attribute::AttributePairs;
use crate::types::{Channels, InStreamId, MediaType, ProtocolVersion};
use crate::utils::{parse_yes_or_no, quote, tag, unquote};
use crate::{Error, RequiredVersion};

/// An [`ExtXMedia`] tag is an alternative rendition of a [`VariantStream`].
///
/// For example an [`ExtXMedia`] tag can be used to specify different audio
/// languages (e.g. english is the default and there also exists an
/// [`ExtXMedia`] stream with a german audio).
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
/// [`VariantStream`]: crate::tags::VariantStream
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtXMedia<'a> {
    /// The [`MediaType`] associated with this tag.
    ///
    /// ### Note
    ///
    /// This field is required.
    pub media_type: MediaType,
    uri: Option<Cow<'a, str>>,
    group_id: Cow<'a, str>,
    language: Option<Cow<'a, str>>,
    assoc_language: Option<Cow<'a, str>>,
    name: Cow<'a, str>,
    /// The value of the `default` flag.
    /// A value of `true` indicates, that the client should play
    /// this rendition of the content in the absence of information
    /// from the user indicating a different choice.
    ///
    /// ### Note
    ///
    /// This field is optional, its absence indicates an implicit value
    /// of `false`.
    pub is_default: bool,
    /// Whether the client may choose to play this rendition in the absence of
    /// explicit user preference.
    ///
    /// ### Note
    ///
    /// This field is optional, its absence indicates an implicit value
    /// of `false`.
    pub is_autoselect: bool,
    /// Whether the rendition contains content that is considered
    /// essential to play.
    pub is_forced: bool,
    /// An [`InStreamId`] identifies a rendition within the
    /// [`MediaSegment`]s in a [`MediaPlaylist`].
    ///
    /// ### Note
    ///
    /// This field is required, if the media type is
    /// [`MediaType::ClosedCaptions`]. For all other media types the
    /// [`InStreamId`] must not be specified!
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    /// [`MediaSegment`]: crate::MediaSegment
    pub instream_id: Option<InStreamId>,
    characteristics: Option<Cow<'a, str>>,
    /// A count of audio channels indicating the maximum number of independent,
    /// simultaneous audio channels present in any [`MediaSegment`] in the
    /// rendition.
    ///
    /// ### Note
    ///
    /// This field is optional, but every instance of [`ExtXMedia`] with
    /// [`MediaType::Audio`] should have this field. If the [`MasterPlaylist`]
    /// contains two renditions with the same codec, but a different number of
    /// channels, then the channels field is required.
    ///
    /// [`MediaSegment`]: crate::MediaSegment
    /// [`MasterPlaylist`]: crate::MasterPlaylist
    pub channels: Option<Channels>,
}

/// Builder for [`ExtXMedia`].
#[derive(Debug, Clone, Default)]
pub struct ExtXMediaBuilder<'a> {
    media_type: Option<MediaType>,
    uri: Option<Cow<'a, str>>,
    group_id: Option<Cow<'a, str>>,
    language: Option<Cow<'a, str>>,
    assoc_language: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    is_default: Option<bool>,
    is_autoselect: Option<bool>,
    is_forced: Option<bool>,
    instream_id: Option<InStreamId>,
    characteristics: Option<Cow<'a, str>>,
    channels: Option<Channels>,
}

impl<'a> ExtXMediaBuilder<'a> {
    /// See [`ExtXMedia::media_type`].
    pub fn media_type(&mut self, value: MediaType) -> &mut Self {
        self.media_type = Some(value);
        self
    }

    /// See [`ExtXMedia::uri`].
    pub fn uri<V: Into<Cow<'a, str>>>(&mut self, value: V) -> &mut Self {
        self.uri = Some(value.into());
        self
    }

    /// See [`ExtXMedia::group_id`].
    pub fn group_id<V: Into<Cow<'a, str>>>(&mut self, value: V) -> &mut Self {
        self.group_id = Some(value.into());
        self
    }

    /// See [`ExtXMedia::language`].
    pub fn language<V: Into<Cow<'a, str>>>(&mut self, value: V) -> &mut Self {
        self.language = Some(value.into());
        self
    }

    /// See [`ExtXMedia::assoc_language`].
    pub fn assoc_language<V: Into<Cow<'a, str>>>(&mut self, value: V) -> &mut Self {
        self.assoc_language = Some(value.into());
        self
    }

    /// See [`ExtXMedia::name`].
    pub fn name<V: Into<Cow<'a, str>>>(&mut self, value: V) -> &mut Self {
        self.name = Some(value.into());
        self
    }

    /// See [`ExtXMedia::is_default`].
    pub fn is_default(&mut self, value: bool) -> &mut Self {
        self.is_default = Some(value);
        self
    }

    /// See [`ExtXMedia::is_autoselect`].
    pub fn is_autoselect(&mut self, value: bool) -> &mut Self {
        self.is_autoselect = Some(value);
        self
    }

    /// See [`ExtXMedia::is_forced`].
    pub fn is_forced(&mut self, value: bool) -> &mut Self {
        self.is_forced = Some(value);
        self
    }

    /// See [`ExtXMedia::instream_id`].
    pub fn instream_id(&mut self, value: InStreamId) -> &mut Self {
        self.instream_id = Some(value);
        self
    }

    /// See [`ExtXMedia::characteristics`].
    pub fn characteristics<V: Into<Cow<'a, str>>>(&mut self, value: V) -> &mut Self {
        self.characteristics = Some(value.into());
        self
    }

    /// See [`ExtXMedia::channels`].
    pub fn channels(&mut self, value: Channels) -> &mut Self {
        self.channels = Some(value);
        self
    }

    fn validate(&self) -> Result<MediaType, Error> {
        let media_type = self
            .media_type
            .ok_or_else(|| Error::missing_attribute("MEDIA-TYPE"))?;

        if media_type == MediaType::Subtitles && self.uri.is_none() {
            return Err(Error::missing_attribute("URI"));
        }

        if media_type == MediaType::ClosedCaptions {
            if self.uri.is_some() {
                return Err(Error::unexpected_attribute("URI"));
            }
            if self.instream_id.is_none() {
                return Err(Error::missing_attribute("INSTREAM-ID"));
            }
        } else if self.instream_id.is_some() {
            return Err(Error::custom(
                "InStreamId should only be specified for an ExtXMedia tag with `MediaType::ClosedCaptions`",
            ));
        }

        if self.is_default.unwrap_or(false) && self.is_autoselect.is_some_and(|b| !b) {
            return Err(Error::custom(format!(
                "If `DEFAULT` is true, `AUTOSELECT` has to be true too, if present. Default: {:?}, Autoselect: {:?}!",
                self.is_default, self.is_autoselect,
            )));
        }

        if media_type != MediaType::Subtitles && self.is_forced.unwrap_or(false) {
            return Err(Error::custom(format!(
                concat!(
                    "the forced attribute must not be present, ",
                    "unless the media_type is `MediaType::Subtitles`: ",
                    "media_type: {:?}, is_forced: {:?}"
                ),
                media_type, self.is_forced,
            )));
        }

        Ok(media_type)
    }

    /// Builds a new [`ExtXMedia`].
    ///
    /// # Errors
    ///
    /// If a required field has not been initialized or validation fails.
    pub fn build(&self) -> Result<ExtXMedia<'a>, Error> {
        let media_type = self.validate()?;

        Ok(ExtXMedia {
            media_type,
            uri: self.uri.clone(),
            group_id: self
                .group_id
                .clone()
                .ok_or_else(|| Error::missing_field("ExtXMedia", "group_id"))?,
            language: self.language.clone(),
            assoc_language: self.assoc_language.clone(),
            name: self
                .name
                .clone()
                .ok_or_else(|| Error::missing_field("ExtXMedia", "name"))?,
            is_default: self.is_default.unwrap_or(false),
            is_autoselect: self.is_autoselect.unwrap_or(false),
            is_forced: self.is_forced.unwrap_or(false),
            instream_id: self.instream_id,
            characteristics: self.characteristics.clone(),
            channels: self.channels,
        })
    }
}

impl<'a> ExtXMedia<'a> {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MEDIA:";

    /// An `URI` to a [`MediaPlaylist`].
    ///
    /// ### Note
    ///
    /// - This field is required, if the [`ExtXMedia::media_type`] is
    ///   [`MediaType::Subtitles`].
    /// - This field is not allowed, if the [`ExtXMedia::media_type`] is
    ///   [`MediaType::ClosedCaptions`].
    ///
    /// An absent value indicates that the media data for this rendition is
    /// included in the [`MediaPlaylist`] of any
    /// [`VariantStream::ExtXStreamInf`] tag with the same `group_id` of
    /// this [`ExtXMedia`] instance.
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    /// [`VariantStream::ExtXStreamInf`]:
    /// crate::tags::VariantStream::ExtXStreamInf
    #[must_use]
    pub fn uri(&self) -> Option<&Cow<'a, str>> {
        self.uri.as_ref()
    }

    /// Sets [`ExtXMedia::uri`].
    pub fn set_uri<V: Into<Cow<'a, str>>>(&mut self, value: Option<V>) -> &mut Self {
        self.uri = value.map(Into::into);
        self
    }

    /// The identifier that specifies the group to which the rendition
    /// belongs.
    ///
    /// ### Note
    ///
    /// This field is required.
    #[must_use]
    pub fn group_id(&self) -> &Cow<'a, str> {
        &self.group_id
    }

    /// Sets [`ExtXMedia::group_id`].
    pub fn set_group_id<V: Into<Cow<'a, str>>>(&mut self, value: V) -> &mut Self {
        self.group_id = value.into();
        self
    }

    /// The name of the primary language used in the rendition.
    /// The value has to conform to [`RFC5646`].
    ///
    /// ### Note
    ///
    /// This field is optional.
    ///
    /// [`RFC5646`]: https://tools.ietf.org/html/rfc5646
    #[must_use]
    pub fn language(&self) -> Option<&Cow<'a, str>> {
        self.language.as_ref()
    }

    /// Sets [`ExtXMedia::language`].
    pub fn set_language<V: Into<Cow<'a, str>>>(&mut self, value: Option<V>) -> &mut Self {
        self.language = value.map(Into::into);
        self
    }

    /// The name of a language associated with the rendition.
    /// An associated language is often used in a different role, than the
    /// language specified by the [`ExtXMedia::language`] field (e.g., written
    /// versus spoken, or a fallback dialect).
    ///
    /// ### Note
    ///
    /// This field is optional.
    #[must_use]
    pub fn assoc_language(&self) -> Option<&Cow<'a, str>> {
        self.assoc_language.as_ref()
    }

    /// Sets [`ExtXMedia::assoc_language`].
    pub fn set_assoc_language<V: Into<Cow<'a, str>>>(&mut self, value: Option<V>) -> &mut Self {
        self.assoc_language = value.map(Into::into);
        self
    }

    /// A human-readable description of the rendition.
    ///
    /// ### Note
    ///
    /// This field is required.
    ///
    /// If the [`ExtXMedia::language`] field is present, this field should be
    /// in that language.
    #[must_use]
    pub fn name(&self) -> &Cow<'a, str> {
        &self.name
    }

    /// Sets [`ExtXMedia::name`].
    pub fn set_name<V: Into<Cow<'a, str>>>(&mut self, value: V) -> &mut Self {
        self.name = value.into();
        self
    }

    /// The characteristics field contains one or more Uniform Type
    /// Identifiers ([`UTI`]) separated by a comma.
    /// Each [`UTI`] indicates an individual characteristic of the Rendition.
    ///
    /// An `ExtXMedia` instance with [`MediaType::Subtitles`] may include the
    /// following characteristics:
    /// - `"public.accessibility.transcribes-spoken-dialog"`,
    /// - `"public.accessibility.describes-music-and-sound"`, and
    /// - `"public.easy-to-read"` (which indicates that the subtitles have been
    ///   edited for ease of reading).
    ///
    /// An `ExtXMedia` instance with [`MediaType::Audio`] may include the
    /// following characteristic:
    /// - `"public.accessibility.describes-video"`
    ///
    /// The characteristics field may include private UTIs.
    ///
    /// ### Note
    ///
    /// This field is optional.
    ///
    /// [`UTI`]: https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-05#ref-UTI
    #[must_use]
    pub fn characteristics(&self) -> Option<&Cow<'a, str>> {
        self.characteristics.as_ref()
    }

    /// Sets [`ExtXMedia::characteristics`].
    pub fn set_characteristics<V: Into<Cow<'a, str>>>(&mut self, value: Option<V>) -> &mut Self {
        self.characteristics = value.map(Into::into);
        self
    }

    /// Makes a new [`ExtXMedia`] tag with the associated [`MediaType`], the
    /// identifier that specifies the group to which the rendition belongs
    /// (group id) and a human-readable description of the rendition. If the
    /// [`language`] is specified it should be in that language.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let media = ExtXMedia::new(MediaType::Video, "vg1", "1080p video stream");
    /// ```
    ///
    /// [`language`]: #method.language
    #[must_use]
    pub fn new<T, K>(media_type: MediaType, group_id: T, name: K) -> Self
    where
        T: Into<Cow<'a, str>>,
        K: Into<Cow<'a, str>>,
    {
        Self {
            media_type,
            uri: None,
            group_id: group_id.into(),
            language: None,
            assoc_language: None,
            name: name.into(),
            is_default: false,
            is_autoselect: false,
            is_forced: false,
            instream_id: None,
            characteristics: None,
            channels: None,
        }
    }

    /// Returns a builder for [`ExtXMedia`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXMedia;
    /// use hls_m3u8::types::MediaType;
    ///
    /// let media = ExtXMedia::builder()
    ///     .media_type(MediaType::Subtitles)
    ///     .uri("french/ed.ttml")
    ///     .group_id("subs")
    ///     .language("fra")
    ///     .assoc_language("fra")
    ///     .name("French")
    ///     .is_autoselect(true)
    ///     .is_forced(true)
    ///     // concat! joins multiple `&'static str`
    ///     .characteristics(concat!(
    ///         "public.accessibility.transcribes-spoken-dialog,",
    ///         "public.accessibility.describes-music-and-sound"
    ///     ))
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn builder() -> ExtXMediaBuilder<'a> {
        ExtXMediaBuilder::default()
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> ExtXMedia<'static> {
        ExtXMedia {
            media_type: self.media_type,
            uri: self.uri.map(|v| Cow::Owned(v.into_owned())),
            group_id: Cow::Owned(self.group_id.into_owned()),
            language: self.language.map(|v| Cow::Owned(v.into_owned())),
            assoc_language: self.assoc_language.map(|v| Cow::Owned(v.into_owned())),
            name: Cow::Owned(self.name.into_owned()),
            is_default: self.is_default,
            is_autoselect: self.is_autoselect,
            is_forced: self.is_forced,
            instream_id: self.instream_id,
            characteristics: self.characteristics.map(|v| Cow::Owned(v.into_owned())),
            channels: self.channels,
        }
    }
}

/// This tag requires either `ProtocolVersion::V1` or if there is an
/// `instream_id` it requires it's version.
impl RequiredVersion for ExtXMedia<'_> {
    fn required_version(&self) -> ProtocolVersion {
        self.instream_id
            .map_or(ProtocolVersion::V1, |i| i.required_version())
    }
}

impl fmt::Display for ExtXMedia<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl<'a> TryFrom<&'a str> for ExtXMedia<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let input = tag(input, Self::PREFIX)?;

        let mut builder = Self::builder();

        for (key, value) in AttributePairs::new(input) {
            match key {
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
                    builder.channels(unquote(value).parse::<Channels>()?);
                }
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized
                    // AttributeName.
                }
            }
        }

        builder.build()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! generate_tests {
        ( $( { $struct:expr, $str:expr } ),+ $(,)* ) => {
            #[test]
            fn test_display() {
                $(
                    assert_eq!($struct.to_string(), $str.to_string());
                )+
            }

            #[test]
            fn test_parser() {
                $(
                    assert_eq!($struct, TryFrom::try_from($str).unwrap());
                )+
            }
        }
    }

    generate_tests! {
        {
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio")
                .language("eng")
                .name("English")
                .is_default(true)
                .uri("eng/prog_index.m3u8")
                .build()
                .unwrap(),
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "URI=\"eng/prog_index.m3u8\",",
                "GROUP-ID=\"audio\",",
                "LANGUAGE=\"eng\",",
                "NAME=\"English\",",
                "DEFAULT=YES",
            )
        },
        {
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
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "URI=\"eng/prog_index.m3u8\",",
                "GROUP-ID=\"audio\",",
                "LANGUAGE=\"eng\",",
                "NAME=\"English\",",
                "DEFAULT=YES,",
                "AUTOSELECT=YES"
            )
        },
        {
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .uri("fre/prog_index.m3u8")
                .group_id("audio")
                .language("fre")
                .name("Français")
                .is_default(false)
                .is_autoselect(true)
                .build()
                .unwrap(),
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "URI=\"fre/prog_index.m3u8\",",
                "GROUP-ID=\"audio\",",
                "LANGUAGE=\"fre\",",
                "NAME=\"Français\",",
                "AUTOSELECT=YES"
            )
        },
        {
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
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "URI=\"sp/prog_index.m3u8\",",
                "GROUP-ID=\"audio\",",
                "LANGUAGE=\"sp\",",
                "NAME=\"Espanol\",",
                "AUTOSELECT=YES"
            )
        },
        {
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
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "URI=\"englo/prog_index.m3u8\",",
                "GROUP-ID=\"audio-lo\",",
                "LANGUAGE=\"eng\",",
                "NAME=\"English\",",
                "DEFAULT=YES,",
                "AUTOSELECT=YES"
            )
        },
        {
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
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "URI=\"frelo/prog_index.m3u8\",",
                "GROUP-ID=\"audio-lo\",",
                "LANGUAGE=\"fre\",",
                "NAME=\"Français\",",
                "AUTOSELECT=YES"
            )
        },
        {
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
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "URI=\"splo/prog_index.m3u8\",",
                "GROUP-ID=\"audio-lo\",",
                "LANGUAGE=\"es\",",
                "NAME=\"Espanol\",",
                "AUTOSELECT=YES"
            )
        },
        {
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
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "URI=\"eng/prog_index.m3u8\",",
                "GROUP-ID=\"audio-hi\",",
                "LANGUAGE=\"eng\",",
                "NAME=\"English\",",
                "DEFAULT=YES,",
                "AUTOSELECT=YES"
            )
        },
        {
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
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "URI=\"fre/prog_index.m3u8\",",
                "GROUP-ID=\"audio-hi\",",
                "LANGUAGE=\"fre\",",
                "NAME=\"Français\",",
                "AUTOSELECT=YES"
            )
        },
        {
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
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "URI=\"sp/prog_index.m3u8\",",
                "GROUP-ID=\"audio-hi\",",
                "LANGUAGE=\"es\",",
                "NAME=\"Espanol\",",
                "AUTOSELECT=YES"
            )
        },
        {
            ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("audio-aacl-312")
                .language("en")
                .name("English")
                .is_autoselect(true)
                .is_default(true)
                .channels(Channels::new(2))
                .build()
                .unwrap(),
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=AUDIO,",
                "GROUP-ID=\"audio-aacl-312\",",
                "LANGUAGE=\"en\",",
                "NAME=\"English\",",
                "DEFAULT=YES,",
                "AUTOSELECT=YES,",
                "CHANNELS=\"2\""
            )
        },
        {
            ExtXMedia::builder()
                .media_type(MediaType::Subtitles)
                .uri("french/ed.ttml")
                .group_id("subs")
                .language("fra")
                .assoc_language("fra")
                .name("French")
                .is_autoselect(true)
                .is_forced(true)
                .characteristics("public.accessibility.transcribes-spoken\
                -dialog,public.accessibility.describes-music-and-sound")
                .build()
                .unwrap(),
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=SUBTITLES,",
                "URI=\"french/ed.ttml\",",
                "GROUP-ID=\"subs\",",
                "LANGUAGE=\"fra\",",
                "ASSOC-LANGUAGE=\"fra\",",
                "NAME=\"French\",",
                "AUTOSELECT=YES,",
                "FORCED=YES,",
                "CHARACTERISTICS=\"",
                "public.accessibility.transcribes-spoken-dialog,",
                "public.accessibility.describes-music-and-sound",
                "\""
            )
        },
        {
            ExtXMedia::builder()
                .media_type(MediaType::ClosedCaptions)
                .group_id("cc")
                .language("sp")
                .name("CC2")
                .instream_id(InStreamId::Cc2)
                .is_autoselect(true)
                .build()
                .unwrap(),
            concat!(
                "#EXT-X-MEDIA:",
                "TYPE=CLOSED-CAPTIONS,",
                "GROUP-ID=\"cc\",",
                "LANGUAGE=\"sp\",",
                "NAME=\"CC2\",",
                "AUTOSELECT=YES,",
                "INSTREAM-ID=\"CC2\""
            )
        },
        {
            ExtXMedia::new(MediaType::Audio, "foo", "bar"),
            "#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"foo\",NAME=\"bar\""
        },
    }

    #[test]
    fn test_parser_error() {
        assert!(ExtXMedia::try_from("").is_err());
        assert!(ExtXMedia::try_from("garbage").is_err());

        assert!(
            ExtXMedia::try_from("#EXT-X-MEDIA:TYPE=CLOSED-CAPTIONS,URI=\"http://www.example.com\"")
                .is_err()
        );
        assert!(ExtXMedia::try_from("#EXT-X-MEDIA:TYPE=AUDIO,INSTREAM-ID=CC1").is_err());

        assert!(ExtXMedia::try_from("#EXT-X-MEDIA:TYPE=AUDIO,DEFAULT=YES,AUTOSELECT=NO").is_err());

        assert!(ExtXMedia::try_from("#EXT-X-MEDIA:TYPE=AUDIO,FORCED=YES").is_err());
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
