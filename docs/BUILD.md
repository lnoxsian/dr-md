# Build Guide

This project is a native Rust desktop application. You can build it directly with Cargo, through the provided task files, or inside Docker.

## Requirements

### Rust Toolchain

- Rust stable
- `cargo`

### Linux Build Libraries

The Dockerfile shows the packages needed for a Debian-based system build:

- `pkg-config`
- `libx11-dev`
- `libxcursor-dev`
- `libxrandr-dev`
- `libxi-dev`
- `libasound2-dev`
- `libegl1-mesa-dev`
- `libgl1-mesa-dev`
- `libwayland-dev`
- `libxkbcommon-dev`
- `libssl-dev`
- `cmake`
- `build-essential`

On other distributions, install the equivalent X11, Wayland, OpenGL, and audio development packages. If a build fails at link time, compare your package list against the one used in the Dockerfile and install the matching development packages for your distribution.

## Build With Cargo

```bash
cargo build
cargo build --release
```

The debug build is best for development. The release build produces the optimized binary in `target/release/dr-md`.

## Run The App

```bash
cargo run
cargo run --release
```

The application starts with an empty vault if no folder is provided. If a config file already exists, dr.md restores the last opened folder.

## Verification

```bash
cargo test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Task Commands

The repository includes both `Justfile` and `Makefile` targets:

- `build` -> `cargo build`
- `release` -> `cargo build --release`
- `check` -> `cargo check`
- `run` -> `cargo run`
- `run-release` -> `cargo run --release`
- `test` -> `cargo test`
- `clean` -> `cargo clean`
- `fmt` -> `cargo fmt --all -- --check`
- `clippy` -> `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `package-deb` -> packages the release binary and desktop files into a `.deb` package

The `make` and `just` targets are intentionally aligned so either workflow can be used without learning a second command set. See [PACKAGING.md](PACKAGING.md) for detailed installation instructions.

## Docker Build

```bash
docker build -t dr-md .
```

This repository also contains a multistage Dockerfile that builds a minimal runtime image with the required native libraries.

Use Docker when you want a repeatable Linux build environment or when the host machine is missing the desktop dependencies.

## Docker Export

```bash
make docker-export
```

This produces a release binary at `target/docker/dr-md`.

The exported binary is copied out of the builder stage, so it can be distributed or archived without the full image.

## Asset Generation

```bash
make generate-icons
make generate-logos
make generate-assets
```

The asset generator is `scripts/generate_assets.py`.

The asset pipeline keeps logo variants and icon sizes under version control so releases have consistent branding and packaging assets.

## Version Update

```bash
make update-version version=0.1.9
```

This updates both `Cargo.toml` and `VERSION`.

Use the version task when preparing a release so the crate metadata and local version file stay in sync.
