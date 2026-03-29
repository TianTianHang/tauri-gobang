## Context

**当前问题（闭包陷阱）：**

```typescript
// App.tsx:100-114 (当前有问题的代码)
const setupNetworkListeners = useCallback(
  (newMode: GameMode) => {
    const unlistenPromises = [
      listen<string>("network:opponent_moved", (event) => {
        const { row, col } = JSON.parse(event.payload);
        invoke<MoveResult>("make_move", {
          state: gameState,  // ⚠️ 闭包陷阱：这是创建时的旧值！
          row, col
        })
      })
    ];
  },
  [gameState, cleanupListeners, startNewGame]  // gameState 变化会重新创建
);
```

**问题场景：**
1. `handleHostGame` 调用 `setupNetworkListeners("online_host")`
2. 此时 `gameState = null`
3. 事件监听器创建，闭包捕获 `gameState = null`
4. `startNewGame()` 更新 `gameState` 为新值
5. 收到 `network:opponent_moved` 事件
6. 调用 `make_move(null, row, col)` - **错误！**

**相关 Bug：**
- `handleRejectRestart` 调用 `network_send_restart_request` 而非 `restart_reject`
- `handleNewGame` 不等待对方 Accept/Reject

**技术背景：**
- React Hooks 依赖闭包捕获值
- `useCallback` 的依赖数组变化时会重新创建函数，但已注册的事件监听器不会自动更新
- `useRef` 提供可变的 ref 对象，其 `.current` 属性始终指向最新值

## Goals / Non-Goals

**Goals:**
- 修复所有事件监听器的闭包陷阱
- 修复 Restart 流程的 bug
- 添加状态验证以防止 null 状态调用
- 确保 `gameState` 在所有异步操作中始终是最新的

**Non-Goals:**
- 不修改网络协议
- 不改变事件名称或格式
- 不引入新功能（如状态快照同步）

## Decisions

**决策 1: 使用 useRef 存储 gameState**
- **选择**: 添加 `const gameStateRef = useRef(gameState)` 和 useEffect 同步更新
- **替代方案**:
  1. 每次 gameState 变化时重新注册监听器
  2. 将事件监听器放在组件内直接使用 gameState
- **理由**:
  - useRef 是 React 推荐的解决方案（官方文档明确说明）
  - 不需要重新注册监听器（性能更好）
  - 最小化代码变更

**实现方式：**
```typescript
const gameStateRef = useRef(gameState);

useEffect(() => {
  gameStateRef.current = gameState;
}, [gameState]);

// 事件监听器中使用
listen<string>("network:opponent_moved", (event) => {
  invoke<MoveResult>("make_move", {
    state: gameStateRef.current,  // ✓ 总是最新值
    row, col
  })
})
```

**决策 2: 添加状态验证**
- **选择**: 在调用 make_move 前验证 gameState 不为 null
- **理由**:
  - 防御性编程
  - 提供清晰的错误信息
  - 避免后端接收到无效数据

**决策 3: 修复 Restart 流程**
- **选择**: 移除 `handleNewGame` 中的自动 RestartRequest，改为手动触发
- **当前问题**: `handleNewGame` 发送请求后立即返回，不等待响应
- **修复方案**:
  - 移除自动发送 RestartRequest 的逻辑
  - 用户点击"重新开始"按钮时发送 `network_send_restart_request`
  - 等待对方的 `restart_accept` 事件后再调用 `startNewGame()`

**决策 4: 修复 RestartReject 命令**
- **选择**: 添加 `network_send_restart_reject` 命令到后端
- **当前问题**: 前端调用不存在的命令
- **实现**: 在 `lib.rs` 中添加新命令

## Risks / Trade-offs

**风险 1**: useRef 引入新的复杂度
- **缓解**:
  - 添加清晰的注释说明用途
  - 确保 useEffect 正确同步
  - 测试验证 ref 始终是最新的

**风险 2**: 修改 Restart 流程可能影响用户体验
- **缓解**:
  - 保留"新游戏"按钮（本地重置）
  - "重新开始"按钮改为协商式（需要对方同意）
  - UI 保持清晰的等待提示

**风险 3**: 添加后端命令可能引入新 bug
- **缓解**:
  - 简单的命令转发（无复杂逻辑）
  - 充分测试

**Trade-off**: 代码复杂度 vs 正确性
- 接受略微增加的代码复杂度（ref）以换取正确性

## Migration Plan

**部署步骤：**

1. **添加 gameStateRef**（App.tsx）
   ```typescript
   const gameStateRef = useRef(gameState);
   useEffect(() => {
     gameStateRef.current = gameState;
   }, [gameState]);
   ```

2. **修改事件监听器**（App.tsx）
   - 所有 `gameState` 改为 `gameStateRef.current`
   - 添加 null 检查

3. **修复 Restart 流程**（App.tsx）
   - 移除 `handleNewGame` 中的网络逻辑
   - 确保只通过事件监听器触发 restart

4. **添加后端命令**（src-tauri/src/lib.rs）
   ```rust
   #[tauri::command]
   fn network_send_restart_reject(
     state: tauri::State<'_, Arc<Mutex<NetworkState>>>,
   ) -> Result<(), String> {
     network::send_message(&state.inner(),
       &network::NetworkMessage::RestartReject)
   }
   ```

5. **修复 handleRejectRestart**（App.tsx）
   ```typescript
   await invoke<void>("network_send_restart_reject");
   ```

6. **测试验证**
   - 编译测试
   - 完整对局测试
   - Restart 测试
   - 快速落子测试

**回滚策略：**
- Git revert
- 无状态迁移

**测试计划：**
- **基础对局**: 双方各下 10 步，验证 board 同步
- **快速落子**: 连续快速落子，验证无竞态条件
- **Restart 测试**: 请求、同意、拒绝三个流程
- **边缘情况**: 断线重连后的状态同步

## Open Questions

无（技术方案明确，React useRef 是标准解决方案）
