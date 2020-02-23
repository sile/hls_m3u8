use std::time::Duration;

use hls_m3u8::tags::{ExtInf, ExtXByteRange, ExtXMediaSequence, ExtXTargetDuration};
use hls_m3u8::{MediaPlaylist, MediaSegment};
use pretty_assertions::assert_eq;

#[test]
fn test_media_playlist_with_byterange() {
    assert_eq!(
        MediaPlaylist::builder()
            .target_duration(ExtXTargetDuration::new(Duration::from_secs(10)))
            .media_sequence(ExtXMediaSequence::new(0))
            .segments(vec![
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(10.0)))
                    .byte_range(ExtXByteRange::from(0..75232))
                    .uri("video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(10.0)))
                    .byte_range(ExtXByteRange::from(752321..82112 + 752321))
                    .uri("video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(10.0)))
                    .byte_range(ExtXByteRange::from(..69864))
                    .uri("video.ts")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",
            "#EXT-X-TARGETDURATION:10\n",
            "#EXT-X-VERSION:4\n",
            "#EXT-X-MEDIA-SEQUENCE:0\n",
            "#EXTINF:10.0,\n",
            "#EXT-X-BYTERANGE:75232@0\n",
            "video.ts\n",
            "#EXT-X-BYTERANGE:82112@752321\n",
            "#EXTINF:10.0,\n",
            "video.ts\n",
            "#EXTINF:10.0,\n",
            "#EXT-X-BYTERANGE:69864\n",
            "video.ts\n"
        )
        .parse::<MediaPlaylist>()
        .unwrap()
    )
}
