//! [HLS] m3u8 parser/generator.
//!
//! [HLS]: https://tools.ietf.org/html/rfc8216
//!
//! # Examples
//!
//! TODO
#![warn(missing_docs)]
#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};
pub use master_playlist::MasterPlaylist;
pub use media_playlist::MediaPlaylist;

pub mod segment {
    //! Media segment.
    pub use super::media_segment::{MediaSegment, MediaSegmentBuilder};
}
pub mod tag;
pub mod value;
pub mod version;

mod attribute;
mod error;
mod line;
mod master_playlist;
mod media_playlist;
mod media_segment;
mod string;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
