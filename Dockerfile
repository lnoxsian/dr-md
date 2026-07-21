# --- Stage 1: Build the binary and packages ---
FROM rust:1-slim-bookworm AS builder

# Install GUI development library dependencies needed for egui/eframe on Linux
# Also install rpm for rpm package generation
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libasound2-dev \
    libegl1-mesa-dev \
    libgl1-mesa-dev \
    libwayland-dev \
    libxkbcommon-dev \
    libssl-dev \
    cmake \
    build-essential \
    rpm \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-deb and cargo-generate-rpm for package generation
RUN cargo install cargo-deb cargo-generate-rpm

WORKDIR /app

# Copy dependency manifests and local crates
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Create dummy main.rs to cache cargo build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --profile release-optimized

# Copy actual source code and assets
COPY src ./src
COPY assets ./assets
COPY README.md ./

# Build the release binary
RUN touch src/main.rs && cargo build --profile release-optimized

# Build .deb package
RUN cargo deb --profile release-optimized --no-build

# Build .rpm package (using strip before packaging)
# We copy to target/release to ensure cargo-generate-rpm finds it easily
RUN mkdir -p target/release && cp target/release-optimized/dr-md target/release/dr-md
RUN strip target/release-optimized/dr-md
RUN cargo generate-rpm -o target/generate-rpm/ || true

# --- Stage 2: Create a minimal runtime image ---
FROM debian:bookworm-slim AS runtime

# Install runtime GUI libraries required for running the native X11/Wayland binary
RUN apt-get update && apt-get install -y --no-install-recommends \
    libx11-6 \
    libxcursor1 \
    libxrandr2 \
    libxi6 \
    libasound2 \
    libegl1 \
    libgl1 \
    libwayland-client0 \
    libxkbcommon0 \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release-optimized/dr-md /app/dr-md

ENTRYPOINT ["/app/dr-md"]
