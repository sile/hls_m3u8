use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::media_playlist::benches,
}
