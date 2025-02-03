use core::convert::TryFrom;
use core::fmt;
use std::borrow::Cow;

use derive_builder::Builder;
use shorthand::ShortHand;

use crate::attribute::AttributePairs;
use crate::types::{Codecs, HdcpLevel, ProtocolVersion, Resolution};
use crate::utils::{quote, unquote};
use crate::{Error, RequiredVersion};

/// The [`StreamData`] struct contains the data that is shared between both
/// variants of the [`VariantStream`].
///
/// [`VariantStream`]: crate::tags::VariantStream
#[derive(ShortHand, Builder, PartialOrd, Debug, Clone, PartialEq, Eq, Hash, Ord)]
#[builder(setter(strip_option))]
#[builder(derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash))]
#[shorthand(enable(must_use, into))]
pub struct StreamData<'a> {
    /// The peak segment bitrate of the [`VariantStream`] in bits per second.
    ///
    /// If all the [`MediaSegment`]s in a [`VariantStream`] have already been
    /// created, the bandwidth value must be the largest sum of peak segment
    /// bitrates that is produced by any playable combination of renditions.
    ///
    /// (For a [`VariantStream`] with a single [`MediaPlaylist`], this is just
    /// the peak segment bit rate of that [`MediaPlaylist`].)
    ///
    /// An inaccurate value can cause playback stalls or prevent clients from
    /// playing the variant. If the [`MasterPlaylist`] is to be made available
    /// before all [`MediaSegment`]s in the presentation have been encoded, the
    /// bandwidth value should be the bandwidth value of a representative
    /// period of similar content, encoded using the same settings.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamData;
    /// #
    /// let mut stream = StreamData::new(20);
    ///
    /// stream.set_bandwidth(5);
    /// assert_eq!(stream.bandwidth(), 5);
    /// ```
    ///
    /// # Note
    ///
    /// This field is required.
    ///
    /// [`VariantStream`]: crate::tags::VariantStream
    /// [`MediaSegment`]: crate::MediaSegment
    /// [`MasterPlaylist`]: crate::MasterPlaylist
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    #[shorthand(disable(into))]
    bandwidth: u64,
    /// The average bandwidth of the stream in bits per second.
    ///
    /// It represents the  average segment bitrate of the [`VariantStream`]. If
    /// all the [`MediaSegment`]s in a [`VariantStream`] have already been
    /// created, the average bandwidth must be the largest sum of average
    /// segment bitrates that is produced by any playable combination of
    /// renditions.
    ///
    /// (For a [`VariantStream`] with a single [`MediaPlaylist`], this is just
    /// the average segment bitrate of that [`MediaPlaylist`].)
    ///
    /// An inaccurate value can cause playback stalls or prevent clients from
    /// playing the variant. If the [`MasterPlaylist`] is to be made available
    /// before all [`MediaSegment`]s in the presentation have been encoded, the
    /// average bandwidth should be the average bandwidth of a representative
    /// period of similar content, encoded using the same settings.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamData;
    /// #
    /// let mut stream = StreamData::new(20);
    ///
    /// stream.set_average_bandwidth(Some(300));
    /// assert_eq!(stream.average_bandwidth(), Some(300));
    /// ```
    ///
    /// # Note
    ///
    /// This field is optional.
    ///
    /// [`MediaSegment`]: crate::MediaSegment
    /// [`MasterPlaylist`]: crate::MasterPlaylist
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    /// [`VariantStream`]: crate::tags::VariantStream
    #[builder(default)]
    #[shorthand(enable(copy), disable(into, option_as_ref))]
    average_bandwidth: Option<u64>,
    /// A list of formats, where each format specifies a media sample type that
    /// is present in one or more renditions specified by the [`VariantStream`].
    ///
    /// Valid format identifiers are those in the ISO Base Media File Format
    /// Name Space defined by "The 'Codecs' and 'Profiles' Parameters for
    /// "Bucket" Media Types" ([RFC6381]).
    ///
    /// For example, a stream containing AAC low complexity (AAC-LC) audio and
    /// H.264 Main Profile Level 3.0 video would be
    ///
    /// ```
    /// # use hls_m3u8::types::Codecs;
    /// let codecs = Codecs::from(&["mp4a.40.2", "avc1.4d401e"]);
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamData;
    /// use hls_m3u8::types::Codecs;
    ///
    /// let mut stream = StreamData::new(20);
    ///
    /// stream.set_codecs(Some(&["mp4a.40.2", "avc1.4d401e"]));
    /// assert_eq!(
    ///     stream.codecs(),
    ///     Some(&Codecs::from(&["mp4a.40.2", "avc1.4d401e"]))
    /// );
    /// ```
    ///
    /// # Note
    ///
    /// This field is optional, but every instance of
    /// [`VariantStream::ExtXStreamInf`] should include a codecs attribute.
    ///
    /// [`VariantStream`]: crate::tags::VariantStream
    /// [`VariantStream::ExtXStreamInf`]:
    /// crate::tags::VariantStream::ExtXStreamInf
    /// [RFC6381]: https://tools.ietf.org/html/rfc6381
    #[builder(default, setter(into))]
    codecs: Option<Codecs<'a>>,
    /// The resolution of the stream.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamData;
    /// use hls_m3u8::types::Resolution;
    ///
    /// let mut stream = StreamData::new(20);
    ///
    /// stream.set_resolution(Some((1920, 1080)));
    /// assert_eq!(stream.resolution(), Some(Resolution::new(1920, 1080)));
    /// # stream.set_resolution(Some((1280, 10)));
    /// # assert_eq!(stream.resolution(), Some(Resolution::new(1280, 10)));
    /// ```
    ///
    /// # Note
    ///
    /// This field is optional, but it is recommended if the [`VariantStream`]
    /// includes video.
    ///
    /// [`VariantStream`]: crate::tags::VariantStream
    #[builder(default, setter(into))]
    #[shorthand(enable(copy))]
    resolution: Option<Resolution>,
    /// High-bandwidth Digital Content Protection level of the
    /// [`VariantStream`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamData;
    /// use hls_m3u8::types::HdcpLevel;
    /// #
    /// let mut stream = StreamData::new(20);
    ///
    /// stream.set_hdcp_level(Some(HdcpLevel::None));
    /// assert_eq!(stream.hdcp_level(), Some(HdcpLevel::None));
    /// ```
    ///
    /// # Note
    ///
    /// This field is optional.
    ///
    /// [`VariantStream`]: crate::tags::VariantStream
    #[builder(default)]
    #[shorthand(enable(copy), disable(into))]
    hdcp_level: Option<HdcpLevel>,
    /// It indicates the set of video renditions, that should be used when
    /// playing the presentation.
    ///
    /// It must match the value of the [`ExtXMedia::group_id`] attribute
    /// [`ExtXMedia`] tag elsewhere in the [`MasterPlaylist`] whose
    /// [`ExtXMedia::media_type`] attribute is video. It indicates the set of
    /// video renditions that should be used when playing the presentation.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamData;
    /// #
    /// let mut stream = StreamData::new(20);
    ///
    /// stream.set_video(Some("video_01"));
    /// assert_eq!(stream.video(), Some(&"video_01".into()));
    /// ```
    ///
    /// # Note
    ///
    /// This field is optional.
    ///
    /// [`ExtXMedia::group_id`]: crate::tags::ExtXMedia::group_id
    /// [`ExtXMedia`]: crate::tags::ExtXMedia
    /// [`MasterPlaylist`]: crate::MasterPlaylist
    /// [`ExtXMedia::media_type`]: crate::tags::ExtXMedia::media_type
    #[builder(default, setter(into))]
    video: Option<Cow<'a, str>>,
}

