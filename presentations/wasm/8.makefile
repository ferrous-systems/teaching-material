SHELL := /bin/bash

all:
	cargo build --target=wasm32-unknown-emscripten --release
	cp target/wasm32-unknown-emscripten/release/deps/*.wasm site/site.wasm
	cp target/wasm32-unknown-emscripten/release/deps/*[!.asm].js site/site.js