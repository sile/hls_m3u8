use std::fmt;
use std::str::FromStr;

use {Error, ErrorKind, Result};
use attribute::AttributePairs;
use types::{ClosedCaptions, DecimalFloatingPoint, DecimalResolution, DecryptionKey, HdcpLevel,
            InStreamId, MediaType, ProtocolVersion, QuotedString, SessionData, SingleLineString};
use super::{parse_yes_or_no, parse_u64};

/// [4.3.4.1. EXT-X-MEDIA]
///
/// [4.3.4.1. EXT-X-MEDIA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.1
///
/// TODO: Add `ExtXMediaBuilder`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXMedia {
    media_type: MediaType,
    uri: Option<QuotedString>,
    group_id: QuotedString,
    language: Option<QuotedString>,
    assoc_language: Option<QuotedString>,
    name: QuotedString,
    default: bool,
    autoselect: bool,
    forced: bool,
    instream_id: Option<InStreamId>,
    characteristics: Option<QuotedString>,
    channels: Option<QuotedString>,
}
impl ExtXMedia {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MEDIA:";

    /// Makes a new `ExtXMedia` tag.
    pub fn new(media_type: MediaType, group_id: QuotedString, name: QuotedString) -> Self {
        ExtXMedia {
            media_type,
            uri: None,
            group_id,
            language: None,
            assoc_language: None,
            name,
            default: false,
            autoselect: false,
            forced: false,
            instream_id: None,
            characteristics: None,
            channels: None,
        }
    }

    /// Returns the type of the media associated with this tag.
    pub fn media_type(&self) -> MediaType {
        self.media_type
    }

    /// Returns the identifier that specifies the group to which the rendition belongs.
    pub fn group_id(&self) -> &QuotedString {
        &self.group_id
    }

    /// Returns a human-readable description of the rendition.
    pub fn name(&self) -> &QuotedString {
        &self.name
    }

    /// Returns the URI that identifies the media playlist.
    pub fn uri(&self) -> Option<&QuotedString> {
        self.uri.as_ref()
    }

    /// Returns the name of the primary language used in the rendition.
    pub fn language(&self) -> Option<&QuotedString> {
        self.language.as_ref()
    }

    /// Returns the name of a language associated with the rendition.
    pub fn assoc_language(&self) -> Option<&QuotedString> {
        self.assoc_language.as_ref()
    }

    /// Returns whether this is the default rendition.
    pub fn default(&self) -> bool {
        self.default
    }

    /// Returns whether the client may choose to
    /// play this rendition in the absence of explicit user preference.
    pub fn autoselect(&self) -> bool {
        self.autoselect
    }

    /// Returns whether the rendition contains content that is considered essential to play.
    pub fn forced(&self) -> bool {
        self.forced
    }

    /// Returns the identifier that specifies a rendition within the segments in the media playlist.
    pub fn instream_id(&self) -> Option<InStreamId> {
        self.instream_id
    }

    /// Returns a string that represents uniform type identifiers (UTI).
    ///
    /// Each UTI indicates an individual characteristic of the rendition.
    pub fn characteristics(&self) -> Option<&QuotedString> {
        self.characteristics.as_ref()
    }

