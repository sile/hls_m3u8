use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::{Extend, FromIterator};
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::{quote, unquote};
use crate::Error;
use crate::RequiredVersion;

/// A list of numbers that can be used to indicate which version(s)
/// this instance complies with, if more than one version of a particular
/// [`KeyFormat`] is defined.
///
/// ## Note on maximum size
///
/// To reduce the memory usage and to make this struct implement [`Copy`], a
/// fixed size array is used internally (`[u8; 9]`), which can store a maximum
/// number of 9 `u8` numbers.
///
/// If you encounter any m3u8 file, which fails to parse, because the buffer is
/// too small, feel free to [make an issue](https://github.com/sile/hls_m3u8/issues).
///
/// ## Example
///
/// ```
/// use hls_m3u8::types::KeyFormatVersions;
///
/// assert_eq!(
///     KeyFormatVersions::from([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]).to_string(),
///     "\"255/255/255/255/255/255/255/255/255\"".to_string()
/// );
/// ```
///
/// [`KeyFormat`]: crate::types::KeyFormat
#[derive(Debug, Clone, Copy)]
pub struct KeyFormatVersions {
    // NOTE(Luro02): if the current array is not big enough one can easily increase
    //               the number of elements or change the type to something bigger,
    //               but it would be kinda wasteful to use a `Vec` here, which requires
    //               allocations and has a size of at least 24 bytes
    //               (::std::mem::size_of::<Vec<u8>>() = 24).
    buffer: [u8; 9],
    // Indicates the number of used items in the array.
    len: u8,
}

impl KeyFormatVersions {
    /// Constructs an empty [`KeyFormatVersions`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let versions = KeyFormatVersions::new();
    ///
    /// assert_eq!(versions, KeyFormatVersions::default());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Add a value to the end of [`KeyFormatVersions`].
    ///
    /// # Panics
    ///
    /// This function panics, if you try to push more elements, than
    /// [`KeyFormatVersions::remaining`] returns.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let mut versions = KeyFormatVersions::new();
    ///
    /// versions.push(1);
    /// assert_eq!(versions, KeyFormatVersions::from([1]));
    /// ```
    ///
    /// This will panic, because it exceeded the maximum number of elements:
    ///
    /// ```{.should_panic}
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let mut versions = KeyFormatVersions::new();
    ///
    /// for _ in 0..=versions.capacity() {
    ///     versions.push(1); // <- panics
    /// }
    /// ```
    pub fn push(&mut self, value: u8) {
        if self.len as usize == self.buffer.len() {
            panic!("reached maximum number of elements in KeyFormatVersions");
        }

        self.buffer[self.len()] = value;
        self.len += 1;
    }

    /// `KeyFormatVersions` has a limited capacity and this function returns how
    /// many elements can be pushed, until it panics.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let mut versions = KeyFormatVersions::new();
    ///
    /// assert_eq!(versions.remaining(), versions.capacity());
    ///
    /// versions.push(1);
    /// versions.push(2);
    /// versions.push(3);
    /// assert_eq!(versions.remaining(), 6);
    /// ```
    #[inline]
    #[must_use]
    pub fn remaining(&self) -> usize { self.capacity().saturating_sub(self.len()) }

    /// Returns the number of elements.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let mut versions = KeyFormatVersions::new();
    ///
    /// assert_eq!(versions.len(), 0);
    ///
    /// versions.push(2);
    /// assert_eq!(versions.len(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.len as usize }

    /// Returns the total number of elements that can be stored.
    ///
    /// # Note
    ///
    /// It should not be relied on that this function will always return 9. In
    /// the future this number might increase.
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize { self.buffer.len() }

    /// Shortens the internal array to the provided length.
    ///
    /// # Note
    ///
    /// If `len` is greater than the current length, this has no effect.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let mut versions = KeyFormatVersions::from([1, 2, 3, 4, 5, 6]);
    /// versions.truncate(3);
    ///
    /// assert_eq!(versions, KeyFormatVersions::from([1, 2, 3]));
    /// ```
    pub fn truncate(&mut self, len: usize) {
        if len > self.len() {
            return;
        }

        self.len = len as u8;
    }

