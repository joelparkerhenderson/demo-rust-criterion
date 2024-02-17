use criterion::{criterion_group, criterion_main, Criterion};
use std::fmt::Write;
use rand::{distributions::Alphanumeric, Rng};

/// Generate a random string of length 8
#[inline]
pub fn random_item() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

/// Combine input strings into a single string,
/// via the function `fold` and an output string.
#[inline]
pub fn combine_via_fold(items: &Vec<String>) -> String {
    items.iter().fold(String::new(), |mut output, item| {
        let _ = writeln!(output, "<p>{item}</p>");
        output
    })
}

/// Combine input strings into a single string,
/// via the functions `map` `collect`.
#[inline]
pub fn combine_via_map_collect(items: &Vec<String>) -> String {
    items.iter().map(|item|
        format!("<p>{}</p>\n", &item)
    ).collect::<String>()
}

fn bench_combine(c: &mut Criterion){
    let items = (0..10000).map(|_| random_item()).collect();
    c.bench_function("combine_via_fold", |b| b.iter(|| combine_via_fold(&items)));    
    c.bench_function("combine_via_map_collect", |b| b.iter(|| combine_via_map_collect(&items)));
}

criterion_group!(benches, bench_combine);
criterion_main!(benches);
