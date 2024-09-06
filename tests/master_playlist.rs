use std::convert::TryFrom;

use hls_m3u8::tags::{ExtXMedia, VariantStream};
use hls_m3u8::types::{MediaType, StreamData};
use hls_m3u8::MasterPlaylist;

use pretty_assertions::assert_eq;

macro_rules! generate_tests {
    ( $( $fnname:ident => { $struct:expr, $str:expr }),+ $(,)* ) => {
        $(
            #[test]
            fn $fnname() {
                assert_eq!($struct, TryFrom::try_from($str).unwrap());

                assert_eq!($struct.to_string(), $str.to_string());
            }
        )+
    }
}

generate_tests! {
    test_alternate_audio => {
        MasterPlaylist::builder()
            .media(vec![
                ExtXMedia::builder()
                    .media_type(MediaType::Audio)
                    .group_id("audio")
                    .language("eng")
                    .name("English")
                    .is_autoselect(true)
                    .is_default(true)
                    .uri("eng/prog_index.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Audio)
                    .group_id("audio")
                    .language("fre")
                    .name("Français")
                    .is_autoselect(true)
                    .is_default(false)
                    .uri("fre/prog_index.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Audio)
                    .group_id("audio")
                    .language("sp")
                    .name("Espanol")
                    .is_autoselect(true)
                    .is_default(false)
                    .uri("sp/prog_index.m3u8")
                    .build()
                    .unwrap(),
            ])
            .variant_streams(vec![
                VariantStream::ExtXStreamInf {
                    uri: "lo/prog_index.m3u8".into(),
                    frame_rate: None,
                    audio: Some("audio".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(195023)
                        .codecs(["avc1.42e00a", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "hi/prog_index.m3u8".into(),
                    frame_rate: None,
                    audio: Some("audio".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(591680)
                        .codecs(["avc1.42e01e", "mp4a.40.2"])
                        .build()
                        .unwrap()
                }
            ])
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",

            "#EXT-X-MEDIA:",
            "TYPE=AUDIO,",
            "URI=\"eng/prog_index.m3u8\",",
            "GROUP-ID=\"audio\",",
            "LANGUAGE=\"eng\",",
            "NAME=\"English\",",
            "DEFAULT=YES,",
            "AUTOSELECT=YES",
            "\n",

            "#EXT-X-MEDIA:",
            "TYPE=AUDIO,",
            "URI=\"fre/prog_index.m3u8\",",
            "GROUP-ID=\"audio\",",
            "LANGUAGE=\"fre\",",
            "NAME=\"Français\",",
            "AUTOSELECT=YES",
            "\n",

            "#EXT-X-MEDIA:",
            "TYPE=AUDIO,",
            "URI=\"sp/prog_index.m3u8\",",
            "GROUP-ID=\"audio\",",
            "LANGUAGE=\"sp\",",
            "NAME=\"Espanol\",",
            "AUTOSELECT=YES",
            "\n",

            "#EXT-X-STREAM-INF:",
            "BANDWIDTH=195023,",
            "CODECS=\"avc1.42e00a,mp4a.40.2\",",
            "AUDIO=\"audio\"",
            "\n",
            "lo/prog_index.m3u8\n",

            "#EXT-X-STREAM-INF:",
            "BANDWIDTH=591680,",
            "CODECS=\"avc1.42e01e,mp4a.40.2\",",
            "AUDIO=\"audio\"",
            "\n",
            "hi/prog_index.m3u8\n"
        )
    }
}
