## Why

Tauri Android 项目的自定义构建配置（打包选项、rapfi 二进制复制任务、NDK 依赖拷贝）当前直接写在 `gen/android/app/build.gradle.kts` 中，与 Tauri 自动生成的模板代码混合在一起。这导致：

1. **关注点不清** — 无法一眼区分哪些是 Tauri 模板、哪些是业务自定义
2. **升级风险** — Tauri CLI 升级时，diff 中 Tauri 模板变更和自定义变更混杂
3. **新人困惑** — `gen/` 目录本质上是 Tauri 管理的生成目录，把自定义逻辑放在这里不符合直觉
4. **`MainActivity.kt` 中存在死代码** — JNI 方法 `getRapfiExecutablePath()` 和 `getNativeLibraryDir()` 从未被 Rust 端调用（`android_rapfi.rs` 通过解析 `/proc/self/maps` 自行解析路径）

## What Changes

- **新增 `src-tauri/android/app-custom.gradle.kts`** — 将自定义 Gradle 构建逻辑（packagingOptions、copyRapfiBinaries、preBuild 依赖）从 `gen/` 迁移到 git 追踪的 `src-tauri/android/` 目录
- **精简 `gen/android/app/build.gradle.kts`** — 保留 Tauri 模板 + 一行 `apply(from = "../../android/app-custom.gradle.kts")`
- **精简 `gen/android/app/src/main/java/.../MainActivity.kt`** — 删除死代码 JNI 方法，仅保留 Tauri 活动生命周期

## Capabilities

### New Capabilities

- `android-gradle-separation`: 自定义 Android 构建逻辑从 `gen/` 分离到 `src-tauri/android/`，通过 Gradle `apply from` 引用

### Modified Capabilities

无。这是一次纯重构，不改变任何功能行为。

## Impact

**代码影响:**
- 新增: `src-tauri/android/app-custom.gradle.kts` (~90 行，从 `app/build.gradle.kts` 迁出)
- 修改: `gen/android/app/build.gradle.kts` (从 142 行精简为 ~70 行)
- 修改: `gen/android/app/src/main/java/.../MainActivity.kt` (删除 ~9 行死代码)
- 新增: `src-tauri/android/README.md` (说明文件)

**功能影响:**
- 桌面平台: 无影响
- Android 平台: 无功能变化，纯重构

**工作流影响:**
- `pnpm tauri android dev` 和 `pnpm tauri android build` 工作流不变
- Gradle 相对路径 `../../android/app-custom.gradle.kts` 在 `gen/android/app/` 上下文中正确解析到 `src-tauri/android/`
