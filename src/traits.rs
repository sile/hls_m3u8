use std::collections::{BTreeMap, HashMap};

use crate::types::{DecryptionKey, ProtocolVersion};

mod private {
    pub trait Sealed {}
    impl Sealed for crate::MediaSegment {}
    impl Sealed for crate::tags::ExtXMap {}
}

/// Signals that a type or some of the asssociated data might need to be
/// decrypted.
///
/// # Note
///
/// You are not supposed to implement this trait, therefore it is "sealed".
pub trait Decryptable: private::Sealed {
    /// Returns all keys, associated with the type.
    ///
    /// # Example
    ///
    /// ```
    /// use hls_m3u8::tags::ExtXMap;
    /// use hls_m3u8::types::{ByteRange, EncryptionMethod};
    /// use hls_m3u8::Decryptable;
    ///
    /// let map = ExtXMap::with_range("https://www.example.url/", ByteRange::from(2..11));
    ///
    /// for key in map.keys() {
    ///     if key.method == EncryptionMethod::Aes128 {
    ///         // fetch content with the uri and decrypt the result
    ///         break;
    ///     }
    /// }
    /// ```
    #[must_use]
    fn keys(&self) -> Vec<&DecryptionKey>;

    /// Most of the time only a single key is provided, so instead of iterating
    /// through all keys, one might as well just get the first key.
    #[must_use]
    fn first_key(&self) -> Option<&DecryptionKey> {
        <Self as Decryptable>::keys(self).first().copied()
    }

    /// Returns the number of keys.
    #[must_use]
    fn len(&self) -> usize { <Self as Decryptable>::keys(self).len() }

    #[must_use]
    fn is_empty(&self) -> bool { <Self as Decryptable>::len(self) == 0 }
}

/// # Example
///
/// Implementing it:
///
/// ```
/// # use hls_m3u8::RequiredVersion;
/// use hls_m3u8::types::ProtocolVersion;
///
/// struct ExampleTag(u64);
///
/// impl RequiredVersion for ExampleTag {
///     fn required_version(&self) -> ProtocolVersion {
///         if self.0 == 5 {
///             ProtocolVersion::V4
///         } else {
///             ProtocolVersion::V1
///         }
///     }
/// }
/// assert_eq!(ExampleTag(5).required_version(), ProtocolVersion::V4);
/// assert_eq!(ExampleTag(2).required_version(), ProtocolVersion::V1);
/// ```
pub trait RequiredVersion {
    /// Returns the protocol compatibility version that this tag requires.
    ///
    /// # Note
    ///
    /// This is for the latest working [`ProtocolVersion`] and a client, that
    /// only supports an older version would break.
    fn required_version(&self) -> ProtocolVersion;

    /// The protocol version, in which the tag has been introduced.
    fn introduced_version(&self) -> ProtocolVersion { self.required_version() }
}

impl<T: RequiredVersion> RequiredVersion for Vec<T> {
    fn required_version(&self) -> ProtocolVersion {
        self.iter()
            .map(RequiredVersion::required_version)
            .max()
            // return ProtocolVersion::V1, if the iterator is empty:
            .unwrap_or_default()
    }
}

impl<T: RequiredVersion> RequiredVersion for Option<T> {
    fn required_version(&self) -> ProtocolVersion {
        self.iter()
            .map(RequiredVersion::required_version)
            .max()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_required_version_trait() {
        struct Example;

        impl RequiredVersion for Example {
            fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V3 }
        }

        assert_eq!(Example.required_version(), ProtocolVersion::V3);
        assert_eq!(Example.introduced_version(), ProtocolVersion::V3);
    }
}
