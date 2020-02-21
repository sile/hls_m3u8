use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;
use shorthand::ShortHand;

use crate::attribute::AttributePairs;
use crate::types::{Channels, InStreamId, MediaType, ProtocolVersion};
use crate::utils::{parse_yes_or_no, quote, tag, unquote};
use crate::{Error, RequiredVersion};

/// # [4.4.5.1. EXT-X-MEDIA]
///
/// The [`ExtXMedia`] tag is used to relate [`MediaPlaylist`]s,
/// that contain alternative renditions of the same content.
///
/// For example, three [`ExtXMedia`] tags can be used to identify audio-only
/// [`MediaPlaylist`]s, that contain English, French, and Spanish renditions
/// of the same presentation. Or, two [`ExtXMedia`] tags can be used to
/// identify video-only [`MediaPlaylist`]s that show two different camera
/// angles.
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
/// [4.4.5.1. EXT-X-MEDIA]:
/// https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-05#section-4.4.5.1
#[derive(ShortHand, Builder, Debug, Clone, PartialEq, Eq, Hash)]
#[shorthand(enable(must_use, into))]
#[builder(setter(into))]
#[builder(build_fn(validate = "Self::validate"))]
pub struct ExtXMedia {
    /// The [`MediaType`] that is associated with this tag.
    ///
    /// # Note
    ///
    /// This attribute is **required**.
    #[shorthand(enable(copy))]
    media_type: MediaType,
    /// An `URI` to a [`MediaPlaylist`].
    ///
    /// # Note
    ///
    /// - This attribute is **required**, if the [`MediaType`] is
    ///   [`MediaType::Subtitles`].
    /// - This attribute is **not allowed**, if the [`MediaType`] is
    /// [`MediaType::ClosedCaptions`].
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    #[builder(setter(strip_option), default)]
    uri: Option<String>,
    /// The identifier that specifies the group to which the rendition
    /// belongs.
    ///
    /// # Note
    ///
    /// This attribute is **required**.
    group_id: String,
    /// The name of the primary language used in the rendition.
    /// The value has to conform to [`RFC5646`].
    ///
    /// # Note
    ///
    /// This attribute is **optional**.
    ///
    /// [`RFC5646`]: https://tools.ietf.org/html/rfc5646
    #[builder(setter(strip_option), default)]
    language: Option<String>,
    /// The name of a language associated with the rendition.
    /// An associated language is often used in a different role, than the
    /// language specified by the [`language`] attribute (e.g., written versus
    /// spoken, or a fallback dialect).
    ///
    /// # Note
    ///
    /// This attribute is **optional**.
    ///
    /// [`language`]: #method.language
    #[builder(setter(strip_option), default)]
    assoc_language: Option<String>,
    /// A human-readable description of the rendition.
    ///
    /// # Note
    ///
    /// This attribute is **required**.
    ///
    /// If the [`language`] attribute is present, this attribute should be in
    /// that language.
    ///
    /// [`language`]: #method.language
    name: String,
    /// The value of the `default` flag.
    /// A value of `true` indicates, that the client should play
    /// this rendition of the content in the absence of information
    /// from the user indicating a different choice.
    ///
    /// # Note
    ///
    /// This attribute is **optional**, its absence indicates an implicit value
    /// of `false`.
    #[builder(default)]
    is_default: bool,
    /// Whether the client may choose to play this rendition in the absence of
    /// explicit user preference.
    ///
    /// # Note
    ///
    /// This attribute is **optional**, its absence indicates an implicit value
    /// of `false`.
    #[builder(default)]
    is_autoselect: bool,
    /// Whether the rendition contains content that is considered
    /// essential to play.
    #[builder(default)]
    is_forced: bool,
    /// An [`InStreamId`] identifies a rendition within the
    /// [`MediaSegment`]s in a [`MediaPlaylist`].
    ///
    /// # Note
    ///
    /// This attribute is required, if the [`ExtXMedia::media_type`] is
    /// [`MediaType::ClosedCaptions`]. For all other [`ExtXMedia::media_type`]
    /// the [`InStreamId`] must not be specified!
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    /// [`MediaSegment`]: crate::MediaSegment
    #[builder(setter(strip_option), default)]
    #[shorthand(enable(copy))]
    instream_id: Option<InStreamId>,
    /// The characteristics attribute, containing one or more Uniform Type
    /// Identifiers (UTI) separated by comma.
    /// Each [`UTI`] indicates an individual characteristic of the Rendition.
    ///
    /// A [`subtitles`] rendition may include the following characteristics:
    /// "public.accessibility.transcribes-spoken-dialog",
    /// "public.accessibility.describes-music-and-sound", and
    /// "public.easy-to-read" (which indicates that the subtitles have
    /// been edited for ease of reading).
    ///
    /// An AUDIO Rendition MAY include the following characteristic:
    /// "public.accessibility.describes-video".
    ///
    /// The characteristics attribute may include private UTIs.
    ///
    /// [`UTI`]: https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-05#ref-UTI
    /// [`subtitles`]: crate::types::MediaType::Subtitles
    #[builder(setter(strip_option), default)]
    characteristics: Option<String>,
    /// The [`Channels`].
    #[builder(setter(strip_option), default)]
    channels: Option<Channels>,
}

