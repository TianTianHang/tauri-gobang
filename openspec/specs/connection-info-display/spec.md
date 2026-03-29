## ADDED Requirements

### Requirement: Connection info persistent display
The system SHALL display the connection information (IP address, port, and connection status) persistently in the StatusBar during online gameplay, ensuring the user can always see their connection details without navigating away from the game interface.

#### Scenario: Host mode shows room info
- **WHEN** user creates an online game room and enters the game interface
- **THEN** StatusBar SHALL display "房间: <IP>:<PORT>" with a green connection indicator dot
- **AND** the IP address SHALL be the local network IP (e.g., 192.168.1.100)
- **AND** the port SHALL match the port used when creating the room

#### Scenario: Client mode shows connected server
- **WHEN** user joins an online game and enters the game interface
- **THEN** StatusBar SHALL display "已连接: <IP>:<PORT>" with a green connection indicator dot
- **AND** the IP address SHALL be the server's IP address
- **AND** the port SHALL match the port used when joining

### Requirement: Connection status indicator
The system SHALL provide a visual connection status indicator that changes color based on the current connection state (connected, connecting, disconnected).

#### Scenario: Connected state shows green dot
- **WHEN** the network connection is active and stable
- **THEN** the status indicator SHALL be a green dot (8px diameter)
- **AND** the dot SHALL have a subtle glow effect (box-shadow)

#### Scenario: Connecting state shows yellow dot
- **WHEN** the system is establishing a network connection
- **THEN** the status indicator SHALL be a yellow dot
- **AND** the dot SHALL have a pulsing animation

#### Scenario: Disconnected state shows red dot
- **WHEN** the network connection is lost or fails
- **THEN** the status indicator SHALL be a red dot
- **AND** the user SHALL be notified of disconnection (existing behavior)

### Requirement: IP address copy to clipboard
The system SHALL allow the user to copy the IP address and port to the clipboard by clicking on the connection information, providing visual feedback when the copy succeeds.

#### Scenario: Successful copy shows confirmation
- **WHEN** user clicks on the IP address display in StatusBar
- **THEN** the system SHALL copy "<IP>:<PORT>" to the clipboard
- **AND** display "已复制！" feedback text
- **AND** the feedback SHALL disappear after 2 seconds
- **AND** the IP display SHALL have a visual indication that it is clickable (cursor: pointer)

#### Scenario: Copy failure is handled gracefully
- **WHEN** the clipboard copy operation fails (e.g., permission denied)
- **THEN** the system SHALL log the error to console
- **AND** SHALL NOT interrupt the user's gameplay
- **AND** SHALL NOT display an error message (silent failure)

### Requirement: Responsive connection info display
The system SHALL adapt the connection information display for different screen sizes, ensuring readability on mobile devices while maintaining usability on desktop.

#### Scenario: Desktop shows full connection info
- **WHEN** the screen width is 768px or greater
- **THEN** StatusBar SHALL display the full connection text: "房间: 192.168.1.100:5555" or "已连接: 192.168.1.100:5555"

#### Scenario: Mobile shows abbreviated IP
- **WHEN** the screen width is less than 768px
- **THEN** StatusBar MAY abbreviate the IP address (e.g., "房间: 192.168...:5555")
- **OR** hide the "房间:"/"已连接:" prefix to save space
- **AND** the connection status dot SHALL remain visible

#### Scenario: Touch target size compliance
- **WHEN** the user interacts with the IP copy button on a mobile device
- **THEN** the clickable area SHALL be at least 44x44px (WCAG AA standard)
- **AND** the hit area MAY extend beyond the visible text for better usability

### Requirement: Dark mode compatibility
The system SHALL display connection information correctly in both light and dark modes, maintaining proper contrast and readability.

#### Scenario: Light mode visibility
- **WHEN** the system is in light mode
- **THEN** the IP address text SHALL have a contrast ratio of at least 4.5:1 against the background
- **AND** the connection status dot colors SHALL be clearly visible

#### Scenario: Dark mode visibility
- **WHEN** the system is in dark mode (prefers-color-scheme: dark)
- **THEN** the IP address text SHALL have a contrast ratio of at least 4.5:1 against the background
- **AND** the connection status dot colors SHALL be adjusted for dark backgrounds if necessary

### Requirement: Accessibility support
The system SHALL provide proper accessibility attributes for the connection information display, ensuring keyboard and screen reader users can access and understand the connection status.

#### Scenario: Keyboard navigation
- **WHEN** the user navigates using the Tab key
- **THEN** the IP copy button SHALL be focusable
- **AND** SHALL have a visible focus indicator (3-4px outline)

#### Scenario: Screen reader announcement
- **WHEN** a screen reader user focuses on the connection information
- **THEN** the system SHALL provide an aria-label describing the connection status (e.g., "房间: 192.168.1.100:5555, 点击复制")

#### Scenario: Connection status announcement
- **WHEN** the connection status changes (e.g., from connected to disconnected)
- **THEN** the system SHOULD announce the status change to screen reader users (if technically feasible without polling)