    /// Returns `true` if there are no elements.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let mut versions = KeyFormatVersions::new();
    ///
    /// assert_eq!(versions.is_empty(), true);
    ///
    /// versions.push(2);
    /// assert_eq!(versions.is_empty(), false);
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.len() == 0 }

    /// Removes the last element and returns it, or `None` if it is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let mut versions = KeyFormatVersions::new();
    ///
    /// assert_eq!(versions.pop(), None);
    ///
    /// versions.push(2);
    /// assert_eq!(versions.pop(), Some(2));
    /// assert_eq!(versions.is_empty(), true);
    /// ```
    pub fn pop(&mut self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            Some(self.buffer[self.len()])
        }
    }

    /// Returns `true`, if it is either empty or has a length of 1 and the first
    /// element is 1.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::KeyFormatVersions;
    /// let mut versions = KeyFormatVersions::new();
    ///
    /// assert_eq!(versions.is_default(), true);
    ///
    /// versions.push(1);
    /// assert_eq!(versions.is_default(), true);
    ///
    /// assert_eq!(KeyFormatVersions::default().is_default(), true);
    /// ```
    #[must_use]
    pub fn is_default(&self) -> bool {
        self.is_empty() || (self.buffer[self.len().saturating_sub(1)] == 1 && self.len() == 1)
    }
}

impl PartialEq for KeyFormatVersions {
    fn eq(&self, other: &Self) -> bool {
        if self.len() == other.len() {
            // only compare the parts in the buffer, that are used:
            self.as_ref() == self.as_ref()
        } else {
            false
        }
    }
}

impl Eq for KeyFormatVersions {}

impl PartialOrd for KeyFormatVersions {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(<Self as Ord>::cmp(self, other))
    }
}

impl Ord for KeyFormatVersions {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering { self.as_ref().cmp(other.as_ref()) }
}

impl Hash for KeyFormatVersions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.len());
        self.as_ref().hash(state);
    }
}

impl AsRef<[u8]> for KeyFormatVersions {
    #[inline]
    #[must_use]
    fn as_ref(&self) -> &[u8] { &self.buffer[..self.len()] }
}

impl AsMut<[u8]> for KeyFormatVersions {
    #[inline]
    #[must_use]
    fn as_mut(&mut self) -> &mut [u8] {
        // this temporary variable is required, because the compiler does not resolve
        // the borrow to it's value immediately, so there is a shared borrow and
        // therefore no exclusive borrow can be made.
        let len = self.len();
        &mut self.buffer[..len]
    }
}

impl Extend<u8> for KeyFormatVersions {
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        for element in iter {
            if self.remaining() == 0 {
                break;
            }

            self.push(element);
        }
    }
}

impl<'a> Extend<&'a u8> for KeyFormatVersions {
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        <Self as Extend<u8>>::extend(self, iter.into_iter().copied())
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for KeyFormatVersions {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output { self.as_ref().index(index) }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for KeyFormatVersions {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output { self.as_mut().index_mut(index) }
}

impl IntoIterator for KeyFormatVersions {
    type IntoIter = IntoIter<u8>;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter { self.into() }
}

impl FromIterator<u8> for KeyFormatVersions {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut result = Self::default();
        // an array like [0; 9] as empty
        let mut is_empty = true;

        for item in iter {
            if item != 0 {
                is_empty = false;
            }

            if result.remaining() == 0 {
                break;
            }

            result.push(item);
        }

        if is_empty {
            return Self::default();
        }

        result
    }
}

impl<'a> FromIterator<&'a u8> for KeyFormatVersions {
    fn from_iter<I: IntoIterator<Item = &'a u8>>(iter: I) -> Self {
        <Self as FromIterator<u8>>::from_iter(iter.into_iter().copied())
    }
}

impl Default for KeyFormatVersions {
    #[inline]
    fn default() -> Self {
        Self {
            buffer: [0; 9],
            len: 0,
        }
    }
}

/// This tag requires [`ProtocolVersion::V5`].
impl RequiredVersion for KeyFormatVersions {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V5 }
}

impl FromStr for KeyFormatVersions {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = Self::default();

        for item in unquote(input)
            .split('/')
            .map(|v| v.parse().map_err(|e| Error::parse_int(v, e)))
        {
            let item = item?;

            if result.remaining() == 0 {
                return Err(Error::custom(
                    "reached maximum number of elements in KeyFormatVersions",
                ));
            }

            result.push(item);
        }

