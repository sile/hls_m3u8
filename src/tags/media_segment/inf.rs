use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// # [4.3.2.1. EXTINF]
///
/// The [`ExtInf`] tag specifies the duration of a [`Media Segment`]. It applies
/// only to the next [`Media Segment`].
///
/// [`Media Segment`]: crate::media_segment::MediaSegment
/// [4.3.2.1. EXTINF]: https://tools.ietf.org/html/rfc8216#section-4.3.2.1
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtInf {
    duration: Duration,
    title: Option<String>,
}

impl ExtInf {
    pub(crate) const PREFIX: &'static str = "#EXTINF:";

    /// Makes a new [`ExtInf`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtInf;
    /// use std::time::Duration;
    ///
    /// let ext_inf = ExtInf::new(Duration::from_secs(5));
    /// ```
    pub const fn new(duration: Duration) -> Self {
        Self {
            duration,
            title: None,
        }
    }

    /// Makes a new [`ExtInf`] tag with the given title.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtInf;
    /// use std::time::Duration;
    ///
    /// let ext_inf = ExtInf::with_title(Duration::from_secs(5), "title");
    /// ```
    pub fn with_title<T: ToString>(duration: Duration, title: T) -> Self {
        Self {
            duration,
            title: Some(title.to_string()),
        }
    }

    /// Returns the duration of the associated media segment.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtInf;
    /// use std::time::Duration;
    ///
    /// let ext_inf = ExtInf::new(Duration::from_secs(5));
    ///
    /// assert_eq!(ext_inf.duration(), Duration::from_secs(5));
    /// ```
    pub const fn duration(&self) -> Duration { self.duration }

    /// Sets the duration of the associated media segment.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtInf;
    /// use std::time::Duration;
    ///
    /// let mut ext_inf = ExtInf::new(Duration::from_secs(5));
    ///
    /// ext_inf.set_duration(Duration::from_secs(10));
    ///
    /// assert_eq!(ext_inf.duration(), Duration::from_secs(10));
    /// ```
    pub fn set_duration(&mut self, value: Duration) -> &mut Self {
        self.duration = value;
        self
    }

    /// Returns the title of the associated media segment.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtInf;
    /// use std::time::Duration;
    ///
    /// let ext_inf = ExtInf::with_title(Duration::from_secs(5), "title");
    ///
    /// assert_eq!(ext_inf.title(), &Some("title".to_string()));
    /// ```
    pub const fn title(&self) -> &Option<String> { &self.title }

    /// Sets the title of the associated media segment.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtInf;
    /// use std::time::Duration;
    ///
    /// let mut ext_inf = ExtInf::with_title(Duration::from_secs(5), "title");
    ///
    /// ext_inf.set_title(Some("better title"));
    ///
    /// assert_eq!(ext_inf.title(), &Some("better title".to_string()));
    /// ```
    pub fn set_title<T: ToString>(&mut self, value: Option<T>) -> &mut Self {
        self.title = value.map(|v| v.to_string());
        self
    }
}

/// This tag requires [`ProtocolVersion::V1`], if the duration does not have
/// nanoseconds, otherwise it requires [`ProtocolVersion::V3`].
impl RequiredVersion for ExtInf {
    fn required_version(&self) -> ProtocolVersion {
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
        write!(f, "{},", self.duration.as_secs_f64())?;

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
        let tokens = input.splitn(2, ',').collect::<Vec<_>>();

        if tokens.is_empty() {
            return Err(Error::custom(format!(
                "failed to parse #EXTINF tag, couldn't split input: {:?}",
                input
            )));
        }

        let duration = Duration::from_secs_f64(tokens[0].parse()?);

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

        Ok(Self { duration, title })
    }
}

impl From<Duration> for ExtInf {
    fn from(value: Duration) -> Self { Self::new(value) }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

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

        assert!("#EXTINF:".parse::<ExtInf>().is_err());
        assert!("#EXTINF:garbage".parse::<ExtInf>().is_err());
    }

    #[test]
    fn test_title() {
        assert_eq!(ExtInf::new(Duration::from_secs(5)).title(), &None);
        assert_eq!(
            ExtInf::with_title(Duration::from_secs(5), "title").title(),
            &Some("title".to_string())
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtInf::new(Duration::from_secs(4)).required_version(),
            ProtocolVersion::V1
        );
        assert_eq!(
            ExtInf::new(Duration::from_millis(4400)).required_version(),
            ProtocolVersion::V3
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(
            ExtInf::from(Duration::from_secs(1)),
            ExtInf::new(Duration::from_secs(1))
        );
    }
}
