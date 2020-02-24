use std::fmt;
use std::str::FromStr;

use derive_more::{Deref, DerefMut};

use crate::types::ProtocolVersion;
use crate::utils::{quote, unquote};
use crate::Error;
use crate::RequiredVersion;

/// A list of numbers that can be used to indicate which version(s)
/// this instance complies with, if more than one version of a particular
/// [`KeyFormat`] is defined.
///
/// [`KeyFormat`]: crate::types::KeyFormat
#[derive(Deref, DerefMut, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct KeyFormatVersions(Vec<usize>);

impl KeyFormatVersions {
    /// Makes a new [`KeyFormatVersions`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let key_format_versions = KeyFormatVersions::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Add a value to the end of [`KeyFormatVersions`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let mut key_format_versions = KeyFormatVersions::new();
    ///
    /// key_format_versions.push(1);
    /// ```
    pub fn push(&mut self, value: usize) {
        if self.is_default() {
            self.0 = vec![value];
        } else {
            self.0.push(value);
        }
    }

    /// Returns `true`, if [`KeyFormatVersions`] has the default value of
    /// `vec![1]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// assert!(KeyFormatVersions::from(vec![1]).is_default());
    /// ```
    #[must_use]
    pub fn is_default(&self) -> bool {
        //
        self.0 == vec![1] && self.0.len() == 1 || self.0.is_empty()
    }
}

impl Default for KeyFormatVersions {
    fn default() -> Self { Self(vec![1]) }
}

/// This tag requires [`ProtocolVersion::V5`].
impl RequiredVersion for KeyFormatVersions {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V5 }
}

impl FromStr for KeyFormatVersions {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = unquote(input)
            .split('/')
            .map(|v| v.parse().map_err(|e| Error::parse_int(v, e)))
            .collect::<Result<Vec<_>, Error>>()?;

        if result.is_empty() {
            result.push(1);
        }

        Ok(Self(result))
    }
}

impl fmt::Display for KeyFormatVersions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_default() {
            return write!(f, "{}", quote("1"));
        }

        if let Some(value) = self.0.iter().next() {
            write!(f, "\"{}", value)?;

            for value in self.0.iter().skip(1) {
                write!(f, "/{}", value)?;
            }

            write!(f, "\"")?;
        }

        Ok(())
    }
}

impl<I: IntoIterator<Item = usize>> From<I> for KeyFormatVersions {
    fn from(value: I) -> Self { Self(value.into_iter().collect()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            KeyFormatVersions::from(vec![1, 2, 3, 4, 5]).to_string(),
            quote("1/2/3/4/5")
        );

        assert_eq!(KeyFormatVersions::from(vec![]).to_string(), quote("1"));
        assert_eq!(KeyFormatVersions::new().to_string(), quote("1"));
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            KeyFormatVersions::from(vec![1, 2, 3, 4, 5]),
            quote("1/2/3/4/5").parse().unwrap()
        );

        assert_eq!(KeyFormatVersions::from(vec![1]), "1".parse().unwrap());

        assert_eq!(KeyFormatVersions::from(vec![1, 2]), "1/2".parse().unwrap());
        assert!("1/b".parse::<KeyFormatVersions>().is_err());
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            KeyFormatVersions::new().required_version(),
            ProtocolVersion::V5
        )
    }

    #[test]
    fn test_is_default() {
        assert!(KeyFormatVersions::new().is_default());
        assert!(KeyFormatVersions::from(vec![]).is_default());
        assert!(!KeyFormatVersions::from(vec![1, 2, 3]).is_default());
    }

    #[test]
    fn test_push() {
        let mut key_format_versions = KeyFormatVersions::from(vec![]);

        key_format_versions.push(2);
        assert_eq!(KeyFormatVersions::from(vec![2]), key_format_versions);
    }

    #[test]
    fn test_deref() {
        assert!(!KeyFormatVersions::new().is_empty());
    }

    #[test]
    fn test_deref_mut() {
        let mut key_format_versions = KeyFormatVersions::from(vec![1, 2, 3]);
        key_format_versions.pop();
        assert_eq!(key_format_versions, KeyFormatVersions::from(vec![1, 2]));
    }
}
