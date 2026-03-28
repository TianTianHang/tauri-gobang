# Tasks: Android Rapfi Integration

## 1. 代码准备

- [x] 1.1 验证现有文件存在
- [x] 1.2 确认 `src-tauri/binaries/rapfi-aarch64-linux-android` 存在且可执行
- [x] 1.3 确认 `src-tauri/binaries/rapfi-x86_64-linux-android` 存在且可执行
- [x] 1.4 确认 `src-tauri/binaries/config.toml` 和权重文件存在

## 2. Android 提取模块实现

- [x] 2.1 创建 `src-tauri/src/android_rapfi.rs` 文件
- [x] 2.2 实现 `extract_rapfi_binary()` 函数签名
- [x] 2.3 实现架构检测逻辑（使用 `cfg!` 宏）
- [x] 2.4 实现从 assets 复制二进制文件的逻辑
- [x] 2.5 实现设置可执行权限（chmod 755）的逻辑
- [x] 2.6 实现缓存检查逻辑（避免重复提取）
- [x] 2.7 添加错误处理和日志输出
- [x] 2.8 添加完整的文档注释

## 3. 模块集成

- [x] 3.1 修改 `src-tauri/src/lib.rs`
- [x] 3.2 在模块声明区域添加 `mod android_rapfi;`
- [x] 3.3 确保模块声明在其他模块之前

## 4. 平台特定路径解析

- [x] 4.1 修改 `src-tauri/src/rapfi.rs` 的 `get_engine_path()` 函数
- [x] 4.2 在函数开头添加 `#[cfg(target_os = "android")]` 条件编译块
- [x] 4.3 调用 `crate::android_rapfi::extract_rapfi_binary(app)`
- [x] 4.4 实现成功时直接返回路径的逻辑
- [x] 4.5 实现失败时记录日志并继续原有逻辑的回退机制
- [x] 4.6 确保不影响桌面平台的现有行为

## 5. 资源配置验证

- [x] 5.1 检查 `src-tauri/tauri.conf.json` 文件
- [x] 5.2 确认 `bundle.resources` 数组包含 `"binaries/rapfi-*-linux-android"`
- [x] 5.3 如果缺失，添加该资源配置
- [x] 5.4 验证 `bundle.externalBin` 配置正确

## 6. 编译验证

- [x] 6.1 运行 `cargo check --manifest-path src-tauri/Cargo.toml` 检查代码
- [x] 6.2 修复所有编译错误和警告
- [x] 6.3 确保无 `unused_imports` 或 `dead_code` 警告

## 7. 本地开发测试

- [ ] 7.1 运行 `pnpm tauri android dev` 启动开发服务器
- [ ] 7.2 使用 `adb logcat | grep -E "AI|rapfi|Android"` 查看日志
- [ ] 7.3 验证日志中出现 "🤖 [Android] Detected Android platform"
- [ ] 7.4 验证日志中出现 "📦 [Android] Extracting rapfi from assets"
- [ ] 7.5 验证日志中出现 "✅ [Android] rapfi extracted successfully"

## 8. AI 功能验证

- [ ] 8.1 在 Android 设备/模拟器上启动应用
- [ ] 8.2 开始新游戏，选择 "vs AI" 模式
- [ ] 8.3 测试 "简单" 难度级别
- [ ] 8.4 测试 "中等" 难度级别
- [ ] 8.5 测试 "困难" 难度级别
- [ ] 8.6 验证 AI 在合理时间内响应（<3秒）
- [ ] 8.7 验证 AI 落子符合规则

## 9. 缓存机制验证

- [ ] 9.1 完全关闭应用
- [ ] 9.2 重新启动应用
- [ ] 9.3 查看日志验证出现 "✅ [Android] rapfi already extracted"
- [ ] 9.4 验证第二次启动无需重新提取二进制

## 10. 桌面平台回归测试

- [ ] 10.1 在 Windows 上测试 AI 功能
- [ ] 10.2 在 macOS 上测试 AI 功能（如果有 Mac）
- [ ] 10.3 在 Linux 上测试 AI 功能（如果有 Linux）
- [ ] 10.4 验证桌面平台行为与变更前一致
- [ ] 10.5 验证无性能下降

## 11. 错误处理测试

- [ ] 11.1 测试 assets 中缺少二进制文件的情况
- [ ] 11.2 测试权限设置失败的情况（如果可以模拟）
- [ ] 11.3 验证所有错误都有清晰的日志信息
- [ ] 11.4 验证错误不会导致应用崩溃

## 12. 文档更新

- [x] 12.1 更新 README.md（如果需要）
- [x] 12.2 添加 Android 平台的部署说明
- [x] 12.3 更新 AGENTS.md 中的测试命令（如果需要）

## 13. 构建发布版本

- [ ] 13.1 运行 `pnpm tauri build --target android`
- [ ] 13.2 验证 APK 构建成功
- [ ] 13.3 使用 `unzip -l` 验证 APK 包含 rapfi 二进制
- [ ] 13.4 测试安装的 APK 是否可以正常使用 AI 功能

## 14. 清理和收尾

- [x] 14.1 运行 `cargo fmt` 格式化代码
- [x] 14.2 运行 `cargo clippy` 检查代码质量
- [x] 14.3 删除调试日志（如果需要）
- [ ] 14.4 提交代码到版本控制
