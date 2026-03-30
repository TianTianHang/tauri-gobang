## ADDED Requirements

### Requirement: game_start broadcast on both WS connections

The server SHALL broadcast `game_start` to both players when both have established real WebSocket connections to the room.

#### Scenario: Host WS first, then client joins and connects WS

- **WHEN** host connects WS to a Waiting room (only host connected)
- **THEN** no `game_start` is sent (only 1 player)
- **WHEN** client joins via HTTP and then connects WS
- **THEN** server detects both players have real WS connections and broadcasts `game_start` with `black_player` and `white_player`

#### Scenario: Client HTTP join before host WS

- **WHEN** client HTTP-joins a room (room status becomes Playing)
- **THEN** room is in Playing state but no WS messages sent (host WS not connected yet)
- **WHEN** host connects WS
- **THEN** server detects only 1 real WS connection, no broadcast
- **WHEN** client connects WS
- **THEN** server detects both real WS connections and broadcasts `game_start`

#### Scenario: Detecting real vs dummy connections

- **WHEN** server checks `sender.is_closed()` for each player's mpsc channel
- **THEN** dummy channels (receiver dropped) return `true`, real WS channels return `false`
- **WHEN** all players have `is_closed() == false`
- **THEN** all real WS connections are established

### Requirement: Client transitions to waiting mode after HTTP join

The client frontend SHALL transition to waiting mode immediately after a successful HTTP room join.

#### Scenario: Client joins room successfully

- **WHEN** client calls `joinRoom()` and HTTP response is successful
- **THEN** frontend calls `setMode("waiting")` and connects WebSocket
- **THEN** client sees the waiting room UI

#### Scenario: Client join fails

- **WHEN** client calls `joinRoom()` and HTTP response is an error
- **THEN** frontend shows error alert, mode does not change

### Requirement: game_start handler prevents duplicate initialization

The frontend `game_start` handler SHALL only create a new game state if one does not already exist.

#### Scenario: game_state_sync arrives before game_start

- **WHEN** `game_state_sync` message initializes game state and transitions mode
- **THEN** `gameStateRef.current` is set
- **WHEN** `game_start` message arrives later
- **THEN** handler skips `new_game` call, only ensures mode is correct

#### Scenario: game_start arrives first (normal case)

- **WHEN** `game_start` message arrives and `gameStateRef.current` is null
- **THEN** handler calls `new_game`, sets game state, transitions mode
- **WHEN** `game_state_sync` arrives later with empty moves
- **THEN** sync handler skips (empty moves guard)
