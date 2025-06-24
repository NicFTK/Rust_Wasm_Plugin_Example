# Wasm Plugins Examples

This repository serves as a hands-on testbed for building a plugin system with WebAssembly. It is designed to help you explore and validate the usage of relevant Rust crates in a real-world scenario.

## Dependencies

- wasmtime = "32.0.0"
- wit-bindgen = "0.42.0"
- wasmtime-wasi = "32.0.0"
- anyhow = "1.0.97"

## Project Goals

1. Use Wasmtime as the runtime host to execute WebAssembly-based plugins.
2. Compile existing Rust code into WebAssembly modules using wit-bindgen.
3. Share common data structures and utility functions between the host and plugin via a third-party crate.

This project will be published on GitHub as a reference implementation and learning resource.

