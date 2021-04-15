#!/bin/bash
set -eu

# ./setup_web.sh # <- call this first!

# This is required to enable the web_sys clipboard API which egui_web uses
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
# https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis

# Clear output from old stuff:
rm -f docs/presx.wasm

echo "Building rust…"
BUILD=release
cargo build --release -p presx --lib --target wasm32-unknown-unknown

echo "Generating JS bindings for wasm…"
wasm-bindgen "target/wasm32-unknown-unknown/${BUILD}/presx.wasm" \
  --out-dir docs --no-modules --no-typescript

echo "Finished: docs/presx.wasm"
