use std::fmt::Write;
use rand::{distributions::Alphanumeric, Rng};
use rayon::prelude::*;

/// Generate a random string of given length
#[inline]
pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
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
/// to debug, because each item is formatted independently.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implementations() {
        let len = 8;
        let items = (0..10).map(|_| random_string(len)).collect();
        let actual_0 = combine_via_fold(&items);
        let actual_1 = combine_via_collect(&items);
        let actual_2 = combine_via_rayon(&items);
        assert_eq!(actual_0, actual_1, "combine_via_fold and combine_via_collect should be the same");
        assert_eq!(actual_0, actual_2, "combine_via_fold and combine_via_rayon should be the same");
    }
}
