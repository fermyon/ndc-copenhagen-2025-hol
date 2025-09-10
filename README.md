# NDC Copenhagen 2025 - Hands-On Labs

## Prerequisites

- Spin CLI (`3.4.0` or newer)
- Rust (`1.8.9` or newer) to compile the host
- WebAssembly Composition Tooling (`wac`)
  - [https://github.com/bytecodealliance/wac](https://github.com/bytecodealliance/wac)

# Composing With a Custom Transformer

To compose a new proxy with a custom transformer component, use the commands outlined here:

```bash
# Assume a obfuscate component has been implement with Rust in ./components/obfuscate
cd components/obfuscate
cargo build --target wasm32-wasip2 --release

cd ../..

cd host
spin build

#Compose the final application
wac plug --plug ./transformers/rust-transformer/target/wasm32-wasip2/release/obfuscate.wasm \
  target/wasm32-wasip2/release/host.wasm \
  -o composed.wasm
```

You can find examples of transformers in the /transformers directory.

# Run the proxy on your machine

Use the `spin up` command to run the proxy on your local machine.

# The-Do-Nothing Proxy

A `composed.wasm` has been added to the Git repo, to run that "do-nothing" proxy simply execute `spin up` in the `./host` directory
