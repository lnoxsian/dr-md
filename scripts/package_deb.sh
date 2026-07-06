#!/usr/bin/env bash
set -e

# Change directory to the workspace root
cd "$(dirname "$0")/.."

echo "Building release binary..."
cargo build --release

# Extract version from Cargo.toml
VERSION=$(grep -E "^version\s*=\s*" Cargo.toml | head -n 1 | cut -d '"' -f 2)
if [ -z "$VERSION" ]; then
    echo "Error: Could not extract version from Cargo.toml"
    exit 1
fi

# Detect architecture
ARCH=$(dpkg --print-architecture 2>/dev/null || echo "amd64")

echo "Packaging version $VERSION for architecture $ARCH..."

# Define packaging directories
STAGING_DIR="target/debian/dr-md_${VERSION}_${ARCH}"
rm -rf "$STAGING_DIR"
mkdir -p "$STAGING_DIR/DEBIAN"
mkdir -p "$STAGING_DIR/usr/bin"
mkdir -p "$STAGING_DIR/usr/share/applications"
mkdir -p "$STAGING_DIR/usr/share/icons/hicolor/256x256/apps"

# Copy binary
cp target/release/dr-md "$STAGING_DIR/usr/bin/"
chmod 755 "$STAGING_DIR/usr/bin/dr-md"

# Copy icon
if [ -f "assets/icons/dr-md_256x256.png" ]; then
    cp assets/icons/dr-md_256x256.png "$STAGING_DIR/usr/share/icons/hicolor/256x256/apps/dr-md.png"
fi

# Write desktop entry
cat << 'EOF' > "$STAGING_DIR/usr/share/applications/dr-md.desktop"
[Desktop Entry]
Name=dr.md
Comment=Doctor Markdown - A sleek markdown editor
Exec=dr-md %F
Icon=dr-md
Terminal=false
Type=Application
Categories=Utility;TextEditor;Development;
MimeType=text/markdown;text/plain;
EOF
chmod 644 "$STAGING_DIR/usr/share/applications/dr-md.desktop"

# Write control file
cat << EOF > "$STAGING_DIR/DEBIAN/control"
Package: dr-md
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: libc6
Maintainer: dr-md Developers
Description: Sleek, high-performance offline Markdown editor built with egui.
 Doctor Markdown (dr.md) is a lightweight editor featuring live preview,
 syntax highlighting, workspace file tree, and built-in theme support.
EOF

# Build package
echo "Building Debian package..."
dpkg-deb --build "$STAGING_DIR"

echo "Debian package successfully built at: target/debian/dr-md_${VERSION}_${ARCH}.deb"
