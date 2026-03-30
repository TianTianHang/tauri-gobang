# Design: Migrate to Server Architecture

## Context

### Current State
The Gobang game currently uses a peer-to-peer (P2P) TCP architecture:
- Players exchange IP addresses manually (via chat, email, etc.)
- One player acts as "host" (binds TcpListener on a port)
- Other player connects directly to host's IP:port
- Game messages (Move, RestartRequest, etc.) are sent as line-delimited JSON over TCP
- No user authentication or identity system
- No room lobby or matchmaking
- Network code in `src-tauri/src/network.rs` (~250 lines)
- Client UI in `src/components/NetworkSetup.tsx`

### Problems with Current Architecture
- **NAT Traversal**: Players behind routers/firewalls cannot be reached
- **Manual IP Exchange**: Poor user experience, requires external communication
- **No Persistence**: Cannot save game history or player statistics
- **No Social Features**: Cannot add room lobby, spectating, rankings
- **Hard to Extend**: Adding features requires changes to both clients

### Stakeholders
- **Players**: Want easy matchmaking, no network configuration
- **Developers**: Want maintainable architecture, easy to extend
- **Server Operator**: Wants simple deployment, low maintenance

### Constraints
- **Client Technology**: Must remain Tauri + React + TypeScript
- **Server Language**: Must be Rust (as per user requirement)
- **Database**: Prefer lightweight (SQLite) over heavy databases
- **Deployment**: Single binary, minimal configuration
- **Network**: Must work through NAT/firewalls
- **Complexity**: Keep server simple (message relay, no game logic validation)

## Goals / Non-Goals

**Goals:**
- ✅ Enable server-based matchmaking with room lobby
- ✅ Add user authentication (username + password)
- ✅ Implement reliable game message routing through server
- ✅ Support 30-second reconnection window for disconnections
- ✅ Package server as single binary with auto-initialization
- ✅ Minimize latency increase (keep message path efficient)
- ✅ Maintain backward compatibility for local games

**Non-Goals:**
- ❌ Game logic validation on server (trust clients)
- ❌ Spectating or observer mode
- ❌ Chat system
- ❌ Ranking/ELO system
- ❌ Replay system
- ❌ Tournament mode
- ❌ Multiple game modes (only standard 15x15 gobang)

## Decisions

### 1. Communication Protocol: WebSocket over TCP

**Decision**: Use WebSocket (RFC 6455) instead of raw TCP for client-server communication.

**Rationale**:
- **Better Browser Support**: If we ever add a web client, WebSocket works natively
- **Built-in Features**: Ping/pong for keep-alive, message framing, automatic reconnection handling
- **Firewall Friendly**: WebSocket runs over standard HTTP(S) ports (80/443), less likely to be blocked
- **Mature Ecosystem**: `tokio-tungstenite` is well-maintained and async-first

**Alternatives Considered**:
- **Raw TCP**: Lower latency, but requires implementing message framing, ping/pong, reconnection logic manually
- **HTTP Long-Polling**: Too much overhead, not suitable for real-time games
- **gRPC**: Overkill, adds complexity without clear benefits for this use case

**Trade-off**: WebSocket has slightly higher latency than raw TCP due to frame overhead, but this is acceptable for a turn-based board game (not real-time action).

---

### 2. Web Framework: Axum

**Decision**: Use Axum as the HTTP/WebSocket server framework.

**Rationale**:
- **Modern Design**: Built on Tokio, type-safe routing, extractor pattern
- **Excellent WebSocket Support**: Native `WebSocketUpgrade` extractor
- **Ecosystem Integration**: Works well with `tokio-tungstenite`, `sqlx`, `tower-http`
- **Performance**: Comparable to Actix-web, more ergonomic API
- **Learning Curve**: Aligns with modern Rust async patterns

**Alternatives Considered**:
- **Actix-web**: More mature, but heavier, more complex actor model
- **Warp**: Functional style, but harder to read/maintain for large projects
- **Rocket**: Not async-first, less suitable for high-concurrency scenarios

---

### 3. Database: SQLite with SQLx

**Decision**: Use SQLite as the database with SQLx for async database access.

**Rationale**:
- **Zero Configuration**: Single file database, no separate server process
- **Sufficient Performance**: Can handle ~1000 concurrent connections (far beyond expected load)
- **Portability**: Single file easy to backup, migrate, or debug
- **SQLx Benefits**: Compile-time checked queries, async-first, pure Rust
- **Low Resource Usage**: Minimal memory footprint, suitable for small VPS

