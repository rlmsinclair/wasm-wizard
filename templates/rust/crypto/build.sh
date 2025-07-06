#!/bin/bash
# Build script for high-performance WASM

echo "🔨 Building {{name}} with wasm-pack..."
wasm-pack build --target web --out-dir pkg

echo "✨ Build complete! Your WASM module is in pkg/"
echo "📊 To see the 7x performance demo, open demo.html in a browser"