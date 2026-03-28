#!/bin/bash
# 将 rapfi 作为 Android 构建的一部分编译

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RAPFI_SRC="$PROJECT_ROOT/third-party/rapfi"
OUTPUT_DIR="$PROJECT_ROOT/src-tauri/binaries"

echo "🔧 Building rapfi for Android..."

# Apply Android build patches
bash "$(dirname "$0")/apply-rapfi-patches.sh"

# 检查 NDK
if [ -z "$ANDROID_NDK_ROOT" ]; then
    if [ -n "$ANDROID_NDK_HOME" ]; then
        export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"
    else
        echo "❌ Error: ANDROID_NDK_ROOT not set"
        echo "   Please set it to your Android NDK path"
        exit 1
    fi
fi

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

# 构建 ARM64 版本
# 注意: 使用 c++_shared 以生成 PIE (Position-Independent Executable) 二进制
# Android 5.0+ 强制要求 PIE，静态链接会生成 EXEC 类型导致无法执行
echo "📱 Building for ARM64 (aarch64) as PIE..."
cmake -S "$RAPFI_SRC/Rapfi" -B "$RAPFI_SRC/build-android-arm64" \
    -DCMAKE_TOOLCHAIN_FILE="$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake" \
    -DANDROID_ABI=arm64-v8a \
    -DANDROID_PLATFORM=android-24 \
    -DANDROID_STL=c++_shared \
    -DCMAKE_BUILD_TYPE=Release \
    -DUSE_NEON=ON \
    -DUSE_NEON_DOTPROD=ON \
    -DNO_MULTI_THREADING=OFF \
    -DCMAKE_EXE_LINKER_FLAGS="-Wl,--exclude-libs,libgcc.a -Wl,--exclude-libs,libatomic.a"

cmake --build "$RAPFI_SRC/build-android-arm64" -j$(nproc)
cp "$RAPFI_SRC/build-android-arm64/pbrain-rapfi" "$OUTPUT_DIR/rapfi-aarch64-linux-android"

# 构建 x86_64 版本（用于模拟器）
# 注意: 使用 c++_shared 以生成 PIE (Position-Independent Executable) 二进制
# Android 5.0+ 强制要求 PIE，静态链接会生成 EXEC 类型导致无法执行
echo "💻 Building for x86_64 (emulator) as PIE..."
cmake -S "$RAPFI_SRC/Rapfi" -B "$RAPFI_SRC/build-android-x86_64" \
    -DCMAKE_TOOLCHAIN_FILE="$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake" \
    -DANDROID_ABI=x86_64 \
    -DANDROID_PLATFORM=android-24 \
    -DANDROID_STL=c++_shared \
    -DCMAKE_BUILD_TYPE=Release \
    -DNO_MULTI_THREADING=OFF \
    -DUSE_SSE=ON \
    -DCMAKE_EXE_LINKER_FLAGS="-Wl,--exclude-libs,libgcc.a -Wl,--exclude-libs,libatomic.a"

cmake --build "$RAPFI_SRC/build-android-x86_64" -j$(nproc)
cp "$RAPFI_SRC/build-android-x86_64/pbrain-rapfi" "$OUTPUT_DIR/rapfi-x86_64-linux-android"

echo "✅ Android rapfi binaries built successfully!"
ls -lh "$OUTPUT_DIR"/rapfi-*-linux-android
