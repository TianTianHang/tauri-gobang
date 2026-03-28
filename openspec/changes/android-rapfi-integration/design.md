# Design: Android Rapfi Integration

## Context

**当前状态**:
- tauri-gobang 项目已在桌面平台 (Windows/macOS/Linux) 实现 AI 对战功能
- AI 通过 rapfi 引擎实现，使用 Piskvork 协议（基于 stdin/stdout 的文本协议）进行进程间通信
- Android rapfi 二进制已编译并打包到 APK assets 中 (`rapfi-aarch64-linux-android`, `rapfi-x86_64-linux-android`)
- 桌面平台使用 Tauri 的 `externalBin` 配置自动处理二进制文件，但 Android 平台不支持此功能

**约束条件**:
- 必须保持与桌面平台相同的 Piskvork 协议接口
- 不得破坏现有的桌面平台功能
- 必须支持 ARM64 (真机) 和 x86_64 (模拟器) 两种架构
- Android 文件系统权限严格，必须显式设置可执行权限
- APK assets 是只读的，二进制文件必须在运行时提取到可写目录

**相关系统**:
- `src-tauri/src/rapfi.rs`: Piskvork 协议客户端，负责与 rapfi 进程通信
- `src-tauri/src/lib.rs`: Tauri 命令注册和模块声明
- `src-tauri/tauri.conf.json`: Tauri 配置，定义资源和外部二进制

## Goals / Non-Goals

**Goals**:
- 在 Android 平台启用 AI 对战功能
- 自动从 assets 提取 rapfi 二进制到可执行位置
- 设置正确的执行权限 (chmod +x)
- 缓存提取的二进制避免重复操作
- 保持代码清晰，易于维护和调试

**Non-Goals**:
- 不修改 Piskvork 协议或 rapfi 引擎本身
- 不实现 JNI 或其他复杂的集成方式
- 不优化二进制大小（24MB 可接受）
- 不支持 Android 以外的移动平台（iOS 等）

## Decisions

### 1. 运行时提取 vs 编译时集成

**决策**: 采用运行时从 APK assets 提取的方式

**理由**:
- ✅ **简单**: 不需要修改 rapfi 构建系统或添加 JNI 绑定
- ✅ **性能**: rapfi 作为独立进程运行，零性能开销（与桌面一致）
- ✅ **维护**: 代码清晰分离，Android 特定逻辑集中在独立模块中
- ✅ **兼容性**: 与现有的桌面平台集成方式完全兼容

**替代方案**:
- ❌ JNI 集成: 复杂度高，需要修改 rapfi 源码，维护成本大
- ❌ 静态链接: 增加 .so 文件大小，破坏模块化

### 2. 提取目标目录选择

**决策**: 使用应用 cache 目录 (`app_cache_dir()`)

**理由**:
- ✅ **持久化**: 缓存目录在应用生命周期内持久存在
- ✅ **权限**: 应用拥有完全读写权限
- ✅ **无需清理**: 系统会在空间不足时自动清理
- ✅ **Tauri API**: 提供便捷的 `app_cache_dir()` API

**目录路径**: `/data/data/com.tiantian.tauri_gobang/cache/rapfi`

**替代方案**:
- ❌ `files/` 目录: 用户可能清理，导致功能失效
- ❌ `temp/` 目录: 系统可能随时清理
- ❌ 外部存储: 需要额外权限，增加复杂度

### 3. 架构检测机制

**决策**: 使用 Rust 的 `cfg!` 宏在编译时检测架构

**实现**:
```rust
let arch = if cfg!(target_arch = "aarch64") {
    "aarch64"
} else if cfg!(target_arch = "x86_64") {
    "x86_64"
} else {
    return Err("Unsupported architecture");
};
```

**理由**:
- ✅ **零开销**: 编译时确定，运行时无分支
- ✅ **类型安全**: 利用 Rust 类型系统
- ✅ **明确**: 不支持的架构会在编译时失败

### 4. 错误处理策略

