hls_m3u8
=========

[![Crates.io: hls_m3u8](https://img.shields.io/crates/v/hls_m3u8.svg)](https://crates.io/crates/hls_m3u8)
[![Documentation](https://docs.rs/hls_m3u8/badge.svg)](https://docs.rs/hls_m3u8)
[![Build Status](https://travis-ci.org/sile/hls_m3u8.svg?branch=master)](https://travis-ci.org/sile/hls_m3u8)
[![Code Coverage](https://codecov.io/gh/sile/hls_m3u8/branch/master/graph/badge.svg)](https://codecov.io/gh/sile/hls_m3u8/branch/master)
![Crates.io](https://img.shields.io/crates/l/hls_m3u8)

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

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
