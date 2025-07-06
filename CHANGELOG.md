# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of WASM Wizard CLI
- Project scaffolding with Rust, JavaScript, and TypeScript templates
- Build optimization with wasm-opt integration
- Component composition using wasm-compose
- Development server with hot reload capability
- Toolchain management and installation
- Component analysis and health checking
- Language bindings generation
- Cross-platform binary distribution via npm and cargo
- GitHub Actions CI/CD pipeline
- Comprehensive test suite

### Features
- **Project Templates**: Battle-tested templates for Rust, JavaScript, and TypeScript
- **Smart Optimization**: Automatic size reduction and performance optimization
- **Component Composition**: Link multiple WASM components into a single optimized component
- **Developer Experience**: Hot reload, automatic rebuilds, and friendly error messages
- **Multi-platform**: Support for macOS, Linux, and Windows (x64 and ARM64)
- **Package Managers**: Available via both npm and cargo for maximum accessibility

### Commands
- `wasm-wizard new` - Create new WebAssembly component projects
- `wasm-wizard build` - Build and optimize components
- `wasm-wizard compose` - Compose multiple components
- `wasm-wizard optimize` - Optimize existing WASM files
- `wasm-wizard dev` - Start development server
- `wasm-wizard install` - Manage toolchain dependencies
- `wasm-wizard check` - Health check for components
- `wasm-wizard bindings` - Generate language bindings
- `wasm-wizard analyze` - Analyze component performance

### Technical Details
- Written in Rust for maximum performance and reliability
- Async/await throughout for responsive CLI experience
- Comprehensive error handling with helpful error messages
- Modular architecture supporting extensible templates and optimizations
- Zero-copy operations where possible for performance
- Cross-compilation support for all major platforms

## [0.1.0] - 2024-07-06

### Added
- Initial release of WASM Wizard
- Core CLI functionality
- Basic project templates
- Build and optimization features
- Component composition
- Developer tooling

---

## Release Notes

### What's New in 0.1.0

WASM Wizard makes WebAssembly Component Model development incredibly easy by providing:

🏗️ **Project Scaffolding**: Get started instantly with production-ready templates for Rust, JavaScript, and TypeScript.

⚡ **Smart Optimization**: Automatic size reduction of 50-80% through intelligent optimization strategies.

🔗 **Component Composition**: Effortlessly combine multiple WASM components into optimized single deployments.

🛠️ **Developer Experience**: Hot reload, automatic rebuilds, and comprehensive tooling integration.

📊 **Analysis Tools**: Deep insights into component performance, size, and structure.

### Installation

```bash
# Via npm (recommended)
npm install -g wasm-wizard

# Via cargo
cargo install wasm-wizard
```

### Quick Start

```bash
# Create a new component
wasm-wizard new my-component --language rust

# Build and optimize
cd my-component
wasm-wizard build --optimize

# Start development server
wasm-wizard dev --hot-reload
```

### Breaking Changes

None - this is the initial release.

### Migration Guide

This is the first release, so no migration is needed.

---

*Built with ❤️ by the WebAssembly community*