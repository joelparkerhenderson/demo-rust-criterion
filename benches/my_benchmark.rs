use criterion::{criterion_group, criterion_main, Criterion};
use demo_rust_criterion::*;

fn bench_combine(c: &mut Criterion){
    let items = (0..10000).map(|_| random_item()).collect();
    c.bench_function("combine_via_fold", |b| b.iter(|| combine_via_fold(&items)));    
    c.bench_function("combine_via_collect", |b| b.iter(|| combine_via_collect(&items)));
    c.bench_function("combine_via_rayon", |b| b.iter(|| combine_via_rayon(&items)));
}

criterion_group!(benches, bench_combine);
criterion_main!(benches);