impl ExtXMediaBuilder {
    fn validate(&self) -> Result<(), String> {
        // A MediaType is always required!
        let media_type = self
            .media_type
            .ok_or_else(|| Error::missing_attribute("MEDIA-TYPE").to_string())?;

        if media_type == MediaType::Subtitles && self.uri.is_none() {
            return Err(Error::missing_attribute("URI").to_string());
        }

        if media_type == MediaType::ClosedCaptions {
            if self.uri.is_some() {
                return Err(Error::unexpected_attribute("URI").to_string());
            }
            if self.instream_id.is_none() {
                return Err(Error::missing_attribute("INSTREAM-ID").to_string());
            }
        } else if self.instream_id.is_some() {
            return Err(Error::unexpected_attribute("INSTREAM-ID").to_string());
        }

        if self.is_default.unwrap_or(false) && !self.is_autoselect.unwrap_or(false) {
            return Err(Error::custom(format!(
                "If `DEFAULT` is true, `AUTOSELECT` has to be true too, Default: {:?}, Autoselect: {:?}!",
                self.is_default, self.is_autoselect
            ))
            .to_string());
        }

        if media_type != MediaType::Subtitles && self.is_forced.is_some() {
            return Err(Error::invalid_input().to_string());
        }

        Ok(())
    }
}

impl ExtXMedia {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MEDIA:";

    /// Makes a new [`ExtXMedia`] tag.
    pub fn new<T, K>(media_type: MediaType, group_id: T, name: K) -> Self
    where
        T: Into<String>,
        K: Into<String>,
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
    pub fn builder() -> ExtXMediaBuilder { ExtXMediaBuilder::default() }
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

