#!/bin/bash
set -e

# Build for web
cargo build --profile web --target wasm32-unknown-unknown

# Generate JS bindings
# Cargo names the binary artifact after the package, dash and all, while the
# page loads the underscored name, so spell the output name out.
wasm-bindgen --target web --out-dir static --out-name dino_game --no-typescript \
    target/wasm32-unknown-unknown/web/dino-game.wasm

echo "Build complete! Files are in static/"