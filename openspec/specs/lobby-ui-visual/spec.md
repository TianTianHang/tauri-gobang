## ADDED Requirements

### Requirement: Room card status indication
The system SHALL display a visual status indicator on each room card to show availability.

#### Scenario: Display available room with green indicator
- **WHEN** a room has available player slots
- **THEN** system displays a green dot (🟢) next to room name
- **AND** system uses CSS variable `--room-available: #22C55E` for the indicator color

#### Scenario: Display full room with red indicator
- **WHEN** a room has no available player slots (2/2 players)
- **THEN** system displays a red dot (🔴) next to room name
- **AND** system uses CSS variable `--room-full: #EF4444` for the indicator color

#### Scenario: Display waiting room with orange indicator
- **WHEN** a room is in waiting state (not yet started or paused)
- **THEN** system displays an orange dot (🟠) next to room name
- **AND** system uses CSS variable `--room-waiting: #F59E0B` for the indicator color

---

### Requirement: Room player count display
The system SHALL display the current number of players in each room.

#### Scenario: Display player count on room card
- **WHEN** room list is displayed
- **THEN** each room card shows player count in format "X/2"
- **AND** the count is positioned next to the status indicator
- **AND** the count uses the same color as the status indicator

---

### Requirement: Relative time display
The system SHALL display room creation time using relative time format instead of absolute timestamps.

#### Scenario: Display "X minutes ago" for recent rooms
- **WHEN** a room was created less than 60 minutes ago
- **THEN** system displays time as "X分钟前" (e.g., "3分钟前")

#### Scenario: Display "X hours ago" for older rooms
- **WHEN** a room was created less than 24 hours ago
- **THEN** system displays time as "X小时前" (e.g., "2小时前")

#### Scenario: Display absolute date for very old rooms
- **WHEN** a room was created more than 24 hours ago
- **THEN** system displays absolute date in format "MM-DD HH:mm"

---

### Requirement: Visual icons on room cards
The system SHALL display small icons next to room information to improve visual hierarchy.

#### Scenario: Display user icon before host name
- **WHEN** room card displays host information
- **THEN** system shows UserIcon before the host username
- **AND** the icon has size 16x16px
- **AND** the icon uses `--text-secondary` color

#### Scenario: Display clock icon before time
- **WHEN** room card displays creation time
- **THEN** system shows ClockIcon before the time text
- **AND** the icon has size 14x14px
- **AND** the icon uses `--text-secondary` color

---

### Requirement: Empty state visual improvement
The system SHALL display an improved empty state with illustration and friendly messaging when no rooms are available.

#### Scenario: Display empty state when room list is empty
- **WHEN** room list returns zero rooms
- **THEN** system displays an illustration icon (🏠 building emoji or SVG icon)
- **AND** system shows friendly text: "大厅空空如也..."
- **AND** system shows secondary text: "创建一个房间等待朋友吧"
- **AND** the empty state has 40px top/bottom padding
- **AND** text color uses `--text-secondary`

---

### Requirement: Action button visual hierarchy
The system SHALL visually distinguish primary actions from secondary actions in the lobby.

#### Scenario: Highlight primary "Create Room" action
- **WHEN** lobby action buttons are displayed
- **THEN** "创建房间" button uses full-width style with `--accent` background
- **AND** button includes PlusIcon before text
- **AND** button has larger padding (14px vertical)

#### Scenario: Style secondary actions as outlined buttons
- **WHEN** secondary actions (Refresh, Logout) are displayed
- **THEN** these buttons use outlined style with `--border` border
- **AND** button background is transparent or `--bg-secondary`
- **AND** buttons include icons (RefreshIcon, logout icon)

---

### Requirement: Create room dialog placeholder improvement
The system SHALL display helpful placeholder text in the create room dialog input field.

#### Scenario: Show friendly placeholder in room name input
- **WHEN** user opens create room dialog
- **THEN** input field shows placeholder: "给你的房间起个名字"
- **OR** shows more engaging placeholder: "来一局五子棋吧"

#### Scenario: Show helper text below input
- **WHEN** create room dialog is displayed
- **THEN** system displays helper text below input: "💡 给房间起个有趣的名字"
- **AND** helper text uses `--text-secondary` color
- **AND** helper text has font size 13px
