use core::fmt;
use core::ops::Deref;
use core::str::FromStr;

use crate::attribute::AttributePairs;
use crate::traits::RequiredVersion;
use crate::types::{ClosedCaptions, ProtocolVersion, StreamData, UFloat};
use crate::utils::{quote, tag, unquote};
use crate::Error;

/// A server MAY offer multiple Media Playlist files to provide different
/// encodings of the same presentation.  If it does so, it SHOULD provide
/// a Master Playlist file that lists each Variant Stream to allow
/// clients to switch between encodings dynamically.
///
/// Master Playlists describe regular Variant Streams with EXT-X-STREAM-
/// INF tags and I-frame Variant Streams with EXT-X-I-FRAME-STREAM-INF
/// tags.
///
/// If an EXT-X-STREAM-INF tag or EXT-X-I-FRAME-STREAM-INF tag contains
/// the CODECS attribute, the attribute value MUST include every media
/// format [RFC6381] present in any Media Segment in any of the
/// Renditions specified by the Variant Stream.
///
/// The server MUST meet the following constraints when producing Variant
/// Streams in order to allow clients to switch between them seamlessly:
///
/// o  Each Variant Stream MUST present the same content.
///
///
/// o  Matching content in Variant Streams MUST have matching timestamps.
///    This allows clients to synchronize the media.
///
/// o  Matching content in Variant Streams MUST have matching
///    Discontinuity Sequence Numbers (see Section 4.3.3.3).
///
/// o  Each Media Playlist in each Variant Stream MUST have the same
///    target duration.  The only exceptions are SUBTITLES Renditions and
///    Media Playlists containing an EXT-X-I-FRAMES-ONLY tag, which MAY
///    have different target durations if they have an EXT-X-PLAYLIST-
///    TYPE of VOD.
///
/// o  Content that appears in a Media Playlist of one Variant Stream but
///    not in another MUST appear either at the beginning or at the end
///    of the Media Playlist file and MUST NOT be longer than the target
///    duration.
///
/// o  If any Media Playlists have an EXT-X-PLAYLIST-TYPE tag, all Media
///    Playlists MUST have an EXT-X-PLAYLIST-TYPE tag with the same
///    value.
///
/// o  If the Playlist contains an EXT-X-PLAYLIST-TYPE tag with the value
///    of VOD, the first segment of every Media Playlist in every Variant
///    Stream MUST start at the same media timestamp.
///
/// o  If any Media Playlist in a Master Playlist contains an EXT-X-
///    PROGRAM-DATE-TIME tag, then all Media Playlists in that Master
///    Playlist MUST contain EXT-X-PROGRAM-DATE-TIME tags with consistent
///    mappings of date and time to media timestamps.
///
/// o  Each Variant Stream MUST contain the same set of Date Ranges, each
///    one identified by an EXT-X-DATERANGE tag(s) with the same ID
///    attribute value and containing the same set of attribute/value
///    pairs.
///
/// In addition, for broadest compatibility, Variant Streams SHOULD
/// contain the same encoded audio bitstream.  This allows clients to
/// switch between Variant Streams without audible glitching.
///
/// The rules for Variant Streams also apply to alternative Renditions
/// (see Section 4.3.4.2.1).
///
/// [RFC6381]: https://tools.ietf.org/html/rfc6381
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum VariantStream {
    ExtXIFrame {
        /// The URI identifies the I-frame [`MediaPlaylist`] file.
        /// That Playlist file must contain an [`ExtXIFramesOnly`] tag.
        ///
        /// # Note
        ///
        /// This field is required.
        ///
        /// [`MediaPlaylist`]: crate::MediaPlaylist
        uri: String,
        /// Some fields are shared between [`VariantStream::ExtXStreamInf`] and
        /// [`VariantStream::ExtXIFrame`].
        ///
        /// # Note
        ///
        /// This field is optional.
        stream_data: StreamData,
    },
    ExtXStreamInf {
        /// The URI specifies a [`MediaPlaylist`] that carries a rendition of
        /// the [`VariantStream`]. Clients that do not support multiple video
        /// renditions should play this rendition.
        ///
        /// # Note
        ///
        /// This field is required.
        ///
        /// [`MediaPlaylist`]: crate::MediaPlaylist
        uri: String,
        /// The value is an unsigned float describing the maximum frame
        /// rate for all the video in the [`VariantStream`].
        ///
        /// # Note
        ///
        /// Specifying the frame rate is optional, but is recommended if the
        /// [`VariantStream`] includes video. It should be specified if any
        /// video exceeds 30 frames per second.
        frame_rate: Option<UFloat>,
        /// It indicates the set of audio renditions that should be used when
        /// playing the presentation.
        ///
        /// It must match the value of the [`ExtXMedia::group_id`] of an
        /// [`ExtXMedia`] tag elsewhere in the [`MasterPlaylist`] whose
        /// [`ExtXMedia::media_type`] is [`MediaType::Audio`].
        ///
        /// # Note
        ///
        /// This field is optional.
        ///
        /// [`ExtXMedia`]: crate::tags::ExtXMedia
        /// [`ExtXMedia::group_id`]: crate::tags::ExtXMedia::group_id
        /// [`MasterPlaylist`]: crate::MasterPlaylist
        /// [`ExtXMedia::media_type`]: crate::tags::ExtXMedia::media_type
        /// [`MediaType::Audio`]: crate::types::MediaType::Audio
        audio: Option<String>,
        /// It indicates the set of subtitle renditions that can be used when
        /// playing the presentation.
        ///
        /// It must match the value of the [`ExtXMedia::group_id`] of an
        /// [`ExtXMedia`] tag elsewhere in the [`MasterPlaylist`] whose
        /// [`ExtXMedia::media_type`] is [`MediaType::Subtitles`].
        ///
        /// # Note
        ///
        /// This field is optional.
        ///
        /// [`ExtXMedia`]: crate::tags::ExtXMedia
        /// [`ExtXMedia::group_id`]: crate::tags::ExtXMedia::group_id
        /// [`MasterPlaylist`]: crate::MasterPlaylist
        /// [`ExtXMedia::media_type`]: crate::tags::ExtXMedia::media_type
        /// [`MediaType::Subtitles`]: crate::types::MediaType::Subtitles
        subtitles: Option<String>,
        /// It indicates the set of closed-caption renditions that can be used
        /// when playing the presentation.
        ///
        /// # Note
        ///
        /// This field is optional.
        closed_captions: Option<ClosedCaptions>,
        /// Some fields are shared between [`VariantStream::ExtXStreamInf`] and
        /// [`VariantStream::ExtXIFrame`].
        ///
        /// # Note
        ///
        /// This field is optional.
        stream_data: StreamData,
    },
}

