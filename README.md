# 五子棋 / Gobang

跨平台五子棋游戏，支持人机对战（Rapfi 引擎）和局域网联机对战。

> **本项目 100% 由 AI 编写**，包括全部前端代码（React/TypeScript/CSS）、后端逻辑（Rust）、构建脚本、文档以及集成方案。无任何人工手写代码。

## 功能

- 人机对战 — 集成 [Rapfi](https://github.com/aawwsss/Rapfi) 五子棋引擎（Alpha-Beta + NNUE，2000+ ELO）
- 三档难度 — 简单 / 中等 / 困难
- 局域网联机 — TCP P2P，支持创建房间和加入房间
- 悔棋 — 人机模式下可悔两步
- 跨平台 — Windows / macOS / Linux / Android（arm64-v8a, x86_64）
- 深色模式 — 跟随系统偏好
- 触控优化 — 移动端触控预览、触觉反馈

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri 2 |
| 前端 | React 19 + TypeScript 5.8 |
| 构建 | Vite 7 + pnpm |
| 后端 | Rust 2021 |
| AI 引擎 | Rapfi（外部二进制，Gomocup 协议） |
| 联网 | TCP（line-delimited JSON） |

## 快速开始

```bash
# 克隆（含子模块）
git clone --recursive <repo-url>
cd tauri-gobang

# 安装依赖
pnpm install

# 下载 Rapfi 引擎
cd src-tauri && ./download-sidecar.sh && cd ..

# 开发模式
pnpm tauri dev

# 构建
pnpm tauri build
```

**如果克隆时忘记 `--recursive`：**

```bash
git submodule update --init --recursive
```

## 项目结构

```
src/                              # 前端
  App.tsx                         # 状态中枢 + 模式路由
  types/game.ts                   # 共享类型定义
  components/
    MainMenu.tsx                  # 模式选择
    GameBoard.tsx                 # Canvas 棋盘渲染
    StatusBar.tsx                 # 状态栏
    MenuDrawer.tsx                # 侧边设置抽屉
    NetworkSetup.tsx              # 联机表单
    Icons.tsx                     # SVG 图标
  hooks/
    useBoardSize.ts               # 响应式棋盘尺寸
    useHapticFeedback.ts          # 触觉反馈
    useTouchPreview.ts            # 触控预览

src-tauri/src/                    # Rust 后端
  lib.rs                          # Tauri commands + run()
  game.rs                         # 棋盘逻辑 + 胜负判定
  rapfi.rs                        # Rapfi 引擎封装
  network.rs                      # TCP 联网
  android_rapfi.rs                # Android 路径解析

third-party/rapfi/                # Rapfi 引擎源码（git submodule）
scripts/                          # 构建/同步脚本
docs/                             # 文档
```

## Android 支持

- 支持架构：`arm64-v8a`、`x86_64`（不支持 32 位）
- 最低版本：Android 7.0 (API 24)
- 引擎从 APK 的 `jniLibs` 加载，无需额外权限

```bash
pnpm tauri android dev     # 开发模式
pnpm tauri android build   # 构建 APK
```

## 更新 NNUE 权重

```bash
cd third-party/rapfi
git submodule update --remote Networks
cd ../..
bash scripts/sync-weights-from-networks.sh
```

查看当前权重版本：

```bash
bash scripts/sync-weights-from-networks.sh --info
```

## 文档

- [集成指南](docs/SIDECAR_INTEGRATION.md)
- [Android 构建](docs/ANDROID_RAPFI_BUILD.md)
- [快速参考](docs/QUICK_START.md)

## 故障排除

**克隆后找不到 rapfi 源码**
```bash
git submodule update --init --recursive
```

**Networks 子模块未初始化（权重缺失）**
```bash
cd third-party/rapfi && git submodule update --init --recursive
```

**rapfi.tmp 迁移冲突**
```bash
mv third-party/rapfi.tmp third-party/rapfi.tmp.backup
git submodule update --init --recursive
bash scripts/apply-rapfi-patches.sh
```

**Android 构建失败**
```bash
git submodule update --init --recursive
bash scripts/apply-rapfi-patches.sh
bash scripts/build-android-rapfi.sh
```
