# Proposal: Android Rapfi Integration

## Why

Android 用户无法使用 tauri-gobang 的 AI 对战功能。虽然 rapfi AI 引擎已编译为 Android 二进制并打包到 APK 中，但存在两个关键问题：

1. **原始问题**: 缺少运行时从 assets 提取可执行文件的逻辑，导致找不到 rapfi 进程
2. **新发现问题**: x86_64 二进制是非 PIE 的静态链接 EXEC 类型，Android 5.0+ linker 拒绝执行，且 SELinux 策略阻止直接执行

**根本原因**:
- x86_64 使用 `NO_COMMAND_MODULES=ON` 编译，触发静态链接
- 静态链接配置导致生成 `ET_EXEC` 而非 `ET_DYN` (PIE) 类型
- Android 强制要求 PIE (Position-Independent Executable)
- SELinux `execute_no_trans` 策略即使对于 PIE 也阻止直接执行

## What Changes

- **新增 Android 二进制提取模块**: 在首次运行时从 APK assets 自动提取 rapfi 二进制到应用 cache 目录，并设置可执行权限
- **添加平台特定路径解析**: 在 rapfi.rs 的 `get_engine_path()` 函数中添加 Android 平台的特殊处理逻辑
- **模块声明更新**: 在 lib.rs 中添加 android_rapfi 模块声明
- **资源配置验证**: 确保 tauri.conf.json 的 `bundle.resources` 包含 rapfi 二进制文件

## Capabilities

### New Capabilities

- `android-rapfi-runtime`: Android 平台上 rapfi 二进制的运行时提取、权限设置和缓存管理
- `android-pie-execution`: 确保所有 Android rapfi 二进制为 PIE (Position-Independent Executable) 类型，兼容 Android 5.0+
- `android-shell-wrapper`: 通过 shell wrapper 绕过 SELinux `execute_no_trans` 限制，在 Android 上执行 rapfi

### Modified Capabilities

- `rapfi-execution`: 扩展到支持 Android 平台的特定执行方式（shell wrapper）

## Impact

**代码影响**:
- 修改: `src-tauri/src/lib.rs` (添加 1 行模块声明)
- 修改: `src-tauri/src/rapfi.rs` (添加 ~30 行 Android 平台处理逻辑 + shell wrapper)
- 新增: `src-tauri/src/android_rapfi.rs` (~80 行，Android 二进制提取模块)
- 重建: `src-tauri/binaries/rapfi-x86_64-linux-android` (从 1.9MB 静态链接 EXEC 重建为 ~24MB 动态链接 PIE)
- 验证: `src-tauri/tauri.conf.json` (确认资源配置)
- 新增文档: `docs/ANDROID_RAPFI_REBUILD_GUIDE.md` (重建指南)
- 新增文档: `docs/ANDROID_RAPFI_QUICK_REFERENCE.md` (快速参考)

**依赖影响**:
- 无新增外部依赖
- 仅使用现有的 Tauri API (`BaseDirectory::Resource`, `app_cache_dir()`)

**平台影响**:
- **Android x86_64**: 从完全不可用变为可用（重建 PIE 二进制 + shell wrapper）
- **Android ARM64**: 保持可用（已有 PIE 二进制，添加 shell wrapper）
- **桌面** (Windows/macOS/Linux): 无影响，保持现有行为

**性能影响**:
- 首次启动: 额外 100-200ms 用于提取 24MB 二进制文件
- 后续启动: 无影响（使用缓存的二进制）
- AI 响应时间: 无变化（使用原生进程，性能与桌面一致）
- Shell wrapper 开销: 可忽略不计（<1ms）

**用户体验影响**:
- Android 用户可以流畅使用 AI 对战功能（包括 x86_64 模拟器用户）
- 支持 99% 的 Android 设备 (ARM64 + x86_64)
- 三个难度级别 (简单/中等/困难) 完全可用
- APK 大小增加 ~22MB（从非 PIE 1.9MB 到 PIE 24MB），但仍在可接受范围
