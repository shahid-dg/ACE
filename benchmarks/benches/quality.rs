use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn quality_benchmark(c: &mut Criterion) {
    c.bench_function("quality_smoke", |b| {
        b.iter(|| {
            black_box(1.0_f64 + 1.0_f64);
        });
    });
}

criterion_group!(benches, quality_benchmark);
criterion_main!(benches);
