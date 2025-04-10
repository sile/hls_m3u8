use std::borrow::Cow;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt;

use derive_builder::Builder;

use crate::line::{Line, Lines, Tag};
use crate::tags::{
    ExtM3u, ExtXIndependentSegments, ExtXMedia, ExtXSessionData, ExtXSessionKey, ExtXStart,
    ExtXVersion, VariantStream,
};
use crate::types::{ClosedCaptions, MediaType, ProtocolVersion};
use crate::utils::{tag, BoolExt};
use crate::{Error, RequiredVersion};

/// The master playlist describes all of the available variants for your
/// content.
///
/// Each variant is a version of the stream at a particular bitrate and is
/// contained in a separate playlist called [`MediaPlaylist`].
///
/// # Examples
///
/// A [`MasterPlaylist`] can be parsed from a `str`:
///
/// ```
/// use core::convert::TryFrom;
/// use hls_m3u8::MasterPlaylist;
///
/// // the concat! macro joins multiple `&'static str`.
/// let master_playlist = MasterPlaylist::try_from(concat!(
///     "#EXTM3U\n",
///     "#EXT-X-STREAM-INF:",
///     "BANDWIDTH=150000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n",
///     "http://example.com/low/index.m3u8\n",
///     "#EXT-X-STREAM-INF:",
///     "BANDWIDTH=240000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n",
///     "http://example.com/lo_mid/index.m3u8\n",
///     "#EXT-X-STREAM-INF:",
///     "BANDWIDTH=440000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n",
///     "http://example.com/hi_mid/index.m3u8\n",
///     "#EXT-X-STREAM-INF:",
///     "BANDWIDTH=640000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=640x360\n",
///     "http://example.com/high/index.m3u8\n",
///     "#EXT-X-STREAM-INF:BANDWIDTH=64000,CODECS=\"mp4a.40.5\"\n",
///     "http://example.com/audio/index.m3u8\n"
/// ))?;
///
/// println!("{}", master_playlist.has_independent_segments);
/// # Ok::<(), hls_m3u8::Error>(())
/// ```
///
/// or it can be constructed through a builder
///
/// ```
/// # use hls_m3u8::MasterPlaylist;
/// use hls_m3u8::tags::{ExtXStart, VariantStream};
/// use hls_m3u8::types::{Float, StreamData};
///
/// MasterPlaylist::builder()
///     .variant_streams(vec![
///         VariantStream::ExtXStreamInf {
///             uri: "http://example.com/low/index.m3u8".into(),
///             frame_rate: None,
///             audio: None,
///             subtitles: None,
///             closed_captions: None,
///             stream_data: StreamData::builder()
///                 .bandwidth(150000)
///                 .codecs(["avc1.42e00a", "mp4a.40.2"])
///                 .resolution((416, 234))
///                 .build()
///                 .unwrap(),
///         },
///         VariantStream::ExtXStreamInf {
///             uri: "http://example.com/lo_mid/index.m3u8".into(),
///             frame_rate: None,
///             audio: None,
///             subtitles: None,
///             closed_captions: None,
///             stream_data: StreamData::builder()
///                 .bandwidth(240000)
///                 .codecs(["avc1.42e00a", "mp4a.40.2"])
///                 .resolution((416, 234))
///                 .build()
///                 .unwrap(),
///         },
///     ])
///     .has_independent_segments(true)
///     .start(ExtXStart::new(Float::new(1.23)))
///     .build()?;
/// # Ok::<(), Box<dyn ::std::error::Error>>(())
/// ```
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
#[derive(Builder, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[builder(build_fn(validate = "Self::validate"))]
#[builder(setter(into, strip_option))]
#[non_exhaustive]
pub struct MasterPlaylist<'a> {
    /// Indicates that all media samples in a [`MediaSegment`] can be
    /// decoded without information from other segments.
    ///
    /// ### Note
    ///
    /// This field is optional and by default `false`. If the field is `true`,
    /// it applies to every [`MediaSegment`] in every [`MediaPlaylist`] of this
    /// [`MasterPlaylist`].
    ///
    /// [`MediaSegment`]: crate::MediaSegment
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    #[builder(default)]
    pub has_independent_segments: bool,
    /// A preferred point at which to start playing a playlist.
    ///
    /// ### Note
    ///
    /// This field is optional and by default the playlist should be played from
    /// the start.
    #[builder(default)]
    pub start: Option<ExtXStart>,
    /// A list of all [`ExtXMedia`] tags, which describe an alternative
    /// rendition.
    ///
    /// For example, three [`ExtXMedia`] tags can be used to identify audio-only
    /// [`MediaPlaylist`]s, that contain English, French, and Spanish
    /// renditions of the same presentation. Or, two [`ExtXMedia`] tags can
    /// be used to identify video-only [`MediaPlaylist`]s that show two
    /// different camera angles.
    ///
    /// ### Note
    ///
    /// This field is optional.
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    #[builder(default)]
    pub media: Vec<ExtXMedia<'a>>,
    /// A list of all streams of this [`MasterPlaylist`].
    ///
    /// ### Note
    ///
    /// This field is optional.
    #[builder(default)]
    pub variant_streams: Vec<VariantStream<'a>>,
    /// The [`ExtXSessionData`] tag allows arbitrary session data to be
    /// carried in a [`MasterPlaylist`].
    ///
    /// ### Note
    ///
    /// This field is optional.
    #[builder(default)]
    pub session_data: Vec<ExtXSessionData<'a>>,
    /// A list of [`ExtXSessionKey`]s, that allows the client to preload
    /// these keys without having to read the [`MediaPlaylist`]s first.
    ///
    /// ### Note
    ///
    /// This field is optional.
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    #[builder(default)]
    pub session_keys: Vec<ExtXSessionKey<'a>>,
    /// A list of all tags that could not be identified while parsing the input.
    ///
    /// ### Note
    ///
    /// This field is optional.
    #[builder(default)]
    pub unknown_tags: Vec<Cow<'a, str>>,
}

