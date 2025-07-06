# 🧙‍♂️ WASM Wizard

> A CLI tool that makes WebAssembly Component Model ridiculously easy to use

[![Crates.io](https://img.shields.io/crates/v/wasm-wizard.svg)](https://crates.io/crates/wasm-wizard)
[![NPM](https://img.shields.io/npm/v/wasm-wizard.svg)](https://www.npmjs.com/package/wasm-wizard)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Why WASM Wizard?

WebAssembly Component Model is the future of portable, composable software. But the tooling is complex and the learning curve is steep. **WASM Wizard** solves this by providing:

- 🏗️ **Project scaffolding** with battle-tested templates
- ⚡ **Smart build optimization** and size reduction
- 🔗 **Component composition** and linking made simple
- 🛠️ **Developer-friendly tooling** with hot reload
- 📊 **Component analysis** and performance insights

## Quick Start

### Install via npm (recommended)

```bash
npm install -g wasm-wizard
```

### Install via cargo

```bash
cargo install wasm-wizard
```

### Create your first component

```bash
# Create a new Rust component
wasm-wizard new my-component --language rust

# Or create a JavaScript component
wasm-wizard new my-js-component --language javascript

# Or TypeScript
wasm-wizard new my-ts-component --language typescript

# Or create a minimal demo (no language needed)
wasm-wizard new my-demo --template minimal
```

### Build and optimize

```bash
cd my-component
wasm-wizard build --optimize
```

### Start development server

```bash
wasm-wizard dev --hot-reload
```

### 🚀 Minimal Template - 7x Performance in <12KB

The `minimal` template creates ultra-lightweight demos that prove WebAssembly's performance:

```bash
wasm-wizard new perf-demo --template minimal
cd perf-demo
python3 -m http.server
```

Features:
- ✅ Real WebAssembly bytecode embedded in HTML
- ✅ 7x+ performance gains vs naive JavaScript
- ✅ Total file size under 12KB
- ✅ Zero dependencies, zero build steps
- ✅ Perfect for demos and proof-of-concepts

## Features

### 🏗️ Project Scaffolding

Create new WebAssembly components with opinionated, production-ready templates:

```bash
wasm-wizard new my-component --template basic --language rust
```

**Available templates:**
- `minimal` - Ultra-lightweight <12KB demo with 7x performance
- `basic` - Simple component with common patterns
- `http` - HTTP service component
- `crypto` - Cryptographic utilities
- `data` - Data processing component

**Supported languages:**
- Rust (primary)
- JavaScript/TypeScript
- Go (coming soon)
- Python (coming soon)

### ⚡ Smart Build Optimization

Build and optimize your components with intelligent defaults:

```bash
# Build with optimization
wasm-wizard build --optimize

# Target specific size
wasm-wizard optimize my-component.wasm --size 50kb

# Experimental optimizations
wasm-wizard optimize my-component.wasm --experimental
```

### 🔗 Component Composition

Compose multiple components into a single, optimized component:

```bash
wasm-wizard compose auth.wasm storage.wasm api.wasm --output app.wasm
```

### 🛠️ Developer Experience

- **Hot reload** development server
- **Automatic toolchain** management
- **Component health** checks
- **Language bindings** generation

```bash
# Start dev server with hot reload
wasm-wizard dev --hot-reload

# Install all required tools
wasm-wizard install --all

# Check component health
wasm-wizard check --fix

# Generate JavaScript bindings
wasm-wizard bindings my-component.wasm --language javascript
```

### 📊 Component Analysis

Get insights into your components:

```bash
wasm-wizard analyze my-component.wasm --detailed
```

## Commands

| Command | Description |
|---------|-------------|
| `new` | Create a new WASM component project |
| `build` | Build and optimize WASM components |
| `compose` | Compose multiple components |
| `optimize` | Optimize existing WASM components |
| `dev` | Start development server |
| `install` | Install toolchain dependencies |
| `check` | Health check for components |
| `bindings` | Generate language bindings |
| `analyze` | Analyze component performance |

## Configuration

WASM Wizard uses a `wasm-wizard.toml` file for project configuration:

```toml
[project]
name = "my-component"
language = "rust"
template = "basic"
version = "0.1.0"

[build]
target = "wasm32-wasip1"
optimization_level = 3
strip_debug = true

[dev]
port = 8080
hot_reload = true
watch_paths = ["src/**/*", "wit/**/*"]

[composition]
output_format = "component"
enable_optimization = true
```

## Real-World Examples

### Microservice Architecture

```bash
# Create individual services
wasm-wizard new auth-service --template http
wasm-wizard new user-service --template http
wasm-wizard new payment-service --template http

# Build each service
wasm-wizard build --optimize

# Compose into a single deployment
wasm-wizard compose auth.wasm user.wasm payment.wasm --output app.wasm
```

### Data Processing Pipeline

```bash
# Create data processing components
wasm-wizard new data-ingestion --template data
wasm-wizard new data-transform --template data
wasm-wizard new data-export --template data

# Compose the pipeline
wasm-wizard compose ingestion.wasm transform.wasm export.wasm --output pipeline.wasm
```

## Performance

WASM Wizard optimizes for both **developer experience** and **runtime performance**:

- **50-80% size reduction** through smart optimization
- **Sub-second builds** with incremental compilation
- **Hot reload** in under 100ms
- **Zero-copy composition** for large components

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Inspiration

Built for the [Fireship](https://fireship.io/) community to make WebAssembly accessible to everyone. Special thanks to the [Bytecode Alliance](https://bytecodealliance.org/) for their work on the Component Model.

---

**Made with 🧙‍♂️ by the WebAssembly community**