//! The tags in this section can appear in either [`MasterPlaylist`]s or
//! [`MediaPlaylist`]s. If one of these tags appears in a [`MasterPlaylist`],
//! it should not appear in any [`MediaPlaylist`] referenced by that
//! [`MasterPlaylist`]. A tag that appears in both must have the same value;
//! otherwise, clients should ignore the value in the [`MediaPlaylist`](s).
//!
//! These tags must not appear more than once in a Playlist. If a tag
//! appears more than once, clients must fail to parse the Playlist.
//!
//! [`MediaPlaylist`]: crate::MediaPlaylist
//! [`MasterPlaylist`]: crate::MasterPlaylist
mod independent_segments;
mod start;

pub use independent_segments::*;
pub use start::*;
