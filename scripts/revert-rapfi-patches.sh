#!/bin/bash
# Revert rapfi Android build patches
# Usage: bash scripts/revert-rapfi-patches.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RAPFI_SRC="$PROJECT_ROOT/third-party/rapfi"

echo "🔄 Reverting rapfi Android build patches..."

cd "$RAPFI_SRC"

# Check if there are changes to revert
if git diff --quiet HEAD Rapfi/CMakeLists.txt Rapfi/external/zip/src/zip.c; then
    echo "ℹ️  No patches to revert (files are clean)"
else
    echo "📦 Reverting patches..."
    git restore Rapfi/CMakeLists.txt Rapfi/external/zip/src/zip.c
    echo "✅ Patches reverted successfully!"
fi
