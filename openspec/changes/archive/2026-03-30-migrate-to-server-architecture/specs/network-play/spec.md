# Capability: Network Play

## ADDED Requirements

### Requirement: Server-based matchmaking
The system SHALL provide a server-based matchmaking system for network play.

#### Scenario: Browse available rooms
- **WHEN** a logged-in user accesses the game lobby
- **THEN** the system displays a list of available rooms
- **AND** each room shows: room name, host username, creation time
- **AND** the list is refreshed every 5 seconds

#### Scenario: Create new room
- **WHEN** a user clicks "创建房间" and enters a room name
- **THEN** the system creates a new room
- **AND** displays a waiting screen with the room ID
- **AND** automatically starts the game when another player joins

#### Scenario: Join existing room
- **WHEN** a user clicks "加入" on a room in the list
- **THEN** the system attempts to join the room
- **AND** if successful, automatically starts the game
- **AND** if the room is full, displays an error message

### Requirement: Server-mediated message routing
The system SHALL route all game messages through the server.

#### Scenario: Send move to opponent
- **WHEN** a player makes a move on the board
- **THEN** the move is sent to the server via WebSocket
- **AND** the server forwards the move to the opponent
- **AND** the opponent's board updates

#### Scenario: Send restart request
- **WHEN** a player requests to restart the game
- **THEN** the request is sent to the server
- **AND** the server forwards the request to the opponent
- **AND** the opponent can accept or reject

## REMOVED Requirements

### Requirement: Direct IP connection
**Reason**: Replaced by server-based room system which eliminates NAT traversal issues and manual IP exchange.

**Migration**: Users must use the room lobby system instead of entering IP addresses. The server automatically handles connection establishment between players.

#### Scenario: No longer supported
- **WHEN** a user attempts to enter an IP address directly
- **THEN** this UI flow is no longer available
- **AND** users must use the room list to join games

### Requirement: Host game on local IP
**Reason**: Server-based architecture eliminates the need for players to host games on their local machines. The server manages all game rooms centrally.

**Migration**: Players create rooms on the server instead of hosting locally. The server handles all connection management.

#### Scenario: No local hosting
- **WHEN** a user wants to host a game
- **THEN** the user creates a room on the server
- **AND** does not need to open ports or configure NAT
- **AND** does not need to share their IP address

### Requirement: Join game by IP
**Reason**: Players no longer need to know or enter opponent's IP address. The server room system provides a user-friendly lobby for matchmaking.

**Migration**: Use the room list browser to find and join games. No manual IP entry is required.

#### Scenario: No IP input needed
- **WHEN** a user wants to play a network game
- **THEN** the user browses the room list
- **AND** clicks "加入" on a desired room
- **AND** does not need to enter any IP address

## MODIFIED Requirements

### Requirement: Network connection establishment
The system SHALL establish network connections via WebSocket to the server instead of direct TCP between players.

#### Scenario: Connect to server on game start
- **WHEN** a player joins a room or creates a room
- **THEN** the client automatically establishes a WebSocket connection to the server
- **AND** uses the format: `ws://server/game/{room_id}?token={session_token}`
- **AND** the connection is maintained for the duration of the game

#### Scenario: Authentication required for connection
- **WHEN** establishing a game connection
- **THEN** a valid session token from login is required
- **AND** the server validates the token before accepting the connection
- **AND** invalid tokens result in connection rejection

### Requirement: Game state synchronization
The system SHALL synchronize game state through server-mediated message passing.

#### Scenario: Real-time move synchronization
- **WHEN** either player makes a move
- **THEN** the move is sent to the server
- **AND** the server immediately forwards it to the opponent
- **AND** the opponent receives and displays the move

#### Scenario: Game start synchronization
- **WHEN** both players are connected to the room
- **THEN** the server sends a GameStart message to both players
- **AND** both players receive the message simultaneously
- **AND** the game begins on both clients

### Requirement: Connection error handling
The system SHALL handle connection errors with reconnection support instead of immediate game termination.

#### Scenario: Temporary disconnect handling
- **WHEN** a network connection is temporarily lost
- **THEN** the client displays a reconnection dialog
- **AND** automatically attempts to reconnect every 5 seconds for up to 30 seconds
- **AND** if successful, the game resumes

#### Scenario: Opponent disconnect notification
- **WHEN** the opponent's connection is lost
- **THEN** the player is notified: "对手已断开连接"
- **AND** a countdown timer shows the remaining reconnection window (30 seconds)
- **AND** the game board is disabled during this period

#### Scenario: Reconnection success
- **WHEN** a disconnected player successfully reconnects
- **THEN** both players are notified: "对手已重新连接"
- **AND** the game continues from the previous state

#### Scenario: Reconnection timeout
- **WHEN** 30 seconds pass without the opponent reconnecting
- **THEN** the connected player is declared the winner
- **AND** the game ends with reason "opponent_disconnected"

### Requirement: Player identification
The system SHALL use server-managed user identities instead of anonymous IP-based identification.

#### Scenario: Players identified by username
- **WHEN** viewing a room or playing a game
- **THEN** players are identified by their registered usernames
- **AND** usernames are displayed in the room list and game interface

#### Scenario: Persistent identity across sessions
- **WHEN** a user logs in and plays multiple games
- **THEN** the same username is used across all games
- **AND** opponents can recognize the player by username

### Requirement: Network game UI workflow
The system SHALL provide a different UI workflow for starting network games.

#### Scenario: Network game entry point
- **WHEN** a user selects "联机对战" from the main menu
- **THEN** the user is prompted to log in if not already logged in
- **AND** after login, the room lobby is displayed
- **AND** the user can create a room or join an existing room

#### Scenario: No direct IP input UI
- **WHEN** accessing network play features
- **THEN** there is no UI for entering IP addresses
- **AND** all matchmaking happens through the room lobby

### Requirement: Game protocol
The system SHALL use the same game message format (NetworkMessage) but transmit it via WebSocket instead of direct TCP.

#### Scenario: Message format unchanged
- **WHEN** sending game messages
- **THEN** the message format remains the same (Move, RestartRequest, etc.)
- **AND** messages are serialized as JSON
- **AND** but transmission is via WebSocket to the server instead of TCP directly

#### Scenario: Server as transparent relay
- **WHEN** game messages are transmitted
- **THEN** the server acts as a transparent relay
- **AND** does not modify message content
- **AND** does not validate game logic (trusts clients)
