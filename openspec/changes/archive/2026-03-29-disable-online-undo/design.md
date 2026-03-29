## Context

**当前问题：**
1. **严重的状态同步 bug**: 联机模式 Undo 功能使用闭包捕获的 `gameState`，导致基于旧状态执行操作，双方历史记录可能永久不一致（差 1 步）
2. **复杂的协商机制**: Undo 需要双方同意（Request/Accept/Reject），增加了 UI 复杂度和代码维护负担
3. **竞技性冲突**: 联机对弈本质上是竞技性的，悔棋与"落子无悔"原则冲突

**技术背景：**
- Undo 功能依赖 3 种网络消息：UndoRequest, UndoAccept, UndoReject
- 前端维护 `undoRequested` 状态和对应的对话框 UI
- 事件监听器在 `setupNetworkListeners` 中创建，存在闭包陷阱
- AI 模式的 Undo 是本地直接执行，没有这些复杂问题

**约束：**
- 保留 AI 模式的 Undo 功能（单机，无同步问题）
- 不影响其他联机功能（Move, Restart）
- 最小化代码变更范围

## Goals / Non-Goals

**Goals:**
- 完全移除联机模式的 Undo 协商机制
- 修复状态一致性问题（通过删除有问题的代码）
- 简化 UI（删除悔棋相关的按钮和对话框）
- 减少网络协议复杂度（-3 种消息类型）

**Non-Goals:**
- 不修改 AI 模式的 Undo 功能
- 不改变游戏逻辑（undo_move 命令保留）
- 不添加替代功能（如"重新开始"已有）

## Decisions

**决策 1: 完全删除而非条件禁用**
- **选择**: 删除所有联机 Undo 代码，而不是通过配置开关禁用
- **替代方案**: 添加 `ONLINE_UNDO_ENABLED` feature flag
- **理由**:
  - 代码已有严重 bug，不值得保留
  - 减少维护负担（不需要维护死代码）
  - 避免未来误用（如果启用开关会重现 bug）

**决策 2: 保留后端 undo 命令**
- **选择**: `undo_move` 和 `undo_two_moves` 命令保留
- **理由**:
  - AI 模式需要这些命令
  - 前端直接调用，不通过网络
  - 保留对单机模式的支持

**决策 3: UI 层的条件渲染**
- **选择**: GameInfo 和 MenuDrawer 组件通过 `mode === "ai"` 条件渲染悔棋按钮
- **理由**:
  - 最小化组件修改
  - 保持 AI 模式的用户体验
  - 避免引入新的 props

**决策 4: 不添加用户提示**
- **选择**: 不在 UI 中告知"联机模式已禁用悔棋"
- **理由**:
  - 该功能本身有 bug，用户可能不知道它的存在
  - "重新开始"功能可以替代（重新开局）
  - 简化实现

## Risks / Trade-offs

**风险 1**: 用户习惯改变
- **缓解**: AI 模式保留 Undo，用户可以在单机练习时使用
- **影响**: 联机模式下需要更谨慎，符合竞技惯例

**风险 2**: 删除关键代码导致编译错误
- **缓解**:
  - 逐步删除（先删除事件监听，再删除 UI，最后删除后端）
  - 每步后运行 `cargo check` 和 `pnpm build`
  - Git 历史可回滚

**风险 3**: 意外删除 AI 模式的 Undo
- **缓解**:
  - 仔细检查 `mode === "ai"` 条件
  - 测试 AI 模式悔棋功能
  - 保留后端 undo 命令

**Trade-off**: 灵活性 vs 简洁性
- 接受失去联机 Undo 的灵活性，换取代码简洁性和正确性

## Migration Plan

**部署步骤：**
1. 修改 `src/App.tsx`：删除 undo 相关状态和事件监听
2. 修改 `src/components/GameInfo.tsx`：删除联机 Undo UI
3. 修改 `src/components/MenuDrawer.tsx`：删除联机 Undo UI
4. 修改 `src-tauri/src/network.rs`：删除 3 个 NetworkMessage 变体
5. 修改 `src-tauri/src/lib.rs`：删除 3 个 Tauri 命令
6. 运行编译测试：`cargo check` 和 `pnpm build`
7. 手动测试 AI 模式 Undo 功能
8. 手动测试联机模式无 Undo 选项

**回滚策略：**
- Git revert 即可
- 无数据迁移或状态变更

**测试计划：**
- **AI 模式测试**:
  1. 开始 AI 对局
  2. 下几步棋
  3. 点击"悔棋"按钮
  4. 验证棋局回退 2 步
- **联机模式测试**:
  1. Host 房间，Client 加入
  2. 双方下棋
  3. 验证无"悔棋请求"按钮
  4. 验证"重新开始"功能正常

## Open Questions

无（这是功能删除，技术方案明确）
