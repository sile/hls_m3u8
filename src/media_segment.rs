use std::borrow::Cow;
use std::fmt;

use crate::tags::{
    ExtInf, ExtXByteRange, ExtXDateRange, ExtXDiscontinuity, ExtXKey, ExtXMap, ExtXProgramDateTime,
};
use crate::types::{DecryptionKey, ProtocolVersion};
use crate::{Decryptable, Error, RequiredVersion};

/// A video is split into smaller chunks called [`MediaSegment`]s, which are
/// specified by a uri and optionally a byte range.
///
/// Each `MediaSegment` must carry the continuation of the encoded bitstream
/// from the end of the segment with the previous [`MediaSegment::number`],
/// where values in a series such as timestamps and continuity counters must
/// continue uninterrupted. The only exceptions are the first [`MediaSegment`]
/// ever to appear in a [`MediaPlaylist`] and [`MediaSegment`]s that are
/// explicitly signaled as discontinuities.
/// Unmarked media discontinuities can trigger playback errors.
///
/// Any `MediaSegment` that contains video should include enough information
/// to initialize a video decoder and decode a continuous set of frames that
/// includes the final frame in the segment; network efficiency is optimized if
/// there is enough information in the segment to decode all frames in the
/// segment.
///
/// For example, any `MediaSegment` containing H.264 video should
/// contain an Instantaneous Decoding Refresh (IDR); frames prior to the first
/// IDR will be downloaded but possibly discarded.
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MediaSegment<'a> {
    pub(crate) number: usize,
    pub(crate) explicit_number: bool,
    /// This field specifies how to decrypt a [`MediaSegment`], which can only
    /// be encrypted with one [`EncryptionMethod`], using one [`DecryptionKey`]
    /// and [`DecryptionKey::iv`].
    ///
    /// However, a server may offer multiple ways to retrieve that key by
    /// providing multiple keys with different [`DecryptionKey::format`]s.
    ///
    /// Any unencrypted segment that is preceded by an encrypted segment must
    /// have an [`ExtXKey::empty`]. Otherwise, the client will misinterpret
    /// those segments as encrypted.
    ///
    /// The server may set the HTTP Expires header in the key response to
    /// indicate the duration for which the key can be cached.
    ///
    /// ## Note
    ///
    /// This field is optional and a missing value or an [`ExtXKey::empty()`]
    /// indicates an unencrypted media segment.
    ///
    /// [`ExtXMap`]: crate::tags::ExtXMap
    /// [`KeyFormat`]: crate::types::KeyFormat
    /// [`EncryptionMethod`]: crate::types::EncryptionMethod
    pub keys: Vec<ExtXKey<'a>>,
    /// This field specifies how to obtain the Media Initialization Section
    /// required to parse the applicable `MediaSegment`s.
    ///
    /// ## Note
    ///
    /// This field is optional, but should be specified for media segments in
    /// playlists with an [`ExtXIFramesOnly`] tag when the first `MediaSegment`
    /// in the playlist (or the first segment following a segment marked with
    /// [`MediaSegment::has_discontinuity`]) does not immediately follow the
    /// Media Initialization Section at the beginning of its resource.
    ///
    /// [`ExtXIFramesOnly`]: crate::tags::ExtXIFramesOnly
    pub map: Option<ExtXMap<'a>>,
    /// This field indicates that a `MediaSegment` is a sub-range of the
    /// resource identified by its URI.
    ///
    /// ## Note
    ///
    /// This field is optional.
    pub byte_range: Option<ExtXByteRange>,
    /// This field associates a date-range (i.e., a range of time defined by a
    /// starting and ending date) with a set of attribute/value pairs.
    ///
    /// ## Note
    ///
    /// This field is optional.
    pub date_range: Option<ExtXDateRange<'a>>,
    /// This field indicates a discontinuity between the `MediaSegment` that
    /// follows it and the one that preceded it.
    ///
    /// ## Note
    ///
    /// This field is required if any of the following characteristics change:
    /// - file format
    /// - number, type, and identifiers of tracks
    /// - timestamp, sequence
    ///
    /// This field should be present if any of the following characteristics
    /// change:
    /// - encoding parameters
    /// - encoding sequence
    pub has_discontinuity: bool,
    /// This field associates the first sample of a media segment with an
    /// absolute date and/or time.
    ///
    /// ## Note
    ///
    /// This field is optional.
    pub program_date_time: Option<ExtXProgramDateTime<'a>>,
    /// This field indicates the duration of a media segment.
    ///
    /// ## Note
    ///
    /// This field is required.
    pub duration: ExtInf<'a>,
    uri: Cow<'a, str>,
}

