use std::fmt;
use std::ops::Deref;

use Result;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct M3u8String(String);
impl M3u8String {
    pub fn new<T: Into<String>>(s: T) -> Result<Self> {
        // TODO: validate
        Ok(M3u8String(s.into()))
    }
    pub unsafe fn new_unchecked<T: Into<String>>(s: T) -> Self {
        M3u8String(s.into())
    }
}
impl Deref for M3u8String {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<str> for M3u8String {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for M3u8String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
