# Android Rapfi 集成实施方案

## 📋 执行摘要

**目标**: 将 rapfi 五子棋 AI 引擎集成到 tauri-gobang Android 应用中

**当前状态**: 
- ✅ 预编译的 Android rapfi 二进制已存在 (24MB)
- ✅ Rust Piskvork 协议客户端已实现
- ⚠️ **缺失**: Android 运行时二进制提取逻辑

**推荐方案**: 在首次运行时从 APK assets 提取 rapfi 到应用 cache 目录

---

## 🎯 技术方案

### 方案选择对比

| 方案 | 复杂度 | 性能 | 维护性 | 推荐度 |
|------|--------|------|--------|--------|
| 1. 运行时提取 assets | 低 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ✅ **推荐** |
| 2. 编译为 .so 库 | 高 | ⭐⭐⭐⭐⭐ | ⭐⭐ | ❌ 复杂 |
| 3. 静态链接到 Rust | 中 | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⚠️ 可选 |

### 方案 1 详细设计 (推荐)

```
┌─────────────────────────────────────────────────────────────┐
│                   APK 结构                                   │
├─────────────────────────────────────────────────────────────┤
│ assets/binaries/                                            │
│   ├─ rapfi-aarch64-linux-android  (资源文件)               │
│   ├─ rapfi-x86_64-linux-android   (资源文件)               │
│   ├─ config.toml                                            │
│   └─ *.bin.lz4  (NNUE 权重)                                 │
└─────────────────────────────────────────────────────────────┘
                        │
                        ▼ 首次启动
┌─────────────────────────────────────────────────────────────┐
│              应用运行时提取流程                              │
├─────────────────────────────────────────────────────────────┤
│ 1. 检测 CPU 架构 (aarch64 / x86_64)                         │
│ 2. 从 assets/ 复制对应 rapfi 到                             │
│    /data/data/com.tiantian.tauri_gobang/cache/rapfi        │
│ 3. chmod +x 设置执行权限                                    │
│ 4. 后续直接使用提取的版本                                   │
└─────────────────────────────────────────────────────────────┘
```

---

## 📝 实施步骤

### 步骤 1: 确认资源配置 (5 分钟)

**检查文件**: `src-tauri/tauri.conf.json`

```json
{
  "bundle": {
    "resources": [
      "binaries/config.toml",
      "binaries/*.bin.lz4",
      "binaries/rapfi-*-linux-android"  // ← 确保包含这行
    ]
  }
}
```

**验证**:
```bash
# 检查二进制文件存在
ls -lh src-tauri/binaries/rapfi-*-linux-android

# 应该看到:
# -rwxr-xr-x 1 tiantian tiantian 24M ... rapfi-aarch64-linux-android
# -rwxr-xr-x 1 tiantian tiantian 24M ... rapfi-x86_64-linux-android
```

### 步骤 2: 修改 Rust 代码 (15 分钟)

**文件**: `src-tauri/src/lib.rs`

添加模块导入:
```rust
mod android_rapfi;  // ← 添加这行
mod rapfi;
// ... 其他模块
```

**文件**: `src-tauri/src/rapfi.rs`

修改 `get_engine_path()` 函数，在开头添加 Android 处理:

```rust
fn get_engine_path<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Result<PathBuf, String> {
    // ===== Android 特殊处理 =====
    #[cfg(target_os = "android")]
    {
        eprintln!("🤖 [Android] Detected Android platform");
        
        // 使用 Android 提取逻辑
        match crate::android_rapfi::extract_rapfi_binary(app) {
            Ok(path) => {
                eprintln!("✅ [Android] Using extracted rapfi: {}", path.display());
                return Ok(path);
            }
            Err(e) => {
                eprintln!("❌ [Android] Failed to extract rapfi: {}", e);
                // 继续尝试其他路径...
            }
        }
    }
    // ===========================

    // 原有的桌面平台逻辑保持不变...
    if let Ok(path) = app.path().resolve("rapfi", BaseDirectory::Resource) {
        // ... (省略)
    }
    // ... (其他路径查找逻辑)
}
```

### 步骤 3: 创建 Android 提取模块 (已完成 ✅)

**文件**: `src-tauri/src/android_rapfi.rs`

此文件已创建，包含:
- `extract_rapfi_binary()`: 从 assets 提取并设置权限
- 自动检测架构 (aarch64 / x86_64)
- 缓存避免重复提取

### 步骤 4: 测试构建 (10 分钟)

```bash
# 开发模式测试
pnpm tauri android dev

# 查看日志
adb logcat | grep -E "AI|rapfi|Android"

# 应该看到:
# 🤖 [Android] Detected Android platform
# 📦 [Android] Extracting rapfi from assets: binaries/rapfi-aarch64-linux-android
# ✅ [Android] rapfi extracted successfully: /data/data/.../cache/rapfi
```

### 步骤 5: 验证功能 (10 分钟)

在 Android 设备/模拟器上测试 AI 功能:

1. 启动应用
2. 开始新游戏
3. 选择 "vs AI" 模式
4. 落子，观察 AI 响应
5. 检查日志确认 rapfi 正常工作

---

## 🔍 调试指南

### 启用详细日志

```bash
# 设置环境变量
export TAURI_GOBANG_DEBUG=1

# 重新运行
pnpm tauri android dev
```

### 常见问题排查

#### 问题 1: "Asset not found"

**日志**: `Asset not found: binaries/rapfi-aarch64-linux-android`

**原因**: APK 中未包含 rapfi 二进制

