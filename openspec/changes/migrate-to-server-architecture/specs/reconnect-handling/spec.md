# Capability: Reconnect Handling

## ADDED Requirements

### Requirement: Detect player disconnection
The system SHALL detect when a player's WebSocket connection is lost.

#### Scenario: Detect network disconnect
- **WHEN** a player's WebSocket connection closes unexpectedly
- **THEN** the system detects the disconnection event
- **AND** identifies which player and room were affected

#### Scenario: Detect explicit disconnect
- **WHEN** a player's WebSocket connection closes normally
- **THEN** the system detects the disconnection event
- **AND** handles it as a normal disconnect (not timeout)

### Requirement: Notify opponent of disconnection
The system SHALL notify the opponent when a player disconnects.

#### Scenario: Notify opponent with reconnect window
- **WHEN** a player disconnects during an active game
- **THEN** the system sends an OpponentDisconnected message to the opponent
- **AND** the message includes `can_reconnect: true`
- **AND** the message includes `timeout_seconds: 30`
- **AND** the message includes the disconnected player's username

### Requirement: Reconnection timeout window
The system SHALL provide a 30-second window for disconnected players to reconnect.

#### Scenario: Start reconnection timeout
- **WHEN** a player disconnects during a game
- **THEN** the system records the disconnection timestamp
- **AND** starts a 30-second timeout task for that room
- **AND** the room is marked as having a disconnected player

#### Scenario: Player reconnects within timeout
- **WHEN** the disconnected player reconnects within 30 seconds
- **THEN** the system cancels the timeout task
- **AND** restores the player's WebSocket connection to the room
- **AND** sends a PlayerReconnected message to both players
- **AND** the game resumes normally

#### Scenario: Timeout expires without reconnection
- **WHEN** 30 seconds pass without the disconnected player reconnecting
- **THEN** the system declares the opponent the winner
- **AND** sends a GameEnded message with reason "opponent_disconnected"
- **AND** updates the room status to "ended"
- **AND** records the game result in the database

### Requirement: Player reconnection flow
The system SHALL allow disconnected players to reconnect to the same room.

#### Scenario: Reconnect with valid token
- **WHEN** a disconnected player reconnects to `ws://server/game/{room_id}?token={valid_token}`
- **AND** the reconnection occurs within the timeout window
- **THEN** the system accepts the reconnection
- **AND** adds the new WebSocket connection to the room
- **AND** sends a PlayerReconnected message
- **AND** optionally sends the current game state to the reconnected player

#### Scenario: Reconnect with invalid token
- **WHEN** a player attempts to reconnect with an invalid or expired token
- **THEN** the system rejects the reconnection
- **AND** closes the WebSocket connection
- **AND** the timeout task continues (if still within window)

#### Scenario: Reconnect after timeout
- **WHEN** a player attempts to reconnect after the 30-second timeout has expired
- **THEN** the system rejects the reconnection
- **AND** returns an error message "重连时间已过期"
- **AND** the room status is already "ended"

### Requirement: Client reconnection retry logic
The client SHALL automatically attempt to reconnect after disconnection.

#### Scenario: Client detects disconnect
- **WHEN** the WebSocket connection closes
- **THEN** the client displays a reconnection dialog
- **AND** shows the remaining timeout (countdown from 30 seconds)

#### Scenario: Client retry loop
- **WHEN** the client detects a disconnect
- **THEN** the client attempts to reconnect every 5 seconds
- **AND** stops retrying after 6 attempts (30 seconds total)
- **AND** updates the reconnection dialog with each attempt

#### Scenario: Successful reconnection
- **WHEN** a reconnection attempt succeeds
- **THEN** the client hides the reconnection dialog
- **AND** displays a "重新连接成功" message
- **AND** resumes the game interface

#### Scenario: Reconnection failure after timeout
- **WHEN** all 6 reconnection attempts fail
- **THEN** the client displays a "连接失败" message
- **AND** offers to return to the lobby
- **AND** may display the game result (if available)

### Requirement: Opponent experience during disconnect
The system SHALL provide a good experience for the connected opponent.

#### Scenario: Opponent sees disconnect notification
- **WHEN** the opponent disconnects
- **THEN** the connected player sees a message "对手已断开连接"
- **AND** sees a countdown timer showing remaining reconnection time
- **AND** the game board is disabled (no moves can be made)

#### Scenario: Opponent sees reconnection success
- **WHEN** the disconnected player successfully reconnects
- **THEN** the connected player sees a message "对手已重新连接"
- **AND** the game board is re-enabled
- **AND** play resumes

#### Scenario: Opponent wins by timeout
- **WHEN** the reconnection timeout expires
- **THEN** the connected player sees a "你赢了！" message
- **AND** the message includes reason "对手超时未重连"
- **AND** the player can return to lobby or start a new game

### Requirement: Game state preservation during disconnect
The system SHALL preserve the game state during a temporary disconnect.

#### Scenario: Room persists during timeout
- **WHEN** a player disconnects
- **THEN** the room remains in memory with status "playing"
- **AND** the reconnection timeout is tracked
- **AND** the game state (move history, current player) is preserved

#### Scenario: Game state available on reconnect
- **WHEN** a player successfully reconnects
- **THEN** the server can optionally send the current game state
- **AND** the client can resynchronize the board state

### Requirement: Prevent new connections during disconnect
The system SHALL not allow third parties to join a room during a disconnect.

#### Scenario: Room not joinable during disconnect
- **WHEN** a room has a disconnected player (within timeout)
- **THEN** the room status remains "playing"
- **AND** other users cannot join the room
- **AND** join attempts return "房间不可加入"

### Requirement: Double disconnect handling
The system SHALL handle the case where both players disconnect.

#### Scenario: Both players disconnect
- **WHEN** both players in a room disconnect
- **THEN** the system starts timeout for the first disconnect
- **AND** when the second player disconnects, the room is cleaned up
- **AND** the game is marked as "ended" with no winner (or tie)
