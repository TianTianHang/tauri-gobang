# Capability: Room Management

## ADDED Requirements

### Requirement: Create game room
The system SHALL allow authenticated users to create a new game room with a custom name.

#### Scenario: Successful room creation
- **WHEN** an authenticated user sends a request to create a room with a name
- **THEN** the system generates a unique room ID (UUID)
- **AND** creates a room record in the database with status "waiting"
- **AND** stores the user ID as the host
- **AND** creates an in-memory Room object with no player connections
- **AND** returns the room ID and WebSocket game URL to the client

#### Scenario: Room creation without authentication
- **WHEN** an unauthenticated user attempts to create a room
- **THEN** the system rejects the request
- **AND** returns HTTP 401 Unauthorized

### Requirement: Join existing room
The system SHALL allow authenticated users to join an existing room that is in "waiting" status.

#### Scenario: Successful room join
- **WHEN** an authenticated user requests to join a room in "waiting" status
- **THEN** the system updates the room's player2_id in the database
- **AND** changes the room status to "playing"
- **AND** updates the in-memory Room object
- **AND** notifies the host player via WebSocket that an opponent has joined
- **AND** returns the WebSocket game URL to the joining player

#### Scenario: Join non-existent room
- **WHEN** a user attempts to join a room with an invalid room ID
- **THEN** the system rejects the request
- **AND** returns an error message "房间不存在"

#### Scenario: Join full room
- **WHEN** a user attempts to join a room that is not in "waiting" status
- **THEN** the system rejects the request
- **AND** returns an error message "房间不可加入"

#### Scenario: Join own room
- **WHEN** a user attempts to join a room they created
- **THEN** the system rejects the request
- **AND** returns an error message "无法加入自己创建的房间"

### Requirement: List available rooms
The system SHALL allow authenticated users to query a list of available rooms.

#### Scenario: Get room list
- **WHEN** an authenticated user requests the room list
- **THEN** the system returns all rooms with status "waiting"
- **AND** each room includes: room ID, name, host username, created timestamp
- **AND** results are ordered by creation time (newest first)

#### Scenario: Empty room list
- **WHEN** no rooms are available
- **THEN** the system returns an empty list

### Requirement: Automatic game start
The system SHALL automatically start the game when a second player joins a room.

#### Scenario: Game starts on second player join
- **WHEN** a second player successfully joins a room
- **THEN** the system immediately sends a "GameStart" message to both players via WebSocket
- **AND** the message specifies which player is black (host) and which is white (joiner)
- **AND** the room status in memory changes to "playing"

### Requirement: Room status lifecycle
The system SHALL manage room status through waiting, playing, and ended states.

#### Scenario: Room transitions to playing
- **WHEN** a second player joins a waiting room
- **THEN** the database room status updates to "playing"
- **AND** the in-memory room status updates to "playing"

#### Scenario: Room transitions to ended
- **WHEN** a game concludes (normal end, disconnect timeout, or resignation)
- **THEN** the database room status updates to "ended"
- **AND** the ended_at timestamp is recorded
- **AND** the in-memory Room object is removed or marked for cleanup

### Requirement: Room persistence in database
The system SHALL persist room information in the SQLite database.

#### Scenario: Room record creation
- **WHEN** a room is created
- **THEN** a record is inserted into the rooms table with: id, name, host_id, status, created_at

#### Scenario: Room player update
- **WHEN** a second player joins
- **THEN** the room record updates player2_id and status

#### Scenario: Room completion record
- **WHEN** a game ends
- **THEN** the room record updates status to "ended" and sets ended_at timestamp

### Requirement: In-memory room management
The system SHALL maintain active rooms in server memory for fast access.

#### Scenario: Room object creation
- **WHEN** a room is created
- **THEN** an in-memory Room object is created with: id, name, status, players map, host_id

#### Scenario: Player connection tracking
- **WHEN** a player connects via WebSocket
- **THEN** their WebSocket sender is added to the room's players map

#### Scenario: Room cleanup after game
- **WHEN** a game ends and both players disconnect
- **THEN** the in-memory Room object is removed from memory
