#!/bin/bash
# 编译 Rapfi for Android (ARM64 only)
# 仅编译 aarch64-linux-android，覆盖绝大多数现代 Android 设备

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 设置路径
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RAPIFI_SRC="/tmp/rapfi/Rapfi"
OUTPUT_DIR="$PROJECT_ROOT/binaries"
NDK_ROOT="/opt/android-ndk"

# ARM64 配置
TRIPLE="aarch64-linux-android"
ARCH_NAME="arm64-v8a"
MIN_SDK=21

# 检查 NDK
if [ ! -d "$NDK_ROOT" ]; then
    log_error "NDK not found at $NDK_ROOT"
    exit 1
fi

log_info "NDK: $NDK_ROOT"

# 检查 Rapfi 源码
if [ ! -d "$RAPIFI_SRC" ]; then
    log_error "Rapfi source not found at $RAPIFI_SRC"
    exit 1
fi

log_info "Rapfi source: $RAPIFI_SRC"

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

BUILD_DIR="$RAPIFI_SRC/build/android-$ARCH_NAME"
OUTPUT_BIN="$OUTPUT_DIR/rapfi-$TRIPLE"

log_info "========================================="
log_info "Building Rapfi for Android ARM64"
log_info "========================================="

# 清理旧的构建
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# 设置 NDK 工具链
TOOLCHAIN="$NDK_ROOT/build/cmake/android.toolchain.cmake"

log_info "Configuring CMake with NEON support..."
cmake -S "$RAPIFI_SRC" -B "$BUILD_DIR" \
    -DCMAKE_TOOLCHAIN_FILE="$TOOLCHAIN" \
    -DANDROID_ABI="$ARCH_NAME" \
    -DANDROID_PLATFORM="android-$MIN_SDK" \
    -DANDROID_STL=c++_shared \
    -DCMAKE_BUILD_TYPE=Release \
    -DUSE_NEON=ON \
    -GNinja

log_info "Compiling (this may take a few minutes)..."
cmake --build "$BUILD_DIR" --config Release -j$(nproc)

# 查找生成的可执行文件
RAPIFI_BIN=$(find "$BUILD_DIR" -name "pbrain-rapfi" -type f -executable | head -1)

if [ -z "$RAPIFI_BIN" ]; then
    log_error "Failed to find pbrain-rapfi binary"
    exit 1
fi

log_info "Found binary: $RAPIFI_BIN"

# 复制到输出目录
cp "$RAPIFI_BIN" "$OUTPUT_BIN"
chmod +x "$OUTPUT_BIN"

log_info "========================================="
log_info "✓ Build successful!"
log_info "========================================="
ls -lh "$OUTPUT_BIN"

# 复制 NNUE 模型文件
if [ -d "/tmp/rapfi/Networks" ]; then
    log_info "Copying NNUE model files..."
    cp /tmp/rapfi/Networks/*.nnue "$OUTPUT_DIR/" 2>/dev/null || log_warn "No NNUE files found"
    log_info "✓ NNUE models copied"
fi

log_info ""
log_info "Built: $OUTPUT_BIN"
log_info ""
log_info "Note: This ARM64 build covers 99% of modern Android devices."
log_info "For x86/x86_64 Android devices (emulators), the app will use the built-in Rust AI."
