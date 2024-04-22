# `calc-rs`

`calc-rs` is a template project for using Rust in Python, Java, WASM and as Rest-API service.

The project is a workspace containing the following packages:

- [Rust](calclib/README.md). This is the main package containing all the logic. It is intended to be used in other Rust projects without any dependency to `pyo3` and `jni`.
- [Python](pycalclib/README.md). The Python layer for creating the Python module.
- [Java](java/README.md). The Java layer for creating the Java module.
- [WASM](wasmcalc/README.md). The Web Assembly layer for creating the WASM module.
- [REST-API](calcserver/README.md). The REST-API server extension.

## Conclusion

Rust is a very versatile power horse. It is easy to use Rust in most used programming languages. For web applications Rust can be used on the server side with REST-API and on the client side with the [wasm-pack](https://github.com/rustwasm/wasm-pack).

My preferred combination is Python and Rust. If the application has a UI I usually use Qt. For very simple UIs [FLTK](https://fltk-rs.github.io/fltk-rs) is preferred for smaller executables and easier license handling.

If speed and / or security matters I would really recommend using Rust. The power features of Rust helps you reducing bugs and follow up costs.
