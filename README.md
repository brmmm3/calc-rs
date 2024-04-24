# `calc-rs`

`calc-rs` is a template project for using Rust in Python, Java, WASM and as Rest-API service.

The project is a workspace containing the following packages:

- [Rust](calclib/README.md). This is the main package containing all the logic. It is intended to be used in other Rust projects without any dependency to [pyo3](https://github.com/pyo3/pyo3) and [jni](https://github.com/jni-rs/jni-rs).
- [Python](pycalclib/README.md). The Python layer for creating the Python module.
- [Java](java/README.md). The Java layer for creating the Java module.
- [WASM](wasmcalc/README.md). The Web Assembly layer for creating the WASM module.
- [REST-API](calcserver/README.md). The REST-API server extension.

## Recommendations

For learning Rust you should read this [book](https://doc.rust-lang.org/stable/book).
You REALLY should read this book before you start coding, because Rust is not easy to learn. Especially to understand and solve the findings of the Rust compiler can drive you crazy. But in the end, after solving all the findings you'll understand that the compiler was absolutely right and it saved you a lot of future work finding and fixing the bugs by yourself.

For testing simple code examples online, without installing Rust locally, you can use the [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021).

If you've issues you can't fix then it is a good idea to ask in the [user forum](https://users.rust-lang.org). My experience is that you'll get useful help within a short time.
When asking questions you can use the playground to paste a short (working) example, which describes your problem.

And last but not least at [This Week in Rust](https://this-week-in-rust.org) you'll find interesting news and projects.

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
