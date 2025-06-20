#!/bin/bash
set -e

# Build for web
cargo build --profile web --target wasm32-unknown-unknown

# Generate JS bindings
wasm-bindgen --target web --out-dir static --no-typescript \
    target/wasm32-unknown-unknown/web/my_bevy_game.wasm

echo "Build complete! Files are in static/"