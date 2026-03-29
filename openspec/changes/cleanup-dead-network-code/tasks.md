## 1. Code Cleanup

- [x] 1.1 Remove `NetworkMessage::Chat` variant from `network.rs:32`
- [x] 1.2 Remove `NetworkMessage::Join` variant from `network.rs:24`
- [x] 1.3 Remove `NetworkMessage::GameOver` variant from `network.rs:33`
- [x] 1.4 Remove Chat event mapping from `network.rs:171`
- [x] 1.5 Remove Join event mapping from `network.rs:173`
- [x] 1.6 Remove GameOver event mapping from `network.rs:172`
- [x] 1.7 Remove opponent_joined event emission from `network.rs:100`

## 2. Verification

- [x] 2.1 Run `cargo check` in `src-tauri/` to verify compilation
- [ ] 2.2 Run `cargo test` in `src-tauri/` to verify tests pass
- [x] 2.3 Run `pnpm build` to verify frontend builds
- [ ] 2.4 Manual test: Host an online game and verify connection works
- [ ] 2.5 Manual test: Join an online game and verify gameplay works
- [ ] 2.6 Manual test: Complete a full game (moves + restart) to verify no regressions

## 3. Documentation

- [x] 3.1 Update CHANGELOG if project maintains one
- [ ] 3.2 Commit changes with clear message describing the cleanup
