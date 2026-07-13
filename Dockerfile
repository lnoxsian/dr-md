# --- Stage 1: Build the binary ---
FROM rust:1-slim-bookworm AS builder

# Install GUI development library dependencies needed for egui/eframe on Linux
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
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to cache cargo build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --profile release-optimized

# Copy actual source code
COPY src ./src

# Build the release binary
RUN touch src/main.rs && cargo build --profile release-optimized

# --- Stage 2: Create a minimal runtime image ---
FROM debian:bookworm-slim

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
