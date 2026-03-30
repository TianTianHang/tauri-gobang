#!/bin/bash
set -e

# Build script for gobang-server cross-platform binaries

VERSION=${1:-"0.1.0"}
BUILD_DIR="build/targets"
mkdir -p "$BUILD_DIR"

echo "Building gobang-server v$VERSION..."

# Linux x86_64
echo "Building for Linux x86_64..."
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/gobang-server "$BUILD_DIR/gobang-server-linux-amd64"

# Linux ARM64
if command -v aarch64-linux-gnu-gcc &> /dev/null; then
    echo "Building for Linux ARM64..."
    cargo build --release --target aarch64-unknown-linux-gnu
    cp target/aarch64-unknown-linux-gnu/release/gobang-server "$BUILD_DIR/gobang-server-linux-arm64"
fi

# macOS x86_64
if [ "$(uname)" = "Darwin" ]; then
    echo "Building for macOS x86_64..."
    cargo build --release --target x86_64-apple-darwin
    cp target/x86_64-apple-darwin/release/gobang-server "$BUILD_DIR/gobang-server-darwin-amd64"
    
    # macOS ARM64 (Apple Silicon)
    if [ "$(uname -m)" = "arm64" ]; then
        echo "Building for macOS ARM64..."
        cargo build --release --target aarch64-apple-darwin
        cp target/aarch64-apple-darwin/release/gobang-server "$BUILD_DIR/gobang-server-darwin-arm64"
    fi
fi

# Windows x86_64
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Building for Windows x86_64..."
    cargo build --release --target x86_64-pc-windows-gnu
    cp target/x86_64-pc-windows-gnu/release/gobang-server.exe "$BUILD_DIR/gobang-server-windows-amd64.exe"
fi

echo "Build complete! Binaries in $BUILD_DIR:"
ls -lh "$BUILD_DIR"
