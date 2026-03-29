## 1. Add gameStateRef and Sync Logic

- [x] 1.1 Add `useRef` import to App.tsx if not present
- [x] 1.2 Add `const gameStateRef = useRef(gameState)` after gameState state declaration
- [x] 1.3 Add `useEffect` to sync gameStateRef.current with gameState
- [x] 1.4 Verify TypeScript compiles with new ref

## 2. Fix Event Listeners Closure Trap

- [x] 2.1 Replace `gameState` with `gameStateRef.current` in `network:opponent_moved` listener
- [x] 2.2 Add null check for gameStateRef.current in `network:opponent_moved` listener
- [x] 2.3 Replace `gameState` with `gameStateRef.current` in `network:undo_accept` listener (if still exists)
- [x] 2.4 Add null check in `network:undo_accept` listener
- [x] 2.5 Verify all event listeners use gameStateRef.current

## 3. Fix Restart Flow

- [x] 3.1 Remove automatic `network_send_restart_request` from `handleNewGame`
- [x] 3.2 Ensure `handleNewGame` only calls `startNewGame()` locally
- [x] 3.3 Verify `network:restart_accept` event listener calls `startNewGame()`
- [x] 3.4 Ensure `restartRequested` state is cleared on accept

## 4. Fix RestartReject Command

- [x] 4.1 Add `network_send_restart_reject` command to `src-tauri/src/lib.rs`
- [x] 4.2 Register the new command in `generate_handler!` macro
- [x] 4.3 Update `handleRejectRestart` to call `network_send_restart_reject`
- [x] 4.4 Verify Rust backend compiles (`cargo check`)

## 5. Add State Validation

- [x] 5.1 Add null check before `invoke<MoveResult>("make_move", ...)` in opponent_moved listener
- [x] 5.2 Add status check (`status === GameStatus.Playing`) before making moves
- [x] 5.3 Add error handling for invalid state cases
- [ ] 5.4 Test with null gameState scenario

## 6. Verification

- [x] 6.1 Run `pnpm build` to verify frontend compiles
- [x] 6.2 Run `cargo check` in `src-tauri/` to verify backend compiles
- [ ] 6.3 Test: Host game, join with client, make 10 moves each, verify boards are identical
- [ ] 6.4 Test: Rapid-fire moves (both players click quickly), verify no state inconsistency
- [ ] 6.5 Test: Request restart, accept, verify both sides start new game
- [ ] 6.6 Test: Request restart, reject, verify game continues with old state
- [ ] 6.7 Test: Disconnect and reconnect (if applicable), verify state sync

## 7. Documentation

- [x] 7.1 Add comments explaining gameStateRef purpose
- [ ] 7.2 Update CHANGELOG with bug fix details
- [ ] 7.3 Commit with message describing the closure trap fix