        builder.build().map_err(Error::builder)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display_and_parse() {
        // TODO: https://developer.apple.com/documentation/http_live_streaming/example_playlists_for_http_live_streaming/adding_alternate_media_to_a_playlist

        macro_rules! generate_tests {
            ( $( { $media:expr, $string:tt } ),* $(,)* ) => {
                $(
                    assert_eq!(
                        $media.to_string(),
                        $string.to_string()
                    );

                    assert_eq!(
                        $media,
                        $string.parse::<ExtXMedia>().unwrap(),
                    );
                )*
            }
        }

        generate_tests! {
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
                "#EXT-X-MEDIA:\
                 TYPE=AUDIO,\
                 URI=\"eng/prog_index.m3u8\",\
                 GROUP-ID=\"audio\",\
                 LANGUAGE=\"eng\",\
                 NAME=\"English\",\
                 DEFAULT=YES,\
                 AUTOSELECT=YES"
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
                "#EXT-X-MEDIA:\
                 TYPE=AUDIO,\
                 URI=\"fre/prog_index.m3u8\",\
                 GROUP-ID=\"audio\",\
                 LANGUAGE=\"fre\",\
                 NAME=\"Français\",\
                 AUTOSELECT=YES"
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
                "#EXT-X-MEDIA:\
                 TYPE=AUDIO,\
                 URI=\"sp/prog_index.m3u8\",\
                 GROUP-ID=\"audio\",\
                 LANGUAGE=\"sp\",\
                 NAME=\"Espanol\",\
                 AUTOSELECT=YES"
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
                "#EXT-X-MEDIA:\
                 TYPE=AUDIO,\
                 URI=\"englo/prog_index.m3u8\",\
                 GROUP-ID=\"audio-lo\",\
                 LANGUAGE=\"eng\",\
                 NAME=\"English\",\
                 DEFAULT=YES,\
                 AUTOSELECT=YES"
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
                "#EXT-X-MEDIA:\
                 TYPE=AUDIO,\
                 URI=\"frelo/prog_index.m3u8\",\
                 GROUP-ID=\"audio-lo\",\
                 LANGUAGE=\"fre\",\
                 NAME=\"Français\",\
                 AUTOSELECT=YES"
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
                "#EXT-X-MEDIA:\
                 TYPE=AUDIO,\
                 URI=\"splo/prog_index.m3u8\",\
                 GROUP-ID=\"audio-lo\",\
                 LANGUAGE=\"es\",\
                 NAME=\"Espanol\",\
                 AUTOSELECT=YES"
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
                "#EXT-X-MEDIA:\
                 TYPE=AUDIO,\
                 URI=\"eng/prog_index.m3u8\",\
                 GROUP-ID=\"audio-hi\",\
                 LANGUAGE=\"eng\",\
                 NAME=\"English\",\
                 DEFAULT=YES,\
                 AUTOSELECT=YES"
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
                "#EXT-X-MEDIA:\
                 TYPE=AUDIO,\
                 URI=\"fre/prog_index.m3u8\",\
                 GROUP-ID=\"audio-hi\",\
                 LANGUAGE=\"fre\",\
                 NAME=\"Français\",\
                 AUTOSELECT=YES"
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
                "#EXT-X-MEDIA:\
                 TYPE=AUDIO,\
                 URI=\"sp/prog_index.m3u8\",\
                 GROUP-ID=\"audio-hi\",\
                 LANGUAGE=\"es\",\
                 NAME=\"Espanol\",\
                 AUTOSELECT=YES"
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
                "#EXT-X-MEDIA:\
                 TYPE=AUDIO,\
                 GROUP-ID=\"audio-aacl-312\",\
                 LANGUAGE=\"en\",\
                 NAME=\"English\",\
                 DEFAULT=YES,\
                 AUTOSELECT=YES,\
                 CHANNELS=\"2\""
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
                "#EXT-X-MEDIA:\
                TYPE=SUBTITLES,\
                URI=\"french/ed.ttml\",\
                GROUP-ID=\"subs\",\
                LANGUAGE=\"fra\",\
                ASSOC-LANGUAGE=\"fra\",\
                NAME=\"French\",\
                AUTOSELECT=YES,\
                FORCED=YES,\
                CHARACTERISTICS=\"public.accessibility.\
                transcribes-spoken-dialog,public.accessibility.describes-music-and-sound\""
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
                "#EXT-X-MEDIA:\
                 TYPE=CLOSED-CAPTIONS,\
                 GROUP-ID=\"cc\",\
                 LANGUAGE=\"sp\",\
                 NAME=\"CC2\",\
                 AUTOSELECT=YES,\
                 INSTREAM-ID=\"CC2\""
            },
            {
                ExtXMedia::new(MediaType::Audio, "foo", "bar"),
                "#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"foo\",NAME=\"bar\""
            },
        };
    }

    #[test]
    fn test_parser_error() {
        assert!("".parse::<ExtXMedia>().is_err());
        assert!("garbage".parse::<ExtXMedia>().is_err());

        assert!(
            "#EXT-X-MEDIA:TYPE=CLOSED-CAPTIONS,URI=\"http://www.example.com\""
                .parse::<ExtXMedia>()
                .is_err()
        );
        assert!("#EXT-X-MEDIA:TYPE=AUDIO,INSTREAM-ID=CC1"
            .parse::<ExtXMedia>()
            .is_err());

        assert!("#EXT-X-MEDIA:TYPE=AUDIO,DEFAULT=YES,AUTOSELECT=NO"
            .parse::<ExtXMedia>()
            .is_err());

        assert!("#EXT-X-MEDIA:TYPE=AUDIO,FORCED=YES"
            .parse::<ExtXMedia>()
            .is_err());
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
