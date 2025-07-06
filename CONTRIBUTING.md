# Contributing to WASM Wizard

🧙‍♂️ Thank you for your interest in contributing to WASM Wizard! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) (for npm publishing)
- [Git](https://git-scm.com/)

### Development Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/wasm-wizard.git
   cd wasm-wizard
   ```

3. Set up the development environment:
   ```bash
   make setup
   ```

4. Test that everything works:
   ```bash
   make test-cli
   ```

## Development Workflow

### Making Changes

1. Create a new branch for your feature/fix:
   ```bash
   git checkout -b feature/awesome-feature
   ```

2. Make your changes and test them:
   ```bash
   make test
   ```

3. Format and lint your code:
   ```bash
   make fmt
   make lint
   ```

4. Commit your changes:
   ```bash
   git commit -m "Add awesome feature"
   ```

5. Push to your fork:
   ```bash
   git push origin feature/awesome-feature
   ```

6. Create a Pull Request

### Code Style

- Follow standard Rust conventions
- Use `cargo fmt` to format code
- Fix all `cargo clippy` warnings
- Write tests for new functionality
- Add documentation for public APIs

### Testing

- Run the full test suite: `make test`
- Run integration tests: `make test-integration`
- Test the CLI manually: `make test-cli`

## Types of Contributions

### Bug Reports

When filing a bug report, please include:
- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment information (OS, Rust version, etc.)
- Minimal reproduction case if possible

### Feature Requests

For new features:
- Describe the use case
- Explain why it would be valuable
- Consider implementation complexity
- Discuss alternative approaches

### Code Contributions

We welcome contributions in these areas:

#### Templates
- New language templates
- Improved existing templates
- Template customization options

#### Optimizations
- New optimization strategies
- Performance improvements
- Size reduction techniques

#### Developer Experience
- Better error messages
- Improved CLI UX
- Enhanced debugging tools

#### Platform Support
- New platform targets
- Installation improvements
- Cross-compilation fixes

## Project Structure

```
wasm-wizard/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── commands/            # Command implementations
│   │   ├── new.rs           # Project scaffolding
│   │   ├── build.rs         # Build and optimization
│   │   ├── compose.rs       # Component composition
│   │   └── ...
│   ├── scaffolder.rs        # Project template system
│   ├── optimizer.rs         # WASM optimization
│   ├── composer.rs          # Component composition
│   └── utils.rs             # Utility functions
├── templates/               # Project templates
│   ├── rust/
│   ├── javascript/
│   └── typescript/
├── tests/                   # Integration tests
└── docs/                    # Documentation
```

## Adding New Templates

1. Create template directory structure:
   ```
   templates/language/template-name/
   ├── files...
   └── ...
   ```

2. Add template files with Handlebars placeholders:
   ```rust
   // Example: {{name}} becomes the project name
   ```

3. Register the template in `src/scaffolder.rs`:
   ```rust
   self.templates.insert("language-template".to_string(), template_info);
   ```

4. Add tests for the new template

## Adding New Commands

1. Create command module in `src/commands/`:
   ```rust
   // src/commands/your_command.rs
   use super::{Command, YourCommand};
   
   #[async_trait]
   impl Command for YourCommand {
       async fn execute(&self) -> Result<()> {
           // Implementation
       }
   }
   ```

2. Add command to `src/commands/mod.rs`

3. Register in `src/main.rs`

4. Add integration tests

## Release Process

1. Update version in `Cargo.toml` and `package.json`
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag v0.1.0`
4. Push tag: `git push origin v0.1.0`
5. GitHub Actions will automatically build and publish

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help create a welcoming environment
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)

## Questions?

- Open an issue for bugs or feature requests
- Join the discussion in existing issues
- Reach out to maintainers for questions

## Recognition

Contributors will be recognized in:
- Release notes
- README contributors section
- Git commit history

Thank you for helping make WebAssembly development more accessible! 🚀