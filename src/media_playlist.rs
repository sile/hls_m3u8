use std::borrow::Cow;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use derive_builder::Builder;
use stable_vec::StableVec;

use crate::line::{Line, Lines, Tag};
use crate::media_segment::MediaSegment;
use crate::tags::{
    ExtM3u, ExtXByteRange, ExtXDiscontinuitySequence, ExtXEndList, ExtXIFramesOnly,
    ExtXIndependentSegments, ExtXKey, ExtXMediaSequence, ExtXStart, ExtXTargetDuration,
    ExtXVersion,
};
use crate::types::{
    DecryptionKey, EncryptionMethod, InitializationVector, KeyFormat, PlaylistType, ProtocolVersion,
};
use crate::utils::{tag, BoolExt};
use crate::{Error, RequiredVersion};

/// Media playlist.
#[derive(Builder, Debug, Clone, PartialEq, Eq)]
#[builder(build_fn(skip), setter(strip_option))]
#[non_exhaustive]
pub struct MediaPlaylist<'a> {
    /// Specifies the maximum [`MediaSegment::duration`]. A typical target
    /// duration is 10 seconds.
    ///
    /// ### Note
    ///
    /// This field is required.
    pub target_duration: Duration,
    /// The [`MediaSegment::number`] of the first [`MediaSegment`] that
    /// appears in a [`MediaPlaylist`].
    ///
    /// ### Note
    ///
    /// This field is optional and by default a value of 0 is assumed.
    #[builder(default)]
    pub media_sequence: usize,
    /// Allows synchronization between different renditions of the same
    /// [`VariantStream`].
    ///
    /// ### Note
    ///
    /// This field is optional and by default a vaule of 0 is assumed.
    ///
    /// [`VariantStream`]: crate::tags::VariantStream
    #[builder(default)]
    pub discontinuity_sequence: usize,
    /// Provides mutability information about a [`MediaPlaylist`].
    ///
    /// - [`PlaylistType::Vod`] indicates that the playlist must not change.
    ///
    /// - [`PlaylistType::Event`] indicates that the server does not change or
    /// delete any part of the playlist, but may append new lines to it.
    ///
    /// ### Note
    ///
    /// This field is optional.
    #[builder(default, setter(into))]
    pub playlist_type: Option<PlaylistType>,
    /// Indicates that each [`MediaSegment`] in the playlist describes a single
    /// I-frame. I-frames are encoded video frames, whose decoding does not
    /// depend on any other frame. I-frame Playlists can be used for trick
    /// play, such as fast forward, rapid reverse, and scrubbing.
    ///
    /// ### Note
    ///
    /// This field is optional.
    #[builder(default)]
    pub has_i_frames_only: bool,
    /// This indicates that all media samples in a [`MediaSegment`] can be
    /// decoded without information from other segments.
    ///
    /// ### Note
    ///
    /// This field is optional and by default `false`. If the value is `true` it
    /// applies to every [`MediaSegment`] in this [`MediaPlaylist`].
    #[builder(default)]
    pub has_independent_segments: bool,
    /// Indicates a preferred point at which to start playing a playlist. By
    /// default, clients should start playback at this point when beginning a
    /// playback session.
    ///
    /// ### Note
    ///
    /// This field is optional.
    #[builder(default, setter(into))]
    pub start: Option<ExtXStart>,
    /// Indicates that no more [`MediaSegment`]s will be added to the
    /// [`MediaPlaylist`] file.
    ///
    /// ### Note
    ///
    /// This field is optional and by default `false`.
    /// A `false` indicates that the client should reload the [`MediaPlaylist`]
    /// from the server, until a playlist is encountered, where this field is
    /// `true`.
    #[builder(default)]
    pub has_end_list: bool,
    /// A list of all [`MediaSegment`]s.
    ///
    /// ### Note
    ///
    /// This field is required.
    #[builder(setter(custom))]
    pub segments: StableVec<MediaSegment<'a>>,
    /// The allowable excess duration of each media segment in the
    /// associated playlist.
    ///
    /// ### Error
    ///
    /// If there is a media segment of which duration exceeds
    /// `#EXT-X-TARGETDURATION + allowable_excess_duration`,
    /// the invocation of `MediaPlaylistBuilder::build()` method will fail.
    ///
    ///
    /// ### Note
    ///
    /// This field is optional and the default value is
    /// `Duration::from_secs(0)`.
    #[builder(default = "Duration::from_secs(0)")]
    pub allowable_excess_duration: Duration,
    /// A list of unknown tags.
    ///
    /// ### Note
    ///
    /// This field is optional.
    #[builder(default, setter(into))]
    pub unknown: Vec<Cow<'a, str>>,
}

