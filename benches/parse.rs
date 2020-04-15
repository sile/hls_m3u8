use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use hls_m3u8;
use hls_m3u8::tags::{ExtXDateRange, ExtXProgramDateTime};
use hls_m3u8::types::Value;
use hls_m3u8::{MediaPlaylist, MediaSegment};
use std::str::FromStr;

fn create_manifest_data() -> Vec<u8> {
    let mut builder = MediaPlaylist::builder();
    builder.media_sequence(826176645);
    builder.has_independent_segments(true);
    builder.target_duration(std::time::Duration::from_secs(2));
    for i in 0..4000 {
        let mut seg = MediaSegment::builder();
        seg.duration(std::time::Duration::from_secs_f64(1.92))
            .uri(format!(
                "avc_unencrypted_global-video=3000000-{}.ts?variant=italy",
                826176659 + i
            ));
        if i == 0 {
            seg.program_date_time(ExtXProgramDateTime::new("2020-04-07T11:32:38Z"));
        }
        if i % 100 == 0 {
            let mut date_range =
                ExtXDateRange::new(format!("{}", i / 100), "2020-04-07T11:40:02.040000Z");
            date_range.duration = Some(std::time::Duration::from_secs_f64(65.2));
            date_range.client_attributes.insert("SCTE35-OUT".to_string(), Value::Hex(hex::decode("FC302500000000000000FFF0140500001C207FEFFE0030E3A0FE005989E000010000000070BA5ABF").unwrap()));
            seg.date_range(date_range);
        }
        builder.push_segment(seg.build().unwrap());
    }
    builder.build().unwrap().to_string().into_bytes()
}

fn criterion_benchmark(c: &mut Criterion) {
    let buf = create_manifest_data();
    let mut group = c.benchmark_group("parser");
    group.throughput(Throughput::Bytes(buf.len() as u64));
    group.bench_function("throughput", |b| {
        b.iter(|| {
            let buf = String::from_utf8_lossy(&buf);
            hls_m3u8::MediaPlaylist::from_str(&buf).unwrap()
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
