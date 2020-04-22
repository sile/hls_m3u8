# hls_m3u8

## {next}

 * Performance improvements:
    + Changed `MediaPlaylist::segments` from `BTreeMap<usize, MediaSegment>`
      to `StableVec<MediaSegment>`
    + Added `perf` feature, which can be used to improve performance in the future
    + Changed all instances of `String` to `Cow<'a, str>` to reduce `Clone`-ing.

 * Most structs now implement [`TryFrom<&'a str>`][TryFrom] instead of [`FromStr`][FromStr].


[TryFrom]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
[FromStr]: https://doc.rust-lang.org/std/str/trait.FromStr.html