impl<'a> MediaPlaylistBuilder<'a> {
    fn validate(&self) -> Result<(), String> {
        if let Some(target_duration) = &self.target_duration {
            self.validate_media_segments(*target_duration)
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn validate_media_segments(&self, target_duration: Duration) -> crate::Result<()> {
        let mut last_range_uri = None;

        if let Some(segments) = &self.segments {
            // verify the independent segments
            if self.has_independent_segments.unwrap_or(false) {
                // If the encryption METHOD is AES-128 and the Playlist contains an EXT-
                // X-I-FRAMES-ONLY tag, the entire resource MUST be encrypted using
                // AES-128 CBC with PKCS7 padding [RFC5652].
                //
                // from the rfc: https://tools.ietf.org/html/rfc8216#section-6.2.3

                let is_aes128 = segments
                    .values()
                    // convert iterator of segments to iterator of keys
                    .flat_map(|s| s.keys.iter())
                    // filter out all empty keys
                    .filter_map(ExtXKey::as_ref)
                    .any(|k| k.method == EncryptionMethod::Aes128);

                if is_aes128 {
                    for key in segments.values().flat_map(|s| s.keys.iter()) {
                        if let ExtXKey(Some(key)) = key {
                            if key.method != EncryptionMethod::Aes128 {
                                return Err(Error::custom(concat!(
                                    "if any independent segment is encrypted with Aes128,",
                                    " all must be encrypted with Aes128"
                                )));
                            }
                        } else {
                            return Err(Error::custom(concat!(
                                "if any independent segment is encrypted with Aes128,",
                                " all must be encrypted with Aes128"
                            )));
                        }
                    }
                }
            }

            for segment in segments.values() {
                // CHECK: `#EXT-X-TARGETDURATION`
                let segment_duration = segment.duration.duration();

                // round the duration if it is .5s
                let rounded_segment_duration =
                    Duration::from_secs(segment_duration.as_secs_f64().round() as u64);

                let max_segment_duration = self
                    .allowable_excess_duration
                    .as_ref()
                    .map_or(target_duration, |value| target_duration + *value);

                if rounded_segment_duration > max_segment_duration {
                    return Err(Error::custom(format!(
                        "Too large segment duration: actual={:?}, max={:?}, target_duration={:?}, uri={:?}",
                        segment_duration,
                        max_segment_duration,
                        target_duration,
                        segment.uri()
                    )));
                }

                // CHECK: `#EXT-X-BYTE-RANGE`
                if let Some(range) = &segment.byte_range {
                    if range.start().is_none() {
                        // TODO: error messages
                        if last_range_uri.ok_or_else(Error::invalid_input)? != segment.uri() {
                            return Err(Error::invalid_input());
                        }
                    } else {
                        last_range_uri = Some(segment.uri());
                    }
                } else {
                    last_range_uri = None;
                }
            }
        }

        Ok(())
    }

    /// Adds a media segment to the resulting playlist and assigns the next free
    /// [`MediaSegment::number`] to the segment.
    pub fn push_segment(&mut self, segment: MediaSegment<'a>) -> &mut Self {
        let segments = self.segments.get_or_insert_with(StableVec::new);

        if segment.explicit_number {
            segments.reserve_for(segment.number);
            segments.insert(segment.number, segment);
        } else {
            segments.push(segment);
        }

        self
    }

    /// Parse the rest of the [`MediaPlaylist`] from an m3u8 file.
    pub fn parse(&mut self, input: &'a str) -> crate::Result<MediaPlaylist<'a>> {
        parse_media_playlist(input, self)
    }

    /// Adds segments to the resulting playlist and assigns a
    /// [`MediaSegment::number`] to each segment.
    ///
    /// ## Note
    ///
    /// The [`MediaSegment::number`] will be assigned based on the order of the
    /// input (e.g. the first element will be 0, second element 1, ..) or if a
    /// number has been set explicitly. This function assumes, that all segments
    /// will be present in the final media playlist and the following is only
    /// possible if the segment is marked with `ExtXDiscontinuity`.
    pub fn segments(&mut self, segments: Vec<MediaSegment<'a>>) -> &mut Self {
        let mut vec = StableVec::<MediaSegment<'a>>::with_capacity(segments.len());
        let mut remaining = Vec::with_capacity(segments.len());

        for segment in segments {
            if segment.explicit_number {
                vec.insert(segment.number, segment);
            } else {
                remaining.push(segment);
            }
        }

        for segment in remaining {
            vec.push(segment);
        }

        self.segments = Some(vec);
        self
    }

    /// Builds a new `MediaPlaylist`.
    ///
    /// # Errors
    ///
    /// If a required field has not been initialized.
    pub fn build(&self) -> Result<MediaPlaylist<'a>, String> {
        // validate builder
        self.validate()?;

        let sequence_number = self.media_sequence.unwrap_or(0);

        let mut segments = self
            .segments
            .clone()
            .ok_or_else(|| "missing field `segments`".to_string())?;

        // no segment should exist before the sequence_number
        if let Some(first_segment) = segments.find_first() {
            if sequence_number > first_segment.number && first_segment.explicit_number {
                return Err(format!(
                    "there should be no segment ({}) before the sequence_number ({})",
                    first_segment, sequence_number,
                ));
            }
        }

        let mut previous_range: Option<ExtXByteRange> = None;

        for (i, segment) in segments.iter_mut() {
            // assign the correct number to all implcitly numbered segments:
            if !segment.explicit_number {
                segment.number = i + sequence_number;
            }

            // add the segment number as iv, if the iv is missing:
            for key in &mut segment.keys {
                if let ExtXKey(Some(DecryptionKey {
                    method, iv, format, ..
                })) = key
                {
                    if *method == EncryptionMethod::Aes128 && *iv == InitializationVector::Missing {
                        if format.is_none() {
                            *iv = InitializationVector::Number(segment.number as u128);
                        } else if let Some(KeyFormat::Identity) = format {
                            *iv = InitializationVector::Number(segment.number as u128);
                        }
                    }
                }
            }

            // add the lower bound to the byterange automatically
            if let Some(range) = &mut segment.byte_range {
                if range.start().is_none() {
                    if let Some(previous_range) = previous_range {
                        // the end of the previous_range is the start of the next range
                        *range = range.saturating_add(previous_range.end());
                        range.set_start(Some(previous_range.end()));
                    } else {
                        // assume that the byte range starts at zero
                        range.set_start(Some(0));
                    }
                }

                previous_range = segment.byte_range;
            }
        }

        // TODO: can segments be missing?
        if !segments.is_compact() {
            // find the missing segment by iterating through all segments:
            // let missing = segments
            //     .iter()
            //     .enumerate()
            //     .find_map(|(i, e)| e.is_none().athen(i))
            //     .unwrap();
            return Err(format!("a segment is missing"));
        }

        Ok(MediaPlaylist {
            target_duration: self
                .target_duration
                .ok_or_else(|| "missing field `target_duration`".to_string())?,
            media_sequence: self.media_sequence.unwrap_or(0),
            discontinuity_sequence: self.discontinuity_sequence.unwrap_or(0),
            playlist_type: self.playlist_type.unwrap_or(None),
            has_i_frames_only: self.has_i_frames_only.unwrap_or(false),
            has_independent_segments: self.has_independent_segments.unwrap_or(false),
            start: self.start.unwrap_or(None),
            has_end_list: self.has_end_list.unwrap_or(false),
            segments,
            allowable_excess_duration: self
                .allowable_excess_duration
                .unwrap_or_else(|| Duration::from_secs(0)),
            unknown: self.unknown.clone().unwrap_or_else(Vec::new),
        })
    }
}

impl<'a> RequiredVersion for MediaPlaylistBuilder<'a> {
    fn required_version(&self) -> ProtocolVersion {
        required_version![
            self.target_duration.map(ExtXTargetDuration),
            (self.media_sequence.unwrap_or(0) != 0)
                .athen(|| ExtXMediaSequence(self.media_sequence.unwrap_or(0))),
            (self.discontinuity_sequence.unwrap_or(0) != 0)
                .athen(|| ExtXDiscontinuitySequence(self.discontinuity_sequence.unwrap_or(0))),
            self.playlist_type,
            self.has_i_frames_only
                .unwrap_or(false)
                .athen_some(ExtXIFramesOnly),
            self.has_independent_segments
                .unwrap_or(false)
                .athen_some(ExtXIndependentSegments),
            self.start,
            self.has_end_list.unwrap_or(false).athen_some(ExtXEndList),
            self.segments
        ]
    }
}

impl<'a> MediaPlaylist<'a> {
    /// Returns a builder for [`MediaPlaylist`].
    #[must_use]
    #[inline]
    pub fn builder() -> MediaPlaylistBuilder<'a> { MediaPlaylistBuilder::default() }

    /// Computes the `Duration` of the [`MediaPlaylist`], by adding each segment
    /// duration together.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.segments.values().map(|s| s.duration.duration()).sum()
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> MediaPlaylist<'static> {
        MediaPlaylist {
            target_duration: self.target_duration,
            media_sequence: self.media_sequence,
            discontinuity_sequence: self.discontinuity_sequence,
            playlist_type: self.playlist_type,
            has_i_frames_only: self.has_i_frames_only,
            has_independent_segments: self.has_independent_segments,
            start: self.start,
            has_end_list: self.has_end_list,
            segments: {
                self.segments
                    .into_iter()
                    .map(|(_, s)| s.into_owned())
                    .collect()
            },
            allowable_excess_duration: self.allowable_excess_duration,
            unknown: {
                self.unknown
                    .into_iter()
                    .map(|v| Cow::Owned(v.into_owned()))
                    .collect()
            },
        }
    }
}

impl<'a> RequiredVersion for MediaPlaylist<'a> {
    fn required_version(&self) -> ProtocolVersion {
        required_version![
            ExtXTargetDuration(self.target_duration),
            (self.media_sequence != 0).athen(|| ExtXMediaSequence(self.media_sequence)),
            (self.discontinuity_sequence != 0)
                .athen(|| ExtXDiscontinuitySequence(self.discontinuity_sequence)),
            self.playlist_type,
            self.has_i_frames_only.athen_some(ExtXIFramesOnly),
            self.has_independent_segments
                .athen_some(ExtXIndependentSegments),
            self.start,
            self.has_end_list.athen_some(ExtXEndList),
            self.segments
        ]
    }
}

impl<'a> fmt::Display for MediaPlaylist<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", ExtM3u)?;

