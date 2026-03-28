# Android Rapfi 执行问题 - 快速参考

## 🚨 问题摘要

**症状**: Android x86_64 模拟器上无法执行 rapfi AI 引擎

**错误**: `linker: error: "rapfi" has unexpected e_type: 2`

**根本原因**:
1. x86_64 二进制是 `EXEC` 类型（非 PIE）
2. Android 5.0+ 强制要求 `DYN` (PIE) 类型
3. SELinux 阻止 `execute_no_trans`

---

## ✅ 解决方案

### 1. 重建 x86_64 为 PIE

```bash
# 克隆源码
git clone https://github.com/dhbloo/rapfi.git third-party/rapfi
cd third-party/rapfi
git submodule update --init --recursive

# 修改构建配置
cmake -S Rapfi -B build-android-x86_64 \
    -DCMAKE_TOOLCHAIN_FILE=$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake \
    -DANDROID_ABI=x86_64 \
    -DANDROID_PLATFORM=android-24 \
    -DANDROID_STL=c++_shared \    # 关键
    -DCMAKE_BUILD_TYPE=Release

# 构建
cmake --build build-android-x86_64

# 复制到项目
cp build-android-x86_64/pbrain-rapfi ../../src-tauri/binaries/rapfi-x86_64-linux-android
```

### 2. 验证 PIE

```bash
# 快速检查（应该都是 ✅）
file src-tauri/binaries/rapfi-x86_64-linux-android | grep pie
readelf -h src-tauri/binaries/rapfi-x86_64-linux-android | grep "DYN"
readelf -d src-tauri/binaries/rapfi-x86_64-linux-android | grep libc++_shared
```

### 3. 修改代码使用 Shell Wrapper

```rust
// 在 rapfi.rs 中
#[cfg(target_os = "android")]
{
    let process = Command::new("/system/bin/sh")
        .arg("-c")
        .arg(&format!("{} --mode gomocup", rapfi_path))
        .current_dir(&cache_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
}
```

---

## 📊 对比表

| 项目 | 旧 x86_64 | 新 x86_64 | aarch64 |
|------|-----------|-----------|---------|
| Type | EXEC ❌ | DYN (PIE) ✅ | DYN (PIE) ✅ |
| Linking | 静态 | 动态 | 动态 |
| Size | 1.9MB | ~24MB | 24MB |
| 可执行 | ❌ | ✅ | ✅ |

---

## 📚 详细文档

- 完整重建指南: `docs/ANDROID_RAPFI_REBUILD_GUIDE.md`
- 架构文档: `docs/ANDROID_RAPFI_ARCHITECTURE.md`
- OpenSpec 设计: `openspec/changes/android-rapfi-integration/design.md`
- 任务清单: `openspec/changes/android-rapfi-integration/tasks.md`

---

## ⚠️ 常见错误

### 错误 1: 仍然不是 PIE

```
检查: readelf -h rapfi | grep Type
输出: Type: EXEC（不是 DYN）

原因: 可能仍使用了 -DANDROID_STL=c++_static
解决: 确保 cmake 参数包含 -DANDROID_STL=c++_shared
```

### 错误 2: Permission denied

```
检查: adb logcat | grep linker
输出: Permission denied（即使 PIE 正确）

原因: SELinux execute_no_trans 限制
解决: 使用 shell wrapper（见上面步骤 3）
```

### 错误 3: 文件太大

```
症状: x86_64 变成 24MB（原来 1.9MB）

原因: 从静态链接改为动态链接+完整功能
评估: 这是正常的，APK 总大小增加 ~22MB，可接受
```

---

## 🎯 快速检查清单

```
PIE 验证:
  [ ] file 显示 "pie executable"
  [ ] readelf -h 显示 "Type: DYN"
  [ ] readelf -l 显示有 INTERP 段
  [ ] readelf -d 显示依赖 libc++_shared.so
  [ ] 文件大小 ~24MB

代码修改:
  [ ] rapfi.rs 使用 shell wrapper
  [ ] 添加 #[cfg(target_os = "android")] 条件编译

测试验证:
  [ ] x86_64 模拟器可以执行
  [ ] ARM64 真机可以执行
  [ ] logcat 没有 linker 错误
  [ ] AI 正常响应
```

---

**状态**: 🔧 待实施  
**优先级**: 高（阻塞性问题）  
**预计时间**: 30 分钟（克隆 + 构建 + 验证）
