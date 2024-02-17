# Demo Rust criterion

Demonstration of:

* the Rust programming language

* the Criterion benchmark crate

This demonstration shows the timing difference for some simple functions that combine strings

See the code:

* [benches/my_benchmark.rs](benches/my_benchmark.rs)

Run:

```sh
cargo bench
```

Result:

```txt
Running benches/my_benchmark.rs …
combine_via_fold     time: [… µs 162.11 …]
combine_via_collect  time: [… µs 336.00 …]
combine_via_rayon    time: [… µs 308.28 …]

```
