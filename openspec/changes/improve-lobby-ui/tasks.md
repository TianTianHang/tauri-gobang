## 1. CSS Variables

- [ ] 1.1 Add room status color variables to `src/App.css` (`--room-available`, `--room-full`, `--room-waiting`)
- [ ] 1.2 Verify CSS variables work in both light and dark modes

## 2. Icon Components

- [ ] 2.1 Add `StatusDotIcon` component to `src/components/Icons.tsx` with `color` prop
- [ ] 2.2 Add `UserIcon` component to `src/components/Icons.tsx` (Heroicons user icon)
- [ ] 2.3 Add `ClockIcon` component to `src/components/Icons.tsx` (Heroicons clock icon)
- [ ] 2.4 Add `PlusIcon` component to `src/components/Icons.tsx` (Heroicons plus icon)
- [ ] 2.5 Export all new icons from `Icons.tsx`

## 3. Utility Functions

- [ ] 3.1 Add `formatRelativeTime` function to `src/components/RoomList.tsx`
- [ ] 3.2 Test `formatRelativeTime` with various timestamps (recent, old, very old)

## 4. RoomList Component - Logic

- [ ] 4.1 Import new icons into `src/components/RoomList.tsx`
- [ ] 4.2 Add status determination logic (available/full/waiting) based on room data
- [ ] 4.3 Render status indicator (StatusDotIcon) next to room name
- [ ] 4.4 Render player count (e.g., "1/2") next to status indicator
- [ ] 4.5 Replace absolute time with relative time using `formatRelativeTime`
- [ ] 4.6 Add `UserIcon` before host username
- [ ] 4.7 Add `ClockIcon` before time display
- [ ] 4.8 Update empty state to show illustration (🏠) and friendly messages
- [ ] 4.9 Update create room dialog placeholder to "给你的房间起个名字" or "来一局五子棋吧"
- [ ] 4.10 Add helper text below room name input: "💡 给房间起个有趣的名字"

## 5. RoomList Component - Actions

- [ ] 5.1 Update "创建房间" button to use primary style (full width, accent background)
- [ ] 5.2 Add `PlusIcon` to "创建房间" button before text
- [ ] 5.3 Update "刷新" and "退出登录" buttons to use secondary outlined style
- [ ] 5.4 Add `RefreshIcon` to "刷新" button
- [ ] 5.5 Ensure button layout works well on mobile (stack vertically)

## 6. RoomList Styles

- [ ] 6.1 Add `.status-dot` style class to `src/components/RoomList.css`
- [ ] 6.2 Add `.player-count` style class with color inheritance from status
- [ ] 6.3 Add `.room-icon` style class for UserIcon and ClockIcon (size 14-16px, text-secondary color)
- [ ] 6.4 Update `.btn-primary` in lobby to use full width and larger padding
- [ ] 6.5 Add `.btn-secondary` style class for Refresh and Logout buttons
- [ ] 6.6 Update `.rooms-empty` style to center content and add spacing
- [ ] 6.7 Add `.helper-text` style class for create room dialog helper text

## 7. Responsive Design

- [ ] 7.1 Test room card layout on mobile (375px width)
- [ ] 7.2 Test room card layout on tablet (768px width)
- [ ] 7.3 Test room card layout on desktop (1024px+ width)
- [ ] 7.4 Adjust padding/margins if cards look cramped on mobile
- [ ] 7.5 Verify text truncation works for long room names

## 8. Testing & Verification

- [ ] 8.1 Visual test: Verify room status indicators (🟢/🟠/🔴) display correctly
- [ ] 8.2 Visual test: Verify player count displays next to status indicator
- [ ] 8.3 Functional test: Verify relative time updates correctly (check multiple rooms with different ages)
- [ ] 8.4 Visual test: Verify icons (UserIcon, ClockIcon, PlusIcon) display with correct size and color
- [ ] 8.5 Visual test: Verify empty state shows illustration and friendly messages
- [ ] 8.6 Visual test: Verify create room dialog has improved placeholder and helper text
- [ ] 8.7 Visual test: Verify action buttons have clear visual hierarchy (primary vs secondary)
- [ ] 8.8 Interaction test: Hover over room cards, verify background color change works
- [ ] 8.9 Interaction test: Click "刷新" button, verify list updates
- [ ] 8.10 Cross-browser test: Open lobby in Chrome, Firefox, Safari (if available), verify consistent rendering
- [ ] 8.11 Accessibility test: Tab through buttons, verify focus states are visible
- [ ] 8.12 Run `pnpm build` to verify TypeScript compilation passes
- [ ] 8.13 Run `pnpm tauri dev` to verify app launches and lobby displays correctly

## 9. Documentation (Optional)

- [ ] 9.1 Update CHANGELOG.md (if project maintains one)
- [ ] 9.2 Add screenshots of improved lobby UI to documentation (if applicable)