        if self.required_version() != ProtocolVersion::V1 {
            writeln!(f, "{}", ExtXVersion::new(self.required_version()))?;
        }

        writeln!(f, "{}", ExtXTargetDuration(self.target_duration))?;

        if self.media_sequence != 0 {
            writeln!(f, "{}", ExtXMediaSequence(self.media_sequence))?;
        }

        if self.discontinuity_sequence != 0 {
            writeln!(
                f,
                "{}",
                ExtXDiscontinuitySequence(self.discontinuity_sequence)
            )?;
        }

        if let Some(value) = &self.playlist_type {
            writeln!(f, "{}", value)?;
        }

        if self.has_i_frames_only {
            writeln!(f, "{}", ExtXIFramesOnly)?;
        }

        if self.has_independent_segments {
            writeln!(f, "{}", ExtXIndependentSegments)?;
        }

        if let Some(value) = &self.start {
            writeln!(f, "{}", value)?;
        }

        let mut available_keys = HashSet::<ExtXKey<'_>>::new();

        for segment in self.segments.values() {
            for key in &segment.keys {
                if let ExtXKey(Some(decryption_key)) = key {
                    // next segment will be encrypted, so the segment can not have an empty key
                    available_keys.remove(&ExtXKey::empty());

                    let mut decryption_key = decryption_key.clone();
                    let key = {
                        if let InitializationVector::Number(_) = decryption_key.iv {
                            // set the iv from a segment number to missing
                            // this does reduce the output size and the correct iv
                            // is automatically set, when parsing.
                            decryption_key.iv = InitializationVector::Missing;
                        }

                        ExtXKey(Some(decryption_key.clone()))
                    };

                    // only do something if a key has been overwritten
                    if available_keys.insert(key.clone()) {
                        let mut remove_key = None;

                        // an old key might be removed:
                        for k in &available_keys {
                            if let ExtXKey(Some(dk)) = k {
                                if dk.format == decryption_key.format && key != *k {
                                    remove_key = Some(k.clone());
                                    break;
                                }
                            } else {
                                unreachable!("empty keys should not exist in `available_keys`");
                            }
                        }

                        if let Some(k) = remove_key {
                            // this should always be true:
                            let res = available_keys.remove(&k);
                            debug_assert!(res);
                        }

                        writeln!(f, "{}", key)?;
                    }
                } else {
                    // the next segment is not encrypted, so remove all available keys
                    available_keys.clear();
                    available_keys.insert(ExtXKey::empty());
                    writeln!(f, "{}", key)?;
                }
            }

            write!(f, "{}", segment)?;
        }

