use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tomli_rust::parse;

/// Benchmarks small, simple documents to track latency.
fn bench_small_toml(c: &mut Criterion) {
    let doc = r#"
        title = "TOML Example"
        [owner]
        name = "Tom Preston-Werner"
    "#;
    c.bench_function("parse small TOML", |b| b.iter(|| parse(black_box(doc))));
}

/// Benchmarks datetime throughput against the Python regex implementation.
fn bench_datetime(c: &mut Criterion) {
    let doc = r#"
        dob = 1979-05-27T07:32:00-08:00
        local = 1979-05-27T07:32:00
    "#;
    c.bench_function("parse datetime", |b| b.iter(|| parse(black_box(doc))));
}

/// Benchmarks recursive array/table generation limits and allocations.
fn bench_deep_nesting(c: &mut Criterion) {
    let doc = r#"
        [a.b.c.d.e.f]
        val = [1, [2, [3, [4]]]]
    "#;
    c.bench_function("parse deep nesting and arrays", |b| {
        b.iter(|| parse(black_box(doc)))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = bench_small_toml, bench_datetime, bench_deep_nesting
);
criterion_main!(benches);
