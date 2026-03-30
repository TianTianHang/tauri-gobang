# Implementation Tasks

## 1. Server Project Setup

- [ ] 1.1 Create `server/` directory with Cargo.toml
- [ ] 1.2 Add dependencies: axum, tokio-tungstenite, sqlx, bcrypt, uuid, serde, tracing, anyhow
- [ ] 1.3 Create module structure: main.rs, auth.rs, db.rs, room.rs, ws.rs, types.rs, protocol/
- [ ] 1.4 Create `migrations/init.sql` with tables (users, rooms, games)
- [ ] 1.5 Add .env configuration with defaults (port 3001, database path, etc.)

## 2. Database Layer

- [ ] 2.1 Implement `db.rs`: SqlitePool connection setup
- [ ] 2.2 Add `init_database()` function to run migrations on startup
- [ ] 2.3 Implement User queries: insert_user, find_by_username, find_by_id
- [ ] 2.4 Implement Room queries: insert_room, find_by_id, update_player2, update_status, list_waiting_rooms
- [ ] 2.5 Implement Game queries: insert_game, find_by_room
- [ ] 2.6 Add database error handling and conversion to HTTP status codes

## 3. Authentication System

- [ ] 3.1 Implement `auth.rs`: password hashing with bcrypt (cost factor 12)
- [ ] 3.2 Implement session token generation (UUID v4)
- [ ] 3.3 Create in-memory session store (HashMap<String, UserId>)
- [ ] 3.4 Implement `register()` handler: validate input, hash password, insert to DB
- [ ] 3.5 Implement `login()` handler: verify password, generate token, store session
- [ ] 3.6 Add middleware for token validation (extract Authorization header)
- [ ] 3.7 Add helper functions: `verify_token()`, `get_user_id_from_token()`

## 4. Room Management REST API

- [ ] 4.1 Create `room.rs`: Room struct (id, name, status, players, host_id, disconnected)
- [ ] 4.2 Implement in-memory room store (HashMap<String, Room>)
- [ ] 4.3 Add `GET /api/rooms` endpoint: return list of waiting rooms
- [ ] 4.4 Add `POST /api/rooms` endpoint: create room, generate UUID, store in DB and memory
- [ ] 4.5 Add `POST /api/rooms/:id/join` endpoint: validate room status, add player2, update status to "playing"
- [ ] 4.6 Add helper: `broadcast_to_room()` to send messages to all players in a room
- [ ] 4.7 Add room lifecycle: auto-cleanup after game ends

## 5. WebSocket Game Messaging

- [ ] 5.1 Create `ws.rs` module with WebSocket upgrade handler
- [ ] 5.2 Implement `WS /game/:room_id` endpoint with token validation
- [ ] 5.3 Add connection handler: extract room_id and token, validate, add connection to Room.players
- [ ] 5.4 Implement message receive loop: parse incoming JSON, forward to opponent
- [ ] 5.5 Add server-initiated messages:
  - [ ] 5.5.1 GameStart message (send when room transitions to "playing")
  - [ ] 5.5.2 OpponentJoined message (send when player2 joins)
  - [ ] 5.5.3 OpponentDisconnected message (send on disconnect)
  - [ ] 5.5.4 PlayerReconnected message (send on reconnect)
  - [ ] 5.5.5 GameEnded message (send on game over)
- [ ] 5.6 Implement message isolation: only forward to opponent, not back to sender
- [ ] 5.7 Add graceful connection close handling

## 6. Reconnection Handling

- [ ] 6.1 Add disconnect detection in WebSocket handler (on_close event)
- [ ] 6.2 Store disconnection info in Room: `disconnected: Option<(UserId, Instant)>`
- [ ] 6.3 Implement 30-second timeout task using `tokio::time::sleep`
- [ ] 6.4 Send OpponentDisconnected message with timeout info to connected player
- [ ] 6.5 Implement reconnection handler:
  - [ ] 6.5.1 Validate token and room_id
  - [ ] 6.5.2 Check if within timeout window
  - [ ] 6.5.3 Cancel timeout task if reconnected
  - [ ] 6.5.4 Restore WebSocket connection to Room.players
  - [ ] 6.5.5 Send PlayerReconnected message to both players
- [ ] 6.6 Implement timeout expiry: declare opponent winner, send GameEnded message
- [ ] 6.7 Update room status to "ended" and remove from memory after timeout

## 7. Server Deployment Features

