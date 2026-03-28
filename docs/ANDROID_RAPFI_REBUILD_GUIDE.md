# Android Rapfi 二进制重新构建指南

## 📋 概述

本文档描述如何重新构建 rapfi Android 二进制文件，解决 x86_64 版本在 Android 5.0+ 上无法执行的问题。

**问题**: 当前 `rapfi-x86_64-linux-android` 是非 PIE 的静态链接 EXEC 二进制，Android 拒绝执行  
**解决方案**: 重新编译为动态链接的 PIE (Position-Independent Executable) 二进制

---

## 🔍 根本原因分析

### 问题症状

```
运行时错误:
  - Failed to spawn engine: Permission denied (os error 13)
  - linker: error: "rapfi" has unexpected e_type: 2
```

### 深层原因

```
┌─────────────────────────────────────────────────────────────┐
│  当前 x86_64 二进制分析                                      │
├─────────────────────────────────────────────────────────────┤
│  ELF Type: EXEC (e_type: 2) ❌                            │
│  Linking: statically linked                                │
│  Modules: NO_COMMAND_MODULES=ON                            │
│  Size: 1.9MB                                               │
│                                                             │
│  为什么是非 PIE?                                            │
│    1. NO_COMMAND_MODULES=ON                                │
│    2. CMake 条件: NO_MULTI_THREADING AND NO_COMMAND_MODULES │
│    3. → 触发静态链接 (-static)                             │
│    4. 静态链接配置 → 生成 EXEC 而非 DYN (PIE)               │
│                                                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  当前 aarch64 二进制对比                                     │
├─────────────────────────────────────────────────────────────┤
│  ELF Type: DYN (PIE) ✅                                    │
│  Linking: dynamically linked (libc++_shared.so)            │
│  Modules: Full command modules                             │
│  Size: 24MB                                                │
│                                                             │
│  为什么是 PIE?                                              │
│    1. 使用动态链接 (libc++_shared)                         │
│    2. Android NDK 默认为所有二进制生成 PIE                 │
│    3. → 正确的 DYN (PIE) 二进制                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Android PIE 要求

- **Android 5.0 (API 21)+**: 强制要求所有可执行文件为 PIE
- **Linker 验证**: 拒绝执行 `ET_EXEC` 类型的 ELF 文件
- **错误代码**: `e_type: 2` (EXEC) 被拒绝，要求 `e_type: 3` (DYN/PIE)

---

## 🛠️ 解决方案设计

### 核心变更

修改 CMake 配置，确保 x86_64 版本使用动态链接：

```cmake
# 旧配置（错误）:
-DANDROID_STL=c++_static
-DNO_COMMAND_MODULES=ON

# 新配置（正确）:
-DANDROID_STL=c++_shared
# NO_COMMAND_MODULES 默认为 OFF（不设置）
```

### 预期结果

```
x86_64 重建后:
  ✅ ELF Type: DYN (PIE)
  ✅ Linking: 动态链接 (libc++_shared.so, libm.so, libdl.so)
  ✅ Size: ~24MB (增加但可接受)
  ✅ 可在 Android 5.0+ 执行
```

---

## 📝 实施步骤

### 前置准备

#### 1. 安装 Android NDK

```bash
# 检查 NDK 是否已安装
echo $ANDROID_NDK_ROOT

# 如果为空，安装 NDK:
# 方法 1: 通过 Android Studio
#   Tools → SDK Manager → SDK Tools → NDK (Side by side)
#   选择版本 r25 或更高

# 方法 2: 命令行安装
# $HOME/Android/Sdk/ndk/版本号

# 设置环境变量
export ANDROID_NDK_ROOT=$HOME/Android/Sdk/ndk/25.2.9519653
```

#### 2. 验证 NDK 版本

```bash
# NDK r25+ 包含正确的 PIE 支持
ls $ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake

# 应该看到工具链文件
```

### 步骤 1: 克隆 rapfi 源码

```bash
# 在项目根目录执行
cd /data/projects/tauri-gobang

# 创建 third-party 目录（如果不存在）
mkdir -p third-party

# 克隆 rapfi 仓库
git clone https://github.com/dhbloo/rapfi.git third-party/rapfi

# 进入 rapfi 目录
cd third-party/rapfi

# 初始化并更新 submodules（获取 Network 权重文件）
git submodule update --init --recursive

