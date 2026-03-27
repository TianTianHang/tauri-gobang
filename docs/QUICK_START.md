# 快速开始指南

## 安装依赖

```bash
pnpm install
```

## 下载 AI 引擎

**必需步骤** - 应用需要 Rapfi 五子棋引擎才能运行 AI 模式。

### Linux
```bash
cd src-tauri
./download-sidecar.sh
```

### Windows
```cmd
cd src-tauri
download-sidecar.bat
```

### macOS
```bash
# 需要从源码编译，详见 docs/SIDECAR_INTEGRATION.md
```

## 开发模式

```bash
pnpm tauri dev
```

## 构建应用

```bash
pnpm tauri build
```

构建完成后，应用位于：
- **Linux**: `src-tauri/target/release/bundle/`
- **Windows**: `src-tauri/target/release/bundle/`
- **macOS**: `src-tauri/target/release/bundle/`

## 使用 AI

应用启动后：
1. 选择 "人机对战" 模式
2. 选择难度（简单/中等/困难）
3. 开始游戏！

AI 会自动使用打包的 Rapfi 引擎。

## 故障排除

### AI 无法启动

**错误**: `Rapfi engine not found`

**解决**:
```bash
cd src-tauri
./download-sidecar.sh  # Linux/macOS
# 或
download-sidecar.bat   # Windows
```

### 引擎下载失败

手动下载：
1. 访问 https://github.com/dhbloo/rapfi/releases
2. 下载对应平台的二进制文件
3. 放到 `src-tauri/binaries/` 目录
4. 重命名为 `rapfi` (Linux/macOS) 或 `rapfi.exe` (Windows)

## 更多信息

- [完整集成指南](SIDECAR_INTEGRATION.md)
- [Rapfi GitHub](https://github.com/dhbloo/rapfi)