        if result.is_empty() {
            result.push(1);
        }

        Ok(result)
    }
}

impl fmt::Display for KeyFormatVersions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_default() || self.is_empty() {
            return write!(f, "{}", quote("1"));
        }

        write!(f, "\"{}", self.buffer[0])?;

        for item in &self.buffer[1..self.len()] {
            write!(f, "/{}", item)?;
        }

        write!(f, "\"")?;

        Ok(())
    }
}

impl<T: AsRef<[usize]>> From<T> for KeyFormatVersions {
    fn from(value: T) -> Self { Self::from_iter(value.as_ref().iter().map(|i| *i as u8)) }
}

/// `Iterator` for [`KeyFormatVersions`].
#[derive(Debug, Clone, PartialEq)]
pub struct IntoIter<T> {
    buffer: [T; 9],
    position: usize,
    len: usize,
}

impl From<KeyFormatVersions> for IntoIter<u8> {
    fn from(value: KeyFormatVersions) -> Self {
        Self {
            buffer: value.buffer,
            position: 0,
            len: value.len(),
        }
    }
}

impl<'a> From<&'a KeyFormatVersions> for IntoIter<u8> {
    fn from(value: &'a KeyFormatVersions) -> Self {
        Self {
            buffer: value.buffer,
            position: 0,
            len: value.len(),
        }
    }
}

impl<T: Copy> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize { self.len.saturating_sub(self.position) }
}

impl<T: Copy> ::core::iter::FusedIterator for IntoIter<T> {}

