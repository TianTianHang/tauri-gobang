# Rapfi 五子棋引擎集成完成

已成功将 **Rapfi** 五子棋AI引擎集成到 Tauri 应用中，使用 Tauri 2 的 Sidecar 功能。

## ✅ 完成的工作

### 1. 核心集成
- ✅ 创建了 `src-tauri/src/rapfi.rs` - Rapfi 引擎的 Rust 封装
- ✅ 实现了 Piskvork 协议通信（标准五子棋引擎协议）
- ✅ 添加了 `ai_move_rapfi` Tauri 命令
- ✅ 配置了 Tauri Sidecar 外部二进制文件支持

### 2. 构建系统
- ✅ 创建了 `src-tauri/build.rs` - 自动检查并打包引擎
- ✅ 配置了 `src-tauri/tauri.conf.json` - externalBin 设置
- ✅ 创建了下载脚本：
  - `src-tauri/download-sidecar.sh` (Linux/macOS)
  - `src-tauri/download-sidecar.bat` (Windows)

### 3. 文档
- ✅ `docs/SIDECAR_INTEGRATION.md` - Sidecar 集成完整指南
- ✅ `docs/BUNDLED_ENGINE.md` - 引擎使用说明
- ✅ `RAPI_INTEGRATION.md` - 通用集成文档

## 📦 如何使用

### 步骤 1: 下载 Rapfi 引擎

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

**macOS (需要从源码编译):**
```bash
# 参考 docs/SIDECAR_INTEGRATION.md 中的详细说明
```

### 步骤 2: 构建应用

```bash
pnpm tauri build
```

引擎会自动打包到应用中！

### 步骤 3: 使用引擎

```typescript
// 使用打包的 Rapfi 引擎（推荐）
const result = await invoke<AiMoveResult>('ai_move_rapfi', {
  state: gameState,
  difficulty: 'hard',
  enginePath: null  // 自动查找 sidecar
});

// 或使用内置 Rust AI
const result = await invoke<AiMoveResult>('ai_move', {
  state: gameState,
  difficulty: 'hard'
});
```

## 🎯 功能对比

| 特性 | 内置 AI (Rust) | Rapfi 引擎 |
|------|----------------|------------|
| **算法** | Negamax + Alpha-Beta | Alpha-Beta + NNUE 神经网络 |
| **强度** | 中等 | 强 (2000+ ELO) |
| **速度** | 快速 | 中等 (可配置) |
| **大小** | 小 | +5-10 MB |
| **依赖** | 无 | 无 (自包含) |
| **打包** | 自动 | 需下载后打包 |

## 📂 文件结构

```
src-tauri/
├── binaries/                    # Sidecar 外部二进制文件目录
│   ├── rapfi                    # Linux 引擎 (运行时自动重命名)
│   ├── rapfi.exe                # Windows 引擎
│   └── rapfi-aarch64-apple-darwin  # macOS ARM64 引擎
├── src/
│   ├── rapfi.rs                 # Rapfi 引擎封装
│   ├── lib.rs                   # 已添加 ai_move_rapfi 命令
│   └── ...
├── build.rs                     # 构建脚本（检查引擎）
├── download-sidecar.sh          # Linux/macOS 下载脚本
├── download-sidecar.bat         # Windows 下载脚本
└── tauri.conf.json             # 已配置 externalBin
```

## 🔧 技术细节

### Piskvork 协议

Rapfi 使用 Piskvork 协议（也称为 Gomocup 协议），这是五子棋引擎的标准通信协议：

```
START 15          → 初始化 15x15 棋盘
BOARD             → 发送棋盘状态
7,7,1             → 坐标 (7,7) 黑棋 (1)
DONE              → 完成发送
INFO timeout_turn 3000  → 设置超时 3 秒
TURN 8,8          → 对手下在 (8,8)
7,9               → AI 回复 (7,9)
END               → 结束
```

