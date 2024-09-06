use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;
use std::time::Duration;

use derive_more::AsRef;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Specifies the duration of a [`Media Segment`].
///
/// [`Media Segment`]: crate::media_segment::MediaSegment
#[derive(AsRef, Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtInf<'a> {
    #[as_ref]
    duration: Duration,
    title: Option<Cow<'a, str>>,
}

impl<'a> ExtInf<'a> {
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
    #[must_use]
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
    #[must_use]
    pub fn with_title<T: Into<Cow<'a, str>>>(duration: Duration, title: T) -> Self {
        Self {
            duration,
            title: Some(title.into()),
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
    #[must_use]
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
    /// assert_eq!(ext_inf.title(), &Some("title".into()));
    /// ```
    #[must_use]
    pub const fn title(&self) -> &Option<Cow<'a, str>> { &self.title }

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
    /// assert_eq!(ext_inf.title(), &Some("better title".into()));
    /// ```
    pub fn set_title<T: Into<Cow<'a, str>>>(&mut self, value: Option<T>) -> &mut Self {
        self.title = value.map(Into::into);
        self
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> ExtInf<'static> {
        ExtInf {
            duration: self.duration,
            title: self.title.map(|v| Cow::Owned(v.into_owned())),
        }
    }
}

/// This tag requires [`ProtocolVersion::V1`], if the duration does not have
/// nanoseconds, otherwise it requires [`ProtocolVersion::V3`].
impl<'a> RequiredVersion for ExtInf<'a> {
    fn required_version(&self) -> ProtocolVersion {
        if self.duration.subsec_nanos() == 0 {
            ProtocolVersion::V1
        } else {
            ProtocolVersion::V3
        }
    }
}

impl<'a> fmt::Display for ExtInf<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "{},", self.duration.as_secs_f64())?;

        if let Some(value) = &self.title {
            write!(f, "{}", value)?;
        }
        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for ExtInf<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut input = tag(input, Self::PREFIX)?.splitn(2, ',');

        let duration = input.next().unwrap();
        let duration = Duration::from_secs_f64(
            duration
                .parse()
                .map_err(|e| Error::parse_float(duration, e))?,
        );

        let title = input
            .next()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(Cow::Borrowed);

        Ok(Self { duration, title })
    }
}

impl<'a> From<Duration> for ExtInf<'a> {
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
            ExtInf::try_from("#EXTINF:5").unwrap(),
            ExtInf::new(Duration::from_secs(5))
        );
        assert_eq!(
            ExtInf::try_from("#EXTINF:5,").unwrap(),
            ExtInf::new(Duration::from_secs(5))
        );
        assert_eq!(
            ExtInf::try_from("#EXTINF:5.5").unwrap(),
            ExtInf::new(Duration::from_millis(5500))
        );
        assert_eq!(
            ExtInf::try_from("#EXTINF:5.5,").unwrap(),
            ExtInf::new(Duration::from_millis(5500))
        );
        assert_eq!(
            ExtInf::try_from("#EXTINF:5.5,title").unwrap(),
            ExtInf::with_title(Duration::from_millis(5500), "title")
        );
        assert_eq!(
            ExtInf::try_from("#EXTINF:5,title").unwrap(),
            ExtInf::with_title(Duration::from_secs(5), "title")
        );

        assert!(ExtInf::try_from("#EXTINF:").is_err());
        assert!(ExtInf::try_from("#EXTINF:garbage").is_err());
    }

    #[test]
    fn test_title() {
        assert_eq!(ExtInf::new(Duration::from_secs(5)).title(), &None);
        assert_eq!(
            ExtInf::with_title(Duration::from_secs(5), "title").title(),
            &Some("title".into())
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
