## Context

The Gobang server currently uses `tracing` for logging but with minimal instrumentation. Key areas like WebSocket connection handling, authentication, and room state transitions have sparse or no logging at critical points. When users report issues like "unable to join room," there's insufficient context to diagnose whether the problem is authentication, room state, network issues, or client-side errors.

**Current state:**
- `tracing_subscriber` configured in main.rs with env filter
- Sparse `tracing::warn!` and `tracing::info!` statements
- WebSocket handler (ws.rs) has 4 critical validation points that fail silently
- No request correlation IDs
- No timing/performance logging
- Error messages logged but not sent to WebSocket clients

**Constraints:**
- Must not break existing functionality
- Should not significantly impact performance
- Must work with existing `tracing` infrastructure
- Should be configurable via environment variables (RUST_LOG)

## Goals / Non-Goals

**Goals:**
- Add comprehensive structured logging at all request lifecycle stages
- Enable request correlation via unique request IDs
- Send error messages to WebSocket clients before closing connections
- Add timing information for slow operations
- Make logs parseable and filterable (structured format)

**Non-Goals:**
- Changing existing API contracts or behaviors
- Adding a separate logging service or external log aggregation
- Implementing log retention/archival policies
- Adding metrics/monitoring dashboards (future work)

## Decisions

### 1. Use `tracing` spans for request correlation

**Decision:** Wrap each HTTP request and WebSocket connection in a `tracing::span` with a unique request ID.

**Rationale:**
- `tracing` is already a dependency
- Spans automatically propagate through async tasks via `.instrument()`
- Request IDs allow correlating logs across async operations
- No external dependency needed

**Alternatives considered:**
- Manual request ID passing: More error-prone, doesn't handle async well
- External tracing system (Jaeger/OTel): Overkill for single-server deployment

### 2. Generate request IDs using `uuid` crate

**Decision:** Add `uuid` dependency and generate v4 UUIDs for each request.

**Rationale:**
- Guaranteed uniqueness across distributed systems
- Standard format, easy to grep in logs
- Lightweight generation
- Already familiar to developers

**Alternatives considered:**
- Timestamp-based IDs: Not unique enough under high concurrency
- Sequential IDs: Requires shared state, harder to correlate

### 3. Add middleware layer for HTTP request logging

**Decision:** Create a logging middleware in main.rs that wraps all HTTP routes.

**Rationale:**
- Centralized logging logic
- Captures request method, path, status code, duration
- Consistent format across all endpoints
- Easy to modify/extend

**Alternatives considered:**
- Per-endpoint logging: Code duplication, inconsistent format
- Tower middleware layer: More complex, but might consider in future

### 4. WebSocket errors sent to client before close

**Decision:** Before returning early from validation failures, send a WebSocket message with error details.

**Rationale:**
- Users need to know why connection failed
- Matches HTTP API behavior (error responses)
- Improves developer experience
- Minimal code change

**Alternatives considered:**
- HTTP-only errors: WebSocket failures would remain mysterious
- Custom error codes: More complex, text messages sufficient for now

### 5. Log levels based on severity

**Decision:** Use appropriate `tracing` levels:
- `ERROR`: Failed operations that prevent request completion
- `WARN`: Recoverable issues, unexpected but not breaking
- `INFO`: Normal operation lifecycle events (connection start/end)
- `DEBUG`: Detailed context (user IDs, room IDs, validation steps)

**Rationale:**
- Standard practice for Rust logging
- Allows filtering via RUST_LOG in production
- Defaults to INFO, noise-free in production

## Risks / Trade-offs

### Performance impact from logging

**Risk:** Additional logging and string formatting could slow down request handling, especially at DEBUG level.

**Mitigation:**
- Use lazy string evaluation with `tracing::debug!` macros
- Default to INFO level in production
- Benchmark before/after logging changes
- Expensive operations (JSON serialization) only at DEBUG level

### Log verbosity in production

**Risk:** DEBUG logs might be too verbose, filling disk or causing performance issues.

**Mitigation:**
- Document recommended RUST_LOG settings for production (INFO or WARN)
- Add rate limiting for repeated error logs (same user failing repeatedly)
- Use structured logging to enable filtering/searching

### WebSocket error message format

**Risk:** Sending error messages before WebSocket close could be problematic if client already closed.

**Mitigation:**
- Use `try_send()` or ignore send errors
- Log anyway even if send fails
- Keep error messages brief and JSON-serializable

### Sensitive information in logs

**Risk:** Logging user IDs, room IDs, or tokens could expose sensitive data.

**Mitigation:**
- NEVER log full tokens, only token hash or first/last few chars
- User IDs are acceptable (already in database)
- Room IDs are acceptable (not sensitive)
- Add note in code review guidelines

## Migration Plan

No migration needed. Changes are additive and backward compatible.

**Deployment steps:**
1. Deploy server with new logging
2. Monitor log file sizes and performance metrics
3. Adjust RUST_LOG if needed based on production observation

**Rollback:**
- Simple git revert to previous version
- No database changes or state to clean up

## Open Questions

None at this time. The design is straightforward enhancement to existing logging infrastructure.
