//! Some tests of this file are from
//! <https://github.com/videojs/m3u8-parser/tree/master/test/fixtures/m3u8>
//!
//! TODO: the rest of the tests

use std::time::Duration;

use hls_m3u8::tags::{ExtInf, ExtXByteRange, ExtXEndList, ExtXMediaSequence, ExtXTargetDuration};
use hls_m3u8::types::PlaylistType;
use hls_m3u8::{MediaPlaylist, MediaSegment};
use pretty_assertions::assert_eq;

macro_rules! generate_tests {
    ( $( $fnname:ident => { $struct:expr, $str:expr }),+ $(,)* ) => {
        $(
            #[test]
            fn $fnname() {
                assert_eq!($struct, $str.parse().unwrap());

                assert_eq!($struct.to_string(), $str.to_string());
            }
        )+
    }
}

generate_tests! {
    test_media_playlist_with_byterange => {
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
            "#EXT-X-VERSION:4\n",
            "#EXT-X-TARGETDURATION:10\n",
            "#EXT-X-MEDIA-SEQUENCE:0\n",

            "#EXT-X-BYTERANGE:75232@0\n",
            "#EXTINF:10,\n",
            "video.ts\n",

            "#EXT-X-BYTERANGE:82112@752321\n",
            "#EXTINF:10,\n",
            "video.ts\n",

            "#EXT-X-BYTERANGE:69864\n",
            "#EXTINF:10,\n",
            "video.ts\n"
        )
    },
    test_absolute_uris => {
        MediaPlaylist::builder()
            .playlist_type(PlaylistType::Vod)
            .target_duration(ExtXTargetDuration::new(Duration::from_secs(10)))
            .segments(vec![
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .uri("http://example.com/00001.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .uri("https://example.com/00002.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .uri("//example.com/00003.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .uri("http://example.com/00004.ts")
                    .build()
                    .unwrap(),
            ])
            // TODO: currently this is treated as a comment
            // .unknown(vec![
            //     "#ZEN-TOTAL-DURATION:57.9911".into()
            // ])
            .end_list(ExtXEndList)
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",
            "#EXT-X-TARGETDURATION:10\n",
            "#EXT-X-PLAYLIST-TYPE:VOD\n",
            "#EXTINF:10,\n",
            "http://example.com/00001.ts\n",
            "#EXTINF:10,\n",
            "https://example.com/00002.ts\n",
            "#EXTINF:10,\n",
            "//example.com/00003.ts\n",
            "#EXTINF:10,\n",
            "http://example.com/00004.ts\n",
            //"#ZEN-TOTAL-DURATION:57.9911\n",
            "#EXT-X-ENDLIST\n"
        )
    },
    test_allow_cache => {
        MediaPlaylist::builder()
            .target_duration(Duration::from_secs(10))
            .media_sequence(0)
            .playlist_type(PlaylistType::Vod)
            .segments(vec![
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .uri("hls_450k_video.ts")
                    .byte_range(0..522_828)
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(522_828..1_110_328)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(1_110_328..1_823_412)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(1_823_412..2_299_992)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(2_299_992..2_835_604)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(2_835_604..3_042_780)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(3_042_780..3_498_680)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(3_498_680..4_155_928)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(4_155_928..4_727_636)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(4_727_636..5_212_676)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(5_212_676..5_921_812)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(5_921_812..6_651_816)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(6_651_816..7_108_092)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(7_108_092..7_576_776)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(7_576_776..8_021_772)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(10)))
                    .byte_range(8_021_772..8_353_216)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(1.4167)))
                    .byte_range(8_353_216..8_397_772)
                    .uri("hls_450k_video.ts")
                    .build()
                    .unwrap(),
            ])
            .end_list(ExtXEndList)
            .unknown(vec![
                // deprecated tag:
                "#EXT-X-ALLOW-CACHE:YES".into()
            ])
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",
            "#EXT-X-VERSION:4\n",
            "#EXT-X-TARGETDURATION:10\n",
            "#EXT-X-MEDIA-SEQUENCE:0\n",
            "#EXT-X-PLAYLIST-TYPE:VOD\n",

            "#EXT-X-BYTERANGE:522828@0\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:587500@522828\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:713084@1110328\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:476580@1823412\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:535612@2299992\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:207176@2835604\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:455900@3042780\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:657248@3498680\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:571708@4155928\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:485040@4727636\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:709136@5212676\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:730004@5921812\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:456276@6651816\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:468684@7108092\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:444996@7576776\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:331444@8021772\n",
            "#EXTINF:10,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-BYTERANGE:44556@8353216\n",
            "#EXTINF:1.4167,\n",
            "hls_450k_video.ts\n",

            "#EXT-X-ALLOW-CACHE:YES\n",
            "#EXT-X-ENDLIST\n"
        )
    },
}
