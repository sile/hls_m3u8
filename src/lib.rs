//! [HLS] m3u8 parser/generator.
//!
//! [HLS]: https://tools.ietf.org/html/rfc8216
//!
//! # Examples
//!
//! TODO
#![warn(missing_docs)]
extern crate chrono;
#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};
pub use master_playlist::{MasterPlaylist, MasterPlaylistBuilder};
pub use media_playlist::{MediaPlaylist, MediaPlaylistBuilder};

pub mod segment {
    //! Media segment.
    pub use super::media_segment::{MediaSegment, MediaSegmentBuilder};
}
pub mod tag; // TODO: s/tag/tags/
pub mod types;

mod attribute;
mod error;
mod line;
mod master_playlist;
mod media_playlist;
mod media_segment;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
