## Why

The server currently has minimal logging, making it extremely difficult to debug issues when users report problems like "unable to join room" (无法加入房间). The WebSocket handler (ws.rs) performs multiple validations that fail silently—logging warnings to the server console but sending no error messages to the client. This lack of observability hinders troubleshooting and creates a poor developer experience.

## What Changes

- Add detailed, structured logging throughout the server's request lifecycle:
  - HTTP API endpoints (authentication, room creation/joining)
  - WebSocket connection lifecycle (handshake, authentication, room joining, reconnection)
  - Room state transitions and player actions
  - Error conditions with full context (user ID, room ID, error details)

- Enhance WebSocket error handling to send error messages to clients before closing connections

- Add request ID tracing for correlating logs across async operations

- Add timing/performance logging for slow operations

## Capabilities

### New Capabilities

- `server-logging`: Comprehensive structured logging across all server components for debugging, monitoring, and observability

### Modified Capabilities

None (this is an implementation-only improvement; no spec-level behavior changes)

## Impact

**Affected code:**
- `server/src/main.rs` - HTTP endpoint handlers
- `server/src/ws.rs` - WebSocket handler and message forwarding
- `server/src/auth.rs` - Authentication and session management
- `server/src/room.rs` - Room state management
- `server/src/db.rs` - Database operations (optional)

**New dependencies:**
- Add `tracing` subscriber with more detailed configuration
- Possibly `uuid` for request IDs if not already present

**API changes:**
- WebSocket error responses will now include error messages before connection closes
- HTTP API responses unchanged (already have error messages)

**Systems:**
- Development: Easier debugging with detailed logs
- Production: Better observability for troubleshooting (can configure log levels)
