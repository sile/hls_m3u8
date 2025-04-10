use core::convert::TryFrom;
use core::fmt;
use core::ops::Deref;
use std::borrow::Cow;

use crate::attribute::AttributePairs;
use crate::tags::ExtXMedia;
use crate::traits::RequiredVersion;
use crate::types::{ClosedCaptions, MediaType, ProtocolVersion, StreamData, UFloat};
use crate::utils::{quote, tag, unquote};
use crate::Error;

/// A server may offer multiple [`MediaPlaylist`] files to provide different
/// encodings of the same presentation.
///
/// If it does so, it should provide
/// a [`MasterPlaylist`] that lists each [`VariantStream`] to allow
/// clients to switch between encodings dynamically.
///
/// The server must meet the following constraints when producing
/// [`VariantStream`]s in order to allow clients to switch between them
/// seamlessly:
///
/// - Each [`VariantStream`] must present the same content.
///
/// - Matching content in [`VariantStream`]s must have matching timestamps. This
///   allows clients to synchronize the media.
///
/// - Matching content in [`VariantStream`]s must have matching
///   [`ExtXDiscontinuitySequence`].
///
/// - Each [`MediaPlaylist`] in each [`VariantStream`] must have the same target
///   duration. The only exceptions are subtitle renditions and
///   [`MediaPlaylist`]s containing an [`ExtXIFramesOnly`] tag, which may have
///   different target durations if they have [`PlaylistType::Vod`].
///
/// - Content that appears in a [`MediaPlaylist`] of one [`VariantStream`] but
///   not in another must appear either at the beginning or at the end of the
///   [`MediaPlaylist`] and must not be longer than the target duration.
///
/// - If any [`MediaPlaylist`]s have an [`PlaylistType`] tag, all
///   [`MediaPlaylist`]s must have an [`PlaylistType`] tag with the same value.
///
/// - If the Playlist contains an [`PlaylistType`] tag with the value of VOD,
///   the first segment of every [`MediaPlaylist`] in every [`VariantStream`]
///   must start at the same media timestamp.
///
/// - If any [`MediaPlaylist`] in a [`MasterPlaylist`] contains an
///   [`ExtXProgramDateTime`] tag, then all [`MediaPlaylist`]s in that
///   [`MasterPlaylist`] must contain [`ExtXProgramDateTime`] tags with
///   consistent mappings of date and time to media timestamps.
///
/// - Each [`VariantStream`] must contain the same set of Date Ranges, each one
///   identified by an [`ExtXDateRange`] tag(s) with the same ID attribute value
///   and containing the same set of attribute/value pairs.
///
/// In addition, for broadest compatibility, [`VariantStream`]s should
/// contain the same encoded audio bitstream. This allows clients to
/// switch between [`VariantStream`]s without audible glitching.
///
/// [RFC6381]: https://tools.ietf.org/html/rfc6381
/// [`ExtXDiscontinuitySequence`]: crate::tags::ExtXDiscontinuitySequence
/// [`PlaylistType::Vod`]: crate::types::PlaylistType::Vod
/// [`MediaPlaylist`]: crate::MediaPlaylist
/// [`MasterPlaylist`]: crate::MasterPlaylist
/// [`ExtXDateRange`]: crate::tags::ExtXDateRange
/// [`ExtXProgramDateTime`]: crate::tags::ExtXProgramDateTime
/// [`PlaylistType`]: crate::types::PlaylistType
/// [`ExtXIFramesOnly`]: crate::tags::ExtXIFramesOnly
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum VariantStream<'a> {
    /// The [`VariantStream::ExtXIFrame`] variant identifies a [`MediaPlaylist`]
    /// file containing the I-frames of a multimedia presentation.
    /// It stands alone, in that it does not apply to a particular URI in the
    /// [`MasterPlaylist`].
    ///
    /// [`MasterPlaylist`]: crate::MasterPlaylist
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    ExtXIFrame {
        /// The URI identifies the I-frame [`MediaPlaylist`] file.
        /// That Playlist file must contain an [`ExtXIFramesOnly`] tag.
        ///
        /// # Note
        ///
        /// This field is required.
        ///
        /// [`MediaPlaylist`]: crate::MediaPlaylist
        /// [`ExtXIFramesOnly`]: crate::tags::ExtXIFramesOnly
        uri: Cow<'a, str>,
        /// Some fields are shared between [`VariantStream::ExtXStreamInf`] and
        /// [`VariantStream::ExtXIFrame`].
        ///
        /// # Note
        ///
        /// This field is optional.
        stream_data: StreamData<'a>,
    },
    /// [`VariantStream::ExtXStreamInf`] specifies a [`VariantStream`], which is
    /// a set of renditions that can be combined to play the presentation.
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
        uri: Cow<'a, str>,
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
        audio: Option<Cow<'a, str>>,
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
        subtitles: Option<Cow<'a, str>>,
        /// It indicates the set of closed-caption renditions that can be used
        /// when playing the presentation.
        ///
        /// # Note
        ///
        /// This field is optional.
        closed_captions: Option<ClosedCaptions<'a>>,
        /// Some fields are shared between [`VariantStream::ExtXStreamInf`] and
        /// [`VariantStream::ExtXIFrame`].
        ///
        /// # Note
        ///
        /// This field is optional.
        stream_data: StreamData<'a>,
    },
}

