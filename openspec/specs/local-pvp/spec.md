## ADDED Requirements

### Requirement: Entry to local PvP mode
The system SHALL provide a "本地对战" (Local Battle) button on the main menu that allows users to enter local two-player mode.

#### Scenario: User enters local PvP from main menu
- **WHEN** user clicks the "本地对战" button on the main menu
- **THEN** system initializes a new game with `mode` set to `"local_pvp"`
- **AND** system displays the game board
- **AND** system sets the starting player to Black

#### Scenario: Main menu displays three buttons
- **WHEN** user views the main menu
- **THEN** system displays three game mode buttons in order: "人机对战", "本地对战", "联机对战"
- **AND** each button displays an icon, title, and description

---

### Requirement: Local PvP turn management
The system SHALL support two players taking turns on the same screen, with Black moving first and players alternating after each valid move.

#### Scenario: Black makes the first move
- **WHEN** game starts in local_pvp mode
- **THEN** Black can place a stone on any empty intersection
- **AND** after Black places a stone, the turn passes to White

#### Scenario: Players alternate turns
- **WHEN** Black places a stone
- **THEN** the current_player changes to White
- **AND** White can place a stone on any empty intersection
- **AND** after White places a stone, the turn passes back to Black

#### Scenario: Opponent player cannot move during turn
- **WHEN** it is Black's turn
- **THEN** White cannot place a stone
- **AND** the board does not respond to clicks when it's not the current player's turn

---

### Requirement: Local PvP status display
The system SHALL display the current player's turn in the status bar during local PvP games.

#### Scenario: Status bar shows current player
- **WHEN** game is in local_pvp mode
- **THEN** status bar displays "⚫ 黑方回合" when Black's turn
- **AND** status bar displays "⚪ 白方回合" when White's turn
- **AND** status bar does NOT display AI thinking status
- **AND** status bar does NOT display opponent name

---

### Requirement: Local PvP menu adaptation
The system SHALL adapt the in-game menu for local PvP mode by hiding irrelevant options.

#### Scenario: Menu drawer hides AI-specific options
- **WHEN** user opens menu drawer during local_pvp game
- **THEN** menu does NOT display difficulty selection
- **AND** menu does NOT display "认输" (Surrender) option
- **AND** menu still displays "悔棋" (Undo) and "新游戏" (New Game) options

#### Scenario: Undo in local PvP
- **WHEN** user clicks "悔棋" during local_pvp game
- **THEN** system reverts the last two moves (Black's and White's last moves)
- **AND** turn passes back to the player who moved first in the undone pair

---

### Requirement: Local PvP game completion
The system SHALL detect win/draw conditions in local PvP mode following standard Gobang rules.

#### Scenario: Black wins by connecting five
- **WHEN** Black places a fifth stone in a row (horizontal, vertical, or diagonal)
- **THEN** game status changes to `BlackWins`
- **AND** system displays victory modal showing "⚫ 黑方获胜!"

#### Scenario: White wins by connecting five
- **WHEN** White places a fifth stone in a row
- **THEN** game status changes to `WhiteWins`
- **AND** system displays victory modal showing "⚪ 白方获胜!"

#### Scenario: Game ends after victory
- **WHEN** a player wins in local_pvp mode
- **THEN** victory modal shows game duration
- **AND** user can click "再来一局" to start a new local_pvp game
- **AND** user can click "返回菜单" to return to main menu

---

### Requirement: Local PvP new game and restart
The system SHALL allow starting new games or restarting without leaving local PvP mode.

#### Scenario: Start new local PvP game from menu
- **WHEN** user clicks "新游戏" in menu drawer during local_pvp game
- **THEN** system resets the board to empty state
- **AND** system sets current_player to Black
- **AND** game status remains in `local_pvp` mode
- **AND** game status changes to `Playing`

#### Scenario: Return to main menu from local PvP
- **WHEN** user clicks "返回菜单" in menu drawer or on victory modal
- **THEN** system clears game state
- **AND** system displays main menu
