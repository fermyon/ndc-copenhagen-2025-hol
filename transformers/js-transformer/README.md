# JS-Transformer

Component implementing the Guest world using JavaScript.

[`jco`](https://github.com/bytecodealliance/jco#quickstart) is a required tool to compile this code into a JavaScript Wasm component.

To compile the component, you need to run the following command:

```shell
jco componentize \
--wit ../../wit/world.wit \
--world-name guest \
--out transformer.wasm \
--disable=all \
transformer.js
```

To "wac" (compose) this transformer with the host, run the following command:

```shell
wac plug ../../host/target/wasm32-wasip2/release/host.wasm --plug transformer.wasm -o ../../host/composed.wasm
```