impl VariantStream<'_> {
    pub(crate) const PREFIX_EXTXIFRAME: &'static str = "#EXT-X-I-FRAME-STREAM-INF:";
    pub(crate) const PREFIX_EXTXSTREAMINF: &'static str = "#EXT-X-STREAM-INF:";

    /// Checks if a [`VariantStream`] and an [`ExtXMedia`] element are
    /// associated.
    ///
    /// # Example
    ///
    /// ```
    /// use hls_m3u8::tags::{ExtXMedia, VariantStream};
    /// use hls_m3u8::types::{ClosedCaptions, MediaType, StreamData};
    ///
    /// let variant_stream = VariantStream::ExtXStreamInf {
    ///     uri: "https://www.example.com/init.bin".into(),
    ///     frame_rate: None,
    ///     audio: Some("ag1".into()),
    ///     subtitles: Some("sg1".into()),
    ///     closed_captions: Some(ClosedCaptions::group_id("cc1")),
    ///     stream_data: StreamData::builder()
    ///         .bandwidth(1_110_000)
    ///         .video("vg1")
    ///         .build()
    ///         .unwrap(),
    /// };
    ///
    /// assert!(variant_stream.is_associated(
    ///     &ExtXMedia::builder()
    ///         .media_type(MediaType::Audio)
    ///         .group_id("ag1")
    ///         .name("audio example")
    ///         .build()
    ///         .unwrap(),
    /// ));
    /// ```
    #[must_use]
    pub fn is_associated(&self, media: &ExtXMedia<'_>) -> bool {
        match &self {
            Self::ExtXIFrame { stream_data, .. } => {
                if let MediaType::Video = media.media_type {
                    if let Some(value) = stream_data.video() {
                        return value == media.group_id();
                    }
                }

                false
            }
            Self::ExtXStreamInf {
                audio,
                subtitles,
                closed_captions,
                stream_data,
                ..
            } => match media.media_type {
                MediaType::Audio => audio.as_ref().is_some_and(|v| v == media.group_id()),
                MediaType::Video => stream_data.video().is_some_and(|v| v == media.group_id()),
                MediaType::Subtitles => subtitles.as_ref().is_some_and(|v| v == media.group_id()),
                MediaType::ClosedCaptions => closed_captions
                    .as_ref()
                    .is_some_and(|v| v == media.group_id()),
            },
        }
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> VariantStream<'static> {
        match self {
            VariantStream::ExtXIFrame { uri, stream_data } => VariantStream::ExtXIFrame {
                uri: Cow::Owned(uri.into_owned()),
                stream_data: stream_data.into_owned(),
            },
            VariantStream::ExtXStreamInf {
                uri,
                frame_rate,
                audio,
                subtitles,
                closed_captions,
                stream_data,
            } => VariantStream::ExtXStreamInf {
                uri: Cow::Owned(uri.into_owned()),
                frame_rate,
                audio: audio.map(|v| Cow::Owned(v.into_owned())),
                subtitles: subtitles.map(|v| Cow::Owned(v.into_owned())),
                closed_captions: closed_captions.map(ClosedCaptions::into_owned),
                stream_data: stream_data.into_owned(),
            },
        }
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for VariantStream<'_> {
    fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }

    fn introduced_version(&self) -> ProtocolVersion {
        match &self {
            Self::ExtXStreamInf {
                audio,
                subtitles,
                stream_data,
                ..
            } => {
                if stream_data.introduced_version() >= ProtocolVersion::V4 {
                    stream_data.introduced_version()
                } else if audio.is_some() || subtitles.is_some() {
                    ProtocolVersion::V4
                } else {
                    ProtocolVersion::V1
                }
            }
            Self::ExtXIFrame { stream_data, .. } => stream_data.introduced_version(),
        }
    }
}