    /// Returns a string that represents the parameters of the rendition.
    pub fn channels(&self) -> Option<&QuotedString> {
        self.channels.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
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
        if let Some(ref x) = self.uri {
            write!(f, ",URI={}", x)?;
        }
        write!(f, ",GROUP-ID={}", self.group_id)?;
        if let Some(ref x) = self.language {
            write!(f, ",LANGUAGE={}", x)?;
        }
        if let Some(ref x) = self.assoc_language {
            write!(f, ",ASSOC-LANGUAGE={}", x)?;
        }
        write!(f, ",NAME={}", self.name)?;
        if self.default {
            write!(f, ",DEFAULT=YES")?;
        }
        if self.autoselect {
            write!(f, ",AUTOSELECT=YES")?;
        }
        if self.forced {
            write!(f, ",FORCED=YES")?;
        }
        if let Some(ref x) = self.instream_id {
            write!(f, ",INSTREAM-ID=\"{}\"", x)?;
        }
        if let Some(ref x) = self.characteristics {
            write!(f, ",CHARACTERISTICS={}", x)?;
        }
        if let Some(ref x) = self.channels {
            write!(f, ",CHANNELS={}", x)?;
        }
        Ok(())
    }
}
impl FromStr for ExtXMedia {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut media_type = None;
        let mut uri = None;
        let mut group_id = None;
        let mut language = None;
        let mut assoc_language = None;
        let mut name = None;
        let mut default = false;
        let mut autoselect = None;
        let mut forced = None;
        let mut instream_id = None;
        let mut characteristics = None;
        let mut channels = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "TYPE" => media_type = Some(track!(value.parse())?),
                "URI" => uri = Some(track!(value.parse())?),
                "GROUP-ID" => group_id = Some(track!(value.parse())?),
                "LANGUAGE" => language = Some(track!(value.parse())?),
                "ASSOC-LANGUAGE" => assoc_language = Some(track!(value.parse())?),
                "NAME" => name = Some(track!(value.parse())?),
                "DEFAULT" => default = track!(parse_yes_or_no(value))?,
                "AUTOSELECT" => autoselect = Some(track!(parse_yes_or_no(value))?),
                "FORCED" => forced = Some(track!(parse_yes_or_no(value))?),
                "INSTREAM-ID" => {
                    let s: QuotedString = track!(value.parse())?;
                    instream_id = Some(track!(s.parse())?);
                }
                "CHARACTERISTICS" => characteristics = Some(track!(value.parse())?),
                "CHANNELS" => channels = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }
        let media_type = track_assert_some!(media_type, ErrorKind::InvalidInput);
        let group_id = track_assert_some!(group_id, ErrorKind::InvalidInput);
        let name = track_assert_some!(name, ErrorKind::InvalidInput);
        if MediaType::ClosedCaptions == media_type {
            track_assert_ne!(uri, None, ErrorKind::InvalidInput);
            track_assert!(instream_id.is_some(), ErrorKind::InvalidInput);
        } else {
            track_assert!(instream_id.is_none(), ErrorKind::InvalidInput);
        }
        if default && autoselect.is_some() {
            track_assert_eq!(autoselect, Some(true), ErrorKind::InvalidInput);
        }
        if MediaType::Subtitles != media_type {
            track_assert_eq!(forced, None, ErrorKind::InvalidInput);
        }
        Ok(ExtXMedia {
            media_type,
            uri,
            group_id,
            language,
            assoc_language,
            name,
            default,
            autoselect: autoselect.unwrap_or(false),
            forced: forced.unwrap_or(false),
            instream_id,
            characteristics,
            channels,
        })
    }
}

/// [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXStreamInf {
    uri: SingleLineString,
    bandwidth: u64,
    average_bandwidth: Option<u64>,
    codecs: Option<QuotedString>,
    resolution: Option<DecimalResolution>,
    frame_rate: Option<DecimalFloatingPoint>,
    hdcp_level: Option<HdcpLevel>,
    audio: Option<QuotedString>,
    video: Option<QuotedString>,
    subtitles: Option<QuotedString>,
    closed_captions: Option<ClosedCaptions>,
}
impl ExtXStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-STREAM-INF:";

    /// Makes a new `ExtXStreamInf` tag.
    pub fn new(uri: SingleLineString, bandwidth: u64) -> Self {
        ExtXStreamInf {
            uri,
            bandwidth,
            average_bandwidth: None,
            codecs: None,
            resolution: None,
            frame_rate: None,
            hdcp_level: None,
            audio: None,
            video: None,
            subtitles: None,
            closed_captions: None,
        }
    }

    /// Returns the URI that identifies the associated media playlist.
    pub fn uri(&self) -> &SingleLineString {
        &self.uri
    }

    /// Returns the peak segment bit rate of the variant stream.
    pub fn bandwidth(&self) -> u64 {
        self.bandwidth
    }

    /// Returns the average segment bit rate of the variant stream.
    pub fn average_bandwidth(&self) -> Option<u64> {
        self.average_bandwidth
    }

    /// Returns a string that represents the list of codec types contained the variant stream.
    pub fn codecs(&self) -> Option<&QuotedString> {
        self.codecs.as_ref()
    }

    /// Returns the optimal pixel resolution at which to display all the video in the variant stream.
    pub fn resolution(&self) -> Option<DecimalResolution> {
        self.resolution
    }

    /// Returns the maximum frame rate for all the video in the variant stream.
    pub fn frame_rate(&self) -> Option<DecimalFloatingPoint> {
        self.frame_rate
    }

    /// Returns the HDCP level of the variant stream.
    pub fn hdcp_level(&self) -> Option<HdcpLevel> {
        self.hdcp_level
    }

    /// Returns the group identifier for the audio in the variant stream.
    pub fn audio(&self) -> Option<&QuotedString> {
        self.audio.as_ref()
    }

    /// Returns the group identifier for the video in the variant stream.
    pub fn video(&self) -> Option<&QuotedString> {
        self.video.as_ref()
    }

    /// Returns the group identifier for the subtitles in the variant stream.
    pub fn subtitles(&self) -> Option<&QuotedString> {
        self.subtitles.as_ref()
    }

    /// Returns the value of `CLOSED-CAPTIONS` attribute.
    pub fn closed_captions(&self) -> Option<&ClosedCaptions> {
        self.closed_captions.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}
