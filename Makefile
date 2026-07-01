.PHONY: default build release check run run-release test clean fmt clippy docker-build docker-export

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

# Build the application container using Docker
docker-build:
	docker build -t dr-md .

# Export the compiled release binary from the Docker container to target/docker/dr-md
docker-export:
	docker build --target builder -t dr-md-builder .
	mkdir -p target/docker
	docker run --rm --entrypoint cat dr-md-builder /app/target/release/dr-md > target/docker/dr-md
	chmod +x target/docker/dr-md

# Update the application version in Cargo.toml and VERSION file
update-version:
	@version=$$(echo "$(version)$(VERSION)" | sed 's/ //g'); \
	if [ -z "$$version" ]; then echo "Usage: make update-version version=x.y.z"; exit 1; fi; \
	sed -i 's/^version = "[^"]*"/version = "'$$version'"/' Cargo.toml; \
	echo "APP_VERSION=$$version" > VERSION; \
	echo "RUST_VERSION=$$(rustc --version | awk '{print $$2}')" >> VERSION