- [ ] 7.1 Implement data directory auto-creation: `~/.gobang-server/` or `%APPDATA%/gobang-server/`
- [ ] 7.2 Add auto-initialization on first run: create database, run migrations
- [ ] 7.3 Implement default configuration with config file generation (`config.toml`)
- [ ] 7.4 Add daemon mode support (`--daemon` flag): fork to background, redirect logs
- [ ] 7.5 Create systemd service file template for Linux
- [ ] 7.6 Implement graceful shutdown on SIGINT/SIGTERM (close connections, DB)
- [ ] 7.7 Add startup logging: display server URL, data directory, PID
- [ ] 7.8 Implement port binding error handling

## 8. Protocol Definition

- [ ] 8.1 Define `protocol/control.rs`: ServerControlMessage enum (Login, Register, etc.)
- [ ] 8.2 Define `protocol/control.rs`: ServerResponse enum (LoginSuccess, RoomCreated, etc.)
- [ ] 8.3 Define `protocol/game.rs`: reuse NetworkMessage enum from client (Move, RestartRequest, etc.)
- [ ] 8.4 Define `protocol/game.rs`: ServerMessage enum (GameStart, OpponentJoined, etc.)
- [ ] 8.5 Add serde serialization/deserialization for all message types

## 9. Client Network Layer Refactor

- [ ] 9.1 Update `src-tauri/src/network.rs`:
  - [ ] 9.1.1 Remove TCP listener/stream code (host_game, join_game functions)
  - [ ] 9.1.2 Add HTTP client for REST API (use fetch API or reqwest)
  - [ ] 9.1.3 Add WebSocket client connection (use browser WebSocket or ws crate)
  - [ ] 9.1.4 Update NetworkState to track server connection, room_id
- [ ] 9.2 Add new Tauri commands in `lib.rs`:
  - [ ] 9.2.1 `register(username, password)`: POST /api/register
  - [ ] 9.2.2 `login(username, password)`: POST /api/login, store token
  - [ ] 9.2.3 `create_room(name, token)`: POST /api/rooms
  - [ ] 9.2.4 `join_room(room_id, token)`: POST /api/rooms/:id/join
  - [ ] 9.2.5 `get_rooms(token)`: GET /api/rooms
  - [ ] 9.2.6 `connect_game_websocket(room_id, token)`: establish WS connection
- [ ] 9.3 Update existing commands:
  - [ ] 9.3.1 Modify `network_send_move()` to send via WebSocket
  - [ ] 9.3.2 Keep NetworkMessage enum unchanged (Move, RestartRequest, etc.)
  - [ ] 9.3.3 Add handlers for server messages (GameStart, etc.)
- [ ] 9.4 Add error handling for network failures (500, 401, etc.)

## 10. Client UI - Authentication

- [ ] 10.1 Create `LoginScreen.tsx` component:
  - [ ] 10.1.1 Username and password input fields
  - [ ] 10.1.2 Login form with validation
  - [ ] 10.1.3 Register form with validation (password length check)
  - [ ] 10.1.4 Error message display
- [ ] 10.2 Add LoginScreen.css styling
- [ ] 10.3 Integrate LoginScreen into App.tsx routing:
  - [ ] 10.3.1 Add "logged_in" state
  - [ ] 10.3.2 Store token in localStorage
  - [ ] 10.3.3 Redirect to room lobby on successful login

## 11. Client UI - Room Lobby

- [ ] 11.1 Refactor `NetworkSetup.tsx` to RoomList.tsx:
  - [ ] 11.1.1 Remove IP/port input fields
  - [ ] 11.1.2 Add room list display (table or cards)
  - [ ] 11.1.3 Add "创建房间" button → dialog with room name input
  - [ ] 11.1.4 Add "刷新房间列表" button
  - [ ] 11.1.5 Add "加入" button for each room
  - [ ] 11.1.6 Implement 5-second polling for room list updates
- [ ] 11.2 Create `WaitingRoom.tsx`:
  - [ ] 11.2.1 Display room name and room ID
  - [ ] 11.2.2 Show "等待对手加入..." message
  - [ ] 11.2.3 Add "复制房间链接" button (share room_id)
  - [ ] 11.2.4 Handle GameStart message → transition to game
- [ ] 11.3 Update App.tsx routing:
  - [ ] 11.3.1 Add route: /login → LoginScreen
  - [ ] 11.3.2 Add route: /lobby → RoomList
  - [ ] 11.3.3 Add route: /waiting/:room_id → WaitingRoom
  - [ ] 11.3.4 Keep existing /game route for GameBoard

## 12. Client UI - Game & Reconnection

- [ ] 12.1 Update `GameBoard.tsx`:
  - [ ] 12.1.1 Display opponent username (from GameStart message)
  - [ ] 12.1.2 Handle GameStart message (initialize game)
  - [ ] 12.1.3 Handle OpponentDisconnected message (show countdown)
  - [ ] 12.1.4 Handle PlayerReconnected message (hide countdown, resume)
  - [ ] 12.1.5 Handle GameEnded message (show result, return to lobby)
