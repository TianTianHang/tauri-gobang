# gobang-server

五子棋游戏服务器 - 支持WebSocket实时对战、房间管理、用户认证。

## 功能特性

- **用户认证**: 用户名+密码注册/登录，bcrypt 密码哈希，session token 管理
- **房间管理**: 创建房间、加入房间、房间列表查询、自动开始游戏
- **实时对战**: WebSocket 消息转发，低延迟游戏体验
- **断线重连**: 30秒重连窗口，超时判负
- **单二进制部署**: 所有资源嵌入，无需额外配置文件

## 快速开始

### 运行服务器

```bash
# 编译
cargo build --release

# 首次运行（自动创建数据库和配置）
./target/release/gobang-server

# 自定义端口和数据库
./target/release/gobang-server --port 8080 --database /path/to/database.db
```

### 后台运行

```bash
# daemon模式（Unix only）
./target/release/gobang-server --daemon

# 或使用systemd服务（见 deploy/gobang-server.service）
sudo systemctl start gobang-server
```

### API 端点

| 端点 | 方法 | 描述 |
|------|------|------|
| `POST /api/register` | 注册 | `{"username": "...", "password": "..."}` |
| `POST /api/login` | 登录 | `{"username": "...", "password": "..."}` |
| `GET /api/rooms?token=...` | 获取房间列表 | 返回所有等待中的房间 |
| `POST /api/rooms?token=...` | 创建房间 | `{"name": "房间名"}` |
| `POST /api/rooms/:id/join?token=...` | 加入房间 | 加入指定房间 |
| `WS /game/:room_id?token=...` | WebSocket | 游戏消息通道 |

### WebSocket 协议

**客户端 → 服务器**:
```json
{"type": "move", "row": 7, "col": 8}
{"type": "restart_request"}
{"type": "restart_accept"}
{"type": "restart_reject"}
{"type": "disconnect"}
```

**服务器 → 客户端**:
```json
{"type": "game_start", "black_player": "Alice", "white_player": "Bob"}
{"type": "opponent_joined", "username": "Bob"}
{"type": "opponent_disconnected", "username": "Bob", "can_reconnect": true, "timeout_seconds": 30}
{"type": "player_reconnected", "username": "Bob"}
{"type": "game_ended", "winner": "Alice", "reason": "opponent_disconnected"}
```

## 配置

首次运行会自动生成 `~/.gobang-server/config.toml`:

```toml
server_host = "0.0.0.0"
server_port = 3001
database_path = "database.db"
log_level = "info"
reconnect_timeout_seconds = 30
password_min_length = 6
```

## 部署

### systemd 服务

```bash
# 复制服务文件
sudo cp deploy/gobang-server.service /etc/systemd/system/

# 创建用户和目录
sudo useradd -r -s /bin/false gobang
sudo mkdir -p /var/lib/gobang-server
sudo chown gobang:gobang /var/lib/gobang-server

# 复制二进制文件
sudo cp target/release/gobang-server /usr/local/bin/

# 启动服务
sudo systemctl daemon-reload
sudo systemctl enable gobang-server
sudo systemctl start gobang-server
```

### Docker (可选)

```dockerfile
FROM rust:1.83 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/gobang-server /usr/local/bin/
EXPOSE 3001
CMD ["gobang-server"]
```

## 日志配置

### 日志级别

服务器使用 `tracing` 进行结构化日志记录。可通过 `RUST_LOG` 环境变量配置日志级别:

```bash
# 生产环境推荐: 仅记录重要事件
RUST_LOG=info ./target/release/gobang-server

# 调试模式: 包含详细调试信息
RUST_LOG=debug ./target/release/gobang-server

# 警告模式: 仅记录警告和错误
RUST_LOG=warn ./target/release/gobang-server

# 模块级别: 仅开启特定模块的调试日志
RUST_LOG=gobang_server=debug ./target/release/gobang-server
```

### 日志输出示例

请求 ID 关联日志（用于追踪单个请求的完整生命周期）:

```
2024-01-15T10:30:00Z INFO [request_id=550e8400-e29b-41d4-a716-446655440000] method=POST path=/api/register request started
2024-01-15T10:30:00Z INFO [request_id=550e8400-e29b-41d4-a716-446655440000] method=POST path=/api/register status=201 duration_ms=45 request completed
2024-01-15T10:30:00Z INFO user_id=user-abc username=alice user registered successfully
2024-01-15T10:30:01Z INFO [connection_id=550e8400-e29b-41d4-a716-446655440001] room_id=room-123 token_hash=abcd1234 websocket connection attempt
2024-01-15T10:30:01Z INFO [connection_id=550e8400-e29b-41d4-a716-446655440001] user_id=user-abc username=alice websocket token verification successful
2024-01-15T10:30:01Z INFO [connection_id=550e8400-e29b-41d4-a716-446655440001] room_id=room-123 user_id=user-abc websocket room validation successful
2024-01-15T10:30:02Z INFO room_id=room-123 room_name=TestRoom host_id=user-abc room created successfully
2024-01-15T10:30:03Z INFO room_id=room-123 user_id=user-xyz player joined room successfully
```

### 根据 Request ID 过滤日志

使用 `grep` 根据请求 ID 过滤日志:

```bash
# 过滤特定请求的所有日志
./target/release/gobang-server 2>&1 | grep "550e8400-e29b-41d4-a716-446655440000"

# 过滤特定房间的所有日志
./target/release/gobang-server 2>&1 | grep "room_id=room-123"

# 过滤特定用户的所有日志
./target/release/gobang-server 2>&1 | grep "user_id=user-abc"
```

### WebSocket 错误消息

WebSocket 连接失败时，服务器会向客户端发送错误消息:

```json
{"type": "invalid_token", "message": "invalid token"}
{"type": "user_not_found", "message": "user not found"}
{"type": "room_not_found", "message": "room not found"}
{"type": "not_participant", "message": "not a room participant"}
```

### 生产环境推荐配置

```bash
# 推荐的生产环境日志级别
RUST_LOG=info

# 如果需要调试特定模块
RUST_LOG=info,gobang_server::ws=debug
```

## 性能指标

- **并发连接**: SQLite WAL 模式可支持约 1000 并发连接
- **消息延迟**: 客户端 → 服务器 → 客户端 < 200ms
- **内存占用**: 100 个活跃房间约 2MB

## 开发

```bash
# 运行测试
cargo test

# 检查代码
cargo check --clippy

# 格式化
cargo fmt

# 文档
cargo doc --open
```

## 安全注意事项

1. **Token 传输**: 当前使用 query string 传输 token（方便测试），生产环境建议迁移到 `Authorization` header
2. **CORS**: 当前使用 `permissive()` 模式（允许所有来源），生产环境应限制来源
3. **密码强度**: bcrypt cost factor = 12，可在 `src/auth.rs` 调整
4. **速率限制**: 当前未实现，建议生产环境添加速率限制（如 `tower-governor`）

## 许可证

GPL-3.0
