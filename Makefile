# WASM Wizard Development Makefile

.PHONY: all build test fmt lint clean install dev release

# Default target
all: fmt lint test build

# Build the project
build:
	cargo build

# Build release version
release:
	cargo build --release

# Run tests
test:
	cargo test

# Run integration tests
test-integration:
	cargo test --test integration_test

# Format code
fmt:
	cargo fmt

# Run linter
lint:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Install locally
install:
	cargo install --path .

# Development build with watch
dev:
	cargo watch -x build

# Run a quick test of the CLI
test-cli: build
	./target/debug/wasm-wizard --help
	./target/debug/wasm-wizard new test-project --no-git --no-install
	ls -la test-project/
	rm -rf test-project/

# Install development dependencies
dev-deps:
	cargo install cargo-watch
	rustup target add wasm32-wasip1

# Check that everything is ready for release
check-release: fmt lint test
	cargo build --release
	cargo test --release
	./target/release/wasm-wizard --help

# Generate documentation
docs:
	cargo doc --open

# Run benchmarks (if any)
bench:
	cargo bench

# Install required tools for WebAssembly development
install-wasm-tools:
	cargo install wasm-tools
	cargo install wasm-opt
	cargo install wit-bindgen-cli
	cargo install wasm-compose

# Quick setup for new developers
setup: dev-deps install-wasm-tools
	@echo "🧙‍♂️ WASM Wizard development environment setup complete!"
	@echo "Run 'make test-cli' to test the CLI"