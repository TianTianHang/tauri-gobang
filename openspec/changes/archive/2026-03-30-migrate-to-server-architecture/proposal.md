# Proposal: Migrate to Server Architecture

## Why

当前五子棋游戏使用 P2P 直连模式（TCP），玩家需要手动交换 IP 地址才能开始对局。这导致多个痛点：
- **NAT 穿透困难**：不同网络环境下的玩家无法连接（如家庭路由器、公司防火墙）
- **无用户系统**：无法识别玩家身份，无法保存对局历史
- **无房间列表**：无法发现可用对局，必须私下约定
- **难以扩展**：无法添加观战、聊天、排位等社交功能

通过迁移到客户端-服务器架构，可以解决这些问题并提供更好的用户体验。

## What Changes

### 服务器端（新增独立项目）
- **新增 server/ 目录**：独立的 Rust 项目，使用 axum + SQLite + WebSocket
- **用户认证**：用户名+密码注册/登录，bcrypt 哈希，session token 管理
- **房间管理**：创建房间、加入房间、房间列表查询、自动开始游戏
- **消息转发**：服务器作为中继转发游戏消息（Move, Restart 等），不做游戏逻辑验证
- **断线重连**：30 秒重连窗口，超时则对手获胜

### 客户端改造
- **UI 改造**：
  - 新增登录/注册界面
  - 改造 NetworkSetup 为房间列表 + 创建/加入房间
  - 断线重连提示对话框
- **通信层改造**：
  - 从 TCP 直连改为 WebSocket 连接服务器
  - 新增 HTTP 调用（登录、创建房间等 REST API）
  - 修改 network.rs 适配服务器模式
- **保持向后兼容**：保留原有本地人机对战模式

### 部署方案
- **单二进制部署**：服务器打包为单个可执行文件
- **自动配置**：首次运行自动创建 SQLite 数据库和配置文件
- **后台运行**：支持 daemon 模式或 systemd service

## Capabilities

### New Capabilities
- `user-auth`: 用户注册、登录、会话管理（简单用户名+密码策略）
- `room-management`: 创建房间、加入房间、查询房间列表、房间状态管理
- `server-messaging`: WebSocket 消息转发、房间内广播、连接管理
- `reconnect-handling`: 检测断线、30 秒重连窗口、超时判负
- `server-deployment`: 单二进制打包、自动初始化数据库、后台运行

### Modified Capabilities
- `network-play`: 现有 P2P 联机功能将完全改造为服务器模式
  - 从 TCP 直连改为 WebSocket 连接服务器
  - 从手动输入 IP 改为房间列表选择
  - 新增登录/注册前置步骤
  - **BREAKING**: 不再支持直接输入 IP 连接（服务器模式）

## Impact

### 代码变更
- **新增**：`server/` 完整的独立服务器项目（约 2000-3000 行 Rust 代码）
- **修改**：
  - `src/components/NetworkSetup.tsx`：改为房间列表 UI
  - `src/components/*.tsx`：新增登录界面组件
  - `src-tauri/src/network.rs`：适配 WebSocket 协议
  - `src-tauri/src/lib.rs`：新增认证相关 Tauri commands
- **保留**：现有本地对战和人机对战逻辑完全不变

### 技术栈变更
- **新增依赖（服务器）**：
  - `axum`: Web 框架
  - `tokio-tungstenite`: WebSocket 支持
  - `sqlx`: 数据库 ORM
  - `bcrypt`: 密码哈希
  - `uuid`: ID 生成
- **新增依赖（客户端）**：
  - WebSocket 客户端库（如浏览器原生 WebSocket）
  - HTTP 客户端（fetch API）

### 部署影响
- **服务器部署**：需要一台公网服务器（VPS）运行 `gobang-server`
- **客户端配置**：需要配置服务器地址（可环境变量或配置文件）
- **数据库**：SQLite 单文件，无需额外数据库服务

### 用户体验变化
- **新用户流程**：注册 → 登录 → 房间列表 → 创建/加入房间 → 开始游戏
- **旧流程不再可用**：无法再通过输入对方 IP 直接连接
- **网络稳定性**：服务器模式下断线可重连，体验更稳定

### 性能影响
- **延迟增加**：从点对点变为客户端-服务器-客户端，增加一跳
- **服务器负载**：所有消息经过服务器，需要考虑并发能力（SQLite 可支持约 1000 并发连接）
- **带宽**：服务器需要处理所有游戏消息（小文本数据，影响小）