**Alternatives Considered**:
- **PostgreSQL**: Overkill for this scale, adds operational complexity
- **Redis**: Would need both Redis and PostgreSQL (Redis for sessions, PG for persistence)
- **In-Memory Only**: Loses all data on restart, cannot save game history

**Trade-off**: SQLite has write locking (only one writer at a time), but this is acceptable because:
- Writes are infrequent (room creation, game end)
- Reads are concurrent (room list queries)
- Use `rwc` mode (read-write concurrency) or WAL mode (write-ahead logging) if needed

---

### 4. Authentication: Session Tokens (Not JWT)

**Decision**: Use simple session tokens (UUIDs) stored in server memory instead of JWT.

**Rationale**:
- **Simplicity**: No token signing/verification logic
- **Immediate Revocation**: Can delete token from memory to log out user
- **No Decoding Overhead**: Simple HashMap lookup is faster than JWT verification
- **Sufficient for Scale**: Memory usage is negligible (even for 10,000 active sessions)

**Alternatives Considered**:
- **JWT (Stateless)**: No server storage, but harder to revoke tokens, larger payload size
- **Opaque Tokens + Redis**: More scalable, but adds Redis dependency

**Trade-off**: Sessions are lost on server restart, but this is acceptable because:
- Users just log in again (minor inconvenience)
- No sensitive data in session (only user_id)
- Server restarts should be infrequent

---

### 5. Password Hashing: bcrypt

**Decision**: Use bcrypt for password hashing.

**Rationale**:
- **Proven Security**: Adaptive hashing (cost factor increases as hardware improves)
- **Built-in Salt**: No need to generate/store salt separately
- **Widely Used**: Battle-tested, well-understood
- **Rust Support**: `bcrypt` crate is pure Rust, well-maintained

**Alternatives Considered**:
- **Argon2**: Newer, stronger, but less widely adopted
- **PBKDF2**: Weaker than bcrypt, requires careful parameter selection
- **Plain Text**: Obviously unacceptable

---

### 6. Room Management: In-Memory + Database Hybrid

**Decision**: Store active rooms in memory for fast access, persist metadata to database.

**Rationale**:
- **Fast Lookups**: In-memory HashMap for O(1) room access during message routing
- **Persistence**: Database stores room history for statistics/audit
- **Best of Both Worlds**: Speed of in-memory, durability of database

**Data Split**:
- **Memory** (Room struct):
  - Active WebSocket connections (players: HashMap<UserId, Sender>)
  - Current status (waiting/playing)
  - Disconnection state (who disconnected, when)
- **Database** (rooms table):
  - Room ID, name, host_id, player2_id
  - Status, created_at, ended_at
  - Used for room list queries and history

**Alternatives Considered**:
- **Pure In-Memory**: Lose all data on restart, cannot show room history
- **Pure Database**: Too slow for message routing (query on every message)
- **Redis Hybrid**: Adds Redis dependency, in-memory is sufficient for our scale

---

### 7. Server Deployment: Single Binary with Embedded Resources

**Decision**: Package server as single executable with embedded SQL migrations.

**Rationale**:
- **Zero Deployment Friction**: User downloads one file, runs it
- **Cross-Platform**: Easy to build for Linux, macOS, Windows, ARM64
- **Embedded Resources**: Use `include_str!()` to embed SQL migrations at compile time
- **Auto-Initialization**: Detect first run, create database, run migrations

**Directory Structure**:
```
~/.gobang-server/          # Data directory (auto-created)
├── database.db            # SQLite database (auto-created)
├── config.toml            # Optional config (auto-generated with defaults)
└── server.log             # Log file (daemon mode)
```

**Alternatives Considered**:
- **Docker Image**: More complex deployment, larger download
- **System Package (deb/rpm)**: More work to build/maintain
- **Installer Wizard**: Overkill for a server binary

---

### 8. Protocol Design: Dual WebSocket Endpoints

**Decision**: Use separate WebSocket endpoints for control and game messages.

**Endpoints**:
- `WS /control`: For future admin/control features (not in MVP)
- `WS /game/{room_id}?token={session_token}`: For game messages

**Rationale**:
- **Clean Separation**: Game messages don't mix with control messages
- **Future-Proof**: Can add admin features without breaking game protocol
- **Security**: Game endpoint requires room membership, control endpoint doesn't