**决策**: 提取失败时回退到原有的路径查找逻辑

**实现**:
```rust
#[cfg(target_os = "android")]
{
    match crate::android_rapfi::extract_rapfi_binary(app) {
        Ok(path) => return Ok(path),
        Err(e) => {
            eprintln!("Android extraction failed: {}", e);
            // 继续尝试其他路径...
        }
    }
}
// 原有的桌面平台逻辑
```

**理由**:
- ✅ **向后兼容**: 如果提取逻辑失败，不会破坏现有功能
- ✅ **调试友好**: 错误会打印到日志，便于排查问题
- ✅ **渐进增强**: 提取功能是增强，不是硬性依赖

### 5. PIE 编译要求

**决策**: 确保所有 Android rapfi 二进制编译为 PIE (Position-Independent Executable)

**背景**:
- Android 5.0 (API 21+) 强制要求所有可执行文件为 PIE
- 非 PIE 的 `ET_EXEC` 类型二进制会被 linker 拒绝执行
- 当前 x86_64 版本是静态链接的 `ET_EXEC`，无法在 Android 上运行

**实施方法**:
```cmake
# CMake 配置
-DANDROID_STL=c++_shared     # 使用动态 C++ 运行时
# 不设置 NO_COMMAND_MODULES   # 保持完整功能，避免静态链接
```

**验证检查**:
```bash
file rapfi-x86_64-linux-android
# 期望: ELF 64-bit LSB pie executable

readelf -h rapfi-x86_64-linux-android | grep Type
# 期望: Type: DYN (Position-Independent Executable)
```

**理由**:
- ✅ **兼容性**: PIE 是 Android 5.0+ 的硬性要求
- ✅ **动态链接**: 使用 `libc++_shared` 避免静态链接触发 EXEC 类型
- ✅ **NDK 默认**: NDK r25+ 默认生成 PIE，符合标准实践

**替代方案**:
- ❌ 使用静态链接 + 手动 `-fPIE -pie` 标志（更复杂，容易出错）
- ❌ 降级到 pre-Android 5.0（不现实，API 21 是最低支持）

### 6. Shell Wrapper 执行

**决策**: 在 Android 上通过 `/system/bin/sh` wrapper 执行 rapfi 二进制

**背景**:
- SELinux 策略阻止 `untrusted_app` domain 直接执行 `app_data_file` 类型
- `execute_no_trans` 权限被拒绝，即使文件有 755 权限
- Shell (runas_app domain) 有权限执行 app_data_file

