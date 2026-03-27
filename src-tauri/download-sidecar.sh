#!/bin/bash
# Sidecar download script for Rapfi engine

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_DIR="${SCRIPT_DIR}/binaries"
RAPFI_VERSION="250615"
RAPFI_URL="https://github.com/dhbloo/rapfi/releases/download"

mkdir -p "$BINARY_DIR"

echo "📦 Downloading Rapfi engine (sidecar)..."
cd "$BINARY_DIR"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"
TARGET=$(rustc -vV | grep "^host:" | awk '{print $2}')

case "${OS}" in
    Linux*)
        if [ "$ARCH" = "x86_64" ]; then
            BINARY_NAME="rapfi-${TARGET}"
            DOWNLOAD_URL="${RAPFI_URL}/${RAPFI_VERSION}/Rapfi-engine.7z"
        else
            echo "❌ Unsupported architecture: $ARCH"
            echo "Only x86_64 is supported. Please download manually from GitHub releases."
            exit 1
        fi
        ;;
    Darwin*)
        BINARY_NAME="rapfi-${TARGET}"
        DOWNLOAD_URL="${RAPFI_URL}/${RAPFI_VERSION}/Rapfi-engine.7z"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        BINARY_NAME="rapfi-${TARGET}.exe"
        DOWNLOAD_URL="${RAPFI_URL}/${RAPFI_VERSION}/Rapfi-engine.7z"
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

# Check if already extracted and configured
if [ -f "$BINARY_NAME" ]; then
    echo "✅ Already configured: $BINARY_DIR/$BINARY_NAME"
else
    # Check if archive is already downloaded
    if [ -f "rapfi.7z" ]; then
        echo "📦 Archive already downloaded, extracting..."
    else
        echo "⬇️  Downloading from: $DOWNLOAD_URL"

        # Check for 7z command
        if ! command -v 7z &> /dev/null; then
            echo "❌ 7z is not installed. Please install p7zip-full:"
            echo "   Ubuntu/Debian: sudo apt-get install p7zip-full"
            echo "   Fedora/RHEL: sudo dnf install p7zip"
            echo "   Arch: sudo pacman -S p7zip"
            exit 1
        fi

        curl -L -o "rapfi.7z" "$DOWNLOAD_URL"
    fi

    # Extract all files from the 7z archive
    echo "📦 Extracting Rapfi engine..."
    7z x -y rapfi.7z

    # Select the best binary variant based on CPU features
    SOURCE_BINARY=""

    if [ "${OS}" = "Linux" ]; then
        # Detect CPU features and select the best variant
        if grep -q "avx512vnni" /proc/cpuinfo 2>/dev/null; then
            SOURCE_BINARY="pbrain-rapfi-linux-clang-avx512vnni"
            echo "🚀 Using AVX512VNNI optimized binary"
        elif grep -q "avx512" /proc/cpuinfo 2>/dev/null; then
            SOURCE_BINARY="pbrain-rapfi-linux-clang-avx512"
            echo "🚀 Using AVX512 optimized binary"
        elif grep -q "avxvnni" /proc/cpuinfo 2>/dev/null; then
            SOURCE_BINARY="pbrain-rapfi-linux-clang-avxvnni"
            echo "🚀 Using AVXVNNI optimized binary"
        elif grep -q "avx2" /proc/cpuinfo 2>/dev/null; then
            SOURCE_BINARY="pbrain-rapfi-linux-clang-avx2"
            echo "🚀 Using AVX2 optimized binary"
        elif grep -q "sse4_2" /proc/cpuinfo 2>/dev/null; then
            SOURCE_BINARY="pbrain-rapfi-linux-clang-sse"
            echo "✓ Using SSE optimized binary"
        else
            SOURCE_BINARY="pbrain-rapfi-linux-clang-sse"
            echo "✓ Using baseline SSE binary"
        fi
    elif [ "${OS}" = "Darwin" ]; then
        SOURCE_BINARY="pbrain-rapfi-macos-apple-silicon"
        echo "✓ Using Apple Silicon binary"
    elif [[ "${OS}" =~ MINGW|MSYS|CYGWIN ]]; then
        # Windows variants
        if grep -q "avx512vnni" /proc/cpuinfo 2>/dev/null; then
            SOURCE_BINARY="pbrain-rapfi-windows-avx512vnni.exe"
            echo "🚀 Using AVX512VNNI optimized binary"
        elif grep -q "avx512" /proc/cpuinfo 2>/dev/null; then
            SOURCE_BINARY="pbrain-rapfi-windows-avx512.exe"
            echo "🚀 Using AVX512 optimized binary"
        elif grep -q "avxvnni" /proc/cpuinfo 2>/dev/null; then
            SOURCE_BINARY="pbrain-rapfi-windows-avxvnni.exe"
            echo "🚀 Using AVXVNNI optimized binary"
        elif grep -q "avx2" /proc/cpuinfo 2>/dev/null; then
            SOURCE_BINARY="pbrain-rapfi-windows-avx2.exe"
            echo "🚀 Using AVX2 optimized binary"
        else
            SOURCE_BINARY="pbrain-rapfi-windows-sse.exe"
            echo "✓ Using baseline SSE binary"
        fi
    fi

    # Copy the selected binary to the target name
    if [ -f "$SOURCE_BINARY" ]; then
        cp "$SOURCE_BINARY" "$BINARY_NAME"
        chmod +x "$BINARY_NAME"
        echo "✅ Configured: $BINARY_DIR/$BINARY_NAME (from $SOURCE_BINARY)"
    else
        echo "❌ Failed to find binary: $SOURCE_BINARY"
        echo "Available binaries:"
        ls -1 pbrain-rapfi-* 2>/dev/null || echo "None found"
        exit 1
    fi

    # Clean up archive (keep extracted files for potential use)
    rm -f "rapfi.7z"
fi

# Verify
if [ -f "$BINARY_NAME" ]; then
    echo ""
    echo "🔍 Binary info:"
    ls -lh "$BINARY_NAME"
    file "$BINARY_NAME"

    echo ""
    echo "✨ Sidecar ready! The engine will be bundled with the application."
    echo "📁 Engine files and models are available in: $BINARY_DIR"
else
    echo "❌ Failed to configure binary"
    exit 1
fi