impl fmt::Display for ExtXStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "BANDWIDTH={}", self.bandwidth)?;
        if let Some(ref x) = self.average_bandwidth {
            write!(f, ",AVERAGE-BANDWIDTH={}", x)?;
        }
        if let Some(ref x) = self.codecs {
            write!(f, ",CODECS={}", x)?;
        }
        if let Some(ref x) = self.resolution {
            write!(f, ",RESOLUTION={}", x)?;
        }
        if let Some(ref x) = self.frame_rate {
            write!(f, ",FRAME-RATE={:.3}", x.as_f64())?;
        }
        if let Some(ref x) = self.hdcp_level {
            write!(f, ",HDCP-LEVEL={}", x)?;
        }
        if let Some(ref x) = self.audio {
            write!(f, ",AUDIO={}", x)?;
        }
        if let Some(ref x) = self.video {
            write!(f, ",VIDEO={}", x)?;
        }
        if let Some(ref x) = self.subtitles {
            write!(f, ",SUBTITLES={}", x)?;
        }
        if let Some(ref x) = self.closed_captions {
            write!(f, ",CLOSED-CAPTIONS={}", x)?;
        }
        write!(f, "\n{}", self.uri)?;
        Ok(())
    }
}
impl FromStr for ExtXStreamInf {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.splitn(2, '\n');
        let first_line = lines.next().expect("Never fails").trim_right_matches('\r');
        let second_line = track_assert_some!(lines.next(), ErrorKind::InvalidInput);

        track_assert!(
            first_line.starts_with(Self::PREFIX),
            ErrorKind::InvalidInput
        );
        let uri = track!(SingleLineString::new(second_line))?;
        let mut bandwidth = None;
        let mut average_bandwidth = None;
        let mut codecs = None;
        let mut resolution = None;
        let mut frame_rate = None;
        let mut hdcp_level = None;
        let mut audio = None;
        let mut video = None;
        let mut subtitles = None;
        let mut closed_captions = None;
        let attrs = AttributePairs::parse(first_line.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "BANDWIDTH" => bandwidth = Some(track!(parse_u64(value))?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some(track!(parse_u64(value))?),
                "CODECS" => codecs = Some(track!(value.parse())?),
                "RESOLUTION" => resolution = Some(track!(value.parse())?),
                "FRAME-RATE" => frame_rate = Some(track!(value.parse())?),
                "HDCP-LEVEL" => hdcp_level = Some(track!(value.parse())?),
                "AUDIO" => audio = Some(track!(value.parse())?),
                "VIDEO" => video = Some(track!(value.parse())?),
                "SUBTITLES" => subtitles = Some(track!(value.parse())?),
                "CLOSED-CAPTIONS" => closed_captions = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }
        let bandwidth = track_assert_some!(bandwidth, ErrorKind::InvalidInput);
        Ok(ExtXStreamInf {
            uri,
            bandwidth,
            average_bandwidth,
            codecs,
            resolution,
            frame_rate,
            hdcp_level,
            audio,
            video,
            subtitles,
            closed_captions,
        })
    }
}

