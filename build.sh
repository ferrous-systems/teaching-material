#!/bin/bash

LOG_FILE=build.log

SUCCESS=1

try_compile() {
	CRATE=$1
	echo ">> Building ${CRATE}" | tee -a ${LOG_FILE}
	cargo build --manifest-path=${CRATE}
	if [ $? -ne 0 ]; then
		echo "!! Failed to build ${CRATE}" | tee -a ${LOG_FILE}
		SUCCESS=0
	fi 
}

# These are not supposed to compile
# try_compile ./assignments/IO-assignment/template/Cargo.toml
# try_compile ./assignments/async-chat-template/Cargo.toml
# try_compile ./assignments/_templates/concurrency/Cargo.toml
try_compile ./assignments/solutions/threaded-mailbox/Cargo.toml
try_compile ./assignments/solutions/threaded-mailbox/mailbox/Cargo.toml
try_compile ./assignments/solutions/threaded-mailbox/redisish/Cargo.toml
try_compile ./assignments/solutions/semver-client-server/semver-client-async/Cargo.toml
try_compile ./assignments/solutions/semver-client-server/semver-client/Cargo.toml
try_compile ./assignments/solutions/semver-client-server/semver-server-threaded/Cargo.toml
try_compile ./assignments/solutions/semver-client-server/semver-server-actix/Cargo.toml
try_compile ./assignments/solutions/semver-client-server/semver-server-async-channels/Cargo.toml
try_compile ./assignments/solutions/semver-client-server/semver-server/Cargo.toml
try_compile ./assignments/solutions/semver-client-server/semver-server-async/Cargo.toml
try_compile ./assignments/solutions/semver-client-server/semver-api/Cargo.toml
try_compile ./assignments/solutions/redis/Cargo.toml
try_compile ./assignments/solutions/async-std-mailbox-tracing/Cargo.toml
try_compile ./assignments/solutions/async-std-mailbox/Cargo.toml
try_compile ./assignments/solutions/async-std-mailbox-connection-limit/Cargo.toml
try_compile ./assignments/solutions/tcp-echo-server/Cargo.toml
try_compile ./assignments/solutions/connected-mailbox/Cargo.toml
try_compile ./assignments/solutions/tcp-client/Cargo.toml
try_compile ./assignments/solutions/calc/Cargo.toml
try_compile ./assignments/solutions/tracing-future/Cargo.toml
try_compile ./assignments/solutions/semver/Cargo.toml
try_compile ./assignments/solutions/redisish/Cargo.toml
try_compile ./assignments/solutions/actix-semver/Cargo.toml
try_compile ./assignments/solutions/fill_in_the_blanks/Cargo.toml
try_compile ./assignments/solutions/green-yellow/Cargo.toml
try_compile ./assignments/solutions/redis-protobuf/Cargo.toml
try_compile ./assignments/solutions/fizzbuzz/Cargo.toml
try_compile ./assignments/solutions/durable_file/Cargo.toml
try_compile ./assignments/solutions/actix/chat-websockets/Cargo.toml
try_compile ./assignments/_preliminary/async_parser_codealong/Cargo.toml
try_compile ./assignments/_preliminary/calculator/calc-ffi/Cargo.toml
try_compile ./assignments/_preliminary/calculator/calc/Cargo.toml
try_compile ./assignments/semver/parse_file/template/Cargo.toml
try_compile ./assignments/semver/parse_file/solution/Cargo.toml
try_compile ./assignments/leveldb-mdbook/solution/Cargo.toml
try_compile ./assignments/actix/chat-websockets/Cargo.toml
try_compile ./examples/ffi_use_c_in_rust/Cargo.toml
try_compile ./examples/ffi_use_rust_in_c/Cargo.toml
# Has uncompilable names for the examples
# try_compile ./semver-codealong/Cargo.toml

if [ ${SUCCESS} -ne 1 ]; then
	echo "Build failed"
	exit 1
else
	echo "Build OK"
	exit 1
fi
