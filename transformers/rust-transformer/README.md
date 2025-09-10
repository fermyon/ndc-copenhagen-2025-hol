# Rust-Transformer

Component implementing the Guest world using Rust.

[`componentize-py`](https://github.com/bytecodealliance/componentize-py#getting-started) is a required tool to compile this code into a JavaScript Wasm component.

To compile the component, you need to run the following command:

```shell
cargo build --target wasm32-wasip2
```

To "wac" (compose) this transformer with the host, run the following command:

```shell
wac plug ../../host/target/wasm32-wasip2/release/host.wasm --plug target/wasm32-wasip2/debug/transformer.wasm -o ../../host/composed.wasm
```
