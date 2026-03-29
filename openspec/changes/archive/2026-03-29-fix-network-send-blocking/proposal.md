## Why

当前 `network::send_message` 函数在 Host 模式下使用 `listener.incoming().next()` 获取连接，这会永久阻塞如果没有活跃连接。当对手断开连接后，任何发送消息的操作（如下棋、Undo、Restart）都会导致线程永久挂起，UI 冻结，无法显示"对手已断开"提示。这是一个严重的可用性问题。

## What Changes

- **重构 NetworkState 结构**: Host 模式也在 accept 后保存 TcpStream 到 `stream` 字段
- **统一发送路径**: Host 和 Client 都使用 `state.stream` 发送消息，移除 `listener.incoming()` 逻辑
- **改进连接管理**: 在 `host_game` accept 连接后将 stream 保存到 NetworkState
- **添加连接状态检查**: 发送消息前验证连接是否活跃
- **改进错误处理**: 提供更清晰的错误信息，便于调试

**BREAKING**: 无（内部实现变更，不改变外部 API）

## Capabilities

### New Capabilities
无

### Modified Capabilities
- **network-transport**: 修复 Host 模式的消息发送阻塞问题
  - 之前：使用 `listener.incoming()` 可能永久阻塞
  - 之后：使用已保存的 `stream`，连接断开时立即返回错误
  - 影响：所有 Host 模式的网络发送操作

## Impact

**影响的代码：**
- `src-tauri/src/network.rs` - 核心重构（约 50 行修改）
  - 修改 `send_message` 函数
  - 修改 `host_game` 函数
  - 修改 `NetworkState` 使用方式

**不影响：**
- 前端代码（API 不变）
- Client 模式的网络发送
- 消息接收逻辑

**修复的 Bug：**
- Host 发送消息永久阻塞
- 对手断开后 UI 冻结
- 无法显示断线提示

**性能改进：**
- 避免不必要的 `incoming()` 迭代器创建
- 减少重复的 `try_clone()` 调用

**测试：**
- Host 发送消息到已断开的对手，验证立即返回错误
- 对手断开后尝试下棋，验证显示错误提示
- 完整的对局流程测试