# 验证结构
ls Rapfi/CMakeLists.txt
ls Networks/mix9svqstandard_bs15.bin.lz4
```

**预期输出**:
```
Rapfi/CMakeLists.txt
Networks/mix9svqstandard_bs15.bin.lz4
Networks/config.toml
... (其他权重文件)
```

### 步骤 2: 修改构建脚本（可选）

如果使用现有的 `scripts/build-android-rapfi.sh`，需要修改 x86_64 部分：

```bash
# 编辑构建脚本
nano scripts/build-android-rapfi.sh
```

找到 x86_64 构建部分（约 42-53 行），修改为：

```bash
# 构建 x86_64 版本（用于模拟器）
echo "💻 Building for x86_64 (emulator)..."
cmake -S "$RAPFI_SRC/Rapfi" -B "$RAPFI_SRC/build-android-x86_64" \
    -DCMAKE_TOOLCHAIN_FILE="$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake" \
    -DANDROID_ABI=x86_64 \
    -DANDROID_PLATFORM=android-24 \
    -DANDROID_STL=c++_shared \          # 关键修改 1: 使用 shared STL
    -DCMAKE_BUILD_TYPE=Release \
    -DNO_MULTI_THREADING=OFF            # 保持多线程
    # 不设置 NO_COMMAND_MODULES（默认 OFF）# 关键修改 2

cmake --build "$RAPFI_SRC/build-android-x86_64" -j$(nproc)
cp "$RAPFI_SRC/build-android-x86_64/pbrain-rapfi" "$OUTPUT_DIR/rapfi-x86_64-linux-android"
```

**关键变更说明**:
1. ✅ `ANDROID_STL=c++_shared`: 使用动态 C++ 运行时库
2. ✅ 移除 `NO_COMMAND_MODULES=ON`: 启用完整功能模块
3. ✅ 默认会生成 PIE 二进制（NDK 行为）

### 步骤 3: 执行构建

```bash
# 确保环境变量设置
export ANDROID_NDK_ROOT=/path/to/your/ndk

# 运行构建脚本
bash scripts/build-android-rapfi.sh
```

**预期输出**:
```
🔧 Building rapfi for Android...
📱 Building for ARM64 (aarch64)...
-- Android: Targeting API 'android-24' with ABI 'arm64-v8a'
-- Configuring done
-- Generating done
-- Build files have been written to: .../build-android-arm64
[ 50%] Built target rapfi
💻 Building for x86_64 (emulator)...
-- Android: Targeting API 'android-24' with ABI 'x86_64'
-- Configuring done
-- Generating done
-- Build files have been written to: .../build-android-x86_64
[100%] Built target rapfi
✅ Android rapfi binaries built successfully!
-rwxr-xr-x 1 user user 24M ... rapfi-aarch64-linux-android
-rwxr-xr-x 1 user user 24M ... rapfi-x86_64-linux-android
```

**注意**: 构建可能需要 5-10 分钟，取决于机器性能。

### 步骤 4: 验证生成的二进制

```bash
# 检查文件类型
file src-tauri/binaries/rapfi-x86_64-linux-android

# 期望输出:
# ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), 
# dynamically linked, interpreter /system/bin/linker64, 
# for Android 24, built by NDK ...

# 关键检查: 必须包含 "pie executable"
```

```bash
# 检查 ELF 类型
readelf -h src-tauri/binaries/rapfi-x86_64-linux-android | grep Type

# 期望输出:
#   Type:                              DYN (Position-Independent Executable file)

# 关键: 必须是 "DYN" 而不是 "EXEC"
```

```bash
# 检查动态依赖
readelf -d src-tauri/binaries/rapfi-x86_64-linux-android | grep NEEDED

# 期望输出:
#   0x0000000000000001 (NEEDED)             Shared library: [libm.so]
#   0x0000000000000001 (NEEDED)             Shared library: [libc++_shared.so]
#   0x0000000000000001 (NEEDED)             Shared library: [libdl.so]
#   0x0000000000000001 (NEEDED)             Shared library: [libc.so]

# 关键: 必须依赖 libc++_shared.so
```

```bash
# 检查 INTERP 段（动态链接器）
readelf -l src-tauri/binaries/rapfi-x86_64-linux-android | grep INTERP

# 期望输出:
#   INTERP         0x0000000000000270  0x0000000000000270 ...

# 关键: 必须有 INTERP 段（表示动态链接）
```

```bash
# 检查文件大小（应该接近 aarch64 版本）
ls -lh src-tauri/binaries/rapfi-*-linux-android

