#!/bin/bash

# $ wasm-pack  --version
# wasm-pack 0.13.1
#
# $ rustc --version
# rustc 1.88.0 (6b00bc388 2025-06-23)
#
# $ node --version
# v22.15.0

set -xe

cd native-app
cargo build --release --target x86_64-unknown-linux-musl
cd -

cd ffi-wasm
cargo build --release --target wasm32-unknown-unknown
cd -

rm -r ffi-wasm/pkg
wasm-pack build ffi-wasm --target nodejs

clear
./target/x86_64-unknown-linux-musl/release/native-app $(realpath ./main.mts)
