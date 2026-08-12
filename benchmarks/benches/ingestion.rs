use criterion::{Criterion,criterion_group, criterion_main};
use std::hint::black_box;

fn ingestion_benchmark(c: &mut Criterion) {
    c.bench_function("ingestion_smoke", |b| {
        b.iter(|| {
            black_box("annotation,data");
        });
    });
}

criterion_group!(benches, ingestion_benchmark);
criterion_main!(benches);
