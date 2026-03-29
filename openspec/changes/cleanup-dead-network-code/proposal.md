## Why

当前网络协议定义了 3 种从未使用的消息类型（`Chat`, `Join`, `GameOver`），这些死代码增加了协议复杂度但没有提供任何功能价值。清理这些代码可以：

- 提高代码可维护性（减少 30% 的死代码）
- 使协议定义与实际使用一致
- 避免未来维护时的混淆
- 为可能的未来扩展留出干净的基础

## What Changes

- **删除 `NetworkMessage::Chat` 变体**
  - 移除消息类型定义（network.rs:32）
  - 移除事件映射（network.rs:171）
  - 无发送/接收代码，无需其他修改

- **删除 `NetworkMessage::Join` 变体**
  - 移除消息类型定义（network.rs:24）
  - 移除事件发送（network.rs:100，前端从未监听）
  - 移除事件映射（network.rs:173，从未触发）
  - 客户端从未发送 Join 消息

- **删除 `NetworkMessage::GameOver` 变体**
  - 移除消息类型定义（network.rs:33）
  - 移除事件映射（network.rs:172）
  - 从未发送，游戏结束通过状态隐式处理

**BREAKING**: 无（这些消息从未被使用）

## Capabilities

### New Capabilities
无（这是代码清理，不引入新功能）

### Modified Capabilities
无（不改变任何现有功能的行为）

## Impact

**影响的代码：**
- `src-tauri/src/network.rs` - 删除约 6 行代码
  - NetworkMessage enum: 3 个变体
  - handle_connection: 4 行事件映射

**不影响：**
- 前端代码（这些消息从未被监听）
- 网络协议的其他部分
- 游戏逻辑
- 用户可见的功能

**测试：**
- 无需新增测试（这些是未使用的代码）
- 现有测试应该继续通过
- 手动验证联机对弈功能正常工作

**风险：** 极低
- 仅删除死代码，不修改活跃代码路径
- 如果未来需要这些功能，可以重新设计并实现
