## 1. Type Definitions

- [ ] 1.1 Add `"local_pvp"` to `GameMode` type in `src/types/game.ts`
- [ ] 1.2 Run TypeScript type check to verify no type errors

## 2. Icon Component

- [ ] 2.1 Add `UserGroupIcon` component to `src/components/Icons.tsx` (Heroicons style SVG)
- [ ] 2.2 Export `UserGroupIcon` from `Icons.tsx`

## 3. Main Menu Updates

- [ ] 3.1 Add `onLocalPlay` prop to `MainMenu` interface in `src/components/MainMenu.tsx`
- [ ] 3.2 Add third button with `UserGroupIcon`, title "本地对战", description "双人同屏轮流下棋"
- [ ] 3.3 Wire up button onClick to call `onLocalPlay`
- [ ] 3.4 Verify 3-button layout renders correctly in `MainMenu.css`

## 4. App Routing Logic

- [ ] 4.1 Add `handleLocalPlay` callback in `src/App.tsx` that calls `startNewGame` and sets `mode` to `"local_pvp"`
- [ ] 4.2 Update `isOnline` check to exclude `local_pvp` mode
- [ ] 4.3 Update `myColor` logic to handle `local_pvp` (should be undefined, not used)
- [ ] 4.4 Update `isMyTurn` logic: in `local_pvp`, check if current player matches (always true for both players)

## 5. StatusBar Adaptation

- [ ] 5.1 Add conditional rendering in `src/components/StatusBar.tsx` for local_pvp mode
- [ ] 5.2 Display "⚫ 黑方回合" when `gameState.current_player === Cell.Black`
- [ ] 5.3 Display "⚪ 白方回合" when `gameState.current_player === Cell.White`
- [ ] 5.4 Ensure AI thinking status and opponent name are NOT shown in local_pvp mode

## 6. Menu Drawer Adaptation

- [ ] 6.1 Update `src/components/MenuDrawer.tsx` to hide difficulty selector when `mode === "local_pvp"`
- [ ] 6.2 Hide "认输" (Surrender) button when `mode === "local_pvp"`
- [ ] 6.3 Keep "悔棋" (Undo) and "新游戏" (New Game) buttons visible and functional
- [ ] 6.4 Ensure "返回菜单" (Back to Menu) works in local_pvp mode

## 7. Game Mechanics

- [ ] 7.1 Update `handleCellClick` in `src/App.tsx` to allow both players to click in local_pvp mode
- [ ] 7.2 Remove turn restriction in local_pvp (both Black and White can place stones when it's their turn)
- [ ] 7.3 Verify undo functionality reverts 2 moves in local_pvp mode

## 8. Game Completion Flow

- [ ] 8.1 Test win detection for Black in local_pvp mode
- [ ] 8.2 Test win detection for White in local_pvp mode
- [ ] 8.3 Verify "再来一局" starts new local_pvp game (not ai or online)
- [ ] 8.4 Verify "返回菜单" clears game state and returns to main menu

## 9. Testing & Verification

- [ ] 9.1 Manual test: Click "本地对战" from main menu, verify game starts with Black's turn
- [ ] 9.2 Manual test: Black places stone, verify turn passes to White
- [ ] 9.3 Manual test: White places stone, verify turn passes back to Black
- [ ] 9.4 Manual test: Win as Black, verify victory modal displays correctly
- [ ] 9.5 Manual test: Win as White, verify victory modal displays correctly
- [ ] 9.6 Manual test: Open menu drawer, verify difficulty and surrender are hidden
- [ ] 9.7 Manual test: Click "悔棋", verify two moves are reverted
- [ ] 9.8 Manual test: Click "新游戏", verify board resets and Black starts
- [ ] 9.9 Manual test: Click "返回菜单", verify main menu displays with 3 buttons
- [ ] 9.10 Run `pnpm build` to verify TypeScript compilation passes
- [ ] 9.11 Run `pnpm tauri dev` to verify app launches and local_pvp mode works end-to-end

## 10. Documentation (Optional)

- [ ] 10.1 Update README.md to mention local PvP mode (if applicable)
- [ ] 10.2 Add screenshots of local PvP gameplay to documentation (if applicable)
