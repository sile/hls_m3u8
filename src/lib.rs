//! [HLS] m3u8 parser/generator.
//!
//! [HLS]: https://tools.ietf.org/html/rfc8216
//!
//! # Examples
//!
//! ```
//! use hls_m3u8::MediaPlaylist;
//!
//! let m3u8 = "#EXTM3U
//! #EXT-X-TARGETDURATION:10
//! #EXT-X-VERSION:3
//! #EXTINF:9.009,
//! http://media.example.com/first.ts
//! #EXTINF:9.009,
//! http://media.example.com/second.ts
//! #EXTINF:3.003,
//! http://media.example.com/third.ts
//! #EXT-X-ENDLIST";
//!
//! assert!(m3u8.parse::<MediaPlaylist>().is_ok());
//! ```
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(const_static_lifetime))]
extern crate chrono;
#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};
pub use master_playlist::{MasterPlaylist, MasterPlaylistBuilder};
pub use media_playlist::{MediaPlaylist, MediaPlaylistBuilder};
pub use media_segment::{MediaSegment, MediaSegmentBuilder};

pub mod tags;
pub mod types;

mod attribute;
mod error;
mod line;
mod master_playlist;
mod media_playlist;
mod media_segment;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
