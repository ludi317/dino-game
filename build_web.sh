#!/bin/bash

# Build for web
cargo build --profile web --target wasm32-unknown-unknown

# Generate JS bindings
wasm-bindgen --target web --out-dir static --no-typescript \
    target/wasm32-unknown-unknown/web/my_bevy_game.wasm

# Copy assets to static directory (if you have any)
cp -r assets static/ 2>/dev/null || :

echo "Build complete! Files are in static/"