impl VariantStream {
    pub(crate) const PREFIX_EXTXIFRAME: &'static str = "#EXT-X-I-FRAME-STREAM-INF:";
    pub(crate) const PREFIX_EXTXSTREAMINF: &'static str = "#EXT-X-STREAM-INF:";
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for VariantStream {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for VariantStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::ExtXIFrame { uri, stream_data } => {
                write!(f, "{}", Self::PREFIX_EXTXIFRAME)?;
                write!(f, "URI={},{}", quote(uri), stream_data)?;
            }
            Self::ExtXStreamInf {
                uri,
                frame_rate,
                audio,
                subtitles,
                closed_captions,
                stream_data,
            } => {
                write!(f, "{}{}", Self::PREFIX_EXTXSTREAMINF, stream_data)?;

                if let Some(value) = frame_rate {
                    write!(f, ",FRAME-RATE={:.3}", value.as_f32())?;
                }

                if let Some(value) = audio {
                    write!(f, ",AUDIO={}", quote(value))?;
                }

                if let Some(value) = subtitles {
                    write!(f, ",SUBTITLES={}", quote(value))?;
                }

                if let Some(value) = closed_captions {
                    write!(f, ",CLOSED-CAPTIONS={}", value)?;
                }

                write!(f, "\n{}", uri)?;
            }
        }

        Ok(())
    }
}

impl FromStr for VariantStream {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Ok(input) = tag(input, Self::PREFIX_EXTXIFRAME) {
            let uri = AttributePairs::new(input)
                .find_map(|(key, value)| {
                    if key == "URI" {
                        Some(unquote(value))
                    } else {
                        None
                    }
                })
                .ok_or_else(|| Error::missing_value("URI"))?;

            Ok(Self::ExtXIFrame {
                uri,
                stream_data: input.parse()?,
            })
        } else if let Ok(input) = tag(input, Self::PREFIX_EXTXSTREAMINF) {
            let mut lines = input.lines();
            let first_line = lines
                .next()
                .ok_or_else(|| Error::missing_value("first_line"))?;
            let uri = lines.next().ok_or_else(|| Error::missing_value("URI"))?;

            let mut frame_rate = None;
            let mut audio = None;
            let mut subtitles = None;
            let mut closed_captions = None;

            for (key, value) in AttributePairs::new(first_line) {
                match key {
                    "FRAME-RATE" => frame_rate = Some(value.parse()?),
                    "AUDIO" => audio = Some(unquote(value)),
                    "SUBTITLES" => subtitles = Some(unquote(value)),
                    "CLOSED-CAPTIONS" => closed_captions = Some(value.parse().unwrap()),
                    _ => {}
                }
            }

            Ok(Self::ExtXStreamInf {
                uri: uri.to_string(),
                frame_rate,
                audio,
                subtitles,
                closed_captions,
                stream_data: first_line.parse()?,
            })
        } else {
            // TODO: custom error type? + attach input data
            Err(Error::custom(format!(
                "invalid start of input, expected either {:?} or {:?}",
                Self::PREFIX_EXTXIFRAME,
                Self::PREFIX_EXTXSTREAMINF
            )))
        }
    }
}

impl Deref for VariantStream {
    type Target = StreamData;

    fn deref(&self) -> &Self::Target {
        match &self {
            Self::ExtXIFrame { stream_data, .. } | Self::ExtXStreamInf { stream_data, .. } => {
                stream_data
            }
        }
    }
}
