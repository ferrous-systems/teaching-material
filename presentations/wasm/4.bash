$ cargo new hello-world
$ cargo build --target wasm32-wasi
$ wasmtime target/wasm32-wasi/debug/main.wasm
