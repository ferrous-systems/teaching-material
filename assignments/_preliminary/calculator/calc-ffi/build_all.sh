#!/bin/bash 
set -euxo pipefail

# build native library
cargo build

# build executable that links to library
gcc \
    demo.c \
    -L./target/debug \
    -lcalc_ffi \
    -lpthread \
    -ldl \
    -lm \
    -o demo