**解决**:
```bash
# 1. 检查 tauri.conf.json 配置
grep -A 5 "resources" src-tauri/tauri.conf.json

# 2. 验证 assets 打包
pnpm tauri build --target android
unzip -l src-tauri/gen/android/app/build/outputs/apk/debug/*.apk | grep rapfi

# 应该看到:
# binaries/rapfi-aarch64-linux-android
# binaries/rapfi-x86_64-linux-android
```

#### 问题 2: "Permission denied"

**日志**: `Failed to start engine: Permission denied`

**原因**: 提取的二进制没有执行权限

**解决**: 检查 `android_rapfi.rs` 中的权限设置代码:
```rust
let mut perms = fs::metadata(&rapfi_path)?.permissions();
perms.set_mode(0o755);  // ← 确保这行存在
fs::set_permissions(&rapfi_path, perms)?;
```

#### 问题 3: "Engine timeout"

**日志**: `Engine timeout`

**解决**: rapfi 在 Android 上可能需要更长时间启动

**方案 1**: 增加超时时间 (在 `rapfi.rs` 中):
```rust
// 将启动超时从默认增加到 10 秒
// (需要修改 Command::spawn() 逻辑，添加 timeout)
```

**方案 2**: 优化 rapfi 配置:
```toml
# src-tauri/binaries/config.toml
default_thread_num = 1  # 降低线程数
```

---

## 🚀 性能优化

### rapfi 性能调优

当前 rapfi 二进制使用 NEON DOTPROD 指令:

```bash
# 查看二进制信息
file src-tauri/binaries/rapfi-aarch64-linux-android

# 应该显示:
# ARM aarch64, for Android 21, built by NDK r29
```

**性能对比** (基于 rapfi 文档):

| 指令集 | 相对性能 | 设备支持 |
|--------|----------|----------|
| 无 SIMD | 100% | 所有设备 |
| NEON | 180% | ARMv8+ (99%+) |
| NEON DOTPROD | 200% | ARMv8.2+ (95%+) |

当前配置: `-DUSE_NEON=ON -DUSE_NEON_DOTPROD=ON`

### 权重文件大小

```bash
# 当前权重文件
ls -lh src-tauri/binaries/*.bin.lz4

# 约 10MB x 4 = 40MB
# mix9svqfreestyle_bsmix.bin.lz4      ~10MB
# mix9svqrenju_bs15_black.bin.lz4    ~9.5MB
# mix9svqrenju_bs15_white.bin.lz4    ~9.4MB
# mix9svqstandard_bs15.bin.lz4       ~10MB
```

**可选优化**: 仅打包需要的权重
- 如果只支持标准五子棋，只需 `mix9svqstandard_bs15.bin.lz4`
- 节省约 30MB APK 体积

---

## 📦 从源码构建 rapfi (可选)

如果你需要更新 rapfi 版本或重新编译:

### 前置要求

```bash
# Android NDK
export ANDROID_NDK_ROOT=$HOME/Android/Sdk/ndk/25.2.9519653

# 验证
ls $ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake
```

### 快速构建脚本

```bash
# 使用提供的脚本
./scripts/build-android-rapfi.sh

# 输出:
# src-tauri/binaries/rapfi-aarch64-linux-android
# src-tauri/binaries/rapfi-x86_64-linux-android
```

### 手动构建

```bash
# ARM64 版本
cmake -S third-party/rapfi/Rapfi -B build-arm64 \
    -DCMAKE_TOOLCHAIN_FILE=$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake \
    -DANDROID_ABI=arm64-v8a \
    -DANDROID_PLATFORM=android-24 \
    -DCMAKE_BUILD_TYPE=Release \
    -DUSE_NEON=ON \
    -DUSE_NEON_DOTPROD=ON

cmake --build build-arm64 -j$(nproc)
cp build-arm64/pbrain-rapfi src-tauri/binaries/rapfi-aarch64-linux-android

# x86_64 版本 (模拟器)
cmake -S third-party/rapfi/Rapfi -B build-x86_64 \
    -DCMAKE_TOOLCHAIN_FILE=$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake \
    -DANDROID_ABI=x86_64 \
    -DANDROID_PLATFORM=android-24 \
    -DCMAKE_BUILD_TYPE=Release

cmake --build build-x86_64 -j$(nproc)
cp build-x86_64/pbrain-rapfi src-tauri/binaries/rapfi-x86_64-linux-android
```

---

## ✅ 验收标准

完成实施后，应满足:

- [ ] Android APK 包含 rapfi 二进制 (aarch64 + x86_64)
- [ ] 首次启动时自动提取到 cache 目录
- [ ] 设置正确的执行权限 (755)
- [ ] AI 功能在真机上正常工作
- [ ] AI 功能在模拟器上正常工作
- [ ] 难度设置 (简单/中等/困难) 正常
- [ ] 响应时间: 简单 <1s, 中等 <2s, 困难 <3s
- [ ] 日志输出清晰，便于调试
- [ ] 无崩溃或权限错误

---

## 📚 参考资料

- [rapfi GitHub](https://github.com/dhbloo/rapfi)
- [rapfi Android 构建文档](docs/ANDROID_RAPFI_BUILD.md)
- [Tauri Android 指南](https://v2.tauri.app/start/migrate/from-tauri-1/#android)
- [Piskvork 协议](https://plastovicka.github.io/protocl2en.htm)
- [Android NDK CMake 指南](https://developer.android.com/ndk/guides/cmake)

---

## 🎉 预期结果

实施完成后:

1. **用户体验**: Android 用户可以流畅使用 AI 对战
2. **性能**: NEON DOTPROD 提供接近原生的性能
3. **兼容性**: 支持 99% 的 Android 设备 (ARM64)
4. **可维护性**: 清晰的代码结构，便于后续更新 rapfi 版本

---

**文档版本**: 1.0  
**创建日期**: 2026-03-28  
**最后更新**: 2026-03-28
