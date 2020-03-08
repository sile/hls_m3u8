#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic, //
    clippy::nursery,
    clippy::cargo,
    clippy::inline_always,
)]
#![allow(
    clippy::multiple_crate_versions,
    clippy::module_name_repetitions,
    clippy::default_trait_access,
    clippy::unnecessary_operation // temporary until derive-builder uses #[allow(clippy::all)]
)]
#![warn(
    clippy::clone_on_ref_ptr,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::option_expect_used,
    clippy::unneeded_field_pattern,
    clippy::wrong_pub_self_convention
)]
// those should not be present in production code:
#![deny(
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented,
    clippy::dbg_macro,
    clippy::use_debug
)]
#![warn(
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts
)]
//! [HLS] m3u8 parser/generator.
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
//!
//! [HLS]: https://tools.ietf.org/html/rfc8216

pub use error::Error;
pub use master_playlist::MasterPlaylist;
pub use media_playlist::MediaPlaylist;
pub use media_segment::MediaSegment;

pub mod tags;
pub mod types;

#[macro_use]
mod utils;
mod attribute;
mod error;
mod line;
mod master_playlist;
mod media_playlist;
mod media_segment;
mod traits;

pub use error::Result;
pub use traits::*;