impl<'a> MasterPlaylist<'a> {
    /// Returns a builder for a [`MasterPlaylist`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::MasterPlaylist;
    /// use hls_m3u8::tags::{ExtXStart, VariantStream};
    /// use hls_m3u8::types::{Float, StreamData};
    ///
    /// MasterPlaylist::builder()
    ///     .variant_streams(vec![
    ///         VariantStream::ExtXStreamInf {
    ///             uri: "http://example.com/low/index.m3u8".into(),
    ///             frame_rate: None,
    ///             audio: None,
    ///             subtitles: None,
    ///             closed_captions: None,
    ///             stream_data: StreamData::builder()
    ///                 .bandwidth(150000)
    ///                 .codecs(["avc1.42e00a", "mp4a.40.2"])
    ///                 .resolution((416, 234))
    ///                 .build()
    ///                 .unwrap(),
    ///         },
    ///         VariantStream::ExtXStreamInf {
    ///             uri: "http://example.com/lo_mid/index.m3u8".into(),
    ///             frame_rate: None,
    ///             audio: None,
    ///             subtitles: None,
    ///             closed_captions: None,
    ///             stream_data: StreamData::builder()
    ///                 .bandwidth(240000)
    ///                 .codecs(["avc1.42e00a", "mp4a.40.2"])
    ///                 .resolution((416, 234))
    ///                 .build()
    ///                 .unwrap(),
    ///         },
    ///     ])
    ///     .has_independent_segments(true)
    ///     .start(ExtXStart::new(Float::new(1.23)))
    ///     .build()?;
    /// # Ok::<(), Box<dyn ::std::error::Error>>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn builder() -> MasterPlaylistBuilder<'a> {
        MasterPlaylistBuilder::default()
    }

    /// Returns all streams, which have an audio group id.
    pub fn audio_streams(&self) -> impl Iterator<Item = &VariantStream<'a>> {
        self.variant_streams
            .iter()
            .filter(|stream| matches!(stream, VariantStream::ExtXStreamInf { audio: Some(_), .. }))
    }

    /// Returns all streams, which have a video group id.
    pub fn video_streams(&self) -> impl Iterator<Item = &VariantStream<'a>> {
        self.variant_streams.iter().filter(|stream| {
            if let VariantStream::ExtXStreamInf { stream_data, .. } = stream {
                stream_data.video().is_some()
            } else if let VariantStream::ExtXIFrame { stream_data, .. } = stream {
                stream_data.video().is_some()
            } else {
                false
            }
        })
    }

    /// Returns all streams, which have no group id.
    pub fn unassociated_streams(&self) -> impl Iterator<Item = &VariantStream<'a>> {
        self.variant_streams.iter().filter(|stream| {
            if let VariantStream::ExtXStreamInf {
                stream_data,
                audio: None,
                subtitles: None,
                closed_captions: None,
                ..
            } = stream
            {
                stream_data.video().is_none()
            } else if let VariantStream::ExtXIFrame { stream_data, .. } = stream {
                stream_data.video().is_none()
            } else {
                false
            }
        })
    }

    /// Returns all `ExtXMedia` tags, associated with the provided stream.
    pub fn associated_with<'b>(
        &'b self,
        stream: &'b VariantStream<'_>,
    ) -> impl Iterator<Item = &'b ExtXMedia<'a>> + 'b {
        self.media
            .iter()
            .filter(move |media| stream.is_associated(media))
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    #[allow(clippy::redundant_closure_for_method_calls)]
    pub fn into_owned(self) -> MasterPlaylist<'static> {
        MasterPlaylist {
            has_independent_segments: self.has_independent_segments,
            start: self.start,
            media: self.media.into_iter().map(|v| v.into_owned()).collect(),
            variant_streams: self
                .variant_streams
                .into_iter()
                .map(|v| v.into_owned())
                .collect(),
            session_data: self
                .session_data
                .into_iter()
                .map(|v| v.into_owned())
                .collect(),
            session_keys: self
                .session_keys
                .into_iter()
                .map(|v| v.into_owned())
                .collect(),
            unknown_tags: self
                .unknown_tags
                .into_iter()
                .map(|v| Cow::Owned(v.into_owned()))
                .collect(),
        }
    }
}

