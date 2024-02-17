use criterion::{criterion_group, criterion_main, Criterion};
use std::fmt::Write;
use rand::{distributions::Alphanumeric, Rng};
use rayon::prelude::*;

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
/// 
/// Pro: fastest, because it can format each string,
/// accumulate it, then release that string's memory.
/// 
/// Con: mutable and non-parallelizable.
/// 
#[inline]
pub fn combine_via_fold(items: &Vec<String>) -> String {
    items.iter().fold(String::new(), |mut output, item| {
        let _ = writeln!(output, "<p>{item}</p>");
        output
    })
}

/// Combine input strings into a single string,
/// via the function `collect` into one string.
/// 
/// Pro: semantically the most flexible, and the easiest
/// to debug, because each item is formatted indpendently.
/// 
/// Con: slowest, because it has to hold memory for all strings.
/// 
#[inline]
pub fn combine_via_collect(items: &Vec<String>) -> String {
    items.iter().map(|item|
        format!("<p>{}</p>\n", &item)
    ).collect::<String>()
}

/// Combine input strings into a single string,
/// via the rayon crate functions `map` and `reduce`.
///
/// Pro: parallel, and semantically the best, because it's
/// explicitly doing a map…reduce design pattern.
/// 
/// Con: requires rayon dependency, and also isn't fast.
/// 
#[inline]
pub fn combine_via_rayon(items: &Vec<String>) -> String {
    items.par_iter().map(|item|
        format!("<p>{}</p>\n", &item)
    ).reduce(||
        String::new(),
        |a, b| [a, b].concat()
    )
}

fn bench_combine(c: &mut Criterion){
    let items = (0..10000).map(|_| random_item()).collect();
    c.bench_function("combine_via_fold", |b| b.iter(|| combine_via_fold(&items)));    
    c.bench_function("combine_via_collect", |b| b.iter(|| combine_via_collect(&items)));
    c.bench_function("combine_via_rayon", |b| b.iter(|| combine_via_rayon(&items)));
}

criterion_group!(benches, bench_combine);
criterion_main!(benches);
