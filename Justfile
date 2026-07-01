default: build

# Fast build (debug profile)
build:
    cargo build

# Slow build (fully optimized release profile)
release:
    cargo build --release

# Fast check (syntax and type checking without code gen)
check:
    cargo check

# Run in debug mode
run:
    cargo run

# Run in release mode
run-release:
    cargo run --release

# Run tests
test:
    cargo test

# Clean build artifacts
clean:
    cargo clean

# Check code formatting
fmt:
    cargo fmt --all -- --check

# Run linter
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings
