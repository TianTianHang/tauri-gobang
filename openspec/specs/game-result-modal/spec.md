## ADDED Requirements

### Requirement: Game result modal overlay
The system SHALL display a full-screen modal overlay when the game ends (victory, defeat, or draw), providing clear visual feedback about the game result and next actions.

#### Scenario: Victory modal for black wins
- **WHEN** the game ends with BlackWins status and the user is playing as black
- **THEN** the system SHALL display a full-screen modal with gold theme
- **AND** the modal SHALL contain a large trophy icon (60px)
- **AND** display "你赢了！" text (36px, bold, gold color)
- **AND** show game statistics (e.g., "黑棋获胜，用时 3:24")
- **AND** provide "再来一局" and "返回菜单" buttons

#### Scenario: Defeat modal for opponent wins
- **WHEN** the game ends with WhiteWins status and the user is playing as black (or vice versa)
- **THEN** the system SHALL display a full-screen modal with gray/soft red theme
- **AND** the modal SHALL contain a large "loss" icon (60px, gray)
- **AND** display "你输了..." text (32px, bold, gray color)
- **AND** show encouraging text (e.g., "白棋获胜，再接再厉！")
- **AND** provide "再来一局" and "返回菜单" buttons

#### Scenario: Draw modal
- **WHEN** the game ends with Draw status
- **THEN** the system SHALL display a full-screen modal with neutral theme
- **AND** the modal SHALL contain a "handshake" or similar icon
- **AND** display "平局！" text
- **AND** show game statistics
- **AND** provide "再来一局" and "返回菜单" buttons

### Requirement: Modal glassmorphism visual design
The system SHALL apply a glassmorphism effect to the game result modal, using backdrop blur and semi-transparent backgrounds to create a modern, layered visual design.

#### Scenario: Glass effect in light mode
- **WHEN** the modal is displayed in light mode
- **THEN** the modal background SHALL have backdrop-filter: blur(20px)
- **AND** the overlay SHALL use rgba(255, 255, 255, 0.8) or higher opacity
- **AND** the modal card SHALL have a subtle border (1px solid rgba(255, 255, 255, 0.2))

#### Scenario: Glass effect in dark mode
- **WHEN** the modal is displayed in dark mode
- **THEN** the modal background SHALL have backdrop-filter: blur(20px)
- **AND** the overlay SHALL use rgba(0, 0, 0, 0.8) for better contrast
- **AND** the modal card SHALL maintain visibility with proper contrast

### Requirement: Modal animation and transition
The system SHALL animate the modal appearance with a fade-in and scale-up effect (300ms ease-out), creating a smooth and visually engaging transition that emphasizes the game result.

#### Scenario: Modal appearance animation
- **WHEN** the game ends and the modal appears
- **THEN** the modal SHALL fade in from opacity: 0 to opacity: 1
- **AND** the modal SHALL scale up from scale(0.95) to scale(1)
- **AND** the animation duration SHALL be 300ms
- **AND** the easing function SHALL be ease-out

#### Scenario: Reduced motion support
- **WHEN** the user has prefers-reduced-motion: reduce enabled
- **THEN** the modal SHALL appear instantly without animation
- **AND** all other animations SHALL be disabled

### Requirement: Modal action buttons
The system SHALL provide clear action buttons in the modal that allow the user to restart the game or return to the main menu, with proper hover/active states and accessibility support.

#### Scenario: Play again button
- **WHEN** the user clicks "再来一局" button
- **THEN** the system SHALL call the onNewGame handler
- **AND** the modal SHALL close
- **AND** a new game SHALL start immediately

#### Scenario: Return to menu button
- **WHEN** the user clicks "返回菜单" button
- **THEN** the system SHALL call the onBackToMenu handler
- **AND** the modal SHALL close
- **AND** the user SHALL be returned to the main menu
- **AND** the network connection SHALL be disconnected (if in online mode)

#### Scenario: Button hover and active states
- **WHEN** the user hovers over a modal button
- **THEN** the button SHALL have a visual feedback (opacity: 0.85)
- **AND** the cursor SHALL change to pointer
- **AND** when clicked, the button SHALL have an active state (transform: scale(0.98))

