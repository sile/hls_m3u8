use std::convert::Infallible;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::{quote, unquote};
use crate::RequiredVersion;

/// A list of [usize], that can be used to indicate which version(s)
/// this instance complies with, if more than one version of a particular
/// [`KeyFormat`] is defined.
///
/// [`KeyFormat`]: crate::types::KeyFormat
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct KeyFormatVersions(Vec<usize>);

impl Default for KeyFormatVersions {
    fn default() -> Self { Self(vec![1]) }
}

impl KeyFormatVersions {
    /// Makes a new [`KeyFormatVersions`].
    pub fn new() -> Self { Self::default() }

    /// Add a value to the [`KeyFormatVersions`].
    pub fn push(&mut self, value: usize) {
        if self.is_default() {
            self.0 = vec![value];
        } else {
            self.0.push(value);
        }
    }

    /// Returns `true`, if [`KeyFormatVersions`] has the default value of
    /// `vec![1]`.
    pub fn is_default(&self) -> bool { self.0 == vec![1] && self.0.len() == 1 || self.0.is_empty() }
}

impl Deref for KeyFormatVersions {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for KeyFormatVersions {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl RequiredVersion for KeyFormatVersions {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V5 }
}

impl FromStr for KeyFormatVersions {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = unquote(input)
            .split('/')
            .filter_map(|v| v.parse().ok())
            .collect::<Vec<_>>();

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

        write!(
            f,
            "{}",
            quote(
                // vec![1, 2, 3] -> "1/2/3"
                self.0
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join("/")
            )
        )
    }
}

impl<T: Into<Vec<usize>>> From<T> for KeyFormatVersions {
    fn from(value: T) -> Self { Self(value.into()) }
}

#[cfg(test)]
mod tests {
    use super::*;

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