/// Builder for a [`MediaSegment`].
#[derive(Debug, Clone, Default)]
pub struct MediaSegmentBuilder<'a> {
    number: Option<usize>,
    explicit_number: Option<bool>,
    keys: Option<Vec<ExtXKey<'a>>>,
    map: Option<ExtXMap<'a>>,
    byte_range: Option<ExtXByteRange>,
    date_range: Option<ExtXDateRange<'a>>,
    has_discontinuity: Option<bool>,
    program_date_time: Option<ExtXProgramDateTime<'a>>,
    duration: Option<ExtInf<'a>>,
    uri: Option<Cow<'a, str>>,
}

impl<'a> MediaSegment<'a> {
    /// The number assigned to this segment.
    #[must_use]
    pub fn number(&self) -> usize {
        self.number
    }

    /// The URI of a media segment.
    ///
    /// ## Note
    ///
    /// This field is required.
    #[must_use]
    pub fn uri(&self) -> &Cow<'a, str> {
        &self.uri
    }

    /// Sets [`MediaSegment::uri`].
    pub fn set_uri<V: Into<Cow<'a, str>>>(&mut self, value: V) -> &mut Self {
        self.uri = value.into();
        self
    }
}

impl MediaSegment<'_> {
    /// Returns a builder for a [`MediaSegment`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::MediaSegment;
    /// use hls_m3u8::tags::ExtXMap;
    /// use std::time::Duration;
    ///
    /// let segment = MediaSegment::builder()
    ///     .map(ExtXMap::new("https://www.example.com/"))
    ///     .byte_range(5..25)
    ///     .has_discontinuity(true)
    ///     .duration(Duration::from_secs(4))
    ///     .uri("http://www.uri.com/")
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn builder() -> MediaSegmentBuilder<'static> {
        MediaSegmentBuilder::default()
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    #[expect(clippy::redundant_closure_for_method_calls)]
    pub fn into_owned(self) -> MediaSegment<'static> {
        MediaSegment {
            number: self.number,
            explicit_number: self.explicit_number,
            keys: self.keys.into_iter().map(|k| k.into_owned()).collect(),
            map: self.map.map(|v| v.into_owned()),
            byte_range: self.byte_range,
            date_range: self.date_range.map(|v| v.into_owned()),
            has_discontinuity: self.has_discontinuity,
            program_date_time: self.program_date_time.map(|v| v.into_owned()),
            duration: self.duration.into_owned(),
            uri: Cow::Owned(self.uri.into_owned()),
        }
    }
}

impl<'a> MediaSegmentBuilder<'a> {
    /// The number of a [`MediaSegment`]. Normally this should not be set
    /// explicitly, because the [`MediaPlaylist::builder`] will automatically
    /// apply the correct number.
    ///
    /// [`MediaPlaylist::builder`]: crate::MediaPlaylist::builder
    pub fn number(&mut self, value: Option<usize>) -> &mut Self {
        self.number = value;
        self.explicit_number = Some(value.is_some());
        self
    }

