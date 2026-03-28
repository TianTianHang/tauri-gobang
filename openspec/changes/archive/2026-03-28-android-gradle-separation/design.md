## Context

**当前状态:**
- Tauri Android 项目通过 `pnpm tauri android init` 生成脚手架文件到 `gen/android/`
- 自定义构建逻辑（packagingOptions、copyRapfiBinaries、preBuild 钩子）直接写在 `gen/android/app/build.gradle.kts` 中
- `MainActivity.kt` 中包含从未被调用的 JNI 方法（`getNativeLibraryDir()`, `getRapfiExecutablePath()`）
- `src-tauri/.gitignore` 不忽略 `gen/` 目录本身（仅忽略 `/gen/schemas`），但 `gen/android/.gitignore` 和 `gen/android/app/.gitignore` 忽略了大部分自动生成文件
- git 中只追踪了 `gen/android/app/build.gradle.kts` 和 `MainActivity.kt` 两个文件

**约束条件:**
- 不改变任何功能行为，纯重构
- `pnpm tauri android dev` 和 `build` 工作流保持不变
- 必须支持"clone → init → dev"的工作流
- 不修改 Tauri CLI 源码

**相关系统:**
- `gen/android/app/build.gradle.kts` — Gradle 构建入口
- `gen/android/app/tauri.build.gradle.kts` — Tauri 自动生成的 plugin 依赖（gitignored）
- Tauri CLI `android init` — 脚手架生成器

## Goals / Non-Goals

**Goals:**
- 自定义构建逻辑从 `gen/` 迁移到 `src-tauri/android/`（git 追踪的安全目录）
- `gen/android/app/build.gradle.kts` 精简为 Tauri 模板 + 一行 `apply(from = ...)`
- `MainActivity.kt` 精简，删除死代码
- 新人拿到项目后能理解哪些是 Tauri 管的、哪些是自定义的

**Non-Goals:**
- 不解决"删 gen/ 再 init 会丢失 apply 行"的问题（这是 Tauri 框架层面的限制，apply 行保留在 git 追踪的 `app/build.gradle.kts` 中）
- 不修改 Tauri CLI 行为
- 不引入额外的 Gradle 插件或复合构建

## Decisions

### 1. 使用 Gradle `apply from` 引用外部文件

**决策**: 在 `gen/android/app/build.gradle.kts` 末尾添加 `apply(from = "../../android/app-custom.gradle.kts")`

**理由:**
- ✅ **Gradle 原生机制** — `apply from` 是 Gradle 内置功能，无需额外依赖
- ✅ **路径可推导** — 从 `gen/android/app/` 出发，`../../android/` 精确指向 `src-tauri/android/`
- ✅ **优先级明确** — `apply` 放在末尾，自定义配置覆盖或补充模板配置
- ✅ **被 Tauri 保护** — `app/build.gradle.kts` 已在 git 中追踪，`tauri android init` 不覆盖已有文件

**替代方案:**
- ❌ Gradle init script — 全局影响，不适合项目级自定义
- ❌ 复合构建 (composite build) — 过度复杂，需要额外 settings.gradle
- ❌ buildSrc 插件 — 需要编译，增加构建时间

### 2. 自定义文件放在 `src-tauri/android/` 而非项目根目录

**决策**: 自定义 Gradle 文件放在 `src-tauri/android/app-custom.gradle.kts`

**理由:**
- ✅ **与 Tauri 项目结构一致** — `src-tauri/` 是 Rust 后端根目录，Android 相关配置归在此处
- ✅ **相对路径简短** — 只需 `../../android/` 两级回溯
- ✅ **语义清晰** — `android/` 目录名明确表示这是 Android 特定配置

**替代方案:**
- ❌ 项目根目录 `android-custom/` — 路径更长，与 Tauri 项目结构不协调
- ❌ `src-tauri/android-custom/` — 含义模糊，`android/` 更标准

### 3. 精简 MainActivity.kt（删除 JNI 死代码）

**决策**: 删除 `getNativeLibraryDir()` 和 `getRapfiExecutablePath()` 方法

**理由:**
- ✅ **Rust 端不调用** — `android_rapfi.rs` 通过解析 `/proc/self/maps` + `base.apk` 路径自行推导 `librapfi.so` 位置，无需 JNI
- ✅ **减少维护负担** — 少一段死代码 = 少一个需要解释的地方
- ✅ **减少文件变更** — `MainActivity.kt` 变得极度精简

**替代方案:**
- ❌ 保留 JNI 方法 — 无害但增加噪音，新人可能误以为有调用方

### 4. 保留 `app/build.gradle.kts` 在 git 中（不移出 gen/）

**决策**: `app/build.gradle.kts` 继续保留在 `gen/` 目录中并追踪

**理由:**
- ✅ **Tauri 保护已有文件** — 实验证明 `tauri android init` 不会覆盖已存在的 `app/build.gradle.kts`
- ✅ **apply 行安全** — 只要文件在 git 中，clone 后就有 apply 行
- ✅ **最小改动** — 不需要修改 Tauri CLI 的行为

**局限:**
- ⚠️ 如果用户手动删除 `gen/` 再 init，apply 行会丢失 — 但这是极端操作，README 中有说明

## Risks / Trade-offs

### Risk 1: Gradle 相对路径在非标准环境中失效

**影响**: 如果 Tauri 改变项目结构或 `gen/android/` 的位置，`../../android/` 路径可能不正确

**缓解措施:**
- 路径由 Tauri CLI 自身生成，变更时会有 breaking change
- README 中说明路径推导逻辑
- 构建失败时 Gradle 会报明确的文件未找到错误

### Risk 2: Tauri CLI 升级可能重写 `app/build.gradle.kts`

**影响**: 未来 Tauri 版本可能改变 `init` 行为，覆盖已存在文件

**缓解措施:**
- 目前 Tauri 不覆盖（已验证）
- 即使覆盖，`src-tauri/android/app-custom.gradle.kts` 安全无损，只需重新加 apply 行
- git diff 能清楚显示丢失了什么

### Trade-off: 多了一个间接层

**权衡**: 开发者需要从 `app/build.gradle.kts` 跳转到 `app-custom.gradle.kts` 才能看到完整构建逻辑

**接受原因:**
- 间接层带来的是清晰的关注点分离
- `apply(from = ...)` 语义明确，不会让人困惑
- 大多数时候只看 `app-custom.gradle.kts`（自定义逻辑的真正所在地）

## Migration Plan

### 部署步骤

1. 创建 `src-tauri/android/app-custom.gradle.kts`（从 `app/build.gradle.kts` 迁出自定义逻辑）
2. 创建 `src-tauri/android/README.md`（说明文件）
3. 精简 `gen/android/app/build.gradle.kts`（保留模板 + apply 行）
4. 精简 `gen/android/app/.../MainActivity.kt`（删除 JNI 死代码）
5. 验证 `pnpm tauri android dev` 正常工作

### 回滚策略

**回滚条件:**
- 构建失败
- Gradle apply from 路径解析异常

**回滚步骤:**
1. 从 git 恢复 `app/build.gradle.kts` 和 `MainActivity.kt`
2. 删除 `src-tauri/android/` 目录

**风险**: 极低 — 回滚只需 1 条 git checkout 命令

## Open Questions

### Q1: 是否需要将 apply 行注入 Tauri CLI 模板？

**背景**: 当前 apply 行依赖于 `app/build.gradle.kts` 在 git 中。如果 Tauri 未来版本改变 init 行为，可能覆盖。

**当前决策**: 暂不处理

**理由**: 当前 Tauri 行为稳定，且覆盖后修复成本极低（加回 1 行）
