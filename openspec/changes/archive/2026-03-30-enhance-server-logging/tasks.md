## 1. Setup and Dependencies

- [x] 1.1 Add `uuid` dependency to server/Cargo.toml with feature "v4"
- [x] 1.2 Verify `tracing` and `tracing-subscriber` are in dependencies
- [x] 1.3 Create logging utility module at server/src/logging.rs with request ID generation helper

## 2. Request ID Infrastructure

- [x] 2.1 Implement `generate_request_id()` function using uuid v4
- [x] 2.2 Create `request_id()` span extension function for extracting request ID from current span
- [x] 2.3 Add request ID as a field to all log macros (debug, info, warn, error)

## 3. HTTP Request Logging Middleware

- [x] 3.1 Create logging middleware in server/src/main.rs that wraps all HTTP routes
- [x] 3.2 Implement request start logging with method, path, and request ID
- [x] 3.3 Implement request completion logging with status code and duration
- [x] 3.4 Add WARN log for requests taking >1 second
- [x] 3.5 Apply middleware to router in main.rs

## 4. Authentication Logging

- [x] 4.1 Add request ID span to `/api/register` endpoint in server/src/main.rs
- [x] 4.2 Log successful registration with user_id and username (INFO level)
- [x] 4.3 Log failed registration with error reason (WARN level)
- [x] 4.4 Add request ID span to `/api/login` endpoint
- [x] 4.5 Log successful login with user_id (INFO level)
- [x] 4.6 Log failed login with error reason (WARN level)
- [x] 4.7 Add request ID span to token verification in server/src/auth.rs
- [x] 4.8 Log token verification failures in ws.rs with WARN level (add request ID)

## 5. WebSocket Connection Lifecycle Logging

- [x] 5.1 Add connection ID span to `ws_handler` in server/src/ws.rs
- [x] 5.2 Log WebSocket connection attempt with room_id and token hash (INFO level)
- [x] 5.3 Log successful token verification with user_id and username (INFO level)
- [x] 5.4 Log failed token verification with error reason (WARN level)
- [x] 5.5 Log successful room validation with room_id and user_id (INFO level)
- [x] 5.6 Log failed room validation (not found) with room_id (WARN level)
- [x] 5.7 Log failed participant check with user_id and room_id (WARN level)
- [x] 5.8 Log connection closure with reason and duration (INFO for normal, WARN for abnormal)
- [x] 5.9 Log message processing errors with context (ERROR level)

## 6. Room State and Player Action Logging

- [x] 6.1 Log room creation in server/src/main.rs create_room handler with room_id, name, host_id (INFO)
- [x] 6.2 Log player joining room in main.rs join_room handler with room_id, user_id (INFO)
- [x] 6.3 Log room status transitions in server/src/room.rs add_player method (INFO)
- [x] 6.4 Log game moves in forward_messages function with room_id, user_id, coordinates (DEBUG)
- [x] 6.5 Log player disconnection in handle_disconnect with room_id, user_id (WARN)
- [x] 6.6 Log player reconnection in handle_ws with room_id, user_id, time_since_disconnect (INFO)
- [x] 6.7 Log timeout triggered in handle_disconnect with room_id, winner (INFO)

## 7. WebSocket Error Messages to Clients

- [x] 7.1 Define WebSocket error message enum/struct in server/src/protocol/game.rs
- [x] 7.2 Implement error message sending in ws.rs before early returns
- [x] 7.3 Add error message for invalid token case (line 39-44)
- [x] 7.4 Add error message for user not found case (line 47-52)
- [x] 7.5 Add error message for room not found case (line 56-62)
- [x] 7.6 Add error message for not participant case (line 64-67)
- [x] 7.7 Ensure error messages are logged at WARN level with connection_id

## 8. Database Operation Logging

- [x] 8.1 Add request ID spans to all db write operations in server/src/db.rs
- [x] 8.2 Log successful user creation with user_id (DEBUG)
- [x] 8.3 Log successful room creation with room_id (DEBUG)
- [x] 8.4 Log successful room update operations (DEBUG)
- [x] 8.5 Log database errors with operation type and error message (ERROR)

## 9. Testing and Verification

- [x] 9.1 Run `cargo test` in server/ to ensure no regressions
- [x] 9.2 Start server and verify HTTP requests are logged in console
- [x] 9.3 Create a room and verify room creation logs appear
- [x] 9.4 Join a room and verify join logs and WebSocket logs appear
- [x] 9.5 Test invalid WebSocket connection attempts and verify error messages are sent
- [x] 9.6 Test with RUST_LOG=debug to verify DEBUG logs appear
- [x] 9.7 Test with RUST_LOG=info to verify DEBUG logs are suppressed
- [x] 9.8 Monitor log file sizes during testing to ensure no excessive logging
- [x] 9.9 Verify request IDs appear in all log messages for a single request

## 10. Documentation

- [x] 10.1 Update server/README.md with logging configuration instructions
- [x] 10.2 Document recommended RUST_LOG values for production (info or warn)
- [x] 10.3 Add example log output showing request ID correlation
- [x] 10.4 Document how to grep logs for specific request IDs
