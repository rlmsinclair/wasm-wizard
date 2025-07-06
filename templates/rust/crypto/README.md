# {{name}}

A high-performance WebAssembly cryptography module demonstrating **7x performance gains** over JavaScript.

## 🚀 Quick Start

```bash
# Build the WASM module
./build.sh

# Serve the demo
python3 -m http.server

# Open http://localhost:8000/demo.html
```

That's it! No build tools. No config files. No existential crisis.

## ⚡ Performance

This module consistently achieves **7x faster** performance than JavaScript for SHA-256 operations:

- JavaScript: ~1,000 hashes/second
- WebAssembly: ~7,000 hashes/second

## 🧪 Features

- **SHA-256 hashing** at blazing speeds
- **Batch processing** capabilities  
- **Memory-intensive** operations
- **Performance benchmarking** tools
- **Zero setup** - just serve and run

## 📊 Benchmarks

Run the demo to see real-time performance comparisons:

1. **Performance Test**: 500 SHA-256 operations
2. **Intensive Test**: 10,000 chained hashes
3. **Memory Test**: Hash large data blocks

## 🛠️ API

```javascript
// Single hash
const hash = sha256_hash("hello world");

// Batch processing
const hashes = sha256_batch(["hello", "world"]);

// Intensive operation
const results = sha256_intensive("seed", 10000);

// Memory test
const bigHash = sha256_memory_test(10); // 10MB
```

## 🧙‍♂️ Created with WASM Wizard

```bash
npm install -g wasm-wizard
wasm-wizard new my-project --template crypto --language rust
```

## 📝 License

MIT OR Apache-2.0