**实施方法**:
```rust
// Android 专用代码
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

**理由**:
- ✅ **绕过 SELinux**: Shell domain 有执行权限
- ✅ **简单实现**: 只需修改 Command 调用，无需复杂的 JNI 或 native code
- ✅ **零开销**: Shell wrapper 的开销可忽略不计
- ✅ **兼容性**: 所有 Android 设备都有 `/system/bin/sh`

**替代方案**:
- ❌ 修改 SELinux 策略（需要 root 或系统签名）
- ❌ 使用 memfd（同样被 SELinux 阻止，见探索日志）
- ❌ JNI 集成（复杂度高，需要修改 rapfi 源码）

### 7. 模块组织

**决策**: 创建独立的 `android_rapfi.rs` 模块

**文件结构**:
```
src-tauri/src/
├── lib.rs              # 添加 `mod android_rapfi;`
├── rapfi.rs            # 添加 Android 平台处理（~15 行）
└── android_rapfi.rs    # 新增提取逻辑（~80 行）
```

**职责分离**:
- `android_rapfi.rs`: 负责二进制提取、权限设置、架构检测
- `rapfi.rs`: 负责 Piskvork 协议通信，调用 `android_rapfi` 获取路径
- `lib.rs`: 模块声明，不包含业务逻辑

**理由**:
- ✅ **单一职责**: 每个模块职责明确
- ✅ **可测试**: `android_rapfi` 可以独立测试
- ✅ **可复用**: 未来其他 Android 二进制可以复用此模块

## Risks / Trade-offs

### Risk 1: 提取过程可能较慢

**影响**: 首次启动时可能增加 100-200ms 延迟

**缓解措施**:
- 仅在首次运行时提取，后续使用缓存
- 提取操作在异步线程执行（如果需要）
- 日志中明确显示提取进度

### Risk 2: 权限设置可能失败

**影响**: 某些设备或 SELinux 配置可能阻止 chmod

**缓解措施**:
- 捕获权限设置错误并记录日志
- 如果权限设置失败，返回明确错误信息
- 在测试中验证多种设备

### Risk 3: 架构不匹配

**影响**: 如果设备架构不是 aarch64 或 x86_64，无法使用 AI

**缓解措施**:
- 提前在编译时检测架构（`cfg!` 宏）
- 运行时返回明确的错误信息
- 文档中说明支持的架构

### Risk 4: 磁盘空间不足

**影响**: 提取 24MB 二进制可能因空间不足失败

**缓解措施**:
- 捕获文件复制错误
- 返回用户友好的错误信息
- 系统会自动清理 cache 目录

### Trade-off: APK 体积增加

**权衡**: 每个架构 24MB，两个架构共 48MB

**接受原因**:
- AI 功能是核心功能，体积增加可接受
- 用户下载 APK 通常在 WiFi 环境下
- 可以在后续版本中优化（如按需下载）

## Migration Plan

### 部署步骤

1. **代码修改** (开发环境)
   ```bash
   # 1. 修改 src-tauri/src/lib.rs
   # 2. 修改 src-tauri/src/rapfi.rs
   # 3. 创建 src-tauri/src/android_rapfi.rs
   # 4. 验证 src-tauri/tauri.conf.json
   ```

2. **本地测试**
   ```bash
   pnpm tauri android dev
   adb logcat | grep -E "AI|rapfi|Android"
   ```

3. **功能验证**
   - 测试 AI 对战
   - 验证三个难度级别
   - 测试首次启动提取逻辑
   - 测试后续启动缓存逻辑

4. **构建发布版本**
   ```bash
   pnpm tauri build --target android
   ```

5. **发布到应用商店**

### 回滚策略

**回滚条件**:
- 在真实设备上发现严重 bug
- 性能严重下降
- 用户报告大量崩溃

**回滚步骤**:
1. 回滚代码修改（删除 `android_rapfi.rs`，恢复 `rapfi.rs` 和 `lib.rs`）
2. 发布新版本 APK
3. 从 Google Play 下架有问题的版本

**风险**: 低
- 修改量小（~96 行代码），回滚简单
- 不影响桌面平台
- 可以通过热修复快速发布

## Open Questions

### Q1: 是否需要支持 ARMv7 (armeabi-v7a) 设备?

**背景**: 部分 2019 年以前的设备使用 32 位 ARM

**当前决策**: 暂不支持

**理由**:
- ARMv7 设备市场份额 < 1%
- 需要额外的编译和测试工作
- NEON DOTPROD 性能优化在 ARMv7 上不可用

**未来考虑**: 如果用户需求强烈，可以添加 ARMv7 二进制

### Q2: 是否需要添加进度提示 UI?

**背景**: 首次提取 24MB 可能需要 100-200ms

**当前决策**: 不添加 UI，仅在日志中显示

**理由**:
- 延迟很短，用户感知不明显
- 添加 UI 增加复杂度
- 日志已经足够调试

**未来考虑**: 如果用户反馈首次启动慢，可以考虑添加进度提示

### Q3: 是否需要定期清理 cache 中的 rapfi?

**背景**: rapfi 版本更新后，cache 中的旧版本可能失效

**当前决策**: 不主动清理，依赖系统自动清理

**理由**:
- rapfi 更新频率低
- cache 中的旧版本仍然可以工作
- 系统会在空间不足时自动清理

**未来考虑**: 可以在启动时检查版本号，如果不匹配则重新提取
