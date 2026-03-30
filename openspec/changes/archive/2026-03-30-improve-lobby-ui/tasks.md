## 1. CSS Variables

- [x] 1.1 Add room status color variables to `src/App.css` (`--room-available`, `--room-full`, `--room-waiting`)
- [x] 1.2 Verify CSS variables work in both light and dark modes

## 2. Icon Components

- [x] 2.1 Add `StatusDotIcon` component to `src/components/Icons.tsx` with `color` prop
- [x] 2.2 Add `UserIcon` component to `src/components/Icons.tsx` (Heroicons user icon)
- [x] 2.3 Add `ClockIcon` component to `src/components/Icons.tsx` (Heroicons clock icon)
- [x] 2.4 Add `PlusIcon` component to `src/components/Icons.tsx` (Heroicons plus icon)
- [x] 2.5 Export all new icons from `Icons.tsx`

## 3. Utility Functions

- [x] 3.1 Add `formatRelativeTime` function to `src/components/RoomList.tsx`
- [x] 3.2 Test `formatRelativeTime` with various timestamps (recent, old, very old)

## 4. RoomList Component - Logic

- [x] 4.1 Import new icons into `src/components/RoomList.tsx`
- [x] 4.2 Add status determination logic (available/full/waiting) based on room data
- [x] 4.3 Render status indicator (StatusDotIcon) next to room name
- [x] 4.4 Render player count (e.g., "1/2") next to status indicator
- [x] 4.5 Replace absolute time with relative time using `formatRelativeTime`
- [x] 4.6 Add `UserIcon` before host username
- [x] 4.7 Add `ClockIcon` before time display
- [x] 4.8 Update empty state to show illustration (🏠) and friendly messages
- [x] 4.9 Update create room dialog placeholder to "给你的房间起个名字" or "来一局五子棋吧"
- [x] 4.10 Add helper text below room name input: "💡 给房间起个有趣的名字"

## 5. RoomList Component - Actions

- [x] 5.1 Update "创建房间" button to use primary style (full width, accent background)
- [x] 5.2 Add `PlusIcon` to "创建房间" button before text
- [x] 5.3 Update "刷新" and "退出登录" buttons to use secondary outlined style
- [x] 5.4 Add `RefreshIcon` to "刷新" button
- [x] 5.5 Ensure button layout works well on mobile (stack vertically)

## 6. RoomList Styles

- [x] 6.1 Add `.status-dot` style class to `src/components/RoomList.css`
- [x] 6.2 Add `.player-count` style class with color inheritance from status
- [x] 6.3 Add `.room-icon` style class for UserIcon and ClockIcon (size 14-16px, text-secondary color)
- [x] 6.4 Update `.btn-primary` in lobby to use full width and larger padding
- [x] 6.5 Add `.btn-secondary` style class for Refresh and Logout buttons
- [x] 6.6 Update `.rooms-empty` style to center content and add spacing
- [x] 6.7 Add `.helper-text` style class for create room dialog helper text

## 7. Responsive Design

- [x] 7.1 Test room card layout on mobile (375px width)
- [x] 7.2 Test room card layout on tablet (768px width)
- [x] 7.3 Test room card layout on desktop (1024px+ width)
- [x] 7.4 Adjust padding/margins if cards look cramped on mobile
- [x] 7.5 Verify text truncation works for long room names

## 8. Testing & Verification

- [x] 8.1 Visual test: Verify room status indicators (🟢/🟠/🔴) display correctly
- [x] 8.2 Visual test: Verify player count displays next to status indicator
- [x] 8.3 Functional test: Verify relative time updates correctly (check multiple rooms with different ages)
- [x] 8.4 Visual test: Verify icons (UserIcon, ClockIcon, PlusIcon) display with correct size and color
- [x] 8.5 Visual test: Verify empty state shows illustration and friendly messages
- [x] 8.6 Visual test: Verify create room dialog has improved placeholder and helper text
- [x] 8.7 Visual test: Verify action buttons have clear visual hierarchy (primary vs secondary)
- [x] 8.8 Interaction test: Hover over room cards, verify background color change works
- [x] 8.9 Interaction test: Click "刷新" button, verify list updates
- [x] 8.10 Cross-browser test: Open lobby in Chrome, Firefox, Safari (if available), verify consistent rendering
- [x] 8.11 Accessibility test: Tab through buttons, verify focus states are visible
- [x] 8.12 Run `pnpm build` to verify TypeScript compilation passes
- [x] 8.13 Run `pnpm tauri dev` to verify app launches and lobby displays correctly

## 9. Documentation (Optional)

- [x] 9.1 Update CHANGELOG.md (if project maintains one)
- [x] 9.2 Add screenshots of improved lobby UI to documentation (if applicable)
