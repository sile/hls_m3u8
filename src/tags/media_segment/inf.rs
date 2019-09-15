use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use crate::types::{DecimalFloatingPoint, ProtocolVersion};
use crate::utils::tag;
use crate::Error;

/// [4.3.2.1. EXTINF](https://tools.ietf.org/html/rfc8216#section-4.3.2.1)
///
/// The [ExtInf] tag specifies the duration of a [Media Segment].  It applies
/// only to the next [Media Segment]. This tag is REQUIRED for each [Media Segment].
///
/// Its format is:
/// ```text
/// #EXTINF:<duration>,[<title>]
/// ```
/// The title is an optional informative title about the [Media Segment].
///
/// [Media Segment]: crate::media_segment::MediaSegment
///
/// # Examples
/// Parsing from a String:
/// ```
/// use std::time::Duration;
/// use hls_m3u8::tags::ExtInf;
///
/// let ext_inf = "#EXTINF:8,".parse::<ExtInf>().expect("Failed to parse tag!");
///
/// assert_eq!(ext_inf.duration(), Duration::from_secs(8));
/// assert_eq!(ext_inf.title(), None);
/// ```
///
/// Converting to a String:
/// ```
/// use std::time::Duration;
/// use hls_m3u8::tags::ExtInf;
///
/// let ext_inf = ExtInf::with_title(
///     Duration::from_millis(88),
///     "title"
/// );
///
/// assert_eq!(ext_inf.duration(), Duration::from_millis(88));
/// assert_eq!(ext_inf.to_string(), "#EXTINF:0.088,title".to_string());
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtInf {
    duration: Duration,
    title: Option<String>,
}

impl ExtInf {
    pub(crate) const PREFIX: &'static str = "#EXTINF:";

    /// Makes a new `ExtInf` tag.
    pub const fn new(duration: Duration) -> Self {
        ExtInf {
            duration,
            title: None,
        }
    }

    /// Makes a new `ExtInf` tag with the given title.
    pub fn with_title<T: ToString>(duration: Duration, title: T) -> Self {
        ExtInf {
            duration,
            title: Some(title.to_string()),
        }
    }

    /// Returns the duration of the associated media segment.
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the title of the associated media segment.
    pub fn title(&self) -> Option<&String> {
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
        write!(f, "{},", duration)?;

        if let Some(value) = &self.title {
            write!(f, "{}", value)?;
        }
        Ok(())
    }
}

impl FromStr for ExtInf {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;
        dbg!(&input);
        let tokens = input.splitn(2, ',').collect::<Vec<_>>();

        if tokens.len() == 0 {
            return Err(Error::custom(format!(
                "failed to parse #EXTINF tag, couldn't split input: {:?}",
                input
            )));
        }

        let duration = tokens[0].parse::<DecimalFloatingPoint>()?.to_duration();

        let title = {
            if tokens.len() >= 2 {
                if tokens[1].trim().is_empty() {
                    None
                } else {
                    Some(tokens[1].to_string())
                }
            } else {
                None
            }
        };

        Ok(ExtInf { duration, title })
    }
}

impl From<Duration> for ExtInf {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            "#EXTINF:5,".to_string(),
            ExtInf::new(Duration::from_secs(5)).to_string()
        );
        assert_eq!(
            "#EXTINF:5.5,".to_string(),
            ExtInf::new(Duration::from_millis(5500)).to_string()
        );
        assert_eq!(
            "#EXTINF:5.5,title".to_string(),
            ExtInf::with_title(Duration::from_millis(5500), "title").to_string()
        );
        assert_eq!(
            "#EXTINF:5,title".to_string(),
            ExtInf::with_title(Duration::from_secs(5), "title").to_string()
        );
    }

    #[test]
    fn test_parser() {
        // #EXTINF:<duration>,[<title>]
        assert_eq!(
            "#EXTINF:5".parse::<ExtInf>().unwrap(),
            ExtInf::new(Duration::from_secs(5))
        );
        assert_eq!(
            "#EXTINF:5,".parse::<ExtInf>().unwrap(),
            ExtInf::new(Duration::from_secs(5))
        );
        assert_eq!(
            "#EXTINF:5.5".parse::<ExtInf>().unwrap(),
            ExtInf::new(Duration::from_millis(5500))
        );
        assert_eq!(
            "#EXTINF:5.5,".parse::<ExtInf>().unwrap(),
            ExtInf::new(Duration::from_millis(5500))
        );
        assert_eq!(
            "#EXTINF:5.5,title".parse::<ExtInf>().unwrap(),
            ExtInf::with_title(Duration::from_millis(5500), "title")
        );
        assert_eq!(
            "#EXTINF:5,title".parse::<ExtInf>().unwrap(),
            ExtInf::with_title(Duration::from_secs(5), "title")
        );
    }

    #[test]
    fn test_title() {
        assert_eq!(ExtInf::new(Duration::from_secs(5)).title(), None);
        assert_eq!(
            ExtInf::with_title(Duration::from_secs(5), "title").title(),
            Some(&"title".to_string())
        );
    }

    #[test]
    fn test_requires_version() {
        assert_eq!(
            ExtInf::new(Duration::from_secs(4)).requires_version(),
            ProtocolVersion::V1
        );
        assert_eq!(
            ExtInf::new(Duration::from_millis(4400)).requires_version(),
            ProtocolVersion::V3
        );
    }
}
