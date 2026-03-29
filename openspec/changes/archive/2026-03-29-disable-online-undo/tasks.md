## 1. Frontend Changes

- [x] 1.1 Remove `undoRequested` state from `App.tsx`
- [x] 1.2 Remove undo-related event listeners from `setupNetworkListeners` (undo_request, undo_accept, undo_reject)
- [x] 1.3 Remove `handleUndoRequest`, `handleAcceptUndo`, `handleRejectUndo` functions from `App.tsx`
- [x] 1.4 Remove undo-related props from GameInfo component calls (onUndoRequest, undoRequested, onAcceptUndo, onRejectUndo)
- [x] 1.5 Remove undo-related props from MenuDrawer component calls
- [x] 1.6 Remove "悔棋请求" button and dialog from GameInfo.tsx (lines 91-103, 125-129)
- [x] 1.7 Remove undo-related props from GameInfo.tsx interface and component
- [x] 1.8 Remove "悔棋请求" button and dialog from MenuDrawer.tsx (lines 121-129, 142-154)
- [x] 1.9 Remove undo-related props from MenuDrawer.tsx interface and component
- [x] 1.10 Ensure AI mode Undo button still works (`mode === "ai"` condition)

## 2. Backend Changes

- [x] 2.1 Remove `NetworkMessage::UndoRequest` from `network.rs:26`
- [x] 2.2 Remove `NetworkMessage::UndoAccept` from `network.rs:27`
- [x] 2.3 Remove `NetworkMessage::UndoReject` from `network.rs:28`
- [x] 2.4 Remove undo event mappings from `network.rs` handle_connection (lines 165-167)
- [x] 2.5 Remove `network_send_undo_request` command from `lib.rs`
- [x] 2.6 Remove `network_send_undo_accept` command from `lib.rs`
- [x] 2.7 Remove `network_send_undo_reject` command from `lib.rs`
- [x] 2.8 Remove undo commands from `generate_handler!` macro in `lib.rs`

## 3. Verification

- [x] 3.1 Run `cargo check` in `src-tauri/` to verify compilation
- [x] 3.2 Run `cargo test` in `src-tauri/` to verify tests pass
- [x] 3.3 Run `pnpm build` to verify frontend builds
- [ ] 3.4 Test AI mode: Start AI game, make moves, click Undo button, verify it works
- [ ] 3.5 Test online mode: Host game, verify no "悔棋请求" button is visible
- [ ] 3.6 Test online mode: Join game, verify no "悔棋请求" button is visible
- [ ] 3.7 Test online mode: Complete moves and restart, verify no undo-related errors

## 4. Documentation

- [ ] 4.1 Update CHANGELOG if project maintains one
- [ ] 4.2 Commit changes with message describing the removal and rationale
