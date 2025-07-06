# Getting Started with WASM Wizard

This example shows how to create, build, and optimize a WebAssembly component using WASM Wizard.

## 1. Create a New Component

```bash
# Create a Rust component
wasm-wizard new calculator --language rust

# Or create a JavaScript component
wasm-wizard new calculator-js --language javascript

# Or create a TypeScript component
wasm-wizard new calculator-ts --language typescript
```

## 2. Explore the Generated Project

```bash
cd calculator
tree .
```

You'll see:
```
calculator/
├── Cargo.toml           # Rust project configuration
├── src/
│   └── lib.rs           # Your component implementation
├── wit/
│   └── world.wit        # WebAssembly Interface Types definition
├── wasm-wizard.toml     # WASM Wizard configuration
└── .gitignore           # Git ignore file
```

## 3. Implement Your Component

Edit `src/lib.rs`:

```rust
wit_bindgen::generate!({
    world: "calculator",
    exports: {
        world: Calculator,
    },
});

struct Calculator;

impl Guest for Calculator {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
    
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
}
```

Update `wit/world.wit`:

```wit
package calculator:component@0.1.0;

world calculator {
    export add: func(a: s32, b: s32) -> s32;
    export subtract: func(a: s32, b: s32) -> s32;
    export multiply: func(a: s32, b: s32) -> s32;
    export divide: func(a: s32, b: s32) -> result<s32, string>;
}
```

## 4. Build and Optimize

```bash
# Build the component
wasm-wizard build

# Build with optimization
wasm-wizard build --optimize

# Or optimize a specific file
wasm-wizard optimize target/wasm32-wasip1/release/calculator.wasm
```

## 5. Analyze Your Component

```bash
# Get component information
wasm-wizard analyze target/wasm32-wasip1/release/calculator.wasm

# Detailed analysis
wasm-wizard analyze target/wasm32-wasip1/release/calculator.wasm --detailed
```

## 6. Generate Bindings

```bash
# Generate JavaScript bindings
wasm-wizard bindings target/wasm32-wasip1/release/calculator.wasm --language javascript

# Generate TypeScript bindings
wasm-wizard bindings target/wasm32-wasip1/release/calculator.wasm --language typescript
```

## 7. Development Server

```bash
# Start development server with hot reload
wasm-wizard dev --hot-reload
```

## 8. Component Composition

Create multiple components and compose them:

```bash
# Create additional components
wasm-wizard new auth --language rust
wasm-wizard new storage --language rust

# Build both
cd auth && wasm-wizard build --optimize && cd ..
cd storage && wasm-wizard build --optimize && cd ..

# Compose them
wasm-wizard compose auth/target/wasm32-wasip1/release/auth.wasm \
                   storage/target/wasm32-wasip1/release/storage.wasm \
                   --output composed-app.wasm \
                   --optimize
```

## 9. Check Component Health

```bash
# Check all components
wasm-wizard check --all

# Check specific component
wasm-wizard check target/wasm32-wasip1/release/calculator.wasm

# Auto-fix issues
wasm-wizard check --fix
```

## 10. Tool Management

```bash
# Install all required tools
wasm-wizard install --all

# Install specific tool
wasm-wizard install wasm-opt

# Check tool status
wasm-wizard install
```

## Real-World Usage

### Microservices

```bash
# Create microservices
wasm-wizard new user-service --template http
wasm-wizard new order-service --template http
wasm-wizard new notification-service --template http

# Build and compose
wasm-wizard build --optimize
wasm-wizard compose user.wasm order.wasm notification.wasm --output microservices.wasm
```

### Data Processing Pipeline

```bash
# Create pipeline components
wasm-wizard new data-ingestion --template data
wasm-wizard new data-transform --template data
wasm-wizard new data-export --template data

# Compose pipeline
wasm-wizard compose ingestion.wasm transform.wasm export.wasm --output pipeline.wasm
```

## Configuration

Customize `wasm-wizard.toml`:

```toml
[project]
name = "calculator"
language = "rust"
version = "0.1.0"

[build]
target = "wasm32-wasip1"
optimization_level = 3
strip_debug = true

[dev]
port = 8080
hot_reload = true
watch_paths = ["src/**/*", "wit/**/*"]
```

## Tips

1. **Size Optimization**: Use `--optimize` flag for production builds
2. **Development**: Use `wasm-wizard dev` for rapid iteration
3. **Composition**: Break large applications into smaller components
4. **Analysis**: Use `wasm-wizard analyze` to understand your components
5. **Tooling**: Let `wasm-wizard install` manage your WebAssembly toolchain

## Next Steps

- Explore different templates
- Try component composition
- Integrate with your existing applications
- Share your components with the community

Happy coding! 🧙‍♂️