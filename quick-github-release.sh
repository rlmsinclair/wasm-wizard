#!/bin/bash

echo "🚀 Quick GitHub Release for WASM Wizard"
echo ""

# Check if git is initialized
if [ ! -d .git ]; then
    echo "📝 Initializing git repository..."
    git init
    git add .
    git commit -m "Initial commit: WASM Wizard - Zero-config WebAssembly tooling"
fi

# Create GitHub repo
echo "🌟 Creating GitHub repository..."
gh repo create wasm-wizard --public --description "CLI tool that makes WebAssembly Component Model ridiculously easy to use" --push

# Build release binary
echo "🔨 Building release binary..."
cargo build --release

# Create release
echo "📦 Creating GitHub release..."
gh release create v0.1.0 \
    target/release/wasm-wizard \
    --title "WASM Wizard v0.1.0" \
    --notes "First release of WASM Wizard!

## Features
- 🏗️ Project scaffolding with battle-tested templates
- ⚡ Smart build optimization and size reduction
- 🔗 Component composition and linking made simple
- 🛠️ Developer-friendly tooling with hot reload
- 📊 Component analysis and performance insights

## Installation
Download the binary for your platform and add it to your PATH.

Or build from source:
\`\`\`bash
cargo install --git https://github.com/rlmsinclair/wasm-wizard
\`\`\`"

echo ""
echo "✅ GitHub release created!"
echo "🔗 Repository: https://github.com/rlmsinclair/wasm-wizard"
echo ""
echo "Now you can tell people to install with:"
echo "cargo install --git https://github.com/rlmsinclair/wasm-wizard"