    /// See [`MediaSegment::keys`].
    pub fn keys<V: Into<Vec<ExtXKey<'a>>>>(&mut self, value: V) -> &mut Self {
        self.keys = Some(value.into());
        self
    }

    /// Pushes an [`ExtXKey`] tag.
    pub fn push_key<V: Into<ExtXKey<'a>>>(&mut self, value: V) -> &mut Self {
        self.keys.get_or_insert_with(Vec::new).push(value.into());
        self
    }

    /// See [`MediaSegment::map`].
    pub fn map(&mut self, value: ExtXMap<'a>) -> &mut Self {
        self.map = Some(value);
        self
    }

    /// See [`MediaSegment::byte_range`].
    pub fn byte_range<V: Into<ExtXByteRange>>(&mut self, value: V) -> &mut Self {
        self.byte_range = Some(value.into());
        self
    }

    /// See [`MediaSegment::date_range`].
    pub fn date_range(&mut self, value: ExtXDateRange<'a>) -> &mut Self {
        self.date_range = Some(value);
        self
    }

    /// See [`MediaSegment::has_discontinuity`].
    pub fn has_discontinuity(&mut self, value: bool) -> &mut Self {
        self.has_discontinuity = Some(value);
        self
    }

    /// See [`MediaSegment::program_date_time`].
    pub fn program_date_time(&mut self, value: ExtXProgramDateTime<'a>) -> &mut Self {
        self.program_date_time = Some(value);
        self
    }

    /// See [`MediaSegment::duration`].
    pub fn duration<V: Into<ExtInf<'a>>>(&mut self, value: V) -> &mut Self {
        self.duration = Some(value.into());
        self
    }

    /// See [`MediaSegment::uri`].
    pub fn uri<V: Into<Cow<'a, str>>>(&mut self, value: V) -> &mut Self {
        self.uri = Some(value.into());
        self
    }

    /// Builds a new [`MediaSegment`].
    ///
    /// # Errors
    ///
    /// If a required field has not been initialized.
    pub fn build(&self) -> Result<MediaSegment<'a>, Error> {
        Ok(MediaSegment {
            number: self.number.unwrap_or(0),
            explicit_number: self.explicit_number.unwrap_or(false),
            keys: self.keys.clone().unwrap_or_default(),
            map: self.map.clone(),
            byte_range: self.byte_range,
            date_range: self.date_range.clone(),
            has_discontinuity: self.has_discontinuity.unwrap_or(false),
            #[cfg(feature = "chrono")]
            program_date_time: self.program_date_time,
            #[cfg(not(feature = "chrono"))]
            program_date_time: self.program_date_time.clone(),
            duration: self
                .duration
                .clone()
                .ok_or_else(|| Error::missing_field("MediaSegment", "duration"))?,
            uri: self
                .uri
                .clone()
                .ok_or_else(|| Error::missing_field("MediaSegment", "uri"))?,
        })
    }
}

impl fmt::Display for MediaSegment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: self.keys will be printed by the `MediaPlaylist` to prevent redundance.

        if let Some(value) = &self.map {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.byte_range {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.date_range {
            writeln!(f, "{}", value)?;
        }

        if self.has_discontinuity {
            writeln!(f, "{}", ExtXDiscontinuity)?;
        }

        if let Some(value) = &self.program_date_time {
            writeln!(f, "{}", value)?;
        }

        writeln!(f, "{}", self.duration)?;
        writeln!(f, "{}", self.uri)?;
        Ok(())
    }
}

impl RequiredVersion for MediaSegment<'_> {
    fn required_version(&self) -> ProtocolVersion {
        required_version![
            self.keys,
            self.map,
            self.byte_range,
            self.date_range,
            {
                if self.has_discontinuity {
                    Some(ExtXDiscontinuity)
                } else {
                    None
                }
            },
            self.program_date_time,
            self.duration
        ]
    }
}

impl<'a> Decryptable<'a> for MediaSegment<'a> {
    fn keys(&self) -> Vec<&DecryptionKey<'a>> {
        //
        self.keys.iter().filter_map(ExtXKey::as_ref).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_display() {
        assert_eq!(
            MediaSegment::builder()
                .map(ExtXMap::new("https://www.example.com/"))
                .byte_range(ExtXByteRange::from(5..25))
                .has_discontinuity(true)
                .duration(ExtInf::new(Duration::from_secs(4)))
                .uri("http://www.uri.com/")
                .build()
                .unwrap()
                .to_string(),
            concat!(
                "#EXT-X-MAP:URI=\"https://www.example.com/\"\n",
                "#EXT-X-BYTERANGE:20@5\n",
                "#EXT-X-DISCONTINUITY\n",
                "#EXTINF:4,\n",
                "http://www.uri.com/\n"
            )
            .to_string()
        );
    }
}
