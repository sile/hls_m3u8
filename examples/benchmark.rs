//! Parse benchmark for hls_m3u8.
//!
//! Run with: `cargo run --release --example benchmark`
//!
//! Optional env var `BENCH_REPEATS` (default 30) controls the number of
//! best-of-N samples per case. Each sample auto-scales its inner loop count
//! to land near 100 ms of wall time, so reported numbers stay stable.
//!
//! Uses `std::time::Instant` rather than `criterion` so the crate keeps a
//! lean dev-dep tree.

#[cfg(feature = "chrono")]
fn main() {
    eprintln!("the benchmark example is not compatible with the `chrono` feature");
    std::process::exit(2);
}

#[cfg(not(feature = "chrono"))]
use bench::main;

#[cfg(not(feature = "chrono"))]
mod bench {
    use std::str::FromStr;
    use std::time::{Duration, Instant};

    use hls_m3u8::tags::{ExtXDateRange, ExtXProgramDateTime};
    use hls_m3u8::types::Value;
    use hls_m3u8::{MediaPlaylist, MediaSegment};

    const TARGET_DURATION: Duration = Duration::from_millis(100);

    struct Row {
        name: String,
        bytes: usize,
        ns_per_op: f64,
    }

    fn measure_one<F: FnMut()>(mut op: F) -> f64 {
        for _ in 0..3 {
            op();
        }
        let mut iters: u64 = 1;
        loop {
            let start = Instant::now();
            for _ in 0..iters {
                op();
            }
            let elapsed = start.elapsed();
            if elapsed >= TARGET_DURATION {
                return elapsed.as_nanos() as f64 / iters as f64;
            }
            let factor =
                (TARGET_DURATION.as_nanos() as f64 / elapsed.as_nanos().max(1) as f64).max(2.0);
            iters = (iters as f64 * factor).ceil() as u64;
        }
    }

    fn best_of<F: FnMut() -> f64>(repeats: usize, mut measure: F) -> f64 {
        (0..repeats)
            .map(|_| measure())
            .fold(f64::INFINITY, f64::min)
    }

    fn print_section(title: &str, rows: &[Row]) {
        println!("=== {title} ===");
        println!(
            "  {:<32} {:>10} {:>14} {:>12}",
            "case", "size", "ns/op", "MB/s"
        );
        for r in rows {
            let mb_s = if r.ns_per_op > 0.0 {
                r.bytes as f64 / r.ns_per_op * 1_000.0
            } else {
                f64::INFINITY
            };
            let size = format!("{} B", r.bytes);
            println!(
                "  {:<32} {:>10} {:>14.1} {:>12.1}",
                r.name, size, r.ns_per_op, mb_s
            );
        }
        println!();
    }

    fn create_manifest_data() -> String {
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
                            Value::Hex(vec![
                                0xFC, 0x30, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                0xFF, 0xF0, 0x14, 0x05, 0x00, 0x00, 0x1C, 0x20, 0x7F, 0xEF, 0xFE,
                                0x00, 0x30, 0xE3, 0xA0, 0xFE, 0x00, 0x59, 0x89, 0xE0, 0x00, 0x01,
                                0x00, 0x00, 0x00, 0x00, 0x70, 0xBA, 0x5A, 0xBF,
                            ]),
                        )
                        .build()
                        .unwrap(),
                );
            }

            builder.push_segment(seg.build().unwrap());
        }

        builder.build().unwrap().to_string()
    }

    fn bench_parse(repeats: usize) -> Vec<Row> {
        let data = create_manifest_data();
        let bytes = data.len();

        let mut rows = Vec::new();

        let ns = best_of(repeats, || {
            measure_one(|| {
                let _ = MediaPlaylist::from_str(&data).unwrap();
            })
        });
        rows.push(Row {
            name: "MediaPlaylist::from_str".into(),
            bytes,
            ns_per_op: ns,
        });

        let ns = best_of(repeats, || {
            measure_one(|| {
                let _ = MediaPlaylist::try_from(data.as_str()).unwrap();
            })
        });
        rows.push(Row {
            name: "MediaPlaylist::try_from".into(),
            bytes,
            ns_per_op: ns,
        });

        rows
    }

    pub fn main() {
        let repeats: usize = std::env::var("BENCH_REPEATS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);

        println!("hls_m3u8 benchmark");
        println!("repeats per case: {repeats} (best-of-N reported)");
        println!();

        let rows = bench_parse(repeats);
        print_section("parse (4000 segments)", &rows);
    }
}
