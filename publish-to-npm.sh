#!/bin/bash

echo "🧙‍♂️ Publishing WASM Wizard to npm..."
echo ""

# Update package.json
echo "📝 Updating package.json..."
cat > package.json << 'EOF'
{
  "name": "wasm-wizard",
  "version": "0.1.0",
  "description": "A CLI tool that makes WebAssembly Component Model ridiculously easy to use 🧙‍♂️",
  "main": "index.js",
  "bin": {
    "wasm-wizard": "./bin/wasm-wizard"
  },
  "scripts": {
    "postinstall": "echo 'WASM Wizard installed successfully! Run wasm-wizard --help to get started.'"
  },
  "keywords": [
    "wasm",
    "webassembly",
    "component-model",
    "cli",
    "tooling",
    "rust",
    "build-tool",
    "zero-config"
  ],
  "author": "rlmsinclair",
  "license": "MIT OR Apache-2.0",
  "repository": {
    "type": "git",
    "url": "git+https://github.com/rlmsinclair/wasm-wizard.git"
  },
  "bugs": {
    "url": "https://github.com/rlmsinclair/wasm-wizard/issues"
  },
  "homepage": "https://github.com/rlmsinclair/wasm-wizard#readme",
  "engines": {
    "node": ">=14.0.0"
  },
  "files": [
    "bin/",
    "README.md",
    "LICENSE-MIT",
    "LICENSE-APACHE"
  ],
  "publishConfig": {
    "access": "public"
  }
}
EOF

# Build the Rust binary
echo "🔨 Building release binary..."
cargo build --release

# Create bin directory and copy binary
echo "📦 Preparing npm package..."
mkdir -p bin
cp target/release/wasm-wizard bin/

# Make it executable
chmod +x bin/wasm-wizard

# Create a simple wrapper for npm
cat > bin/wasm-wizard.js << 'EOF'
#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');

const binary = path.join(__dirname, 'wasm-wizard');
const args = process.argv.slice(2);

const child = spawn(binary, args, { stdio: 'inherit' });

child.on('exit', (code) => {
  process.exit(code);
});
EOF

chmod +x bin/wasm-wizard.js

# Update bin in package.json to point to the JS wrapper
sed -i '' 's|"./bin/wasm-wizard"|"./bin/wasm-wizard.js"|' package.json

echo ""
echo "✅ Package prepared!"
echo ""
echo "To publish:"
echo "1. npm login"
echo "2. npm publish"
echo ""
echo "Or test locally first:"
echo "npm link"
echo "wasm-wizard --help"