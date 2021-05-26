#!/bin/sh
mkdir -p /tmp/r
fd .rs  -x rustc -A warnings --crate-type=lib --out-dir=/tmp/r "{}" \;
