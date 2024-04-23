# `calc-rs`

`calc-rs` is a template project for using Rust in Python, Java, WASM and as Rest-API service.

The project is a workspace containing the following packages:

- [Rust](calclib/README.md). This is the main package containing all the logic. It is intended to be used in other Rust projects without any dependency to [pyo3](https://github.com/pyo3/pyo3) and [jni](https://github.com/jni-rs/jni-rs).
- [Python](pycalclib/README.md). The Python layer for creating the Python module.
- [Java](java/README.md). The Java layer for creating the Java module.
- [WASM](wasmcalc/README.md). The Web Assembly layer for creating the WASM module.
- [REST-API](calcserver/README.md). The REST-API server extension.

## Recommendations

### IDE

For programming Rust the best IDE is VS Code with the following plugins:

- Rust Extension Pack
  - rust-analyzer
  - crates
  - Even Better TOML
- Prettier - Code formatter (Rust)
- Microsoft C++ on Windows
- CodeLLDB on maxOS / Linux
- Python, if you plan to develop Python extensions.

A short guide can be found [here](https://code.visualstudio.com/docs/languages/rust).

A promising alternative is [RustRover](https://www.jetbrains.com/rust). A feature rich Rust IDE from JetBrains. Currently preview releases are available. If you plan to develop Python extensions then you should install the Python plugin.

## Code quality

To improve the code quality I use [clippy](https://doc.rust-lang.org/stable/clippy/usage.html), which is a very powerful linter.

## Conclusion

Rust is a very versatile power horse. It is easy to use Rust in most used programming languages. For web applications Rust can be used on the server side with REST-API and on the client side with the [wasm-pack](https://github.com/rustwasm/wasm-pack).

My preferred combination is Python and Rust. If the application has a UI I usually use Qt. For very simple UIs [FLTK](https://fltk-rs.github.io/fltk-rs) is preferred for smaller executables and easier license handling. In Rust I implement the performance and security critical parts. The rest in Python for reduced development costs.

If speed and / or security matters I would really recommend using Rust. The power features of Rust helps you reducing bugs and follow up costs.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
