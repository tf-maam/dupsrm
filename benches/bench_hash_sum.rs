use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use dupsrm::hasher::*;
use std::path::Path;

pub fn benchmark_sha256sum(c: &mut Criterion) {
    let path = Path::new("test/test_large");
    let mut group = c.benchmark_group("flat-sampling-hash-sum");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("sha256sum", |b| b.iter(|| sha256sum(path)));
    group.finish();
}

pub fn benchmark_sha3_256sum(c: &mut Criterion) {
    let path = Path::new("test/test_large");
    let mut group = c.benchmark_group("flat-sampling-hash-sum");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("sha3_256sum", |b| b.iter(|| sha3_256sum(path)));
    group.finish();
}

pub fn benchmark_sha1sum(c: &mut Criterion) {
    let path = Path::new("test/test_large");
    let mut group = c.benchmark_group("flat-sampling-hash-sum");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("sha1sum", |b| b.iter(|| sha1sum(path)));
    group.finish();
}

pub fn benchmark_md5sum(c: &mut Criterion) {
    let path = Path::new("test/test_large");
    let mut group = c.benchmark_group("flat-sampling-hash-sum");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("md5sum", |b| b.iter(|| md5sum(path)));
    group.finish();
}

pub fn benchmark_whirlpool_sum(c: &mut Criterion) {
    let path = Path::new("test/test_large");
    let mut group = c.benchmark_group("flat-sampling-hash-sum");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("whirlpool_sum", |b| b.iter(|| whirlpool_sum(path)));
    group.finish();
}

pub fn benchmark_ripemd160_sum(c: &mut Criterion) {
    let path = Path::new("test/test_large");
    let mut group = c.benchmark_group("flat-sampling-hash-sum");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("ripemd160_sum", |b| b.iter(|| ripemd160_sum(path)));
    group.finish();
}

pub fn benchmark_blake256_sum(c: &mut Criterion) {
    let path = Path::new("test/test_large");
    let mut group = c.benchmark_group("flat-sampling-hash-sum");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("blake256_sum", |b| b.iter(|| blake256_sum(path)));
    group.finish();
}

criterion_group!(
    benches,
    benchmark_sha256sum,
    benchmark_sha3_256sum,
    benchmark_sha1sum,
    benchmark_md5sum,
    benchmark_whirlpool_sum,
    benchmark_ripemd160_sum,
    benchmark_blake256_sum,
);
criterion_main!(benches);
