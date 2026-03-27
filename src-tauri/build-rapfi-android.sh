#!/bin/bash
# 编译 Rapfi for Android 的脚本
# 使用 Android NDK 交叉编译多个架构

set -e

# 颜色输出
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
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

# Android 架构配置
declare -A ARCHITECTURES=(
    ["aarch64-linux-android"]="arm64-v8a"
    ["armv7-linux-androideabi"]="armeabi-v7a"
    ["i686-linux-android"]="x86"
    ["x86_64-linux-android"]="x86_64"
)

# 检查 NDK
if [ ! -d "$NDK_ROOT" ]; then
    log_error "NDK not found at $NDK_ROOT"
    log_info "Please install NDK first: yay -S android-ndk"
    exit 1
fi

log_info "NDK found at: $NDK_ROOT"

# 检查 Rapfi 源码
if [ ! -d "$RAPIFI_SRC" ]; then
    log_error "Rapfi source not found at $RAPIFI_SRC"
    log_info "Please clone repository first: git clone https://github.com/dhbloo/rapfi.git /tmp/rapfi"
    exit 1
fi

log_info "Rapfi source found at: $RAPIFI_SRC"

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

# 编译每个架构
for TRIPLE in "${!ARCHITECTURES[@]}"; do
    ARCH_NAME="${ARCHITECTURES[$TRIPLE]}"
    BUILD_DIR="$RAPIFI_SRC/build/android-$ARCH_NAME"
    OUTPUT_BIN="$OUTPUT_DIR/rapfi-$TRIPLE"

    log_info "========================================="
    log_info "Building for $TRIPLE ($ARCH_NAME)"
    log_info "========================================="

    # 清理旧的构建
    rm -rf "$BUILD_DIR"
    mkdir -p "$BUILD_DIR"

    # 设置 NDK 工具链
    TOOLCHAIN="$NDK_ROOT/build/cmake/android.toolchain.cmake"

    # 确定 Android API level
    case "$ARCH_NAME" in
        "arm64-v8a"|"x86_64")
            MIN_SDK=21  # 64-bit 架构最低 API 21
            ;;
        "armeabi-v7a"|"x86")
            MIN_SDK=16  # 32-bit 架构可以更低
            ;;
    esac

    log_info "API Level: $MIN_SDK"
    log_info "Build directory: $BUILD_DIR"

    # 使用 CMake 构建
    cmake -S "$RAPIFI_SRC" -B "$BUILD_DIR" \
        -DCMAKE_TOOLCHAIN_FILE="$TOOLCHAIN" \
        -DANDROID_ABI="$ARCH_NAME" \
        -DANDROID_PLATFORM="android-$MIN_SDK" \
        -DANDROID_STL=c++_shared \
        -DCMAKE_BUILD_TYPE=Release \
        -DBUILD_RAPFI_CLI=ON \
        -DUSE_AVX2=OFF \
        -DUSE_AVX512=OFF \
        -DUSE_SSE42=OFF \
        -DUSE_VNNI=OFF \
        -DUSE_POPCNT=OFF \
        -GNinja

    log_info "Compiling..."
    cmake --build "$BUILD_DIR" --config Release -j$(nproc)

    # 查找生成的可执行文件
    RAPIFI_BIN=$(find "$BUILD_DIR" -name "rapfi" -type f -executable | head -1)

    if [ -z "$RAPIFI_BIN" ]; then
        log_error "Failed to find rapfi binary in $BUILD_DIR"
        continue
    fi

    log_info "Found binary: $RAPIFI_BIN"

    # 复制到输出目录
    cp "$RAPIFI_BIN" "$OUTPUT_BIN"
    chmod +x "$OUTPUT_BIN"

    log_info "✓ Built: $OUTPUT_BIN"
    ls -lh "$OUTPUT_BIN"
done

log_info "========================================="
log_info "Build complete!"
log_info "========================================="
log_info "Output directory: $OUTPUT_DIR"
log_info ""
log_info "Built binaries:"
ls -lh "$OUTPUT_DIR"/rapfi-*linux-android 2>/dev/null || log_warn "No Android binaries found"

# 复制 NNUE 模型文件（如果存在）
if [ -d "/tmp/rapfi/Networks" ]; then
    log_info "Copying NNUE model files..."
    cp /tmp/rapfi/Networks/*.nnue "$OUTPUT_DIR/" 2>/dev/null || log_warn "No NNUE files found"
    log_info "✓ Copied NNUE models"
fi

log_info ""
log_info "Done! Android binaries are ready."