impl RequiredVersion for MasterPlaylist<'_> {
    fn required_version(&self) -> ProtocolVersion {
        required_version![
            self.has_independent_segments
                .athen_some(ExtXIndependentSegments),
            self.start,
            self.media,
            self.variant_streams,
            self.session_data,
            self.session_keys
        ]
    }
}

impl MasterPlaylistBuilder<'_> {
    fn validate(&self) -> Result<(), String> {
        if let Some(variant_streams) = &self.variant_streams {
            self.validate_variants(variant_streams)
                .map_err(|e| e.to_string())?;
        }

        self.validate_session_data_tags()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn validate_variants(&self, variant_streams: &[VariantStream<'_>]) -> crate::Result<()> {
        let mut closed_captions_none = false;

        for variant in variant_streams {
            match &variant {
                VariantStream::ExtXStreamInf {
                    audio,
                    subtitles,
                    closed_captions,
                    stream_data,
                    ..
                } => {
                    if let Some(group_id) = &audio {
                        if !self.check_media_group(MediaType::Audio, group_id) {
                            return Err(Error::unmatched_group(group_id));
                        }
                    }

                    if let Some(group_id) = &stream_data.video() {
                        if !self.check_media_group(MediaType::Video, group_id) {
                            return Err(Error::unmatched_group(group_id));
                        }
                    }

                    if let Some(group_id) = &subtitles {
                        if !self.check_media_group(MediaType::Subtitles, group_id) {
                            return Err(Error::unmatched_group(group_id));
                        }
                    }

                    if let Some(closed_captions) = &closed_captions {
                        match &closed_captions {
                            ClosedCaptions::GroupId(group_id) => {
                                if closed_captions_none {
                                    return Err(Error::custom("ClosedCaptions has to be `None`"));
                                }

                                if !self.check_media_group(MediaType::ClosedCaptions, group_id) {
                                    return Err(Error::unmatched_group(group_id));
                                }
                            }
                            _ => {
                                if !closed_captions_none {
                                    closed_captions_none = true;
                                }
                            }
                        }
                    }
                }

                VariantStream::ExtXIFrame { stream_data, .. } => {
                    if let Some(group_id) = stream_data.video() {
                        if !self.check_media_group(MediaType::Video, group_id) {
                            return Err(Error::unmatched_group(group_id));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn validate_session_data_tags(&self) -> crate::Result<()> {
        let mut set = HashSet::new();

        if let Some(values) = &self.session_data {
            set.reserve(values.len());

            for tag in values {
                if !set.insert((tag.data_id(), tag.language())) {
                    return Err(Error::custom(format!("conflict: {}", tag)));
                }
            }
        }

        Ok(())
    }

    fn check_media_group<T: AsRef<str>>(&self, media_type: MediaType, group_id: T) -> bool {
        self.media.as_ref().is_some_and(|value| {
            value.iter().any(|media| {
                media.media_type == media_type && media.group_id().as_ref() == group_id.as_ref()
            })
        })
    }
}

impl RequiredVersion for MasterPlaylistBuilder<'_> {
    fn required_version(&self) -> ProtocolVersion {
        // TODO: the .flatten() can be removed as soon as `recursive traits` are
        //       supported. (RequiredVersion is implemented for Option<T>, but
        //       not for Option<Option<T>>)
        // https://github.com/rust-lang/chalk/issues/12
        required_version![
            self.has_independent_segments
                .unwrap_or(false)
                .athen_some(ExtXIndependentSegments),
            self.start.flatten(),
            self.media,
            self.variant_streams,
            self.session_data,
            self.session_keys
        ]
    }
}

impl fmt::Display for MasterPlaylist<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", ExtM3u)?;

        if self.required_version() != ProtocolVersion::V1 {
            writeln!(f, "{}", ExtXVersion::new(self.required_version()))?;
        }

        for value in &self.media {
            writeln!(f, "{}", value)?;
        }

        for value in &self.variant_streams {
            writeln!(f, "{}", value)?;
        }

        for value in &self.session_data {
            writeln!(f, "{}", value)?;
        }

        for value in &self.session_keys {
            writeln!(f, "{}", value)?;
        }

        if self.has_independent_segments {
            writeln!(f, "{}", ExtXIndependentSegments)?;
        }

        if let Some(value) = &self.start {
            writeln!(f, "{}", value)?;
        }

        for value in &self.unknown_tags {
            writeln!(f, "{}", value)?;
        }

        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for MasterPlaylist<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let input = tag(input, ExtM3u::PREFIX)?;
        let mut builder = Self::builder();

        let mut media = vec![];
        let mut variant_streams = vec![];
        let mut session_data = vec![];
        let mut session_keys = vec![];
        let mut unknown_tags = vec![];

        for line in Lines::from(input) {
            match line? {
                Line::Tag(tag) => {
                    match tag {
                        Tag::ExtXVersion(_) => {
                            // This tag can be ignored, because the
                            // MasterPlaylist will automatically set the
                            // ExtXVersion tag to the minimum required version
                            // TODO: this might be verified?
                        }
                        Tag::ExtInf(_)
                        | Tag::ExtXByteRange(_)
                        | Tag::ExtXDiscontinuity(_)
                        | Tag::ExtXKey(_)
                        | Tag::ExtXMap(_)
                        | Tag::ExtXProgramDateTime(_)
                        | Tag::ExtXDateRange(_)
                        | Tag::ExtXTargetDuration(_)
                        | Tag::ExtXMediaSequence(_)
                        | Tag::ExtXDiscontinuitySequence(_)
                        | Tag::ExtXEndList(_)
                        | Tag::PlaylistType(_)
                        | Tag::ExtXIFramesOnly(_) => {
                            return Err(Error::unexpected_tag(tag));
                        }
                        Tag::ExtXMedia(t) => {
                            media.push(t);
                        }
                        Tag::VariantStream(t) => {
                            variant_streams.push(t);
                        }
                        Tag::ExtXSessionData(t) => {
                            session_data.push(t);
                        }
                        Tag::ExtXSessionKey(t) => {
                            session_keys.push(t);
                        }
                        Tag::ExtXIndependentSegments(_) => {
                            builder.has_independent_segments(true);
                        }
                        Tag::ExtXStart(t) => {
                            builder.start(t);
                        }
                        Tag::Unknown(value) => {
                            // [6.3.1. General Client Responsibilities]
                            // > ignore any unrecognized tags.
                            unknown_tags.push(Cow::Borrowed(value));
                        }
                    }
                }
                Line::Uri(uri) => {
                    return Err(Error::custom(format!("unexpected uri: {:?}", uri)));
                }
                Line::Comment(_) => {}
            }
        }

        builder.media(media);
        builder.variant_streams(variant_streams);
        builder.session_data(session_data);
        builder.session_keys(session_keys);
        builder.unknown_tags(unknown_tags);

        builder.build().map_err(Error::builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StreamData;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_audio_streams() {
        let astreams = vec![
            VariantStream::ExtXStreamInf {
                uri: "http://example.com/low/index.m3u8".into(),
                frame_rate: None,
                audio: Some("ag0".into()),
                subtitles: None,
                closed_captions: None,
                stream_data: StreamData::builder()
                    .bandwidth(150_000)
                    .codecs(["avc1.42e00a", "mp4a.40.2"])
                    .resolution((416, 234))
                    .build()
                    .unwrap(),
            },
            VariantStream::ExtXStreamInf {
                uri: "http://example.com/lo_mid/index.m3u8".into(),
                frame_rate: None,
                audio: Some("ag1".into()),
                subtitles: None,
                closed_captions: None,
                stream_data: StreamData::builder()
                    .bandwidth(240_000)
                    .codecs(["avc1.42e00a", "mp4a.40.2"])
                    .resolution((416, 234))
                    .build()
                    .unwrap(),
            },
        ];

        let master_playlist = MasterPlaylist::builder()
            .variant_streams(astreams.clone())
            .media(vec![
                ExtXMedia::builder()
                    .media_type(MediaType::Audio)
                    .uri("https://www.example.com/ag0.m3u8")
                    .group_id("ag0")
                    .language("english")
                    .name("alternative rendition for ag0")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Audio)
                    .uri("https://www.example.com/ag1.m3u8")
                    .group_id("ag1")
                    .language("english")
                    .name("alternative rendition for ag1")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(
            master_playlist.variant_streams,
            master_playlist.audio_streams().collect::<Vec<_>>()
        );

        let mut audio_streams = master_playlist.audio_streams();

        assert_eq!(audio_streams.next(), Some(&astreams[0]));
        assert_eq!(audio_streams.next(), Some(&astreams[1]));
        assert_eq!(audio_streams.next(), None);
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            MasterPlaylist::try_from(concat!(
                "#EXTM3U\n",
                "#EXT-X-STREAM-INF:",
                "BANDWIDTH=150000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n",
                "http://example.com/low/index.m3u8\n",
                "#EXT-X-STREAM-INF:",
                "BANDWIDTH=240000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n",
                "http://example.com/lo_mid/index.m3u8\n",
                "#EXT-X-STREAM-INF:",
                "BANDWIDTH=440000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n",
                "http://example.com/hi_mid/index.m3u8\n",
                "#EXT-X-STREAM-INF:",
                "BANDWIDTH=640000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=640x360\n",
                "http://example.com/high/index.m3u8\n",
                "#EXT-X-STREAM-INF:BANDWIDTH=64000,CODECS=\"mp4a.40.5\"\n",
                "http://example.com/audio/index.m3u8\n"
            ))
            .unwrap(),
            MasterPlaylist::builder()
                .variant_streams(vec![
                    VariantStream::ExtXStreamInf {
                        uri: "http://example.com/low/index.m3u8".into(),
                        frame_rate: None,
                        audio: None,
                        subtitles: None,
                        closed_captions: None,
                        stream_data: StreamData::builder()
                            .bandwidth(150_000)
                            .codecs(["avc1.42e00a", "mp4a.40.2"])
                            .resolution((416, 234))
                            .build()
                            .unwrap()
                    },
                    VariantStream::ExtXStreamInf {
                        uri: "http://example.com/lo_mid/index.m3u8".into(),
                        frame_rate: None,
                        audio: None,
                        subtitles: None,
                        closed_captions: None,
                        stream_data: StreamData::builder()
                            .bandwidth(240_000)
                            .codecs(["avc1.42e00a", "mp4a.40.2"])
                            .resolution((416, 234))
                            .build()
                            .unwrap()
                    },
                    VariantStream::ExtXStreamInf {
                        uri: "http://example.com/hi_mid/index.m3u8".into(),
                        frame_rate: None,
                        audio: None,
                        subtitles: None,
                        closed_captions: None,
                        stream_data: StreamData::builder()
                            .bandwidth(440_000)
                            .codecs(["avc1.42e00a", "mp4a.40.2"])
                            .resolution((416, 234))
                            .build()
                            .unwrap()
                    },
                    VariantStream::ExtXStreamInf {
                        uri: "http://example.com/high/index.m3u8".into(),
                        frame_rate: None,
                        audio: None,
                        subtitles: None,
                        closed_captions: None,
                        stream_data: StreamData::builder()
                            .bandwidth(640_000)
                            .codecs(["avc1.42e00a", "mp4a.40.2"])
                            .resolution((640, 360))
                            .build()
                            .unwrap()
                    },
                    VariantStream::ExtXStreamInf {
                        uri: "http://example.com/audio/index.m3u8".into(),
                        frame_rate: None,
                        audio: None,
                        subtitles: None,
                        closed_captions: None,
                        stream_data: StreamData::builder()
                            .bandwidth(64000)
                            .codecs(["mp4a.40.5"])
                            .build()
                            .unwrap()
                    },
                ])
                .build()
                .unwrap()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(
            MasterPlaylist::builder()
                .variant_streams(vec![
                    VariantStream::ExtXStreamInf {
                        uri: "http://example.com/low/index.m3u8".into(),
                        frame_rate: None,
                        audio: None,
                        subtitles: None,
                        closed_captions: None,
                        stream_data: StreamData::builder()
                            .bandwidth(150_000)
                            .codecs(["avc1.42e00a", "mp4a.40.2"])
                            .resolution((416, 234))
                            .build()
                            .unwrap()
                    },
                    VariantStream::ExtXStreamInf {
                        uri: "http://example.com/lo_mid/index.m3u8".into(),
                        frame_rate: None,
                        audio: None,
                        subtitles: None,
                        closed_captions: None,
                        stream_data: StreamData::builder()
                            .bandwidth(240_000)
                            .codecs(["avc1.42e00a", "mp4a.40.2"])
                            .resolution((416, 234))
                            .build()
                            .unwrap()
                    },
                    VariantStream::ExtXStreamInf {
                        uri: "http://example.com/hi_mid/index.m3u8".into(),
                        frame_rate: None,
                        audio: None,
                        subtitles: None,
                        closed_captions: None,
                        stream_data: StreamData::builder()
                            .bandwidth(440_000)
                            .codecs(["avc1.42e00a", "mp4a.40.2"])
                            .resolution((416, 234))
                            .build()
                            .unwrap()
                    },
                    VariantStream::ExtXStreamInf {
                        uri: "http://example.com/high/index.m3u8".into(),
                        frame_rate: None,
                        audio: None,
                        subtitles: None,
                        closed_captions: None,
                        stream_data: StreamData::builder()
                            .bandwidth(640_000)
                            .codecs(["avc1.42e00a", "mp4a.40.2"])
                            .resolution((640, 360))
                            .build()
                            .unwrap()
                    },
                    VariantStream::ExtXStreamInf {
                        uri: "http://example.com/audio/index.m3u8".into(),
                        frame_rate: None,
                        audio: None,
                        subtitles: None,
                        closed_captions: None,
                        stream_data: StreamData::builder()
                            .bandwidth(64000)
                            .codecs(["mp4a.40.5"])
                            .build()
                            .unwrap()
                    },
                ])
                .build()
                .unwrap()
                .to_string(),
            concat!(
                "#EXTM3U\n",
                //
                "#EXT-X-STREAM-INF:",
                "BANDWIDTH=150000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n",
                "http://example.com/low/index.m3u8\n",
                //
                "#EXT-X-STREAM-INF:",
                "BANDWIDTH=240000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n",
                "http://example.com/lo_mid/index.m3u8\n",
                //
                "#EXT-X-STREAM-INF:",
                "BANDWIDTH=440000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n",
                "http://example.com/hi_mid/index.m3u8\n",
                //
                "#EXT-X-STREAM-INF:",
                "BANDWIDTH=640000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=640x360\n",
                "http://example.com/high/index.m3u8\n",
                //
                "#EXT-X-STREAM-INF:BANDWIDTH=64000,CODECS=\"mp4a.40.5\"\n",
                "http://example.com/audio/index.m3u8\n"
            )
            .to_string()
        );
    }
}
