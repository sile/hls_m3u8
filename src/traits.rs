use std::collections::{BTreeMap, HashMap};

use stable_vec::StableVec;

use crate::types::{DecryptionKey, ProtocolVersion};

mod private {
    pub trait Sealed {}
    impl<'a> Sealed for crate::MediaSegment<'a> {}
    impl<'a> Sealed for crate::tags::ExtXMap<'a> {}
}

/// Signals that a type or some of the asssociated data might need to be
/// decrypted.
///
/// # Note
///
/// You are not supposed to implement this trait, therefore it is "sealed".
pub trait Decryptable<'a>: private::Sealed {
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
    fn keys(&self) -> Vec<&DecryptionKey<'a>>;

    /// Most of the time only a single key is provided, so instead of iterating
    /// through all keys, one might as well just get the first key.
    #[must_use]
    #[inline]
    fn first_key(&self) -> Option<&DecryptionKey<'a>> {
        <Self as Decryptable>::keys(self).first().copied()
    }

    /// Returns the number of keys.
    #[must_use]
    #[inline]
    fn len(&self) -> usize { <Self as Decryptable>::keys(self).len() }

    /// Returns `true`, if the number of keys is zero.
    #[must_use]
    #[inline]
    fn is_empty(&self) -> bool { <Self as Decryptable>::len(self) == 0 }
}

#[doc(hidden)]
pub trait RequiredVersion {
    /// Returns the protocol compatibility version that this tag requires.
    ///
    /// # Note
    ///
    /// This is for the latest working [`ProtocolVersion`] and a client, that
    /// only supports an older version would break.
    #[must_use]
    fn required_version(&self) -> ProtocolVersion;

    /// The protocol version, in which the tag has been introduced.
    #[must_use]
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

impl<K, V: RequiredVersion> RequiredVersion for BTreeMap<K, V> {
    fn required_version(&self) -> ProtocolVersion {
        self.values()
            .map(RequiredVersion::required_version)
            .max()
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

impl<K, V: RequiredVersion, S> RequiredVersion for HashMap<K, V, S> {
    fn required_version(&self) -> ProtocolVersion {
        self.values()
            .map(RequiredVersion::required_version)
            .max()
            .unwrap_or_default()
    }
}

impl<T: RequiredVersion> RequiredVersion for StableVec<T> {
    fn required_version(&self) -> ProtocolVersion {
        self.values()
            .map(RequiredVersion::required_version)
            .max()
            // return ProtocolVersion::V1, if the iterator is empty:
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
