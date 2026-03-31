#!/usr/bin/env bash
set -e

VERSION=${1:-"0.1.0"}
BUILD_DIR="build/targets"
mkdir -p "$BUILD_DIR"

echo "═══════════════════════════════════════════════════════"
echo "  Gobang Server Build Script v$VERSION"
echo "═══════════════════════════════════════════════════════"

echo "  📌 Rust: $(rustc --version 2>/dev/null || echo 'not found')"
echo "═══════════════════════════════════════════════════════"

build_release() {
    echo ""
    echo "📦 Building release binary..."
    echo "   Target: x86_64-unknown-linux-gnu"
    echo "   Profile: release"
    echo ""

    cargo build --release

    local BIN="target/release/gobang-server"
    local OUT="$BUILD_DIR/gobang-server"

    cp "$BIN" "$OUT"

    echo "✓ Build complete"
    echo ""
    verify_binary "$OUT"
}

verify_binary() {
    local bin=$1

    echo "─────────────────────────────────────────────────────────"
    echo "  Verification"
    echo "─────────────────────────────────────────────────────────"

    file "$bin"
    echo ""
    echo "  Binary info:"
    ls -lh "$bin" | awk '{print "    Size: " $5}'
    echo ""
    echo "  Testing binary:"
    if "$bin" --version >/dev/null 2>&1 || "$bin" --help >/dev/null 2>&1; then
        echo "    ✓ Binary is executable"
    else
        echo "    ⚠ Binary test failed"
        return 1
    fi

    echo "─────────────────────────────────────────────────────────"
}

build_release

echo ""
echo "═══════════════════════════════════════════════════════"
echo "  ✓ Build complete!"
echo "  Output directory: $BUILD_DIR"
echo "═══════════════════════════════════════════════════════"
ls -lh "$BUILD_DIR"
echo "═══════════════════════════════════════════════════════"