impl<'a> StreamData<'a> {
    /// Creates a new [`StreamData`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::StreamData;
    /// #
    /// let stream = StreamData::new(20);
    /// ```
    #[must_use]
    pub const fn new(bandwidth: u64) -> Self {
        Self {
            bandwidth,
            average_bandwidth: None,
            codecs: None,
            resolution: None,
            hdcp_level: None,
            video: None,
        }
    }

    /// Returns a builder for [`StreamData`].
    ///
    /// # Example
    ///
    /// ```
    /// use hls_m3u8::types::{HdcpLevel, StreamData};
    ///
    /// StreamData::builder()
    ///     .bandwidth(200)
    ///     .average_bandwidth(15)
    ///     .codecs(&["mp4a.40.2", "avc1.4d401e"])
    ///     .resolution((1920, 1080))
    ///     .hdcp_level(HdcpLevel::Type0)
    ///     .video("video_01")
    ///     .build()?;
    /// # Ok::<(), Box<dyn ::std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn builder() -> StreamDataBuilder<'a> {
        StreamDataBuilder::default()
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> StreamData<'static> {
        StreamData {
            bandwidth: self.bandwidth,
            average_bandwidth: self.average_bandwidth,
            codecs: self.codecs.map(Codecs::into_owned),
            resolution: self.resolution,
            hdcp_level: self.hdcp_level,
            video: self.video.map(|v| Cow::Owned(v.into_owned())),
        }
    }
}

