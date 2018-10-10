hls_m3u8
=========

[![Crates.io: hls_m3u8](https://img.shields.io/crates/v/hls_m3u8.svg)](https://crates.io/crates/hls_m3u8)
[![Documentation](https://docs.rs/hls_m3u8/badge.svg)](https://docs.rs/hls_m3u8)
[![Build Status](https://travis-ci.org/sile/hls_m3u8.svg?branch=master)](https://travis-ci.org/sile/hls_m3u8)
[![Code Coverage](https://codecov.io/gh/sile/hls_m3u8/branch/master/graph/badge.svg)](https://codecov.io/gh/sile/hls_m3u8/branch/master)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[HLS] m3u8 parser/generator.

[Documentation](https://docs.rs/hls_m3u8)

[HLS]: https://tools.ietf.org/html/rfc8216

Examples
---------

```rust
use hls_m3u8::MediaPlaylist;

let m3u8 = "#EXTM3U
#EXT-X-TARGETDURATION:10
#EXT-X-VERSION:3
#EXTINF:9.009,
http://media.example.com/first.ts
#EXTINF:9.009,
http://media.example.com/second.ts
#EXTINF:3.003,
http://media.example.com/third.ts
#EXT-X-ENDLIST";

assert!(m3u8.parse::<MediaPlaylist>().is_ok());
```
