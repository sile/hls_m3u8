#![doc(html_root_url = "https://docs.rs/hls_m3u8/0.2.1")]
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

/// Builder structs
pub mod builder {
    pub use crate::master_playlist::MasterPlaylistBuilder;
    pub use crate::media_playlist::MediaPlaylistBuilder;
    pub use crate::media_segment::MediaSegmentBuilder;

    /// Builder structs for tags
    pub mod tags {
        // master playlist
        pub use crate::tags::master_playlist::media::ExtXMediaBuilder;
        pub use crate::tags::master_playlist::session_data::ExtXSessionDataBuilder;

        // media segment
        pub use crate::tags::media_segment::date_range::ExtXDateRangeBuilder;

        // media playlist
    }

    /// Builder structs for types
    pub mod types {
        pub use crate::types::decryption_key::DecryptionKeyBuilder;
        pub use crate::types::stream_data::StreamDataBuilder;
    }
}
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
