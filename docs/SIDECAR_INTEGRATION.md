# Sidecar 外部引擎集成指南

本项目使用 Tauri 2 的 **Sidecar** 功能来集成 Rapfi 五子棋引擎。

## 什么是 Sidecar？

Sidecar 是 Tauri 的外部二进制文件管理功能，可以：
- 自动将外部可执行文件打包到应用中
- 在运行时自动定位这些文件
- 跨平台支持（Windows、macOS、Linux）

## 快速开始

### 1. 下载引擎

**Linux:**
```bash
cd src-tauri
./download-sidecar.sh
```

**Windows:**
```cmd
cd src-tauri
download-sidecar.bat
```

**macOS:**
```bash
# 需要从源码编译
cd src-tauri
git clone https://github.com/dhbloo/rapfi.git
cd rapfi/Rapfi
cmake --preset arm64-clang-NEON
cmake --build build/arm64-clang-NEON
cp build/arm64-clang-NEON/rapfi ../binaries/rapfi
```

### 2. 构建应用

```bash
pnpm tauri build
```

Tauri 会自动：
- 将 `binaries/rapfi` 打包到应用中
- 在运行时自动定位引擎文件
- 在开发模式下也可以使用

### 3. 使用外部 AI

```typescript
// 自动使用打包的引擎
const result = await invoke<AiMoveResult>('ai_move_rapfi', {
  state: gameState,
  difficulty: 'hard',
  enginePath: null  // null = 使用 sidecar
});

// 或指定自定义路径
const result = await invoke<AiMoveResult>('ai_move_rapfi', {
  state: gameState,
  difficulty: 'hard',
  enginePath: '/path/to/rapfi'
});
```

## 配置说明

### tauri.conf.json

```json
{
  "bundle": {
    "externalBin": [
      "binaries/rapfi"
    ]
  }
}
```

`externalBin` 配置告诉 Tauri：
- 在开发时，从 `src-tauri/binaries/rapfi` 加载
- 在构建时，将二进制文件打包到应用中
- 在运行时，自动定位到正确的路径

### 路径解析

Tauri Sidecar 会按以下顺序查找引擎：

1. **开发模式:**
   - `src-tauri/binaries/rapfi`

2. **生产构建 (Linux):**
   - `<可执行文件目录>/rapfi`
   - `<可执行文件目录>/binaries/rapfi`

3. **生产构建 (Windows):**
   - `<可执行文件目录>\rapfi.exe`
   - `<可执行文件目录>\binaries\rapfi.exe`

4. **生产构建 (macOS):**
   - `<App.app>/Contents/MacOS/rapfi`
   - `<App.app>/Contents/Resources/binaries/rapfi`

## 文件结构

```
src-tauri/
├── binaries/
│   ├── rapfi          # Linux/macOS 引擎
│   ├── rapfi.exe      # Windows 引擎
│   └── *.nnue         # 可选：神经网络权重文件
├── tauri.conf.json    # 包含 externalBin 配置
├── download-sidecar.sh    # Linux 下载脚本
├── download-sidecar.bat   # Windows 下载脚本
└── src/
    └── rapfi.rs       # 引擎集成代码
```

## 开发模式 vs 生产模式

### 开发模式
```bash
pnpm tauri dev
```
- 从 `src-tauri/binaries/` 加载引擎
- 不需要重新编译引擎
- 便于调试

### 生产模式
```bash
pnpm tauri build
```
- 引擎被打包到应用中
- 用户无需单独下载引擎
- 部署后自动可用

## 故障排除

### 错误：找不到引擎

**症状：** `Rapfi engine not found`

**解决方案：**
1. 检查 `src-tauri/binaries/` 目录是否有引擎文件
2. 运行下载脚本：
   ```bash
   cd src-tauri
   ./download-sidecar.sh  # Linux/macOS
   # 或
   download-sidecar.bat   # Windows
   ```

### 错误：权限被拒绝

**症状：** `Permission denied` (Linux/macOS)

**解决方案：**
```bash
chmod +x src-tauri/binaries/rapfi
```

### Windows 警告：未知发布者

**症状：** Windows SmartScreen 警告

**解决方案：**
- 点击"更多信息" → "仍要运行"
- 或对引擎进行代码签名（生产环境推荐）

### macOS: 无法验证开发者

**症状：** macOS Gatekeeper 阻止运行

**解决方案：**
```bash
# 移除隔离属性
xattr -d com.apple.quarantine src-tauri/binaries/rapfi

# 或允许运行
sudo spctl --add --label "Approved" src-tauri/binaries/rapfi
```

## 性能优化

### 启用 NNUE 神经网络

1. 下载网络权重：
   ```bash
   cd src-tauri/binaries
   curl -L -o networks.zip https://github.com/dhbloo/rapfi-networks/releases/latest/download/networks.zip
   unzip networks.zip
   ```

2. 将 `.nnue` 文件放在 `binaries/` 目录

3. 引擎会自动加载并使用神经网络评估

### CPU 特定优化

Rapfi 支持 CPU 特定优化：
- **AVX2**: 现代处理器
- **AVX-512**: 高端 Intel CPU
- **NEON**: ARM 处理器 (macOS M1/M2)

下载对应的预编译版本可获得最佳性能。

## 分发注意事项

### 许可证

- **Rapfi**: GPL-3.0
- 打包 GPL 软件可能需要：
  - 开源你的应用（如果分发）
  - 或提供源代码
  - 商业使用需谨慎

### 替代方案

对于商业应用：
1. 使用内置的 Rust AI（无需外部引擎）
2. 将引擎设为可选下载（不打包）
3. 寻找商业友好的五子棋引擎

## 高级配置

### 自定义引擎路径

```typescript
const result = await invoke<AiMoveResult>('ai_move_rapfi', {
  state: gameState,
  difficulty: 'hard',
  enginePath: '/opt/engines/rapfi'  // 自定义路径
});
```

### 多引擎支持

可以扩展代码支持多个引擎：
- Rapfi（强引擎，神经网络）
- Carbon-Gomoku（老牌强引擎）
- 其他 Gomocup 引擎

### 动态切换

```typescript
// 简单关卡用内置 AI（快速）
const result1 = await invoke<AiMoveResult>('ai_move', {
  state: gameState,
  difficulty: 'easy'
});

// 困难关卡用 Rapfi（强大）
const result2 = await invoke<AiMoveResult>('ai_move_rapfi', {
  state: gameState,
  difficulty: 'hard',
  enginePath: null
});
```

## 参考资料

- [Tauri Sidecar 官方文档](https://tauri.app/zh-cn/develop/sidecar/)
- [Rapfi GitHub](https://github.com/dhbloo/rapfi)
- [Rapfi Networks](https://github.com/dhbloo/rapfi-networks)
- [Gomocup 比赛](http://gomocup.org/)

## 总结

使用 Tauri Sidecar 集成外部引擎的优势：

✅ **自动化**: 无需手动管理文件路径
✅ **跨平台**: 一套配置，到处运行
✅ **透明**: 用户感知不到外部引擎的存在
✅ **灵活**: 可选使用，不影响内置 AI
✅ **强大**: 使用顶级五子棋 AI（2000+ ELO）