### Tauri Sidecar 命名规范

Tauri 会根据目标平台自动查找带有特定后缀的二进制文件：

- **Linux x64**: `rapfi-x86_64-unknown-linux-gnu`
- **Windows x64**: `rapfi-x86_64-pc-windows-msvc.exe`
- **macOS Intel**: `rapfi-x86_64-apple-darwin`
- **macOS ARM64**: `rapfi-aarch64-apple-darwin`

下载脚本会自动创建这些带后缀的副本。

### 路径解析顺序

代码按以下顺序查找引擎：

1. 可执行文件同目录下的 `rapfi`
2. 可执行文件同目录下的 `binaries/rapfi`
3. 可执行文件上级目录的 `binaries/rapfi`
4. 当前工作目录的 `./binaries/rapfi`

## 🚀 性能优化

### 1. 难度级别映射

```rust
Easy   → 500ms  (快速)
Medium → 1500ms (平衡)
Hard   → 3000ms (深度搜索，可能使用 NNUE)
```

### 2. NNUE 神经网络（可选）

下载并放置 `.nnue` 文件到 `src-tauri/binaries/` 可启用神经网络评估：

```bash
cd src-tauri/binaries
curl -L -o networks.zip https://github.com/dhbloo/rapfi-networks/releases/latest/download/networks.zip
unzip networks.zip
```

### 3. 多线程支持

Rapfi 支持多线程搜索，在 Hard 模式下会自动利用所有 CPU 核心。

## ⚠️ 注意事项

### 1. 许可证

- **Rapfi**: GPL-3.0
- 打包 GPL 软件到商业应用可能需要开源你的代码
- 建议在商业应用中将引擎设为可选下载

### 2. 平台兼容性

| 平台 | 状态 | 备注 |
|------|------|------|
| Linux x64 | ✅ 完全支持 | 预编译二进制可用 |
| Windows x64 | ✅ 完全支持 | 预编译二进制可用 |
| macOS Intel | ⚠️ 需编译 | 需从源码编译 |
| macOS ARM64 | ⚠️ 需编译 | 需从源码编译 |
| Linux ARM32/64 | ⚠️ 需编译 | 需从源码编译 |

### 3. 安全性

Sidecar 二进制文件会随应用一起打包，确保：
- 从官方源下载 Rapfi
- 验证文件哈希（生产环境）
- 考虑代码签名（Windows/macOS）

## 🐛 故障排除

### 问题 1: 编译警告 "Rapfi engine not found"

**原因**: 引擎未下载到 `src-tauri/binaries/`

**解决**: 运行下载脚本
```bash
cd src-tauri
./download-sidecar.sh
```

### 问题 2: 运行时错误 "Failed to start engine"

**原因**: 引擎路径不正确或文件不存在

**解决**:
1. 检查 `src-tauri/binaries/` 目录
2. 确保文件名符合 sidecar 规范
3. 或使用 `enginePath` 参数指定完整路径

### 问题 3: 引擎超时

**原因**: 超时设置过短或硬件性能不足

**解决**:
- 降低难度（Medium → Easy）
- 增加超时时间（修改代码中的 Duration）

## 📚 参考资料

- [Tauri Sidecar 官方文档](https://tauri.app/zh-cn/develop/sidecar/)
- [Rapfi GitHub](https://github.com/dhbloo/rapfi)
- [Rapfi Networks (NNUE)](https://github.com/dhbloo/rapfi-networks)
- [Gomocup 五子棋 AI 比赛](http://gomocup.org/)
- [Piskvork 协议规范](https://plastovicka.github.io/protocl2en.htm)

## 🎉 总结

你现在有两个 AI 选择：

1. **内置 Rust AI** - 快速、轻量、完全开源
2. **Rapfi 外部引擎** - 强大、专业级、2000+ ELO

两者可以自由切换，为用户提供从休闲到竞技的完整体验！
