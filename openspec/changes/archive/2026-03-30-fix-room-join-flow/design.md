## Context

Online multiplayer in the Gobang app uses a REST + WebSocket architecture:

1. **HTTP join** (`POST /api/rooms/:id/join`) — validates auth, updates DB, adds player to in-memory room
2. **WebSocket connect** (`WS /game/:id?token=X`) — establishes bidirectional message channel between players
3. **game_start broadcast** — server notifies both players that the game begins

Two bugs make the entire flow broken:

**Bug 1 (server)**: `room.add_player()` (called by HTTP join) sets `room.status = Playing` immediately. Later, the WS handler checks `if room_status == Waiting && players >= 2` to broadcast `game_start`. Since status is already `Playing`, this condition is never true — the `game_start` broadcast is dead code.

**Bug 2 (frontend)**: `handleJoinRoom` connects WebSocket but never calls `setMode()`. The client stays on the lobby screen until `game_start` arrives (which never does due to Bug 1).

**Additional issue**: HTTP `join_room` sends `OpponentJoined` to the host via a dummy mpsc channel whose receiver is already dropped. The message is silently lost.

## Goals / Non-Goals

**Goals:**
- Fix the room join → game start flow end-to-end
- Support any connection order (host WS first, client HTTP first, etc.)
- Leverage existing infrastructure (`GameStateSync` from reconnect flow)
- Keep changes minimal and focused

**Non-Goals:**
- Redesigning the overall multiplayer architecture
- Adding new game modes or features
- Fixing unrelated issues (CSP, dead NetworkSetup component)

## Decisions

### Decision 1: Use `is_closed()` to detect real vs dummy connections

**Problem**: HTTP handlers create dummy mpsc channels (`let (tx, _rx) = ...;` where `_rx` is dropped). WS handlers create real channels where the receiver is held by a forwarding task. We need to distinguish them.

**Solution**: `mpsc::Sender::is_closed()` returns `true` when the receiver is dropped. For dummy channels, `_rx` is dropped immediately → `is_closed() == true`. For real channels, the receiver lives in the WS task → `is_closed() == false`.

**Alternatives considered:**
- Add `has_ws: bool` field to `PlayerConnection` — More explicit but adds state management complexity
- Use a separate `ws_connected: HashSet<UserId>` on Room — Extra bookkeeping
- Remove dummy channels entirely — Would require restructuring HTTP handlers

`is_closed()` is the cleanest: zero new state, leverages tokio's existing API.

### Decision 2: Keep `add_player()` setting status to Playing

**Why**: HTTP join logically creates the game session. The room IS playing — it just needs physical WS connections. Separating logical state from connection state makes the code clearer.

**Alternatives considered:**
- Don't set Playing in `add_player()`, set it in WS handler instead — This is Option A. It creates a coupling between WS connection timing and game state, making reconnection logic harder.

### Decision 3: Trigger `game_start` from WS handler, not HTTP handler

**Why**: HTTP handler runs before WS connections exist (the HTTP response hasn't even returned to the client yet). Broadcasting `game_start` from HTTP would always hit dummy channels. The WS handler is the right place — each time a player's WS connects, check if both are now connected.

**Mechanism**: Second player's WS connect detects `room_status == Playing`, checks `all is_closed() == false`, broadcasts `game_start` to both.

### Decision 4: Frontend guard against duplicate initialization

**Why**: `game_start` and `game_state_sync` could arrive in different orders depending on timing. The `game_start` handler must check `gameStateRef.current` before calling `new_game` to avoid overwriting a synced state.

## Risks / Trade-offs

- **`is_closed()` timing**: Must check BEFORE `drop(rooms)` and BEFORE spawning the forwarding task, while the receiver is still in scope. If moved after those points, `is_closed()` would return `true` for real connections too. → Mitigation: Check happens at line ~240 in ws.rs, well before `drop(rooms)` at line ~269.
- **`game_start` sent twice**: If host reconnects after game already started, the WS handler could send `game_start` again. → Mitigation: Frontend guard (`if (!gameStateRef.current)`) prevents double initialization.
- **Empty `game_state_sync`**: On fresh game start, `get_current_game_state()` returns empty moves. → Mitigation: `if !moves.is_empty()` guard skips sending.