/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]
///
/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.3
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXIFrameStreamInf {
    uri: QuotedString,
    bandwidth: u64,
    average_bandwidth: Option<u64>,
    codecs: Option<QuotedString>,
    resolution: Option<DecimalResolution>,
    hdcp_level: Option<HdcpLevel>,
    video: Option<QuotedString>,
}
impl ExtXIFrameStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-I-FRAME-STREAM-INF:";

    /// Makes a new `ExtXIFrameStreamInf` tag.
    pub fn new(uri: QuotedString, bandwidth: u64) -> Self {
        ExtXIFrameStreamInf {
            uri,
            bandwidth,
            average_bandwidth: None,
            codecs: None,
            resolution: None,
            hdcp_level: None,
            video: None,
        }
    }

    /// Returns the URI that identifies the associated media playlist.
    pub fn uri(&self) -> &QuotedString {
        &self.uri
    }

    /// Returns the peak segment bit rate of the variant stream.
    pub fn bandwidth(&self) -> u64 {
        self.bandwidth
    }

    /// Returns the average segment bit rate of the variant stream.
    pub fn average_bandwidth(&self) -> Option<u64> {
        self.average_bandwidth
    }

    /// Returns a string that represents the list of codec types contained the variant stream.
    pub fn codecs(&self) -> Option<&QuotedString> {
        self.codecs.as_ref()
    }

    /// Returns the optimal pixel resolution at which to display all the video in the variant stream.
    pub fn resolution(&self) -> Option<DecimalResolution> {
        self.resolution
    }

    /// Returns the HDCP level of the variant stream.
    pub fn hdcp_level(&self) -> Option<HdcpLevel> {
        self.hdcp_level
    }

    /// Returns the group identifier for the video in the variant stream.
    pub fn video(&self) -> Option<&QuotedString> {
        self.video.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}
impl fmt::Display for ExtXIFrameStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "URI={}", self.uri)?;
        write!(f, ",BANDWIDTH={}", self.bandwidth)?;
        if let Some(ref x) = self.average_bandwidth {
            write!(f, ",AVERAGE-BANDWIDTH={}", x)?;
        }
        if let Some(ref x) = self.codecs {
            write!(f, ",CODECS={}", x)?;
        }
        if let Some(ref x) = self.resolution {
            write!(f, ",RESOLUTION={}", x)?;
        }
        if let Some(ref x) = self.hdcp_level {
            write!(f, ",HDCP-LEVEL={}", x)?;
        }
        if let Some(ref x) = self.video {
            write!(f, ",VIDEO={}", x)?;
        }
        Ok(())
    }
}
impl FromStr for ExtXIFrameStreamInf {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut uri = None;
        let mut bandwidth = None;
        let mut average_bandwidth = None;
        let mut codecs = None;
        let mut resolution = None;
        let mut hdcp_level = None;
        let mut video = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "URI" => uri = Some(track!(value.parse())?),
                "BANDWIDTH" => bandwidth = Some(track!(parse_u64(value))?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some(track!(parse_u64(value))?),
                "CODECS" => codecs = Some(track!(value.parse())?),
                "RESOLUTION" => resolution = Some(track!(value.parse())?),
                "HDCP-LEVEL" => hdcp_level = Some(track!(value.parse())?),
                "VIDEO" => video = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let uri = track_assert_some!(uri, ErrorKind::InvalidInput);
        let bandwidth = track_assert_some!(bandwidth, ErrorKind::InvalidInput);
        Ok(ExtXIFrameStreamInf {
            uri,
            bandwidth,
            average_bandwidth,
            codecs,
            resolution,
            hdcp_level,
            video,
        })
    }
}

/// [4.3.4.4. EXT-X-SESSION-DATA]
///
/// [4.3.4.4. EXT-X-SESSION-DATA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.4
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXSessionData {
    data_id: QuotedString,
    data: SessionData,
    language: Option<QuotedString>,
}
impl ExtXSessionData {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-DATA:";

    /// Makes a new `ExtXSessionData` tag.
    pub fn new(data_id: QuotedString, data: SessionData) -> Self {
        ExtXSessionData {
            data_id,
            data,
            language: None,
        }
    }

    /// Makes a new `ExtXSessionData` with the given language.
    pub fn with_language(data_id: QuotedString, data: SessionData, language: QuotedString) -> Self {
        ExtXSessionData {
            data_id,
            data,
            language: Some(language),
        }
    }

    /// Returns the identifier of the data.
    pub fn data_id(&self) -> &QuotedString {
        &self.data_id
    }

    /// Returns the session data.
    pub fn data(&self) -> &SessionData {
        &self.data
    }

