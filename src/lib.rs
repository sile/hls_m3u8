#![warn(
    //clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![warn(missing_docs)]
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

pub use error::{Error, ErrorKind};
pub use master_playlist::{MasterPlaylist, MasterPlaylistBuilder};
pub use media_playlist::{MediaPlaylist, MediaPlaylistBuilder, MediaPlaylistOptions};
pub use media_segment::{MediaSegment, MediaSegmentBuilder};

pub mod tags;
pub mod types;

mod attribute;
mod error;
mod line;
mod master_playlist;
mod media_playlist;
mod media_segment;
mod utils;

pub use error::Result;