# 期望输出:
#   -rwxr-xr-x 1 user user 24M ... rapfi-aarch64-linux-android
#   -rwxr-xr-x 1 user user 24M ... rapfi-x86_64-linux-android

# 关键: 两者大小应该相似（~24MB），不是 1.9MB
```

---

## ✅ 验证测试

### 测试 1: 静态验证（无需设备）

```bash
# 完整的 PIE 验证脚本
cat > /tmp/verify_pie.sh << 'EOF'
#!/bin/bash

echo "=== PIE 验证脚本 ==="
echo ""

# 检查 x86_64
echo "📦 检查 rapfi-x86_64-linux-android..."
FILE="src-tauri/binaries/rapfi-x86_64-linux-android"

# 1. 文件类型
FILE_TYPE=$(file "$FILE")
echo "1. 文件类型: $FILE_TYPE"
if echo "$FILE_TYPE" | grep -q "pie executable"; then
    echo "   ✅ 包含 'pie executable'"
else
    echo "   ❌ 缺少 'pie executable'"
    exit 1
fi

# 2. ELF Type
ELF_TYPE=$(readelf -h "$FILE" | grep "Type:" | awk '{print $2}')
echo "2. ELF Type: $ELF_TYPE"
if [ "$ELF_TYPE" = "DYN" ]; then
    echo "   ✅ 是 DYN (PIE)"
else
    echo "   ❌ 不是 DYN: $ELF_TYPE"
    exit 1
fi

# 3. INTERP 段
INTERP=$(readelf -l "$FILE" | grep -c "INTERP")
echo "3. INTERP 段:"
if [ "$INTERP" -gt 0 ]; then
    echo "   ✅ 有 INTERP 段（动态链接）"
else
    echo "   ❌ 缺少 INTERP 段（静态链接）"
    exit 1
fi

# 4. 动态依赖
LIBCXX=$(readelf -d "$FILE" | grep -c "libc++_shared.so")
echo "4. 动态依赖:"
if [ "$LIBCXX" -gt 0 ]; then
    echo "   ✅ 依赖 libc++_shared.so"
else
    echo "   ❌ 不依赖 libc++_shared.so"
    exit 1
fi

echo ""
echo "=== 所有检查通过 ✅ ==="
EOF

chmod +x /tmp/verify_pie.sh
bash /tmp/verify_pie.sh
```

### 测试 2: 动态验证（需要设备/模拟器）

```bash
# 1. 启动 Android 应用
pnpm tauri android dev

# 2. 在另一个终端监控日志
adb logcat -c  # 清除旧日志
adb logcat | grep -E "rapfi|AI|linker"

# 3. 在应用中开始 AI 对战

# 4. 检查日志中不应该出现:
#    ❌ "has unexpected e_type: 2"
#    ❌ "Permission denied" (来自 linker)
#
#    应该看到:
#    ✅ "granted { execute }"
#    ✅ "rapfi process started"
#    ✅ AI 正常响应
```

---

## ⚠️ 风险和注意事项

### 1. 文件大小增加

```
旧 x86_64: 1.9MB (静态链接, NO_COMMAND_MODULES)
新 x86_64: ~24MB (动态链接, 完整功能)

APK 大小影响:
  - 增加 ~22MB
  - 总 APK 大小: ~50-60MB
  - 仍在可接受范围内（大多数游戏 APK 都在 100MB+）
```

**缓解措施**:
- APK 使用 Release 构建（自动压缩）
- 用户通常在 WiFi 下下载
- 可以考虑将来使用 App Bundle 按架构分发

### 2. 动态库依赖

```
新 x86_64 依赖:
  - libc++_shared.so (Android 系统自带)
  - libm.so (Android 系统自带)
  - libdl.so (Android 系统自带)
  - libc.so (Android 系统自带)
```

**缓解措施**:
- 所有依赖库都是 Android 系统标准库
- API 24+ (Android 7.0+) 保证这些库存在
- 不需要额外打包到 APK

### 3. 构建时间

```
首次构建: 5-10 分钟
  - 克隆 rapfi 仓库: ~1 分钟
  - 克隆 submodules (Networks): ~2-3 分钟
  - 编译 aarch64: ~2-3 分钟
  - 编译 x86_64: ~2-3 分钟

