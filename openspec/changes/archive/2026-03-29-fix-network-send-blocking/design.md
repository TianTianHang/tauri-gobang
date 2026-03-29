## Context

**当前问题：**

```rust
// network.rs:196-212 (当前有问题的代码)
pub fn send_message(state: &Arc<Mutex<NetworkState>>, msg: &NetworkMessage) -> Result<(), String> {
    let ns = state.lock().map_err(|e| e.to_string())?;

    let stream = if ns.is_host {
        let listener = ns.listener.as_ref().ok_or("Not hosting")?;
        listener
            .incoming()              // ← ⚠️ 阻塞式迭代器！
            .next()                  // ← 如果没有连接会永久阻塞！
            .ok_or("No active connection")?
            .map_err(|e| e.to_string())?
    } else {
        ns.stream
            .as_ref()
            .ok_or("Not connected")?
            .try_clone()
            .map_err(|e| e.to_string())?
    };
    // ...
}
```

**问题场景：**
1. Host 创建房间，Client 连接
2. `host_game` 后台线程 accept 连接，调用 `handle_connection(stream, ...)`
3. **但 NetworkState.stream 保持为 None**（Host 不保存 stream）
4. Client 断开连接
5. Host 尝试发送消息（如 Move）
6. `send_message` 调用 `listener.incoming().next()`
7. **永久阻塞**（没有新连接会一直等待）

**技术背景：**
- `TcpListener::incoming()` 返回阻塞迭代器
- 已接受的连接不会在迭代器中再次出现
- Host 的后台线程已经 accept 了连接，但未保存到 NetworkState
- Client 模式正确保存了 stream

## Goals / Non-Goals

**Goals:**
- 修复 Host 模式的 send_message 阻塞问题
- 统一 Host 和 Client 的发送路径
- 改进连接断开时的错误处理
- 提供清晰的错误信息

**Non-Goals:**
- 不修改接收逻辑（handle_connection）
- 不改变网络协议
- 不添加连接重试机制

## Decisions

**决策 1: Host 也保存 stream 到 NetworkState**
- **选择**: 在 `host_game` 的 accept 回调中保存 stream
- **当前问题**: accept 后只传递给 `handle_connection`，不保存
- **实现**:
  ```rust
  // host_game 中
  match listener.accept() {
      Ok((stream, addr)) => {
          // 保存 stream 到 NetworkState
          {
              let mut ns = state_clone.lock().unwrap();
              ns.stream = Some(stream.try_clone().unwrap());
              ns.connected.store(true, Ordering::SeqCst);
          }
          handle_connection(stream, app_clone, conn);
      }
  }
  ```

**决策 2: 统一发送路径**
- **选择**: Host 和 Client 都使用 `state.stream`
- **当前问题**: Host 用 `listener.incoming()`，Client 用 `stream`
- **实现**:
  ```rust
  pub fn send_message(state: &Arc<Mutex<NetworkState>>, msg: &NetworkMessage) -> Result<(), String> {
      let ns = state.lock().map_err(|e| e.to_string())?;
      
      let stream = ns.stream
          .as_ref()
          .ok_or("No active connection")?
          .try_clone()
          .map_err(|e| e.to_string())?;
      
      // 发送逻辑...
  }
  ```
- **理由**: 代码更简洁，避免阻塞风险

**决策 3: 添加连接状态检查**
- **选择**: 发送前检查 `connected` 标志
- **实现**:
  ```rust
  if !ns.connected.load(Ordering::SeqCst) {
      return Err("Not connected".to_string());
  }
  ```
- **理由**: 快速失败，避免不必要的操作

**决策 4: 改进错误消息**
- **选择**: 提供更具体的错误信息
- **当前问题**: "No active connection" 不够明确
- **改进**:
  - Host: "No opponent connected"
  - Client: "Disconnected from host"

## Risks / Trade-offs

**风险 1**: 修改 host_game 的 accept 逻辑可能引入新 bug
- **缓解**:
  - 小步修改，先测试基本功能
  - 保留原 handle_connection 逻辑不变
  - 只添加 stream 保存逻辑

**风险 2**: Stream 的所有权问题
- **缓解**:
  - 使用 `try_clone()` 创建副本
  - handle_connection 仍获得原始 stream 的所有权
  - NetworkState 保存克隆的版本用于发送

**风险 3**: 并发访问 stream
- **缓解**:
  - Mutex 已保护 NetworkState
  - 每次发送都 try_clone 创建独立副本
  - handle_connection 在独立线程，不影响发送

**Trade-off**: 代码复杂度 vs 正确性
- 接受略微增加的连接管理代码以修复严重 bug

## Migration Plan

**部署步骤：**

1. **修改 host_game**（network.rs:90-113）
   ```rust
   match listener.accept() {
       Ok((stream, addr)) => {
           let stream_clone = stream.try_clone()?;
           {
               let mut ns = state_clone.lock().unwrap();
               ns.stream = Some(stream_clone);
               ns.connected.store(true, Ordering::SeqCst);
           }
           // 原有 handle_connection 逻辑
       }
   }
   ```

2. **简化 send_message**（network.rs:196-220）
   ```rust
   pub fn send_message(...) -> Result<(), String> {
       let ns = state.lock()?;
       
       if !ns.connected.load(Ordering::SeqCst) {
           return Err("Not connected".to_string());
       }
       
       let stream = ns.stream
           .as_ref()
           .ok_or("No active connection")?
           .try_clone()?;
       
       // 发送逻辑...
   }
   ```

3. **修改 disconnect**（network.rs:222-234）
   - 确保 stream 被正确清理

4. **测试验证**
   - 编译测试：`cargo check`
   - Host 发送测试：连接断开后发送消息
   - 完整对局测试

**回滚策略：**
- Git revert
- 无状态迁移

**测试计划：**
- **基础连接**: Host 创建房间，Client 连接
- **发送测试**: 双方下棋，验证消息发送成功
- **断线测试**: Client 断开，Host 尝试下棋，验证立即返回错误
- **错误信息**: 验证显示"对手已断开"而非 UI 冻结

## Open Questions

无（技术方案明确）
