use crate::types::{DecimalFloatingPoint, ProtocolVersion, SingleLineString};
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;
use std::time::Duration;
use trackable::error::ErrorKindExt;

/// [4.3.2.1. EXTINF]
///
/// [4.3.2.1. EXTINF]: https://tools.ietf.org/html/rfc8216#section-4.3.2.1
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtInf {
    duration: Duration,
    title: Option<SingleLineString>,
}

impl ExtInf {
    pub(crate) const PREFIX: &'static str = "#EXTINF:";

    /// Makes a new `ExtInf` tag.
    pub fn new(duration: Duration) -> Self {
        ExtInf {
            duration,
            title: None,
        }
    }

    /// Makes a new `ExtInf` tag with the given title.
    pub fn with_title(duration: Duration, title: SingleLineString) -> Self {
        ExtInf {
            duration,
            title: Some(title),
        }
    }

    /// Returns the duration of the associated media segment.
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the title of the associated media segment.
    pub fn title(&self) -> Option<&SingleLineString> {
        self.title.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        if self.duration.subsec_nanos() == 0 {
            ProtocolVersion::V1
        } else {
            ProtocolVersion::V3
        }
    }
}

impl fmt::Display for ExtInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;

        let duration = (self.duration.as_secs() as f64)
            + (f64::from(self.duration.subsec_nanos()) / 1_000_000_000.0);
        write!(f, "{}", duration)?;

        if let Some(ref title) = self.title {
            write!(f, ",{}", title)?;
        }
        Ok(())
    }
}

impl FromStr for ExtInf {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let mut tokens = s.split_at(Self::PREFIX.len()).1.splitn(2, ',');

        let seconds: DecimalFloatingPoint =
            may_invalid!(tokens.next().expect("Never fails").parse())?;
        let duration = seconds.to_duration();

        let title = if let Some(title) = tokens.next() {
            Some(track!(SingleLineString::new(title))?)
        } else {
            None
        };
        Ok(ExtInf { duration, title })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extinf() {
        let tag = ExtInf::new(Duration::from_secs(5));
        assert_eq!("#EXTINF:5".parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), "#EXTINF:5");
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);

        let tag = ExtInf::with_title(
            Duration::from_secs(5),
            SingleLineString::new("foo").unwrap(),
        );
        assert_eq!("#EXTINF:5,foo".parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), "#EXTINF:5,foo");
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);

        let tag = ExtInf::new(Duration::from_millis(1234));
        assert_eq!("#EXTINF:1.234".parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), "#EXTINF:1.234");
        assert_eq!(tag.requires_version(), ProtocolVersion::V3);
    }
}
