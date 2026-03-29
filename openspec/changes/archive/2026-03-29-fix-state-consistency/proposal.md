## Why

当前联机模式存在严重的 React 闭包陷阱 bug，导致 `setupNetworkListeners` 中创建的事件监听器使用的是创建时的旧 `gameState`，而非最新值。这会导致：

- **对手落子时使用旧状态**: `network:opponent_moved` 事件处理函数调用 `make_move(gameState, row, col)` 时，`gameState` 可能是 null 或几个回合前的旧值
- **悔棋时状态不一致**: `network:undo_accept` 使用旧状态执行 `undo_two_moves`，导致双方 history 长度差 1 步
- **游戏状态永久错乱**: 一旦出现不一致，后续所有操作都会基于错误的状态，游戏无法继续进行

这是一个 P0 级别的严重 bug，会导致联机对弈完全无法正常进行。

## What Changes

- **修复闭包陷阱**: 使用 `useRef` 存储 `gameState` 的最新值，事件监听器通过 ref 访问
- **重构事件监听器注册**: 确保 `setupNetworkListeners` 在正确的时机调用，并正确处理依赖
- **添加状态验证**: 在关键操作前验证 `gameState` 不为 null 且处于 Playing 状态
- **改进 Restart 流程**: 修复 `handleNewGame` 不等待对方响应的问题
- **修复 RestartReject bug**: 修正调用错误命令的问题（应该是 `restart_reject` 而非 `restart_request`）

**BREAKING**: 无（这是 bug 修复，不改变预期行为）

## Capabilities

### New Capabilities
无

### Modified Capabilities
- **online-gameplay**: 修复联机模式的状态同步问题
  - 之前：事件监听器使用旧的 gameState，导致状态不一致
  - 之后：使用 ref 获取最新 gameState，确保状态同步
  - 影响：所有联机对弈场景

## Impact

**影响的代码：**
- `src/App.tsx` - 核心修复（约 20-30 行修改）
  - 添加 `gameStateRef`
  - 修改事件监听器以使用 ref
  - 修复 Restart 流程

**不影响：**
- AI 模式（没有这些事件监听器）
- 后端代码
- UI 组件

**修复的 Bug：**
- `network:opponent_moved` 使用旧 gameState
- `network:undo_accept` 使用旧 gameState
- `handleNewGame` 不等待响应
- `handleRejectRestart` 调用错误的命令

**测试：**
- 手动测试完整对局（双方交替落子）
- 手动测试悔棋流程（如未禁用）
- 手动测试重启流程
- 压力测试：快速连续落子验证状态同步