**Alternatives Considered**:
- **Single Endpoint**: Simpler, but harder to extend
- **HTTP + WebSocket**: Use HTTP for room management, WS for game. Rejected because WebSocket is sufficient for both, and HTTP adds complexity.

---

### 9. Reconnection: 30-Second Timeout with Cleanup

**Decision**: Allow 30 seconds for disconnected player to reconnect, then declare opponent winner.

**Rationale**:
- **Balance**: Long enough for brief network hiccups, short enough to not frustrate opponent
- **Implementation**: Track disconnection timestamp, use `tokio::time::sleep` for timeout
- **Cleanup**: After timeout, mark room as "ended", remove from memory

**Alternatives Considered**:
- **No Timeout**: Game stalls indefinitely (poor UX)
- **Immediate Loss**: Too harsh, brief disconnects cause frustration
- **Pause Indefinitely**: Blocks the room, prevents opponent from playing

**Trade-off**: 30 seconds is arbitrary, but empirically reasonable for network blips.

---

### 10. Client Polling: 5-Second Interval for Room List

**Decision**: Use client-side polling every 5 seconds to refresh room list.

**Rationale**:
- **Simplicity**: Easier than server-sent events or WebSocket broadcasts
- **Sufficient**: Real-time updates not critical for room list
- **Low Overhead**: One HTTP GET request every 5 seconds per client

**Alternatives Considered**:
- **Server-Sent Events (SSE)**: Real-time, but adds complexity
- **WebSocket Broadcasts**: Overkill for infrequent updates
- **Manual Refresh**: Poor UX, stale data

**Trade-off**: 5-second polling adds 5-second latency to room list updates, but this is acceptable because users don't need sub-second accuracy.

---

## Architecture

### System Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                         Tauri Client                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ LoginScreen │  │ RoomList    │  │ GameBoard           │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
│         │                 │                     │             │
│         ▼                 ▼                     ▼             │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              Tauri Commands (IPC)                       │ │
│  │  login(), register(), create_room(), join_room()        │ │
│  └─────────────────────────────────────────────────────────┘ │
│                              │                              │
└──────────────────────────────┼──────────────────────────────┘
                               │
                    HTTP / WebSocket
                               │
┌──────────────────────────────┼──────────────────────────────┐
│                        Game Server                          │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Axum HTTP + WebSocket Server                           │ │
│  │                                                          │ │
│  │  REST API:                                              │ │
│  │   POST /api/register                                    │ │
│  │   POST /api/login                                       │ │
│  │   GET  /api/rooms                                       │ │
│  │   POST /api/rooms                                       │ │
│  │   POST /api/rooms/:id/join                              │ │
│  │                                                          │ │
│  │  WebSocket:                                             │ │
│  │   WS /game/:room_id  (game message relay)               │ │
│  └─────────────────────────────────────────────────────────┘ │
│                              │                              │
│  ┌───────────────────────┐  │  ┌─────────────────────────┐  │
│  │  Auth Service         │  │  │  Room Manager           │  │
│  │  - bcrypt passwords   │  │  │  - Create/join rooms    │  │
│  │  - session tokens     │  │  │  - Broadcast messages   │  │
│  └───────────────────────┘  │  │  - Track connections    │  │
│                             │  └─────────────────────────┘  │
│                             │                               │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              SQLite Database                         │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐          │   │
│  │  │ users    │  │ rooms    │  │ games    │          │   │
│  │  └──────────┘  └──────────┘  └──────────┘          │   │
│  └─────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
```

### Server Module Structure

```
server/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, server setup
│   ├── auth.rs              # Password hashing, session management
│   ├── db.rs                # Database connection, queries
│   ├── room.rs              # Room creation, joining, lifecycle
│   ├── ws.rs                # WebSocket handling, message routing
│   └── types.rs             # Shared types (User, Room, etc.)
└── migrations/
    └── init.sql             # Embedded SQL for table creation
```

### Key Data Structures

```rust
// In-memory session store
type Sessions = HashMap<String, UserId>;  // token -> user_id

// In-memory room store
type Rooms = HashMap<String, Room>;

struct Room {
    id: String,
    name: String,
    status: RoomStatus,  // Waiting, Playing, Ended
    players: HashMap<UserId, SplitSink<WebSocket, Message>>,
    host_id: UserId,
    player2_id: Option<UserId>,
    disconnected: Option<(UserId, Instant)>,  // For reconnect timeout
}

