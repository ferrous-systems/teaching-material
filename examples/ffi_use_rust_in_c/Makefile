all: example

rebuild: clean all

clean:
	rm -rf example ./target

example: example.c ./target/debug/libffi_use_rust_in_c.a
	cc -o example example.c ./target/debug/libffi_use_rust_in_c.a

./target/debug/libffi_use_rust_in_c.a: ./src/lib.rs ./Cargo.toml
	cargo build

.PHONY: all clean rebuild
