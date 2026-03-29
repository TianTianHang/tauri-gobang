## Why

联机模式下的悔棋功能存在严重的状态一致性问题（React 闭包陷阱），可能导致双方游戏状态永久不同步。同时，联机对弈本质上是竞技性的，悔棋与"落子无悔"的原则相冲突。禁用该功能可以：

- 修复已知的状态同步 bug（闭包陷阱导致 gameState 使用旧值）
- 简化联机模式的复杂度（删除约 50 行前端代码 + 3 种网络消息）
- 提升竞技公平性（与国际象棋、围棋平台一致）
- 消除维护负担（无需维护复杂的协商逻辑）

AI 模式的悔棋功能将保留，因为单机模式下不存在同步问题。

## What Changes

- **删除联机模式的 Undo 相关网络消息**
  - 移除 `NetworkMessage::UndoRequest`
  - 移除 `NetworkMessage::UndoAccept`
  - 移除 `NetworkMessage::UndoReject`
  - 移除对应的 Tauri 命令（network_send_undo_request/accept/reject）

- **删除前端的联机 Undo UI 和逻辑**
  - 移除 `undoRequested` 状态
  - 移除 `handleUndoRequest`, `handleAcceptUndo`, `handleRejectUndo` 函数
  - 移除 GameInfo 组件中的"悔棋请求"按钮和对话框
  - 移除 MenuDrawer 组件中的"悔棋请求"按钮和对话框
  - 移除 3 个 undo 事件监听器（network:undo_request/accept/reject）

- **保留 AI 模式的 Undo 功能**
  - `handleUndo` 函数继续支持单机模式
  - GameInfo 中的"悔棋"按钮仅在 AI 模式显示
  - 后端的 `undo_move` 和 `undo_two_moves` 命令保留

**BREAKING**: 联机模式下不再支持悔棋（但在使用过程中该功能本身有 bug，实际无法正常工作）

## Capabilities

### New Capabilities
无

### Modified Capabilities
- **online-gameplay**: 联机对弈不再支持悔棋功能
  - 之前：双方协商后可撤销 N 步
  - 之后：禁止悔棋，与竞技平台惯例一致
  - 影响：仅联机模式，AI 模式不受影响

## Impact

**影响的代码：**
- `src-tauri/src/network.rs` - 删除 3 个 NetworkMessage 变体和事件映射
- `src-tauri/src/lib.rs` - 删除 3 个 Tauri 命令
- `src/App.tsx` - 删除约 50 行代码（状态、事件监听、处理函数）
- `src/components/GameInfo.tsx` - 删除约 30 行代码（UI 和对话框）
- `src/components/MenuDrawer.tsx` - 删除约 30 行代码（UI 和对话框）

**不影响：**
- AI 模式的悔棋功能
- 其他联机功能（Move, Restart）
- 游戏逻辑（undo_move 等后端命令保留）

**用户体验：**
- 联机模式下不再显示"悔棋请求"按钮
- 如果下错棋，只能使用"重新开始"功能
- AI 模式继续支持悔棋

**测试：**
- 手动测试 AI 模式悔棋功能正常
- 手动测试联机模式无悔棋选项
- 手动测试联机模式 Restart 功能正常