    /// Returns the language of the data.
    pub fn language(&self) -> Option<&QuotedString> {
        self.language.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}
impl fmt::Display for ExtXSessionData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "DATA-ID={}", self.data_id)?;
        match self.data {
            SessionData::Value(ref x) => write!(f, ",VALUE={}", x)?,
            SessionData::Uri(ref x) => write!(f, ",URI={}", x)?,
        }
        if let Some(ref x) = self.language {
            write!(f, ",LANGUAGE={}", x)?;
        }
        Ok(())
    }
}
impl FromStr for ExtXSessionData {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut data_id = None;
        let mut session_value = None;
        let mut uri = None;
        let mut language = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "DATA-ID" => data_id = Some(track!(value.parse())?),
                "VALUE" => session_value = Some(track!(value.parse())?),
                "URI" => uri = Some(track!(value.parse())?),
                "LANGUAGE" => language = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let data_id = track_assert_some!(data_id, ErrorKind::InvalidInput);
        let data = if let Some(value) = session_value {
            track_assert_eq!(uri, None, ErrorKind::InvalidInput);
            SessionData::Value(value)
        } else if let Some(uri) = uri {
            SessionData::Uri(uri)
        } else {
            track_panic!(ErrorKind::InvalidInput);
        };
        Ok(ExtXSessionData {
            data_id,
            data,
            language,
        })
    }
}

/// [4.3.4.5. EXT-X-SESSION-KEY]
///
/// [4.3.4.5. EXT-X-SESSION-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.4.5
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXSessionKey {
    key: DecryptionKey,
}
impl ExtXSessionKey {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-KEY:";

    /// Makes a new `ExtXSessionKey` tag.
    pub fn new(key: DecryptionKey) -> Self {
        ExtXSessionKey { key }
    }

    /// Returns a decryption key for the playlist.
    pub fn key(&self) -> &DecryptionKey {
        &self.key
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        self.key.requires_version()
    }
}
impl fmt::Display for ExtXSessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.key)
    }
}
impl FromStr for ExtXSessionKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let suffix = s.split_at(Self::PREFIX.len()).1;
        let key = track!(suffix.parse())?;
        Ok(ExtXSessionKey { key })
    }
}

#[cfg(test)]
mod test {
    use types::{EncryptionMethod, InitializationVector};
    use super::*;

    #[test]
    fn ext_x_media() {
        let tag = ExtXMedia::new(MediaType::Audio, quoted_string("foo"), quoted_string("bar"));
        let text = r#"#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID="foo",NAME="bar""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    #[test]
    fn ext_x_stream_inf() {
        let tag = ExtXStreamInf::new(SingleLineString::new("foo").unwrap(), 1000);
        let text = "#EXT-X-STREAM-INF:BANDWIDTH=1000\nfoo";
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    #[test]
    fn ext_x_i_frame_stream_inf() {
        let tag = ExtXIFrameStreamInf::new(quoted_string("foo"), 1000);
        let text = r#"#EXT-X-I-FRAME-STREAM-INF:URI="foo",BANDWIDTH=1000"#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    #[test]
    fn ext_x_session_data() {
        let tag = ExtXSessionData::new(
            quoted_string("foo"),
            SessionData::Value(quoted_string("bar")),
        );
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",VALUE="bar""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);

        let tag =
            ExtXSessionData::new(quoted_string("foo"), SessionData::Uri(quoted_string("bar")));
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",URI="bar""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);

        let tag = ExtXSessionData::with_language(
            quoted_string("foo"),
            SessionData::Value(quoted_string("bar")),
            quoted_string("baz"),
        );
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",VALUE="bar",LANGUAGE="baz""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    #[test]
    fn ext_x_session_key() {
        let tag = ExtXSessionKey::new(DecryptionKey {
            method: EncryptionMethod::Aes128,
            uri: quoted_string("foo"),
            iv: Some(InitializationVector([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
            ])),
            key_format: None,
            key_format_versions: None,
        });
        let text =
            r#"#EXT-X-SESSION-KEY:METHOD=AES-128,URI="foo",IV=0x000102030405060708090a0b0c0d0e0f"#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V2);
    }

    fn quoted_string(s: &str) -> QuotedString {
        QuotedString::new(s).unwrap()
    }
}
