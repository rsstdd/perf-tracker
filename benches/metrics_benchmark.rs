use criterion::{criterion_group, criterion_main, Criterion};
use performance_tracker::LighthouseMetrics;

fn benchmark_to_seconds(c: &mut Criterion) {
    let metrics = LighthouseMetrics {
        first_contentful_paint: 2200.0,
        largest_contentful_paint: 3300.0,
        time_to_interactive: 5000.0,
        total_blocking_time: 150.0,
        ..Default::default()
    };

    c.bench_function("to_seconds", |b| {
        b.iter(|| metrics.to_seconds())
    });
}

criterion_group!(benches, benchmark_to_seconds);
criterion_main!(benches);
