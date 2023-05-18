use criterion::{criterion_group, criterion_main, Criterion};
use losses::summarize_hash;
use std::path::Path;

pub fn benchmark_summarize_hash(c: &mut Criterion) {
    c.bench_function("summarize hash", |b| {
        b.iter(|| summarize_hash(&Path::new("big.csv")))
    });
}

criterion_group!(benches, benchmark_summarize_hash);
criterion_main!(benches);
