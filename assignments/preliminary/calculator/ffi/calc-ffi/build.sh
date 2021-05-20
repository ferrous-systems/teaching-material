#!/bin/bash

# this is a legacy script
echo "use cargo build-ffi"
exit 1

# preserved for documentation purposes

set -euxo pipefail

cargo build
cbindgen -d -l c > ./calc_ffi.h

gcc \
    -I. \
    ./demo.c \
    -L../target/debug \
    -l:libcalc_ffi.a \
    -lpthread \
    -ldl \
    -o demo
