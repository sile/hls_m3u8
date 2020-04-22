use std::convert::TryFrom;
use std::str::FromStr;
use std::time::Duration;

use criterion::{black_box, criterion_group, Criterion, Throughput};

use hls_m3u8::tags::{ExtXDateRange, ExtXProgramDateTime};
use hls_m3u8::types::Value;
use hls_m3u8::{MediaPlaylist, MediaSegment};

fn create_manifest_data() -> Vec<u8> {
    let mut builder = MediaPlaylist::builder();

    builder.media_sequence(826176645);
    builder.has_independent_segments(true);
    builder.target_duration(Duration::from_secs(2));

    for i in 0..4000 {
        let mut seg = MediaSegment::builder();
        seg.duration(Duration::from_secs_f64(1.92)).uri(format!(
            "avc_unencrypted_global-video=3000000-{}.ts?variant=italy",
            826176659 + i
        ));

        if i == 0 {
            seg.program_date_time(ExtXProgramDateTime::new("2020-04-07T11:32:38Z"));
        }

        if i % 100 == 0 {
            seg.date_range(
                ExtXDateRange::builder()
                    .id(format!("date_id_{}", i / 100))
                    .start_date("2020-04-07T11:40:02.040000Z")
                    .duration(Duration::from_secs_f64(65.2))
                    .insert_client_attribute(
                        "SCTE35-OUT",
                        Value::Hex(
                            hex::decode(concat!(
                                "FC30250000",
                                "0000000000",
                                "FFF0140500",
                                "001C207FEF",
                                "FE0030E3A0",
                                "FE005989E0",
                                "0001000000",
                                "0070BA5ABF"
                            ))
                            .unwrap(),
                        ),
                    )
                    .build()
                    .unwrap(),
            );
        }

        builder.push_segment(seg.build().unwrap());
    }

    builder.build().unwrap().to_string().into_bytes()
}

fn media_playlist_from_str(c: &mut Criterion) {
    let data = String::from_utf8(create_manifest_data()).unwrap();

    let mut group = c.benchmark_group("MediaPlaylist::from_str");

    group.throughput(Throughput::Bytes(data.len() as u64));

    group.bench_function("MediaPlaylist::from_str", |b| {
        b.iter(|| MediaPlaylist::from_str(black_box(&data)).unwrap());
    });

    group.finish();
}

fn media_playlist_try_from(c: &mut Criterion) {
    let data = String::from_utf8(create_manifest_data()).unwrap();

    let mut group = c.benchmark_group("MediaPlaylist::try_from");

    group.throughput(Throughput::Bytes(data.len() as u64));

    group.bench_function("MediaPlaylist::try_from", |b| {
        b.iter(|| MediaPlaylist::try_from(black_box(data.as_str())).unwrap());
    });

    group.finish();
}

criterion_group!(benches, media_playlist_from_str, media_playlist_try_from);
