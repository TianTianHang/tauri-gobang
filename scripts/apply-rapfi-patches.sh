#!/bin/bash
# Apply rapfi Android build patches
# Usage: bash scripts/apply-rapfi-patches.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RAPFI_SRC="$PROJECT_ROOT/third-party/rapfi"
PATCH_DIR="$PROJECT_ROOT/patches/rapfi-android"

echo "🔧 Applying rapfi Android build patches..."

if [ ! -d "$RAPFI_SRC" ]; then
    echo "❌ Error: rapfi source not found at $RAPFI_SRC"
    echo "   Please run: git submodule update --init --recursive"
    exit 1
fi

cd "$RAPFI_SRC"

# Check if patches are already applied
if git diff --quiet HEAD Rapfi/CMakeLists.txt Rapfi/external/zip/src/zip.c; then
    echo "📦 Applying patches..."

    # Apply pthread linking fix
    echo "  - Applying 0001-fix-pthread-linking.patch"
    git apply "$PATCH_DIR/0001-fix-pthread-linking.patch"

    # Apply S_IWRITE fix
    echo "  - Applying 0002-fix-siwrite-undefined.patch"
    git apply "$PATCH_DIR/0002-fix-siwrite-undefined.patch"

    echo "✅ Patches applied successfully!"
else
    echo "⚠️  Patches already applied or modified files detected"
    echo "   Current status:"
    git status Rapfi/CMakeLists.txt Rapfi/external/zip/src/zip.c
fi