impl<T: Copy> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == self.len {
            return None;
        }

        self.position += 1;
        Some(self.buffer[self.position - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hash() {
        let mut hasher_left = std::collections::hash_map::DefaultHasher::new();
        let mut hasher_right = std::collections::hash_map::DefaultHasher::new();

        assert_eq!(
            KeyFormatVersions::from([1, 2, 3]).hash(&mut hasher_left),
            KeyFormatVersions::from([1, 2, 3]).hash(&mut hasher_right)
        );

        assert_eq!(hasher_left.finish(), hasher_right.finish());
    }

    #[test]
    fn test_ord() {
        assert_eq!(
            KeyFormatVersions::from([1, 2]).cmp(&KeyFormatVersions::from([1, 2])),
            Ordering::Equal
        );

        assert_eq!(
            KeyFormatVersions::from([2]).cmp(&KeyFormatVersions::from([1, 2])),
            Ordering::Greater
        );

        assert_eq!(
            KeyFormatVersions::from([2, 3]).cmp(&KeyFormatVersions::from([1, 2])),
            Ordering::Greater
        );

        assert_eq!(
            KeyFormatVersions::from([]).cmp(&KeyFormatVersions::from([1, 2])),
            Ordering::Less
        );
    }

    #[test]
    fn test_partial_eq() {
        let mut versions = KeyFormatVersions::from([1, 2, 3, 4, 5, 6]);
        versions.truncate(3);

        assert_eq!(versions, KeyFormatVersions::from([1, 2, 3]));
    }

    #[test]
    fn test_as_ref() {
        assert_eq!(KeyFormatVersions::new().as_ref(), &[]);
        assert_eq!(KeyFormatVersions::from([1, 2, 3]).as_ref(), &[1, 2, 3]);
        assert_eq!(KeyFormatVersions::from([]).as_ref(), &[]);
    }

    #[test]
    fn test_as_mut() {
        assert_eq!(KeyFormatVersions::new().as_mut(), &mut []);
        assert_eq!(KeyFormatVersions::from([1, 2, 3]).as_mut(), &mut [1, 2, 3]);
        assert_eq!(KeyFormatVersions::from([]).as_mut(), &mut []);
    }

    #[test]
    fn test_index() {
        // test index
        assert_eq!(&KeyFormatVersions::new()[..], &[]);
        assert_eq!(&KeyFormatVersions::from([1, 2, 3])[..2], &[1, 2]);
        assert_eq!(&KeyFormatVersions::from([1, 2, 3])[1..2], &[2]);
        assert_eq!(&KeyFormatVersions::from([1, 2, 3])[..], &[1, 2, 3]);

        // test index_mut
        assert_eq!(&mut KeyFormatVersions::new()[..], &mut []);
        assert_eq!(&mut KeyFormatVersions::from([1, 2, 3])[..2], &mut [1, 2]);
        assert_eq!(&mut KeyFormatVersions::from([1, 2, 3])[1..2], &mut [2]);
        assert_eq!(&mut KeyFormatVersions::from([1, 2, 3])[..], &mut [1, 2, 3]);
    }

    #[test]
    fn test_extend() {
        let mut versions = KeyFormatVersions::new();
        versions.extend(&[1, 2, 3]);

        assert_eq!(versions, KeyFormatVersions::from([1, 2, 3]));

        versions.extend(&[1, 2, 3]);
        assert_eq!(versions, KeyFormatVersions::from([1, 2, 3, 1, 2, 3]));

        versions.extend(&[1, 2, 3, 4]);
        assert_eq!(
            versions,
            KeyFormatVersions::from([1, 2, 3, 1, 2, 3, 1, 2, 3])
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(KeyFormatVersions::default(), KeyFormatVersions::new());
    }

    #[test]
    fn test_into_iter() {
        assert_eq!(KeyFormatVersions::new().into_iter().next(), None);
        assert_eq!(KeyFormatVersions::new().into_iter().len(), 0);

        let mut iterator = KeyFormatVersions::from([1, 2, 3, 4, 5]).into_iter();

        assert_eq!(iterator.len(), 5);
        assert_eq!(iterator.next(), Some(1));

        assert_eq!(iterator.len(), 4);
        assert_eq!(iterator.next(), Some(2));

        assert_eq!(iterator.len(), 3);
        assert_eq!(iterator.next(), Some(3));

        assert_eq!(iterator.len(), 2);
        assert_eq!(iterator.next(), Some(4));

        assert_eq!(iterator.len(), 1);
        assert_eq!(iterator.next(), Some(5));

        assert_eq!(iterator.len(), 0);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_from_iter() {
        assert_eq!(
            {
                let mut result = KeyFormatVersions::new();
                result.push(1);
                result.push(2);
                result.push(3);
                result.push(4);
                result
            },
            KeyFormatVersions::from_iter(&[1, 2, 3, 4])
        );

        assert_eq!(
            {
                let mut result = KeyFormatVersions::new();
                result.push(0);
                result.push(1);
                result.push(2);
                result.push(3);
                result.push(4);
                result
            },
            KeyFormatVersions::from_iter(&[0, 1, 2, 3, 4])
        );

        assert_eq!(KeyFormatVersions::new(), KeyFormatVersions::from_iter(&[]));

        assert_eq!(KeyFormatVersions::new(), KeyFormatVersions::from_iter(&[0]));
        assert_eq!(
            KeyFormatVersions::new(),
            KeyFormatVersions::from_iter(&[0, 0])
        );
        assert_eq!(
            {
                let mut result = KeyFormatVersions::new();
                result.push(0);
                result.push(1);
                result.push(2);
                result.push(3);
                result.push(4);
                result.push(5);
                result.push(6);
                result.push(7);
                result.push(8);
                result
            },
            KeyFormatVersions::from_iter(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(
            KeyFormatVersions::from([1, 2, 3, 4, 5]).to_string(),
            quote("1/2/3/4/5")
        );

        assert_eq!(KeyFormatVersions::from([]).to_string(), quote("1"));
        assert_eq!(KeyFormatVersions::new().to_string(), quote("1"));
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            KeyFormatVersions::from([1, 2, 3, 4, 5]),
            quote("1/2/3/4/5").parse().unwrap()
        );

        assert_eq!(KeyFormatVersions::from([1]), "1".parse().unwrap());
        assert_eq!(KeyFormatVersions::from([1, 2]), "1/2".parse().unwrap());

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
        assert_eq!(KeyFormatVersions::new().is_default(), true);
        assert_eq!(KeyFormatVersions::default().is_default(), true);

        assert_eq!(KeyFormatVersions::from([]).is_default(), true);
        assert_eq!(KeyFormatVersions::from([1]).is_default(), true);

        assert_eq!(KeyFormatVersions::from([1, 2, 3]).is_default(), false);
    }

    #[test]
    fn test_push() {
        let mut key_format_versions = KeyFormatVersions::new();
        key_format_versions.push(2);

        assert_eq!(KeyFormatVersions::from([2]), key_format_versions);
    }
}