        for value in &self.unknown {
            writeln!(f, "{}", value)?;
        }

        if self.has_end_list {
            writeln!(f, "{}", ExtXEndList)?;
        }

        Ok(())
    }
}

fn parse_media_playlist<'a>(
    input: &'a str,
    builder: &mut MediaPlaylistBuilder<'a>,
) -> crate::Result<MediaPlaylist<'a>> {
    let input = tag(input, "#EXTM3U")?;

    let mut segment = MediaSegment::builder();
    let mut segments = vec![];

    let mut has_partial_segment = false;
    let mut has_discontinuity_tag = false;
    let mut unknown = vec![];
    let mut available_keys = HashSet::new();

    for line in Lines::from(input) {
        match line? {
            Line::Tag(tag) => {
                match tag {
                    Tag::ExtInf(t) => {
                        has_partial_segment = true;
                        segment.duration(t);
                    }
                    Tag::ExtXByteRange(t) => {
                        has_partial_segment = true;
                        segment.byte_range(t);
                    }
                    Tag::ExtXDiscontinuity(_) => {
                        has_discontinuity_tag = true;
                        has_partial_segment = true;
                        segment.has_discontinuity(true);
                    }
                    Tag::ExtXKey(key) => {
                        has_partial_segment = true;

                        // An ExtXKey applies to every MediaSegment and to every Media
                        // Initialization Section declared by an ExtXMap tag, that appears
                        // between it and the next ExtXKey tag in the Playlist file with the
                        // same KEYFORMAT attribute (or the end of the Playlist file).

                        let mut is_new_key = true;
                        let mut remove = None;

                        if let ExtXKey(Some(decryption_key)) = &key {
                            for old_key in &available_keys {
                                if let ExtXKey(Some(old_decryption_key)) = &old_key {
                                    if old_decryption_key.format == decryption_key.format {
                                        // remove the old key
                                        remove = Some(old_key.clone());

                                        // there are no keys with the same format in
                                        // available_keys so the loop can stop here:
                                        break;
                                    }
                                } else {
                                    // remove an empty key
                                    remove = Some(ExtXKey::empty());
                                    break;
                                }
                            }
                        } else {
                            available_keys.clear();
                            available_keys.insert(ExtXKey::empty());
                            is_new_key = false;
                        }

                        if let Some(key) = &remove {
                            available_keys.remove(key);
                        }

                        if is_new_key {
                            available_keys.insert(key);
                        }
                    }
                    Tag::ExtXMap(mut t) => {
                        has_partial_segment = true;

                        t.keys = available_keys.iter().cloned().collect();
                        segment.map(t);
                    }
                    Tag::ExtXProgramDateTime(t) => {
                        has_partial_segment = true;
                        segment.program_date_time(t);
                    }
                    Tag::ExtXDateRange(t) => {
                        has_partial_segment = true;
                        segment.date_range(t);
                    }
                    Tag::ExtXTargetDuration(t) => {
                        builder.target_duration(t.0);
                    }
                    Tag::ExtXMediaSequence(t) => {
                        builder.media_sequence(t.0);
                    }
                    Tag::ExtXDiscontinuitySequence(t) => {
                        if segments.is_empty() {
                            return Err(Error::invalid_input());
                        }

                        if has_discontinuity_tag {
                            return Err(Error::invalid_input());
                        }

                        builder.discontinuity_sequence(t.0);
                    }
                    Tag::ExtXEndList(_) => {
                        builder.has_end_list(true);
                    }
                    Tag::PlaylistType(t) => {
                        builder.playlist_type(t);
                    }
                    Tag::ExtXIFramesOnly(_) => {
                        builder.has_i_frames_only(true);
                    }
                    Tag::ExtXMedia(_)
                    | Tag::VariantStream(_)
                    | Tag::ExtXSessionData(_)
                    | Tag::ExtXSessionKey(_) => {
                        return Err(Error::unexpected_tag(tag));
                    }
                    Tag::ExtXIndependentSegments(_) => {
                        builder.has_independent_segments(true);
                    }
                    Tag::ExtXStart(t) => {
                        builder.start(t);
                    }
                    Tag::ExtXVersion(_) => {}
                    Tag::Unknown(s) => {
                        // [6.3.1. General Client Responsibilities]
                        // > ignore any unrecognized tags.
                        unknown.push(Cow::Borrowed(s));
                    }
                }
            }
            Line::Uri(uri) => {
                segment.uri(uri);
                segment.keys(available_keys.iter().cloned().collect::<Vec<_>>());
                segments.push(segment.build().map_err(Error::builder)?);

                segment = MediaSegment::builder();
                has_partial_segment = false;
            }
            _ => {}
        }
    }

    if has_partial_segment {
        return Err(Error::custom("Missing URI for the last `MediaSegment`"));
    }

    builder.unknown(unknown);
    builder.segments(segments);
    builder.build().map_err(Error::builder)
}

