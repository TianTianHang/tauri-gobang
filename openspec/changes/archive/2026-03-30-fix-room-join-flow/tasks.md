## 1. Server Fix — WS handler game_start broadcast

- [x] 1.1 Replace dead `if room_status == Waiting` block in `server/src/ws.rs` with `if room_status == Playing` branch that uses `is_closed()` to detect real connections
- [x] 1.2 When both players have real WS connections (`all is_closed() == false`), broadcast `game_start` and `opponent_joined`
- [x] 1.3 Remove broken `OpponentJoined` send from `server/src/main.rs` `join_room` handler (sends to dropped channel)

## 2. Frontend Fix — Client mode transition

- [x] 2.1 Add `setMode("waiting")` to `handleJoinRoom` in `src/App.tsx` after successful HTTP join + WS connect
- [x] 2.2 Add guard in `game_start` handler: only call `new_game` if `gameStateRef.current` is null

## 3. Verification

- [x] 3.1 Build server: `cd server && cargo build`
- [x] 3.2 Run server tests: `cd server && cargo test`
- [x] 3.3 Type-check frontend: `pnpm build`