后续构建: 3-5 分钟（无需重新克隆）
```

**建议**:
- 首次构建时可以喝杯咖啡 ☕
- 考虑将构建结果提交到 Git（避免重复构建）
- 或在 CI/CD 中自动构建

### 4. SELinux 限制（仍需解决）

即使 PIE 二进制正确，仍需修改代码绕过 SELinux `execute_no_trans` 限制：

```rust
// 当前实现（会失败）:
Command::new(&rapfi_path).spawn()?

// 需要改为:
Command::new("/system/bin/sh")
    .arg("-c")
    .arg(&format!("{} --mode gomocup", rapfi_path))
    .spawn()?
```

**相关文档**: 参见 `ANDROID_RAPFI_ARCHITECTURE.md` 中的 SELinux 章节

---

## 🔧 故障排查

### 问题 1: NDK 未找到

```
错误: CMake Error: Could not find CMAKE_TOOLCHAIN_FILE
解决: export ANDROID_NDK_ROOT=/path/to/ndk
验证: ls $ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake
```

### 问题 2: 编译失败

```
错误: clang++: error: no such file or directory
原因: NDK 版本太旧或不兼容
解决: 升级到 NDK r25 或更高
下载: https://github.com/android/ndk/wiki
```

### 问题 3: 生成的二进制仍然不是 PIE

```
验证: readelf -h rapfi | grep Type
输出: Type: EXEC (不是 DYN)

原因:
  1. 可能仍设置了 -DANDROID_STL=c++_static
  2. 或设置了 -DNO_COMMAND_MODULES=ON

解决: 检查 CMake 参数，确保:
  ✅ -DANDROID_STL=c++_shared
  ✅ 不设置 NO_COMMAND_MODULES（或设为 OFF）
```

### 问题 4: 权限错误（在设备上测试时）

```
错误: Permission denied (即使 PIE 正确)
原因: SELinux execute_no_trans 限制

解决: 使用 shell wrapper 执行
  详见: 修改 rapfi.rs 使用 shell 执行
```

---

## 📊 预期结果对比

### 重建前后对比

| 属性 | 旧 x86_64 | 新 x86_64 | 状态 |
|------|-----------|-----------|------|
| ELF Type | EXEC | DYN (PIE) | ✅ 修复 |
| Linking | 静态 | 动态 | ✅ 修复 |
| Modules | NO_COMMAND_MODULES | 完整功能 | ✅ 改进 |
| Size | 1.9MB | ~24MB | ⚠️ 增加 |
| Android 兼容 | ❌ 5.0+ 不可执行 | ✅ 5.0+ 可执行 | ✅ 修复 |
| 执行方式 | - | shell wrapper | ⚠️ 需额外修改 |

### 构建配置对比

```cmake
# 旧配置（错误）:
cmake ... \
    -DANDROID_STL=c++_static \
    -DNO_COMMAND_MODULES=ON

# 新配置（正确）:
cmake ... \
    -DANDROID_STL=c++_shared
    # NO_COMMAND_MODULES 默认 OFF
```

---

## 📚 相关文档

- **架构总览**: `docs/ANDROID_RAPFI_ARCHITECTURE.md`
- **原构建指南**: `docs/ANDROID_RAPFI_BUILD.md`
- **实施计划**: `docs/ANDROID_RAPFI_IMPLEMENTATION_PLAN.md`
- **OpenSpec 设计**: `openspec/changes/android-rapfi-integration/design.md`
- **OpenSpec 任务**: `openspec/changes/android-rapfi-integration/tasks.md`

---

## 🎯 检查清单

完成重建后，使用此清单验证：

- [ ] ✅ x86_64 二进制是 `pie executable` (使用 `file` 命令)
- [ ] ✅ x86_64 ELF Type 是 `DYN` (使用 `readelf -h`)
- [ ] ✅ x86_64 有 INTERP 段 (使用 `readelf -l`)
- [ ] ✅ x86_64 依赖 `libc++_shared.so` (使用 `readelf -d`)
- [ ] ✅ x86_64 大小接近 24MB (使用 `ls -lh`)
- [ ] ✅ aarch64 二进制仍然正常（未受影响）
- [ ] ✅ 所有权限正确 (755, `chmod +x`)
- [ ] ⚠️  （待实现）修改 rapfi.rs 使用 shell wrapper
- [ ] ⚠️  （待测试）在实际设备上验证执行

---

**文档版本**: 1.0  
**创建日期**: 2026-03-28  
**作者**: OpenCode Exploration Mode  
**状态**: 准备实施
