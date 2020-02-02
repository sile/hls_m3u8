use crate::tags::ExtXKey;
use crate::types::{EncryptionMethod, ProtocolVersion};

/// A trait, that is implemented on all tags, that could be encrypted.
///
/// # Example
///
/// ```
/// use hls_m3u8::tags::ExtXKey;
/// use hls_m3u8::types::EncryptionMethod;
/// use hls_m3u8::Encrypted;
///
/// struct ExampleTag {
///     keys: Vec<ExtXKey>,
/// }
///
/// // Implementing the trait is very simple:
/// // Simply expose the internal buffer, that contains all the keys.
/// impl Encrypted for ExampleTag {
///     fn keys(&self) -> &Vec<ExtXKey> { &self.keys }
///
///     fn keys_mut(&mut self) -> &mut Vec<ExtXKey> { &mut self.keys }
/// }
///
/// let mut example_tag = ExampleTag { keys: vec![] };
///
/// // adding new keys:
/// example_tag.set_keys(vec![ExtXKey::empty()]);
/// example_tag.push_key(ExtXKey::new(
///     EncryptionMethod::Aes128,
///     "http://www.example.com/data.bin",
/// ));
///
/// // getting the keys:
/// assert_eq!(
///     example_tag.keys(),
///     &vec![
///         ExtXKey::empty(),
///         ExtXKey::new(EncryptionMethod::Aes128, "http://www.example.com/data.bin",)
///     ]
/// );
///
/// assert_eq!(
///     example_tag.keys_mut(),
///     &mut vec![
///         ExtXKey::empty(),
///         ExtXKey::new(EncryptionMethod::Aes128, "http://www.example.com/data.bin",)
///     ]
/// );
///
/// assert!(example_tag.is_encrypted());
/// assert!(!example_tag.is_not_encrypted());
/// ```
pub trait Encrypted {
    /// Returns a shared reference to all keys, that can be used to decrypt this
    /// tag.
    fn keys(&self) -> &Vec<ExtXKey>;

    /// Returns an exclusive reference to all keys, that can be used to decrypt
    /// this tag.
    fn keys_mut(&mut self) -> &mut Vec<ExtXKey>;

    /// Sets all keys, that can be used to decrypt this tag.
    fn set_keys(&mut self, value: Vec<ExtXKey>) -> &mut Self {
        let keys = self.keys_mut();
        *keys = value;
        self
    }

    /// Add a single key to the list of keys, that can be used to decrypt this
    /// tag.
    fn push_key(&mut self, value: ExtXKey) -> &mut Self {
        self.keys_mut().push(value);
        self
    }

    /// Returns `true`, if the tag is encrypted.
    ///
    /// # Note
    ///
    /// This will return `true`, if any of the keys satisfies
    ///
    /// ```text
    /// key.method() != EncryptionMethod::None
    /// ```
    fn is_encrypted(&self) -> bool {
        if self.keys().is_empty() {
            return false;
        }
        self.keys()
            .iter()
            .any(|k| k.method() != EncryptionMethod::None)
    }

    /// Returns `false`, if the tag is not encrypted.
    ///
    /// # Note
    ///
    /// This is the inverse of [`is_encrypted`].
    ///
    /// [`is_encrypted`]: #method.is_encrypted
    fn is_not_encrypted(&self) -> bool { !self.is_encrypted() }
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
            .map(|v| v.required_version())
            .max()
            // return ProtocolVersion::V1, if the iterator is empty:
            .unwrap_or_default()
    }
}

impl<T: RequiredVersion> RequiredVersion for Option<T> {
    fn required_version(&self) -> ProtocolVersion {
        self.iter()
            .map(|v| v.required_version())
            .max()
            .unwrap_or_default()
    }
}
