## Why

Online multiplayer is completely broken — the game never starts when a client joins a room. Two bugs stack: the server's `game_start` message is dead code (never sent), and the client frontend never transitions from lobby to game view. Neither player can play.

## What Changes

- Fix server WS handler to detect real WebSocket connections and broadcast `game_start` when both players are connected
- Remove dead code in HTTP `join_room` handler that sends messages to dropped channels
- Fix client `handleJoinRoom` to transition to waiting mode after successful join
- Add guard in frontend `game_start` handler to prevent duplicate game initialization

## Capabilities

### New Capabilities

- `room-join-flow`: End-to-end room joining flow — HTTP join creates logical room state, WebSocket connect establishes real channels, game_start broadcast triggers frontend transition to game view

### Modified Capabilities

(none — this is a bug fix, no spec-level behavior changes)

## Impact

- `server/src/ws.rs` — Core logic change in non-reconnect WS handler branch
- `server/src/main.rs` — Remove broken `OpponentJoined` send from `join_room` HTTP handler
- `src/App.tsx` — Add `setMode("waiting")` in `handleJoinRoom`, add guard in `game_start` handler
