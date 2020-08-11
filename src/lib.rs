#![doc(html_root_url = "https://docs.rs/hls_m3u8/0.3.0")]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![warn(
    clippy::pedantic, //
    clippy::nursery,
    clippy::cargo,
    clippy::inline_always,
)]
#![allow(
    clippy::non_ascii_literal,
    clippy::redundant_pub_crate,
    clippy::multiple_crate_versions,
    clippy::module_name_repetitions,
    clippy::default_trait_access,
    clippy::unnecessary_operation // temporary until derive-builder uses #[allow(clippy::all)]
)]
#![warn(
    clippy::clone_on_ref_ptr,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::expect_used,
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
//! use std::convert::TryFrom;
//!
//! let m3u8 = MediaPlaylist::try_from(concat!(
//!     "#EXTM3U\n",
//!     "#EXT-X-TARGETDURATION:10\n",
//!     "#EXT-X-VERSION:3\n",
//!     "#EXTINF:9.009,\n",
//!     "http://media.example.com/first.ts\n",
//!     "#EXTINF:9.009,\n",
//!     "http://media.example.com/second.ts\n",
//!     "#EXTINF:3.003,\n",
//!     "http://media.example.com/third.ts\n",
//!     "#EXT-X-ENDLIST",
//! ));
//!
//! assert!(m3u8.is_ok());
//! ```
//!
//! ## Crate Feature Flags
//!
//! The following crate feature flags are available:
//!
//! - [`backtrace`] (optional)
//!   - Enables the backtrace feature for the `Error` type.
//!   - This feature depends on the following dependencies:
//!     - [`backtrace`]
//! - [`chrono`] (optional)
//!   - Enables parsing dates and verifying them.
//!   - This feature depends on the following dependencies:
//!     - [`chrono`]
//!   - The following things will change:
//!     - [`ExtXProgramDateTime::date_time`] will change from [`String`] to
//!       `DateTime<FixedOffset>`
//!     - [`ExtXDateRange::start_date`] will change from [`String`] to
//!       `DateTime<FixedOffset>`
//!     - [`ExtXDateRange::end_date`] will change from [`String`] to
//!       `DateTime<FixedOffset>`
//!
//! They are configured in your `Cargo.toml` and can be enabled like this
//!
//! ```toml
//! hls_m3u8 = { version = "0.3", features = ["chrono", "backtrace"] }
//! ```
//!
//! [`ExtXProgramDateTime::date_time`]:
//! crate::tags::ExtXProgramDateTime::date_time
//! [`ExtXDateRange::start_date`]:
//! crate::tags::ExtXDateRange::start_date
//! [`ExtXDateRange::end_date`]:
//! crate::tags::ExtXDateRange::end_date
//! [`chrono`]: https://github.com/chronotope/chrono
//! [`backtrace`]: https://github.com/rust-lang/backtrace-rs
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
pub use stable_vec;
pub use traits::*;
