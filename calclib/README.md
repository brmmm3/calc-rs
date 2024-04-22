# `calclib`

This is the pure Rust library without any `pyo3` or `jni` dependencies.  
This package contains all the logic and is intended to be used in other Rust projects.

Build the library in debug mode with

```sh
cargo build
```

and in release mode with

```sh
cargo build --release
```

Run tests with

```sh
cargo test
```

Run the example with

```sh
cargo run --example calculator -- +1.5 -0.9
```

## Benchmarks

The example `calc.rs` in `benches` shows how to implement benchmarking with [criterion](https://docs.rs/criterion/latest/criterion).

Run the benchmarks with the following command

```sh
cargo bench
```