- [ ] 12.2 Create `ReconnectDialog.tsx`:
  - [ ] 12.2.1 Display "对手已断开连接" message
  - [ ] 12.2.2 Show countdown timer (30 seconds)
  - [ ] 12.2.3 Display reconnection attempts ("正在尝试重连... (3/6)")
  - [ ] 12.2.4 Implement auto-reconnect logic (every 5 seconds for 30 seconds)
  - [ ] 12.2.5 Handle reconnect success/failure
- [ ] 12.3 Update `GameInfo.tsx`:
  - [ ] 12.3.1 Add "返回大厅" button
  - [ ] 12.3.2 Show connection status indicator

## 13. Testing

- [ ] 13.1 Write server unit tests:
  - [ ] 13.1.1 Test password hashing and verification
  - [ ] 13.1.2 Test session token generation and validation
  - [ ] 13.1.3 Test room creation and joining
  - [ ] 13.1.4 Test message forwarding (send to opponent)
  - [ ] 13.1.5 Test disconnect detection and timeout
  - [ ] 13.1.6 Test reconnection flow
- [ ] 13.2 Write client tests:
  - [ ] 13.2.1 Test login/register API calls
  - [ ] 13.2.2 Test room list fetching
  - [ ] 13.2.3 Test WebSocket message sending/receiving
  - [ ] 13.2.4 Test reconnection dialog logic
- [ ] 13.3 Integration tests:
  - [ ] 13.3.1 End-to-end: register → login → create room → join → play game
  - [ ] 13.3.2 Test disconnect/reconnect scenarios
  - [ ] 13.3.3 Test multiple concurrent rooms
- [ ] 13.4 Load testing:
  - [ ] 13.4.1 Test 100 concurrent rooms (200 players)
  - [ ] 13.4.2 Monitor server memory usage
  - [ ] 13.4.3 Measure message latency

## 14. Build & Deployment

- [ ] 14.1 Create `server/build.sh` script for cross-platform builds
- [ ] 14.2 Build release binaries:
  - [ ] 14.2.1 Linux x86_64: `cargo build --release --target x86_64-unknown-linux-gnu`
  - [ ] 14.2.2 Windows x86_64: `cargo build --release --target x86_64-pc-windows-gnu`
  - [ ] 14.2.3 macOS x86_64: `cargo build --release --target x86_64-apple-darwin`
  - [ ] 14.2.4 macOS ARM64: `cargo build --release --target aarch64-apple-darwin`
- [ ] 14.3 Create distributable packages:
  - [ ] 14.3.1 Add README.md with usage instructions
  - [ ] 14.3.2 Create tar.gz for Linux/macOS
  - [ ] 14.3.3 Create zip for Windows
- [ ] 14.4 Test server deployment:
  - [ ] 14.4.1 Test first-run initialization (database creation)
  - [ ] 14.4.2 Test daemon mode
  - [ ] 14.4.3 Test systemd service (Linux)
  - [ ] 14.4.4 Test graceful shutdown (Ctrl+C)
- [ ] 14.5 Update client build:
  - [ ] 14.5.1 Update Tauri configuration with server URL
  - [ ] 14.5.2 Build client for all platforms
  - [ ] 14.5.3 Test client-server communication

## 15. Documentation

- [ ] 15.1 Write `server/README.md`:
  - [ ] 15.1.1 Build instructions
  - [ ] 15.1.2 Deployment guide
  - [ ] 15.1.3 Configuration options
  - [ ] 15.1.4 Troubleshooting
- [ ] 15.2 Update `docs/ANDROID_EMULATOR_GUIDE.md` (if applicable)
- [ ] 15.3 Update `AGENTS.md` with new server commands
- [ ] 15.4 Create user guide:
  - [ ] 15.4.1 How to register/login
  - [ ] 15.4.2 How to create/join rooms
  - [ ] 15.4.3 Troubleshooting network issues

## 16. Final Polish

- [ ] 16.1 Add comprehensive logging to server (tracing crate)
- [ ] 16.2 Add error recovery (database connection retry, WebSocket reconnection)
- [ ] 16.3 Optimize performance (database queries, message serialization)
- [ ] 16.4 Add input validation (room name length, username sanitization)
- [ ] 16.5 Security audit:
  - [ ] 16.5.1 Check for SQL injection (sqlx should prevent)
  - [ ] 16.5.2 Verify password hashing strength
  - [ ] 16.5.3 Test for DoS vulnerabilities (rate limiting?)
- [ ] 16.6 Code cleanup and refactoring
- [ ] 16.7 Final testing round
