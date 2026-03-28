# Android Rapfi 集成指南

## 概述

本项目将 [rapfi](https://github.com/dhbloo/rapfi) 五子棋 AI 引擎集成到 Tauri Android 应用中。

## 当前状态

- ✅ 已有预编译的 Android rapfi 二进制 (`src-tauri/binaries/rapfi-*-linux-android`)
- ✅ Rust 代码已实现 Piskvork 协议 (`src-tauri/src/rapfi.rs`)
- ✅ NNUE 权重文件已配置 (`src-tauri/binaries/*.bin.lz4`)
- ⚠️ 需要实现 Android 运行时二进制提取逻辑

## 二进制信息

```bash
# 当前已有的二进制
$ ls -lh src-tauri/binaries/rapfi-*-linux-android
-rwxr-xr-x 1 tiantian tiantian 24M Mar 28 00:51 rapfi-aarch64-linux-android
-rwxr-xr-x 1 tiantian tiantian 24M Mar 28 00:51 rapfi-x86_64-linux-android

# 二进制详情
$ file src-tauri/binaries/rapfi-aarch64-linux-android
ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), 
dynamically linked, interpreter /system/bin/linker64, for Android 21, 
built by NDK r29 (14206865)
```

## 架构支持

| 架构 | ABI | 设备类型 | 状态 |
|------|-----|----------|------|
| ARM64 | arm64-v8a | 真机 (99%+) | ✅ 支持 |
| x86_64 | x86_64 | 模拟器 | ✅ 支持 |
| ARMv7 | armeabi-v7a | 旧设备 | ⚠️ 需要编译 |

## 从源码构建 (可选)

如果你需要从 rapfi 源码构建 Android 二进制：

### 前置要求

1. Android NDK r25 或更高版本
2. CMake 3.18+
3. Clang (随 NDK 提供)

### 构建步骤

```bash
# 1. 设置环境变量
export ANDROID_NDK_ROOT=/path/to/android-ndk

# 2. 克隆 rapfi 源码（如果还未添加为 submodule）
./scripts/setup-rapfi-submodule.sh

# 3. 构建 Android 二进制
./scripts/build-android-rapfi.sh

# 4. 二进制将输出到 src-tauri/binaries/
```

### 手动构建命令

```bash
# ARM64 版本（真机）
cmake -S third-party/rapfi/Rapfi -B build-android-arm64 \
    -DCMAKE_TOOLCHAIN_FILE=$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake \
    -DANDROID_ABI=arm64-v8a \
    -DANDROID_PLATFORM=android-24 \
    -DANDROID_STL=c++_static \
    -DCMAKE_BUILD_TYPE=Release \
    -DUSE_NEON=ON \
    -DUSE_NEON_DOTPROD=ON

cmake --build build-android-arm64 -j$(nproc)
cp build-android-arm64/pbrain-rapfi src-tauri/binaries/rapfi-aarch64-linux-android

# x86_64 版本（模拟器）
cmake -S third-party/rapfi/Rapfi -B build-android-x86_64 \
    -DCMAKE_TOOLCHAIN_FILE=$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake \
    -DANDROID_ABI=x86_64 \
    -DANDROID_PLATFORM=android-24 \
    -DANDROID_STL=c++_static \
    -DCMAKE_BUILD_TYPE=Release

cmake --build build-android-x86_64 -j$(nproc)
cp build-android-x86_64/pbrain-rapfi src-tauri/binaries/rapfi-x86_64-linux-android
```

## Tauri 集成

### 资源配置

`src-tauri/tauri.conf.json`:

```json
{
  "bundle": {
    "resources": [
      "binaries/config.toml",
      "binaries/*.bin.lz4",
      "binaries/rapfi-*-linux-android"
    ],
    "externalBin": [
      "binaries/rapfi"
    ]
  }
}
```

### 运行时提取逻辑

`src-tauri/src/rapfi.rs` 中的 `get_engine_path()` 函数会：

1. **桌面平台**: 直接使用 `externalBin` (Tauri 自动处理)
2. **Android 平台**: 
   - 从 APK 的 assets/ 复制到应用的 cache 目录
   - 设置可执行权限 (chmod +x)
   - 缓存供后续使用

## 调试

```bash
# 启用调试输出
export TAURI_GOBANG_DEBUG=1
pnpm tauri android dev

# 查看日志
adb logcat | grep -E "AI|rapfi|Rapfi"
```

## 性能优化

rapfi 支持 NEON 指令集，在 ARM64 设备上性能优异：

- **NEON**: ARMv8-A SIMD 基础指令
- **NEON DOTPROD**: ARMv8.2-A 点积指令 (性能提升 10-20%)

当前二进制启用了 NEON DOTPROD：
```cmake
-DUSE_NEON=ON
-DUSE_NEON_DOTPROD=ON
```

## 故障排查

### 问题：找不到 rapfi 二进制

**症状**: `Rapfi engine not found`

**解决方案**:
1. 检查 `src-tauri/binaries/` 是否有对应架构的二进制
2. 查看日志确认应用尝试查找的路径
3. 验证 assets 是否正确打包：`unzip -l app-debug.apk | grep rapfi`

### 问题：权限被拒绝

**症状**: `Permission denied` 或 `Failed to start engine`

**解决方案**:
- 确认 `get_engine_path()` 中设置了执行权限
- 检查 SELinux 上下文（某些设备需要）

### 问题：崩溃或超时

**解决方案**:
- 降低线程数：编辑 `config.toml`，设置 `default_thread_num = 1`
- 增加超时时间：修改 `rapfi.rs` 中的 `timeout` 值
- 检查权重文件是否完整

## 参考资料

- [rapfi GitHub](https://github.com/dhbloo/rapfi)
- [Tauri Android 文档](https://v2.tauri.app/start/migrate/from-tauri-1/#android)
- [Android NDK 指南](https://developer.android.com/ndk/guides/cmake)