### Requirement: Modal game statistics display
The system SHALL display relevant game statistics in the modal, providing the user with information about the completed game (e.g., duration, move count).

#### Scenario: Show game duration
- **WHEN** the modal is displayed
- **THEN** the system SHALL show the total game time in "MM:SS" format
- **AND** the time SHALL be calculated from the first move to the winning move

#### Scenario: Show move count
- **WHEN** the modal is displayed
- **THEN** the system SHALL show the total number of moves played
- **AND** the count SHALL match the game history length

#### Scenario: Show winner color (for non-online modes)
- **WHEN** the modal is displayed in AI or local PvP mode
- **THEN** the system SHALL indicate which color won (e.g., "黑棋获胜" or "白棋获胜")
- **AND** the text SHALL use the appropriate stone symbol (○ or ●)

### Requirement: Modal responsiveness
The system SHALL adapt the modal layout for different screen sizes, ensuring proper display and usability on both desktop and mobile devices.

#### Scenario: Desktop modal layout
- **WHEN** the screen width is 768px or greater
- **THEN** the modal card SHALL be centered with max-width: 480px
- **AND** the padding SHALL be 40px
- **AND** buttons SHALL be displayed side-by-side

#### Scenario: Mobile modal layout
- **WHEN** the screen width is less than 768px
- **THEN** the modal card SHALL use max-width: 90% of screen width
- **AND** the padding SHALL be 24px
- **AND** buttons SHALL be stacked vertically
- **AND** button touch targets SHALL be at least 44x44px

#### Scenario: Icon size scaling
- **WHEN** the modal is displayed on a mobile device
- **THEN** the trophy/loss icon SHALL be scaled to 48px (from 60px on desktop)
- **AND** text sizes SHALL be proportionally reduced

### Requirement: Modal accessibility
The system SHALL ensure the game result modal is fully accessible, supporting keyboard navigation, screen readers, and proper focus management.

#### Scenario: Focus trap in modal
- **WHEN** the modal is displayed
- **THEN** keyboard focus SHALL move to the first actionable element (e.g., "再来一局" button)
- **AND** Tab key SHALL cycle focus within the modal only
- **AND** focus SHALL NOT return to the game board until modal is closed

#### Scenario: Escape key closes modal
- **WHEN** the user presses the Escape key while modal is open
- **THEN** the modal SHALL close
- **AND** focus SHALL return to the game board

#### Scenario: Screen reader announcement
- **WHEN** the modal appears
- **THEN** the screen reader SHALL announce the game result (e.g., "游戏结束，你赢了！")
- **AND** the modal SHALL have role="dialog" or role="alertdialog"
- **AND** the modal SHALL have an aria-label describing the result

#### Scenario: Button accessibility
- **WHEN** a screen reader user navigates to a modal button
- **THEN** each button SHALL have an aria-label describing its action
- **AND** the button SHALL announce its current state (if applicable)

### Requirement: Modal z-index layering
The system SHALL ensure the modal appears above all other UI elements, including the MenuDrawer, without conflicting with existing overlays.

#### Scenario: Modal above all UI
- **WHEN** the modal is displayed
- **THEN** the modal overlay SHALL have a z-index higher than MenuDrawer
- **AND** the modal content SHALL be clearly visible without obstruction
- **AND** existing UI elements (StatusBar, GameInfo, GameBoard) SHALL be visually dimmed by the overlay

### Requirement: Modal contrast and readability
The system SHALL maintain proper contrast ratios for all text and elements in the modal, ensuring readability in both light and dark modes.

#### Scenario: Victory modal contrast
- **WHEN** the victory modal is displayed
- **THEN** all text SHALL have a contrast ratio of at least 4.5:1 against the gold gradient background
- **AND** icon SHALL be clearly visible

#### Scenario: Defeat modal contrast
- **WHEN** the defeat modal is displayed
- **THEN** all text SHALL have a contrast ratio of at least 4.5:1 against the gray background
- **AND** icon SHALL be clearly visible
