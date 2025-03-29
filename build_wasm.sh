#!/bin/bash
set -euo pipefail

# Build the project
cargo build -p crayon_app --target wasm32-unknown-unknown --release

# Generate JavaScript bindings
wasm-bindgen --out-dir ./www/src/pkg --target web target/wasm32-unknown-unknown/release/crayon_app.wasm

# Optionally, optimize the WASM binary
wasm-opt -Os -o ./www/src/pkg/crayon_app_bg.wasm ./www/src/pkg/crayon_app_bg.wasm


# Copy assets from /app/assets to /www/assets
mkdir -p www/public/assets
cp -R app/assets/* www/public/assets/