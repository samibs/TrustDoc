#!/bin/bash
set -e

# Build WASM package
echo "Building WASM package..."
wasm-pack build --target web --out-dir pkg

echo "WASM build complete! Output in pkg/"

