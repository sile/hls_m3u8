//! The tags in this section can appear in either Master Playlists or
//! Media Playlists. If one of these tags appears in a Master Playlist,
//! it should not appear in any Media Playlist referenced by that Master
//! Playlist. A tag that appears in both must have the same value;
//! otherwise, clients should ignore the value in the Media Playlist(s).
//!
//! These tags must not appear more than once in a Playlist. If a tag
//! appears more than once, clients must fail to parse the Playlist.
mod independent_segments;
mod start;

pub use independent_segments::*;
pub use start::*;
