# REST-API server

This example shows how to provide the functionality of `calclib` as a REST-API service.  
For `calcserver` the [Actix Web](https://actix.rs/) web framework is used.

The example [calcclient.py](https://github.com/brmmm3/calc-rs/blob/master/calcserver/examples/calcclient.py) shows how to communicate with the web service in Python.

To run the server in `debug` mode just call

```rust
cargo run --bin calcserver
```

To run the server in `release` mode just call

```rust
cargo run --release --bin calcserver
```
