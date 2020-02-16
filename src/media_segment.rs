use std::fmt;

use derive_builder::Builder;
use shorthand::ShortHand;

use crate::tags::{
    ExtInf, ExtXByteRange, ExtXDateRange, ExtXDiscontinuity, ExtXKey, ExtXMap, ExtXProgramDateTime,
};
use crate::types::ProtocolVersion;
use crate::{Encrypted, RequiredVersion};

/// Media segment.
#[derive(ShortHand, Debug, Clone, Builder, PartialEq, PartialOrd)]
#[builder(setter(into, strip_option))]
#[shorthand(enable(must_use, get_mut, collection_magic))]
pub struct MediaSegment {
    /// All [`ExtXKey`] tags.
    #[builder(default)]
    keys: Vec<ExtXKey>,
    /// The [`ExtXMap`] tag associated with the media segment.
    #[builder(default)]
    map: Option<ExtXMap>,
    /// The [`ExtXByteRange`] tag associated with the [`MediaSegment`].
    #[builder(default)]
    byte_range: Option<ExtXByteRange>,
    /// The [`ExtXDateRange`] tag associated with the media segment.
    #[builder(default)]
    date_range: Option<ExtXDateRange>,
    /// The [`ExtXDiscontinuity`] tag associated with the media segment.
    #[builder(default)]
    discontinuity: Option<ExtXDiscontinuity>,
    /// The [`ExtXProgramDateTime`] tag associated with the media
    /// segment.
    #[builder(default)]
    program_date_time: Option<ExtXProgramDateTime>,
    /// The [`ExtInf`] tag associated with the [`MediaSegment`].
    inf: ExtInf,
    /// The `URI` of the [`MediaSegment`].
    #[shorthand(enable(into))]
    uri: String,
}

impl MediaSegment {
    /// Returns a builder for a [`MediaSegment`].
    pub fn builder() -> MediaSegmentBuilder { MediaSegmentBuilder::default() }
}

impl MediaSegmentBuilder {
    /// Pushes an [`ExtXKey`] tag.
    pub fn push_key<VALUE: Into<ExtXKey>>(&mut self, value: VALUE) -> &mut Self {
        if let Some(keys) = &mut self.keys {
            keys.push(value.into());
        } else {
            self.keys = Some(vec![value.into()]);
        }

        self
    }
}

impl fmt::Display for MediaSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.keys will be printed by MediaPlaylist!

        if let Some(value) = &self.map {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.byte_range {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.date_range {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.discontinuity {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.program_date_time {
            writeln!(f, "{}", value)?;
        }

        writeln!(f, "{}", self.inf)?; // TODO: there might be a `,` missing
        writeln!(f, "{}", self.uri)?;
        Ok(())
    }
}

impl RequiredVersion for MediaSegment {
    fn required_version(&self) -> ProtocolVersion {
        required_version![
            self.keys,
            self.map,
            self.byte_range,
            self.date_range,
            self.discontinuity,
            self.program_date_time,
            self.inf
        ]
    }
}

impl Encrypted for MediaSegment {
    fn keys(&self) -> &Vec<ExtXKey> { &self.keys }

    fn keys_mut(&mut self) -> &mut Vec<ExtXKey> { &mut self.keys }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[test]
    fn test_display() {
        assert_eq!(
            MediaSegment::builder()
                //.keys(vec![ExtXKey::empty()])
                .map(ExtXMap::new("https://www.example.com/"))
                .byte_range(ExtXByteRange::new(20, Some(5)))
                //.date_range() // TODO!
                .discontinuity(ExtXDiscontinuity)
                .inf(ExtInf::new(Duration::from_secs(4)))
                .uri("http://www.uri.com/")
                .build()
                .unwrap()
                .to_string(),
            concat!(
                //"#EXT-X-KEY:METHOD=NONE\n",
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