impl fmt::Display for VariantStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl<'a> TryFrom<&'a str> for VariantStream<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        if let Ok(input) = tag(input, Self::PREFIX_EXTXIFRAME) {
            let uri = AttributePairs::new(input)
                .find_map(|(key, value)| (key == "URI").then(|| unquote(value)))
                .ok_or_else(|| Error::missing_value("URI"))?;

            Ok(Self::ExtXIFrame {
                uri,
                stream_data: StreamData::try_from(input)?,
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
                    "CLOSED-CAPTIONS" => {
                        closed_captions = Some(ClosedCaptions::try_from(value).unwrap());
                    }
                    _ => {}
                }
            }

            Ok(Self::ExtXStreamInf {
                uri: Cow::Borrowed(uri),
                frame_rate,
                audio,
                subtitles,
                closed_captions,
                stream_data: StreamData::try_from(first_line)?,
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

impl<'a> Deref for VariantStream<'a> {
    type Target = StreamData<'a>;

    fn deref(&self) -> &Self::Target {
        match &self {
            Self::ExtXIFrame { stream_data, .. } | Self::ExtXStreamInf { stream_data, .. } => {
                stream_data
            }
        }
    }
}

impl<'a> PartialEq<&VariantStream<'a>> for VariantStream<'a> {
    fn eq(&self, other: &&Self) -> bool {
        self.eq(*other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::InStreamId;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_required_version() {
        assert_eq!(
            VariantStream::ExtXStreamInf {
                uri: "https://www.example.com/init.bin".into(),
                frame_rate: None,
                audio: None,
                subtitles: None,
                closed_captions: None,
                stream_data: StreamData::new(1_110_000)
            }
            .required_version(),
            ProtocolVersion::V1
        );
    }

    #[test]
    fn test_is_associated() {
        let mut variant_stream = VariantStream::ExtXStreamInf {
            uri: "https://www.example.com/init.bin".into(),
            frame_rate: None,
            audio: Some("ag1".into()),
            subtitles: Some("sg1".into()),
            closed_captions: Some(ClosedCaptions::group_id("cc1")),
            stream_data: StreamData::builder()
                .bandwidth(1_110_000)
                .video("vg1")
                .build()
                .unwrap(),
        };

        assert!(variant_stream.is_associated(
            &ExtXMedia::builder()
                .media_type(MediaType::Audio)
                .group_id("ag1")
                .name("audio example")
                .build()
                .unwrap(),
        ));

        assert!(variant_stream.is_associated(
            &ExtXMedia::builder()
                .media_type(MediaType::Subtitles)
                .uri("https://www.example.com/sg1.ssa")
                .group_id("sg1")
                .name("subtitle example")
                .build()
                .unwrap(),
        ));

        assert!(variant_stream.is_associated(
            &ExtXMedia::builder()
                .media_type(MediaType::ClosedCaptions)
                .group_id("cc1")
                .name("closed captions example")
                .instream_id(InStreamId::Cc1)
                .build()
                .unwrap(),
        ));

        if let VariantStream::ExtXStreamInf {
            closed_captions, ..
        } = &mut variant_stream
        {
            *closed_captions = Some(ClosedCaptions::None);
        }

        assert!(variant_stream.is_associated(
            &ExtXMedia::builder()
                .media_type(MediaType::ClosedCaptions)
                .group_id("NONE")
                .name("closed captions example")
                .instream_id(InStreamId::Cc1)
                .build()
                .unwrap(),
        ));

        assert!(variant_stream.is_associated(
            &ExtXMedia::builder()
                .media_type(MediaType::Video)
                .group_id("vg1")
                .name("video example")
                .build()
                .unwrap(),
        ));
    }
}
