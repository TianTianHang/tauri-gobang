# Spec: Android Rapfi Runtime

## ADDED Requirements

### Requirement: Automatic binary extraction from assets

当应用在 Android 平台首次启动时，系统 SHALL 自动从 APK assets 中提取 rapfi 二进制文件到应用的 cache 目录。

#### Scenario: First launch extracts binary
- **WHEN** 应用在 Android 设备上首次启动
- **AND** cache 目录中不存在 rapfi 二进制
- **THEN** 系统从 `assets/binaries/rapfi-<arch>-linux-android` 复制二进制到 `<cache-dir>/rapfi`
- **AND** 设置文件权限为 755 (rwxr-xr-x)
- **AND** 记录提取成功的日志信息

#### Scenario: Subsequent launch uses cached binary
- **WHEN** 应用在 Android 设备上非首次启动
- **AND** cache 目录中已存在 rapfi 二进制
- **THEN** 系统直接使用缓存的二进制文件
- **AND** 不执行提取操作
- **AND** 记录使用缓存的日志信息

#### Scenario: Extraction fails with clear error
- **WHEN** 从 assets 复制二进制文件失败
- **THEN** 系统返回明确的错误信息
- **AND** 错误信息包含失败原因（如文件不存在、权限不足等）
- **AND** 记录错误日志

### Requirement: Architecture-specific binary selection

系统 SHALL 根据设备的 CPU 架构自动选择对应的 rapfi 二进制文件。

#### Scenario: ARM64 device uses ARM64 binary
- **WHEN** 应用运行在 ARM64 (aarch64) 设备上
- **THEN** 系统提取 `rapfi-aarch64-linux-android` 二进制
- **AND** 使用 NEON DOTPROD 优化

#### Scenario: x86_64 device uses x86_64 binary
- **WHEN** 应用运行在 x86_64 设备上（如模拟器）
- **THEN** 系统提取 `rapfi-x86_64-linux-android` 二进制

#### Scenario: Unsupported architecture returns error
- **WHEN** 应用运行在非 ARM64 或 x86_64 的设备上
- **THEN** 系统返回 "不支持的架构" 错误
- **AND** 错误信息包含当前架构名称

### Requirement: Executable permission setting

系统 SHALL 为提取的 rapfi 二进制文件设置可执行权限，使其可以作为独立进程运行。

#### Scenario: Permission granted on extraction
- **WHEN** 二进制文件成功提取到 cache 目录
- **THEN** 系统设置文件权限为 755 (rwxr-xr-x)
- **AND** 所有者可读、可写、可执行
- **AND** 组用户和其他用户可读、可执行

#### Scenario: Permission setting failure returns error
- **WHEN** 设置文件权限失败
- **THEN** 系统返回明确的错误信息
- **AND** 错误信息包含权限设置失败的详情

### Requirement: Cache directory management

系统 SHALL 使用应用的 cache 目录存储提取的 rapfi 二进制，并确保目录存在。

#### Scenario: Cache directory created if missing
- **WHEN** cache 目录不存在
- **THEN** 系统创建 cache 目录及其所有父目录
- **AND** 使用默认权限（遵循系统策略）

#### Scenario: Cache directory persists across app restarts
- **WHEN** 应用重启
- **AND** cache 目录未被系统清理
- **THEN** 系统使用已存在的 cache 目录
- **AND** 已提取的 rapfi 二进制仍然可用

### Requirement: Fallback to existing path resolution

当 Android 提取逻辑失败时，系统 SHALL 回退到原有的路径查找逻辑，保持向后兼容。

#### Scenario: Extraction failure triggers fallback
- **WHEN** Android 提取逻辑失败或返回错误
- **THEN** 系统继续执行原有的桌面平台路径查找逻辑
- **AND** 尝试从其他位置查找 rapfi 二进制
- **AND** 记录回退的日志信息

#### Scenario: Desktop platforms unaffected
- **WHEN** 应用运行在非 Android 平台（Windows/macOS/Linux）
- **THEN** 系统不执行 Android 提取逻辑
- **AND** 使用原有的 externalBin 或路径查找逻辑
- **AND** 行为与变更前完全一致

### Requirement: Logging and debugging

系统 SHALL 在关键步骤记录日志信息，便于调试和问题排查。

#### Scenario: Successful extraction logged
- **WHEN** 二进制提取成功
- **THEN** 系统记录 "提取成功" 日志
- **AND** 日志包含二进制文件路径和架构信息

#### Scenario: Platform detection logged
- **WHEN** 系统检测到 Android 平台
- **THEN** 系统记录 "检测到 Android 平台" 日志
- **AND** 日志包含架构信息（aarch64 或 x86_64）

#### Scenario: Error conditions logged
- **WHEN** 任何步骤失败（提取、权限设置等）
- **THEN** 系统记录详细的错误日志
- **AND** 日志包含失败原因和上下文信息
