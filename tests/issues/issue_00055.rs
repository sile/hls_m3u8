// The relevant issue:
// https://github.com/sile/hls_m3u8/issues/55
use std::convert::TryFrom;

use hls_m3u8::tags::{ExtXMedia, VariantStream};
use hls_m3u8::types::{MediaType, StreamData, UFloat};
use hls_m3u8::MasterPlaylist;

use pretty_assertions::assert_eq;

#[test]
fn parse() {
    let file = include_str!("assets/issue_00055.m3u8");

    assert_eq!(
        MasterPlaylist::try_from(file).unwrap(),
        MasterPlaylist::builder()
            .has_independent_segments(true)
            .media(vec![
                ExtXMedia::builder()
                    .media_type(MediaType::Audio)
                    .group_id("audio_aac_1")
                    .language("eng")
                    .name("English")
                    .is_autoselect(true)
                    .is_default(true)
                    .uri("https://www.example.com/file_01.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Audio)
                    .group_id("audio_aac_2")
                    .language("eng")
                    .name("English")
                    .is_autoselect(true)
                    .is_default(true)
                    .uri("https://www.example.com/file_02.m3u8")
                    .build()
                    .unwrap(),
            ])
            .variant_streams(vec![
                VariantStream::ExtXStreamInf {
                    uri: "https://www.example.com/file_03.m3u8".into(),
                    frame_rate: Some(UFloat::new(24.000)),
                    audio: Some("audio_aac_1".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(609683)
                        .average_bandwidth(337111)
                        .resolution((426, 240))
                        .codecs(vec!["avc1.4D401F", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "https://www.example.com/file_04.m3u8".into(),
                    frame_rate: Some(UFloat::new(24.000)),
                    audio: Some("audio_aac_2".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(672828)
                        .average_bandwidth(401121)
                        .resolution((426, 240))
                        .codecs(vec!["avc1.4D401F", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "https://www.example.com/file_05.m3u8".into(),
                    frame_rate: Some(UFloat::new(24.000)),
                    audio: Some("audio_aac_1".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(963123)
                        .average_bandwidth(498553)
                        .resolution((640, 360))
                        .codecs(vec!["avc1.4D401F", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "https://www.example.com/file_06.m3u8".into(),
                    frame_rate: Some(UFloat::new(24.000)),
                    audio: Some("audio_aac_2".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(1026268)
                        .average_bandwidth(562563)
                        .resolution((640, 360))
                        .codecs(vec!["avc1.4D401F", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "https://www.example.com/file_07.m3u8".into(),
                    frame_rate: Some(UFloat::new(24.000)),
                    audio: Some("audio_aac_1".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(1365255)
                        .average_bandwidth(652779)
                        .resolution((852, 480))
                        .codecs(vec!["avc1.4D401F", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "https://www.example.com/file_08.m3u8".into(),
                    frame_rate: Some(UFloat::new(24.000)),
                    audio: Some("audio_aac_2".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(1428400)
                        .average_bandwidth(716789)
                        .resolution((852, 480))
                        .codecs(vec!["avc1.4D401F", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "https://www.example.com/file_09.m3u8".into(),
                    frame_rate: Some(UFloat::new(24.000)),
                    audio: Some("audio_aac_1".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(2342667)
                        .average_bandwidth(1030774)
                        .resolution((1280, 720))
                        .codecs(vec!["avc1.4D4020", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "https://www.example.com/file_10.m3u8".into(),
                    frame_rate: Some(UFloat::new(24.000)),
                    audio: Some("audio_aac_2".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(2405812)
                        .average_bandwidth(1094784)
                        .resolution((1280, 720))
                        .codecs(vec!["avc1.4D4020", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "https://www.example.com/file_11.m3u8".into(),
                    frame_rate: Some(UFloat::new(24.000)),
                    audio: Some("audio_aac_1".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(4635327)
                        .average_bandwidth(1687626)
                        .resolution((1920, 1080))
                        .codecs(vec!["avc1.64002A", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "https://www.example.com/file_12.m3u8".into(),
                    frame_rate: Some(UFloat::new(24.000)),
                    audio: Some("audio_aac_2".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(4698472)
                        .average_bandwidth(1751636)
                        .resolution((1920, 1080))
                        .codecs(vec!["avc1.64002A", "mp4a.40.2"])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXIFrame {
                    uri: "https://www.example.com/file_13.m3u8".into(),
                    stream_data: StreamData::builder()
                        .resolution((426, 240))
                        .codecs(vec!["avc1.4D401F"])
                        .bandwidth(92496)
                        .average_bandwidth(31745)
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXIFrame {
                    uri: "https://www.example.com/file_14.m3u8".into(),
                    stream_data: StreamData::builder()
                        .resolution((640, 360))
                        .codecs(vec!["avc1.4D401F"])
                        .bandwidth(252672)
                        .average_bandwidth(53787)
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXIFrame {
                    uri: "https://www.example.com/file_15.m3u8".into(),
                    stream_data: StreamData::builder()
                        .resolution((852, 480))
                        .codecs(vec!["avc1.4D401F"])
                        .bandwidth(392544)
                        .average_bandwidth(72767)
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXIFrame {
                    uri: "https://www.example.com/file_16.m3u8".into(),
                    stream_data: StreamData::builder()
                        .resolution((1280, 720))
                        .codecs(vec!["avc1.4D4020"])
                        .bandwidth(649728)
                        .average_bandwidth(108944)
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXIFrame {
                    uri: "https://www.example.com/file_17.m3u8".into(),
                    stream_data: StreamData::builder()
                        .resolution((1920, 1080))
                        .codecs(vec!["avc1.64002A"])
                        .bandwidth(1328784)
                        .average_bandwidth(161039)
                        .build()
                        .unwrap()
                },
            ])
            .build()
            .unwrap()
    );
}