impl fmt::Display for StreamData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BANDWIDTH={}", self.bandwidth)?;

        if let Some(value) = &self.average_bandwidth {
            write!(f, ",AVERAGE-BANDWIDTH={}", value)?;
        }
        if let Some(value) = &self.codecs {
            write!(f, ",CODECS={}", quote(value))?;
        }
        if let Some(value) = &self.resolution {
            write!(f, ",RESOLUTION={}", value)?;
        }
        if let Some(value) = &self.hdcp_level {
            write!(f, ",HDCP-LEVEL={}", value)?;
        }
        if let Some(value) = &self.video {
            write!(f, ",VIDEO={}", quote(value))?;
        }
        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for StreamData<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut bandwidth = None;
        let mut average_bandwidth = None;
        let mut codecs = None;
        let mut resolution = None;
        let mut hdcp_level = None;
        let mut video = None;

        for (key, value) in AttributePairs::new(input) {
            match key {
                "BANDWIDTH" => {
                    bandwidth = Some(
                        value
                            .parse::<u64>()
                            .map_err(|e| Error::parse_int(value, e))?,
                    );
                }
                "AVERAGE-BANDWIDTH" => {
                    average_bandwidth = Some(
                        value
                            .parse::<u64>()
                            .map_err(|e| Error::parse_int(value, e))?,
                    );
                }
                "CODECS" => codecs = Some(TryFrom::try_from(unquote(value))?),
                "RESOLUTION" => resolution = Some(value.parse()?),
                "HDCP-LEVEL" => {
                    hdcp_level = Some(value.parse::<HdcpLevel>().map_err(Error::strum)?);
                }
                "VIDEO" => video = Some(unquote(value)),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized
                    // AttributeName.
                }
            }
        }

        let bandwidth = bandwidth.ok_or_else(|| Error::missing_value("BANDWIDTH"))?;

        Ok(Self {
            bandwidth,
            average_bandwidth,
            codecs,
            resolution,
            hdcp_level,
            video,
        })
    }
}

/// This struct requires [`ProtocolVersion::V1`].
impl RequiredVersion for StreamData<'_> {
    fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }

    fn introduced_version(&self) -> ProtocolVersion {
        if self.video.is_some() {
            ProtocolVersion::V4
        } else {
            ProtocolVersion::V1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        let mut stream_data = StreamData::new(200);
        stream_data.set_average_bandwidth(Some(15));
        stream_data.set_codecs(Some(&["mp4a.40.2", "avc1.4d401e"]));
        stream_data.set_resolution(Some((1920, 1080)));
        stream_data.set_hdcp_level(Some(HdcpLevel::Type0));
        stream_data.set_video(Some("video"));

        assert_eq!(
            stream_data.to_string(),
            concat!(
                "BANDWIDTH=200,",
                "AVERAGE-BANDWIDTH=15,",
                "CODECS=\"mp4a.40.2,avc1.4d401e\",",
                "RESOLUTION=1920x1080,",
                "HDCP-LEVEL=TYPE-0,",
                "VIDEO=\"video\""
            )
            .to_string()
        );
    }

    #[test]
    fn test_parser() {
        let mut stream_data = StreamData::new(200);
        stream_data.set_average_bandwidth(Some(15));
        stream_data.set_codecs(Some(&["mp4a.40.2", "avc1.4d401e"]));
        stream_data.set_resolution(Some((1920, 1080)));
        stream_data.set_hdcp_level(Some(HdcpLevel::Type0));
        stream_data.set_video(Some("video"));

        assert_eq!(
            stream_data,
            StreamData::try_from(concat!(
                "BANDWIDTH=200,",
                "AVERAGE-BANDWIDTH=15,",
                "CODECS=\"mp4a.40.2,avc1.4d401e\",",
                "RESOLUTION=1920x1080,",
                "HDCP-LEVEL=TYPE-0,",
                "VIDEO=\"video\""
            ))
            .unwrap()
        );

        assert!(StreamData::try_from("garbage").is_err());
    }
}
