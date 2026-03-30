# Server Logging Specification

## ADDED Requirements

### Requirement: Request correlation via unique IDs

The system SHALL generate a unique request identifier for each HTTP request and WebSocket connection and include it in all log messages within that request's scope.

#### Scenario: HTTP request includes request ID in logs
- **WHEN** a client makes an HTTP request to any endpoint
- **THEN** the system generates a UUID v4 as the request ID
- **AND** all log messages during that request processing include the request ID
- **AND** the request ID can be used to correlate logs across async operations

#### Scenario: WebSocket connection includes request ID in logs
- **WHEN** a client establishes a WebSocket connection
- **THEN** the system generates a UUID v4 as the connection ID
- **AND** all log messages during that WebSocket session include the connection ID
- **AND** the connection ID appears in connection establishment, message handling, and disconnection logs

### Requirement: HTTP request lifecycle logging

The system SHALL log the start, completion, and duration of every HTTP request.

#### Scenario: Successful HTTP request logged
- **WHEN** an HTTP request completes successfully
- **THEN** the system logs the request method, path, response status code, and duration in milliseconds
- **AND** the log level is INFO
- **AND** the log includes the request ID

#### Scenario: Failed HTTP request logged
- **WHEN** an HTTP request fails with a 4xx or 5xx status code
- **THEN** the system logs the request method, path, error status code, and duration
- **AND** the log level is WARN for 4xx errors
- **AND** the log level is ERROR for 5xx errors
- **AND** the log includes the request ID

#### Scenario: Slow HTTP request logged
- **WHEN** an HTTP request takes longer than 1 second to complete
- **THEN** the system logs the request method, path, and duration
- **AND** the log level is WARN
- **AND** the log message indicates it was slow

### Requirement: Authentication and authorization logging

The system SHALL log all authentication attempts and authorization failures with appropriate detail.

#### Scenario: Successful authentication logged
- **WHEN** a user successfully authenticates via login or register
- **THEN** the system logs the user ID and username
- **AND** the log level is INFO
- **AND** the log does NOT include the password or full token

#### Scenario: Failed authentication logged
- **WHEN** authentication fails due to invalid credentials or missing token
- **THEN** the system logs the failure reason
- **AND** the log level is WARN
- **AND** the log includes the request ID but NOT the invalid credentials

#### Scenario: Authorization failure logged
- **WHEN** a user attempts to access a resource they are not authorized for
- **THEN** the system logs the user ID, requested resource, and denial reason
- **AND** the log level is WARN

### Requirement: WebSocket connection lifecycle logging

The system SHALL log all stages of WebSocket connection establishment, message handling, and disconnection.

#### Scenario: WebSocket connection establishment logged
- **WHEN** a WebSocket connection is initiated
- **THEN** the system logs the connection attempt with room ID and token hash
- **AND** the log level is INFO

#### Scenario: WebSocket authentication validation logged
- **WHEN** the WebSocket handler validates the authentication token
- **THEN** the system logs whether validation succeeded or failed
- **AND** on success, logs the user ID and username
- **AND** on failure, logs the failure reason at WARN level
- **AND** the token hash (not full token) is logged

#### Scenario: WebSocket room validation logged
- **WHEN** the WebSocket handler validates room existence and user participation
- **THEN** the system logs the room ID and user ID
- **AND** if validation fails, logs the failure reason at WARN level

#### Scenario: WebSocket disconnection logged
- **WHEN** a WebSocket connection closes (normally or abnormally)
- **THEN** the system logs the disconnection reason and user ID
- **AND** the log level is INFO for normal closure
- **AND** the log level is WARN for abnormal closure
- **AND** the log includes the connection ID and duration

### Requirement: Room state transition logging

The system SHALL log all room state changes and player actions within rooms.

#### Scenario: Room creation logged
- **WHEN** a new room is created
- **THEN** the system logs the room ID, room name, and host user ID
- **AND** the log level is INFO

#### Scenario: Player joining room logged
- **WHEN** a player joins a room
- **THEN** the system logs the room ID, joining user ID, and new room status
- **AND** the log level is INFO

#### Scenario: Room status transition logged
- **WHEN** a room's status changes (Waiting → Playing → Ended)
- **THEN** the system logs the room ID, old status, and new status
- **AND** the log level is INFO

#### Scenario: Game move logged
- **WHEN** a player makes a move in a game
- **THEN** the system logs the room ID, player ID, and move coordinates at DEBUG level
- **AND** the log level is DEBUG (to avoid log spam in long games)

#### Scenario: Player disconnection handling logged
- **WHEN** a player disconnects during an active game
- **THEN** the system logs the room ID, disconnected user ID, and timeout duration
- **AND** the log level is WARN

#### Scenario: Player reconnection logged
- **WHEN** a player reconnects to an active game
- **THEN** the system logs the room ID, reconnecting user ID, and time since disconnection
- **AND** the log level is INFO

### Requirement: WebSocket error messages to clients

The system SHALL send error messages to WebSocket clients before closing connections due to validation failures.

#### Scenario: Invalid token error sent to client
- **WHEN** a WebSocket connection attempt fails due to invalid authentication token
- **THEN** the system sends an error message to the client before closing the connection
- **AND** the error message includes the reason (e.g., "invalid_token")
- **AND** the system logs the failure with the connection ID

#### Scenario: Room not found error sent to client
- **WHEN** a WebSocket connection attempt fails because the room does not exist
- **THEN** the system sends an error message to the client before closing the connection
- **AND** the error message includes the reason (e.g., "room_not_found")
- **AND** the system logs the failure with the connection ID and room ID

#### Scenario: Not a participant error sent to client
- **WHEN** a WebSocket connection attempt fails because the user is not a room participant
- **THEN** the system sends an error message to the client before closing the connection
- **AND** the error message includes the reason (e.g., "not_participant")
- **AND** the system logs the failure with the connection ID, user ID, and room ID

### Requirement: Database operation logging

The system SHALL log all database write operations and significant failures.

#### Scenario: Database write logged
- **WHEN** a database write operation completes successfully (user creation, room creation, etc.)
- **THEN** the system logs the operation type and affected entity ID
- **AND** the log level is DEBUG
- **AND** sensitive data (passwords, tokens) is never logged

#### Scenario: Database error logged
- **WHEN** a database operation fails
- **THEN** the system logs the error details including operation type and error message
- **AND** the log level is ERROR

### Requirement: Structured log format

All log messages SHALL be structured with key-value pairs to enable filtering and parsing.

#### Scenario: Log includes context fields
- **WHEN** any log message is emitted
- **THEN** the log includes standard fields: request_id, timestamp, log level
- **AND** context-specific fields are included when available: user_id, room_id, connection_id
- **AND** the format is parseable by log aggregation tools

### Requirement: Configurable log levels

The system SHALL support configuring log verbosity via environment variables.

#### Scenario: Log level configurable via RUST_LOG
- **WHEN** the server starts with the RUST_LOG environment variable set
- **THEN** the system uses the specified log level
- **AND** valid values are: error, warn, info, debug, trace
- **AND** the default level is info

#### Scenario: Module-specific log levels
- **WHEN** the server starts with RUST_LOG set to a module-specific pattern (e.g., "gobang::ws=debug")
- **THEN** the specified module uses that log level
- **AND** other modules use the default level