impl FromStr for MediaPlaylist<'static> {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_media_playlist(input, &mut Self::builder())?.into_owned())
    }
}

impl<'a> TryFrom<&'a str> for MediaPlaylist<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parse_media_playlist(input, &mut Self::builder())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn too_large_segment_duration_test() {
        let playlist = concat!(
            "#EXTM3U\n",
            "#EXT-X-TARGETDURATION:8\n",
            "#EXT-X-VERSION:3\n",
            "#EXTINF:9.009,\n",
            "http://media.example.com/first.ts\n",
            "#EXTINF:9.509,\n",
            "http://media.example.com/second.ts\n",
            "#EXTINF:3.003,\n",
            "http://media.example.com/third.ts\n",
            "#EXT-X-ENDLIST\n"
        );

        // Error (allowable segment duration = target duration = 8)
        assert!(MediaPlaylist::try_from(playlist).is_err());

        // Error (allowable segment duration = 9)
        assert!(MediaPlaylist::builder()
            .allowable_excess_duration(Duration::from_secs(1))
            .parse(playlist)
            .is_err());

        // Ok (allowable segment duration = 10)
        assert_eq!(
            MediaPlaylist::builder()
                .allowable_excess_duration(Duration::from_secs(2))
                .parse(playlist)
                .unwrap(),
            MediaPlaylist::builder()
                .allowable_excess_duration(Duration::from_secs(2))
                .target_duration(Duration::from_secs(8))
                .segments(vec![
                    MediaSegment::builder()
                        .duration(Duration::from_secs_f64(9.009))
                        .uri("http://media.example.com/first.ts")
                        .build()
                        .unwrap(),
                    MediaSegment::builder()
                        .duration(Duration::from_secs_f64(9.509))
                        .uri("http://media.example.com/second.ts")
                        .build()
                        .unwrap(),
                    MediaSegment::builder()
                        .duration(Duration::from_secs_f64(3.003))
                        .uri("http://media.example.com/third.ts")
                        .build()
                        .unwrap(),
                ])
                .has_end_list(true)
                .build()
                .unwrap()
        );
    }

    #[test]
    fn test_segment_number_simple() {
        let playlist = MediaPlaylist::builder()
            .allowable_excess_duration(Duration::from_secs(2))
            .target_duration(Duration::from_secs(8))
            .segments(vec![
                MediaSegment::builder()
                    .duration(Duration::from_secs_f64(9.009))
                    .uri("http://media.example.com/first.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .duration(Duration::from_secs_f64(9.509))
                    .uri("http://media.example.com/second.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .duration(Duration::from_secs_f64(3.003))
                    .uri("http://media.example.com/third.ts")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let mut segments = playlist.segments.into_iter().map(|(k, v)| (k, v.number));
        assert_eq!(segments.next(), Some((0, 0)));
        assert_eq!(segments.next(), Some((1, 1)));
        assert_eq!(segments.next(), Some((2, 2)));
        assert_eq!(segments.next(), None);
    }

    #[test]
    fn test_segment_number_sequence() {
        let playlist = MediaPlaylist::builder()
            .target_duration(Duration::from_secs(8))
            .media_sequence(2680)
            .segments(vec![
                MediaSegment::builder()
                    .duration(Duration::from_secs_f64(7.975))
                    .uri("https://priv.example.com/fileSequence2680.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .duration(Duration::from_secs_f64(7.941))
                    .uri("https://priv.example.com/fileSequence2681.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .duration(Duration::from_secs_f64(7.975))
                    .uri("https://priv.example.com/fileSequence2682.ts")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();
        let mut segments = playlist.segments.into_iter().map(|(k, v)| (k, v.number));
        assert_eq!(segments.next(), Some((0, 2680)));
        assert_eq!(segments.next(), Some((1, 2681)));
        assert_eq!(segments.next(), Some((2, 2682)));
        assert_eq!(segments.next(), None);
    }

    #[test]
    fn test_empty_playlist() {
        let playlist = "";
        assert!(MediaPlaylist::try_from(playlist).is_err());
    }
}
