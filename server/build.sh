#!/usr/bin/env bash
set -e

VERSION=${1:-"0.1.0"}
MODE=${2:-"production"}
BUILD_DIR="build/targets"
mkdir -p "$BUILD_DIR"

echo "═══════════════════════════════════════════════════════"
echo "  Gobang Server Build Script v$VERSION"
echo "═══════════════════════════════════════════════════════"

# Environment detection
if [ -n "$IN_NIX_SHELL" ]; then
    echo "  📌 Environment: Nix"
    if [ -n "$MUSL_STATIC" ]; then
        echo "  📌 Mode: Musl static compilation"
    fi
else
    echo "  📌 Environment: Native"
fi

# Check Rust toolchain
echo "  📌 Rust: $(rustc --version 2>/dev/null || echo 'not found')"

# Check available targets
if command -v rustup &> /dev/null; then
    echo "  📌 Installed targets:"
    rustup target list --installed 2>/dev/null | grep -E "(musl|gnu)" | sed 's/^/      /'
else
    echo "  📌 Installed targets: (rustup not available in Nix)"
fi

echo "═══════════════════════════════════════════════════════"

build_static() {
    echo ""
    echo "📦 Building STATIC binary (musl) for production..."
    echo "   Target: x86_64-unknown-linux-musl"
    echo "   Profile: production (LTO + strip)"
    echo ""

    # Detect Nix environment
    local IN_NIX=0
    if [ -n "$IN_NIX_SHELL" ]; then
        IN_NIX=1
        echo "   ✓ Detected Nix environment"
    fi

    # Find musl-gcc in system (works for both Nix and non-Nix)
    local MUSL_GCC=""
    if command -v musl-gcc &> /dev/null; then
        MUSL_GCC=$(command -v musl-gcc)
        echo "   ✓ Found musl-gcc: $MUSL_GCC"
    elif [ "$IN_NIX" -eq 1 ]; then
        echo "   ℹ  Nix environment: using Rust's bundled musl target"
        echo "   ℹ  Make sure x86_64-unknown-linux-musl is installed:"
        echo "      rustup target add x86_64-unknown-linux-musl"
    else
        echo "   ⚠  musl-gcc not found"
        echo "   ℹ  Install with: apt install musl-tools  (Ubuntu/Debian)"
        echo "               or:  pacman -S musl          (Arch)"
    fi

    # Set linker environment variable for musl target (only if musl-gcc found)
    if [ -n "$MUSL_GCC" ]; then
        export CC="$MUSL_GCC"
        export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="$MUSL_GCC"
    fi

    # Static linking flags for musl
    export PKG_CONFIG_ALL_STATIC=1
    export PKG_CONFIG_ALL_DYNAMIC=0
    export SQLITE_STATIC=1
    export SQLX_OFFLINE=true

    cargo build --target x86_64-unknown-linux-musl --profile production

    local BIN="target/x86_64-unknown-linux-musl/production/gobang-server"
    local OUT="$BUILD_DIR/gobang-server-linux-amd64-static"

    cp "$BIN" "$OUT"

    echo "✓ Static build complete"
    echo ""
    verify_binary "$OUT" "Static (musl)"
}

build_dynamic() {
    echo ""
    echo "📦 Building DYNAMIC binary (glibc) for development..."
    echo "   Target: x86_64-unknown-linux-gnu"
    echo "   Profile: release"
    echo ""

    cargo build --release --target x86_64-unknown-linux-gnu

    local BIN="target/x86_64-unknown-linux-gnu/release/gobang-server"
    local OUT="$BUILD_DIR/gobang-server-linux-amd64-dynamic"

    cp "$BIN" "$OUT"

    echo "✓ Dynamic build complete"
    echo ""
    verify_binary "$OUT" "Dynamic (glibc)"
}

verify_binary() {
    local bin=$1
    local type=$2

    echo "─────────────────────────────────────────────────────────"
    echo "  Verification: $type"
    echo "─────────────────────────────────────────────────────────"

    file "$bin"

    echo ""
    echo "  Binary info:"
    ls -lh "$bin" | awk '{print "    Size: " $5}'

    echo ""
    echo "  Checking dependencies:"
    if ldd "$bin" 2>&1 | grep -q "not a dynamic"; then
        echo "    ✓ Statically linked (no dependencies)"
    else
        echo "    → Dynamically linked:"
        ldd "$bin" | grep -E "(\./|glibc|sqlite)" | head -5 | sed 's/^/      /'
    fi

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

case "$MODE" in
    "production"|"static")
        build_static
        ;;
    "development"|"dynamic")
        build_dynamic
        ;;
    "both"|"all")
        build_dynamic
        echo ""
        build_static
        ;;
    *)
        echo "Usage: $0 [VERSION] [MODE]"
        echo ""
        echo "Modes:"
        echo "  production, static  - Build static musl binary (for production)"
        echo "  development, dynamic - Build dynamic glibc binary (for development)"
        echo "  both, all           - Build both versions"
        echo ""
        echo "Environments:"
        echo "  Native:             Requires musl-tools package for static builds"
        echo "  Nix:                Use 'nix develop .#musl' for static builds"
        echo ""
        echo "Examples:"
        echo "  $0 0.1.0 production    # Build static only"
        echo "  $0 0.1.0 both          # Build both"
        echo ""
        echo "Nix workflow:"
        echo "  nix develop .#musl     # Enter musl environment"
        echo "  ./build.sh production  # Build static binary"
        exit 1
        ;;
esac

echo ""
echo "═══════════════════════════════════════════════════════"
echo "  ✓ Build complete!"
echo "  Output directory: $BUILD_DIR"
echo "═══════════════════════════════════════════════════════"
ls -lh "$BUILD_DIR"
echo "═══════════════════════════════════════════════════════"