// Database models
struct User {
    id: String,
    username: String,
    password_hash: String,
    created_at: i64,
}

struct RoomRecord {
    id: String,
    name: String,
    host_id: String,
    player2_id: Option<String>,
    status: String,
    created_at: i64,
    ended_at: Option<i64>,
}
```

---

## Risks / Trade-offs

### Risk 1: Server Becomes Single Point of Failure
**Impact**: If server goes down, all active games are interrupted.

**Mitigation**:
- Implement graceful shutdown (wait for connections to close)
- Client can show "server disconnected" message
- Future: Add redundant servers or load balancing (out of scope for MVP)

---

### Risk 2: Increased Latency Due to Server Relay
**Impact**: Messages travel client → server → client instead of direct.

**Mitigation**:
- Use low-latency server provider (e.g., AWS, DigitalOcean)
- Place server centrally (or use CDN if global)
- Acceptable for turn-based game (not real-time action)
- Measure: If latency < 200ms, imperceptible for board game

---

### Risk 3: Password Hashing Cost Under Load
**Impact**: bcrypt is intentionally slow (security feature), may slow login/registration under high load.

**Mitigation**:
- Use reasonable cost factor (e.g., 12, not 15)
- Async bcrypt operations (don't block executor)
- Monitor login times, adjust cost factor if needed
- Alternative: Use faster hash (Argon2) if load testing reveals issues

---

### Risk 4: SQLite Write Locking Under High Concurrency
**Impact**: SQLite allows only one writer at a time, may cause contention if many rooms are created/ended simultaneously.

**Mitigation**:
- Writes are infrequent (not every game move)
- Use WAL mode (Write-Ahead Logging) for better concurrency
- Monitor database lock contention
- Fallback: Migrate to PostgreSQL if scale demands it

---

### Risk 5: Memory Usage for In-Memory Rooms
**Impact**: Each room stores WebSocket connections and state. With many concurrent rooms, memory usage increases.

**Mitigation**:
- Estimate: 1000 rooms × 2 players × 1KB ≈ 2MB (negligible)
- Monitor memory usage in production
- Implement room timeout (cleanup inactive rooms)
- Alternative: Use Redis if memory becomes issue (unlikely)

---

### Risk 6: No Game Logic Validation Enables Cheating
**Impact**: Clients could send illegal moves, hack client to show false state.

**Mitigation**:
- **Accepted Trade-off**: This is a casual game, not competitive
- Future: Add server-side validation if needed ( GameState::make_move() already exists)
- Trust model: Assume clients are honest (simpler architecture)

---

### Risk 7: Session Loss on Server Restart
**Impact**: Users are logged out when server restarts.

**Mitigation**:
- Minor inconvenience (just log in again)
- Announce scheduled maintenance in advance
- Future: Persist sessions to Redis if frequent restarts

---

### Risk 8: Reconnection Timeout Too Short/Long
**Impact**: 30 seconds may be too short (slow networks) or too long (frustrates opponent).

**Mitigation**:
- 30 seconds is empirically reasonable for network blips
- Future: Make timeout configurable or adaptive
- Client shows countdown so opponent knows wait time

---

## Migration Plan

### Phase 1: Server Development (Week 1-2)
1. **Setup Server Project**
   - Initialize `server/` directory with Cargo.toml
   - Add dependencies: axum, tokio-tungstenite, sqlx, bcrypt, uuid
   - Create module structure (main.rs, auth.rs, db.rs, room.rs, ws.rs, types.rs)

2. **Database & Auth**
   - Write `migrations/init.sql` with tables (users, rooms, games)
   - Implement `db.rs`: connection pooling, queries
   - Implement `auth.rs`: password hashing, session management
   - Add REST endpoints: POST /api/register, POST /api/login

3. **Room Management**
   - Implement `room.rs`: Room struct, create/join logic
   - Add REST endpoints: GET /api/rooms, POST /api/rooms, POST /api/rooms/:id/join
   - Implement database persistence for rooms

4. **WebSocket Game Relay**
   - Implement `ws.rs`: WebSocket upgrade, message routing
   - Add WS endpoint: WS /game/:room_id?token=...
   - Implement broadcast logic (forward messages to opponent)
   - Add server messages (GameStart, OpponentJoined, OpponentDisconnected)

5. **Reconnection Handling**
   - Implement disconnect detection
   - Add 30-second timeout task
   - Handle reconnection (restore connection to room)
   - Add game end on timeout

### Phase 2: Client Modification (Week 3-4)
1. **Network Layer Refactor**
   - Modify `src-tauri/src/network.rs`:
     - Remove TCP listener/stream code
     - Add WebSocket client (use browser's WebSocket or `ws` crate)
     - Add HTTP client for REST API (fetch API)
   - Update Tauri commands:
     - Replace `network_host()`/`network_join()` with `login()`, `create_room()`, `join_room()`
     - Keep message sending compatible (same NetworkMessage enum)

2. **UI Changes**
   - Add `LoginScreen.tsx`: username/password forms
   - Modify `NetworkSetup.tsx`:
     - Replace IP input with room list display
     - Add "创建房间" and "加入" buttons
     - Add reconnection dialog (countdown timer)
   - Update `App.tsx`: Add login state, routing between screens

3. **Game Integration**
   - Update `GameBoard.tsx`: Handle server messages (GameStart, etc.)
   - Add loading states (connecting to server, waiting for opponent)
   - Handle disconnection gracefully

### Phase 3: Testing & Polish (Week 5)
1. **Unit Tests**
   - Server: auth logic, room management, message routing
   - Client: state management, error handling

2. **Integration Tests**
   - End-to-end: register → login → create room → join → play game
   - Disconnect scenarios: timeout, reconnect, game end

3. **Deployment**
   - Build release binaries for Linux, macOS, Windows
   - Test server auto-initialization (database creation, etc.)
   - Create README with deployment instructions

### Phase 4: Rollout
1. **Beta Testing**
   - Deploy server to test environment
   - Invite small group of users to test
   - Gather feedback, fix bugs

2. **Migration from P2P**
   - Keep P2P code as fallback (add feature flag?)
   - Or: Hard switch, require server for network play
   - Announce migration to users

3. **Production Launch**
   - Deploy server to production
   - Update client (new Tauri release)
   - Monitor logs, performance

### Rollback Strategy
- **If Server Crashes**: Restart server, users reconnect (sessions lost but can log in again)
- **If Critical Bug**: Revert to previous P2P version (but require coordinated client rollback)
- **If Performance Issues**: Add caching (Redis) or migrate to PostgreSQL

---

## Open Questions

### Q1: Should we keep P2P mode as fallback?
**Status**: Pending user decision.

**Options**:
- **A**: Remove P2P entirely (simpler code, forces server adoption)
- **B**: Keep P2P as "Direct Connection" option (more complex, but provides offline alternative)

**Recommendation**: Remove P2P to reduce complexity and encourage server adoption.

---

### Q2: What is the maximum number of concurrent rooms the server should support?
**Status**: Not defined yet.

**Context**: SQLite can handle ~1000 concurrent writes. In-memory rooms are limited by RAM.

**Recommendation**: Start with no limit, monitor performance, add limit if needed (e.g., 500 active rooms).

---

### Q3: Should we implement server-side game validation?
**Status**: Out of scope for MVP, but may be needed if cheating becomes issue.

**Recommendation**: Trust clients for now (casual game). Add validation in Phase 2 if users report cheating.

---

### Q4: How should we handle server IP configuration?
**Status**: Need to decide on UX.

**Options**:
- **A**: Hardcode server URL in client (simplest, but requires client rebuild to change)
- **B**: Config file (more flexible, but users must edit file)
- **C**: Environment variable (good for Docker, but not for GUI app)
- **D**: Settings UI (best UX, but requires new UI components)

**Recommendation**: Start with (B) config file (`~/.config/gobang/client.toml`), add (D) settings UI in future.

---

### Q5: Should we implement a "quick join" feature?
**Status**: Not in MVP, but common user request.

**Context**: Automatically join the first available room instead of browsing list.

**Recommendation**: Add in Phase 2 if users request it. Implementation: `POST /api/rooms/quick-join`.

---

## Success Criteria

- [ ] Server can handle 100 concurrent rooms (200 players) without performance degradation
- [ ] Client login/registration works flawlessly
- [ ] Room list updates within 5 seconds of room creation
- [ ] Game messages arrive within 200ms (client → server → opponent)
- [ ] Disconnection/reconnection flow works smoothly (30-second window)
- [ ] Server auto-initializes on first run (database creation, migrations)
- [ ] Binary runs on Linux, macOS, Windows without issues
- [ ] Server uses < 100MB RAM with 100 active rooms
- [ ] No memory leaks detected (run for 24 hours, monitor usage)
- [ ] Client can reconnect after server restart (with new login)
