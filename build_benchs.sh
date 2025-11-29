#!/usr/bin/env bash

cd rust_benchs
WASM_BENCHS_LOCATION=../web_benchmark/public/rust_benchs_pkg
rm -rf $WASM_BENCHS_LOCATION
if ! command -v wasm-pack &> /dev/null
then
  echo "wasm-pack could not be found, installing it..."
  cargo install wasm-pack
fi
wasm-pack build --release --target web --out-dir $WASM_BENCHS_LOCATION