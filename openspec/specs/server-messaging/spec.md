# Capability: Server Messaging

## ADDED Requirements

### Requirement: WebSocket game connection
The system SHALL allow players to connect to a game room via WebSocket.

#### Scenario: Player connects to game WebSocket
- **WHEN** a player connects to `ws://server/game/{room_id}?token={session_token}`
- **THEN** the system validates the session token
- **AND** verifies the player is a participant in the room
- **AND** adds the WebSocket connection to the room's players map
- **AND** the connection is ready to receive and send game messages

#### Scenario: Invalid token connection attempt
- **WHEN** a player connects with an invalid or missing session token
- **THEN** the system rejects the WebSocket connection
- **AND** closes the connection immediately

#### Scenario: Non-participant connection attempt
- **WHEN** a user attempts to connect to a room they are not participating in
- **THEN** the system rejects the WebSocket connection
- **AND** returns an error message

### Requirement: Game message forwarding
The system SHALL forward game messages between players without validating game logic.

#### Scenario: Forward move message
- **WHEN** a player sends a Move message `{ type: "move", row: 7, col: 8 }`
- **THEN** the system forwards the message to the opponent player in the same room
- **AND** does not validate if the move is legal
- **AND** does not parse or modify the message content

#### Scenario: Forward restart request
- **WHEN** a player sends a RestartRequest message
- **THEN** the system forwards the message to the opponent
- **AND** does not automatically restart the game

#### Scenario: Forward restart accept
- **WHEN** a player sends a RestartAccept message
- **THEN** the system forwards the message to the opponent

#### Scenario: Forward restart reject
- **WHEN** a player sends a RestartReject message
- **THEN** the system forwards the message to the opponent

#### Scenario: Forward disconnect message
- **WHEN** a player sends a Disconnect message
- **THEN** the system forwards the message to the opponent
- **AND** may close the connection

### Requirement: Server-initiated messages
The system SHALL send server messages to players about room events.

#### Scenario: Notify opponent joined
- **WHEN** a second player joins a room
- **THEN** the system sends an OpponentJoined message to the host player
- **AND** the message includes the opponent's username

#### Scenario: Send game start
- **WHEN** a room transitions to "playing" status
- **THEN** the system sends a GameStart message to both players
- **AND** the message specifies black player and white player

#### Scenario: Notify player disconnected
- **WHEN** a player's WebSocket connection is lost
- **THEN** the system sends an OpponentDisconnected message to the other player
- **AND** the message indicates if reconnection is possible
- **AND** includes the reconnect timeout (30 seconds)

#### Scenario: Notify player reconnected
- **WHEN** a disconnected player successfully reconnects
- **THEN** the system sends a PlayerReconnected message to both players

#### Scenario: Notify game ended
- **WHEN** a game ends (by normal completion, timeout, or resignation)
- **THEN** the system sends a GameEnded message to both players
- **AND** the message includes the winner and end reason

### Requirement: Broadcast to room
The system SHALL broadcast messages to all players in a room except the sender.

#### Scenario: Broadcast to opponent
- **WHEN** player A sends a game message
- **THEN** the system sends the message only to player B
- **AND** does not echo the message back to player A

#### Scenario: Broadcast server message
- **WHEN** the system generates a server event message
- **THEN** the system sends the message to all relevant players in the room

### Requirement: WebSocket connection lifecycle
The system SHALL manage WebSocket connection establishment and termination.

#### Scenario: Connection establishment
- **WHEN** a WebSocket connection is established
- **THEN** the system stores the connection sender in the room's players map
- **AND** the player is marked as connected

#### Scenario: Graceful disconnect
- **WHEN** a player's WebSocket connection closes normally
- **THEN** the system removes the player's connection from the room
- **AND** notifies the opponent
- **AND** may trigger reconnection timeout handling

#### Scenario: Abrupt disconnect
- **WHEN** a player's WebSocket connection is lost (network error)
- **THEN** the system detects the disconnection
- **AND** triggers reconnection timeout handling

### Requirement: Message format preservation
The system SHALL preserve the original message format when forwarding.

#### Scenario: Forward JSON message as-is
- **WHEN** a player sends a game message as JSON
- **THEN** the system forwards the exact JSON string to the opponent
- **AND** does not re-serialize or modify the structure

### Requirement: Room-specific message isolation
The system SHALL ensure messages are only forwarded within the same room.

#### Scenario: Message isolation between rooms
- **WHEN** player in room A sends a message
- **THEN** the message is only forwarded to other players in room A
- **AND** players in room B do not receive the message

#### Scenario: Multiple concurrent rooms
- **WHEN** multiple rooms are active simultaneously
- **THEN** each room's messages are isolated to that room only
