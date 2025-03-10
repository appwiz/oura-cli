# Oura CLI Development Guide

## Build/Test Commands
```
cargo build                # Build the project
cargo run -- --help        # Run with help flag
cargo test                 # Run all tests
cargo test test_name       # Run a specific test
cargo fmt                  # Format code
cargo clippy               # Lint code
cargo tarpaulin            # Generate code coverage
```

## Code Style Guidelines
- **Naming**: snake_case for variables/functions, PascalCase for types/structs/enums
- **Formatting**: Follow rustfmt conventions (4-space indentation)
- **Imports**: Group standard library, external crates, then local modules
- **Error Handling**: Use Result types with ? operator, provide descriptive error messages
- **Types**: Use strong typing and avoid unwrap() in production code
- **Documentation**: Document public functions and modules with rustdoc comments
- **Testing**: Write unit tests for core functionality
- **Constants**: Use SCREAMING_SNAKE_CASE for constants, with explicit types

Follow the command-line interface pattern established with clap for consistency.