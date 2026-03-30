## 1. Icon Components

- [ ] 1.1 Add `EyeIcon` component to `src/components/Icons.tsx` (Heroicons eye icon)
- [ ] 1.2 Add `EyeOffIcon` component to `src/components/Icons.tsx` (Heroicons eye-slash icon)
- [ ] 1.3 Add `LockIcon` component to `src/components/Icons.tsx` (Heroicons lock-closed icon)
- [ ] 1.4 Add `ExclamationCircleIcon` component to `src/components/Icons.tsx` (Heroicons exclamation-circle icon)
- [ ] 1.5 Export all new icons from `Icons.tsx`

## 2. State Management

- [ ] 2.1 Add `showPassword` state to `src/components/LoginScreen.tsx` for password visibility
- [ ] 2.2 Add `showConfirmPassword` state for confirm password field visibility
- [ ] 2.3 Refactor error state from single string to object: `errors: { username?: string, password?: string, confirmPassword?: string }`
- [ ] 2.4 Create `clearError(field)` function to clear specific field errors
- [ ] 2.5 Create `validateField(field, value)` function for field-level validation

## 3. Brand Identity

- [ ] 3.1 Import `BlackStoneIcon` into `src/components/LoginScreen.tsx`
- [ ] 3.2 Render brand icon container with BlackStoneIcon (48x48px)
- [ ] 3.3 Add conditional title rendering: "欢迎回到五子棋" (login) / "创建五子棋账号" (register)
- [ ] 3.4 Add conditional subtitle rendering: "登录以开始对战" (login) / "加入全球玩家社区" (register)

## 4. Input Field Layout

- [ ] 4.1 Wrap each input with `.input-group` div
- [ ] 4.2 Render `UserIcon` (18x18px) inside username input group
- [ ] 4.3 Render `LockIcon` (18x18px) inside password input group
- [ ] 4.4 Render `LockIcon` (18x18px) inside confirm password input group
- [ ] 4.5 Update input placeholder text to be action-oriented ("输入你的用户名", "输入你的密码", etc.)
- [ ] 4.6 Add `padding-left: 40px` to all input fields for icon spacing

## 5. Password Visibility Toggle

- [ ] 5.1 Add toggle button after password input field
- [ ] 5.2 Render `EyeOffIcon` / `EyeIcon` based on `showPassword` state
- [ ] 5.3 Wire onClick handler to toggle `showPassword` state
- [ ] 5.4 Set input type to `"text"` when `showPassword` is true, `"password"` when false
- [ ] 5.5 Repeat steps 5.1-5.4 for confirm password field with `showConfirmPassword` state
- [ ] 5.6 Ensure toggle buttons have `type="button"` to prevent form submission

## 6. Inline Error Messages

- [ ] 6.1 Remove global error rendering from top of form
- [ ] 6.2 Add inline error rendering below username input (when `errors.username` exists)
- [ ] 6.3 Add inline error rendering below password input (when `errors.password` exists)
- [ ] 6.4 Add inline error rendering below confirm password input (when `errors.confirmPassword` exists)
- [ ] 6.5 Render `ExclamationCircleIcon` (16x16px) before error text
- [ ] 6.6 Update `handleSubmit` to set field-level errors instead of global error
- [ ] 6.7 Add `onChange` handlers to clear errors when user starts typing

## 7. Mode Switch Button

- [ ] 7.1 Change `.toggle-mode-btn` from text link to button with dashed border
- [ ] 7.2 Update button text: "立即注册" (login mode) / "去登录" (register mode)
- [ ] 7.3 Add hover effect: background changes to `--bg-secondary`, border to `--accent`
- [ ] 7.4 Ensure button has `type="button"` to prevent form submission

## 8. Secondary Action Button

- [ ] 8.1 Update "返回" button to use secondary outlined style
- [ ] 8.2 Set background to transparent
- [ ] 8.3 Add 1px solid border using `--border` color
- [ ] 8.4 Set text color to `--text-secondary`

## 9. CSS Styling

- [ ] 9.1 Add `.login-brand` style class to `src/components/LoginScreen.css` with center alignment
- [ ] 9.2 Add `.brand-icon` style class (48x48px, `--accent` color)
- [ ] 9.3 Add `.login-title` style class (24px, bold, center, margin-bottom: 8px)
- [ ] 9.4 Add `.login-subtitle` style class (14px, center, `--text-secondary`, margin-bottom: 24px)
- [ ] 9.5 Add `.input-group` style class with `position: relative`
- [ ] 9.6 Add `.input-icon` style class (absolute, left: 12px, top: 50%, translate-y: -50%, 18x18px)
- [ ] 9.7 Add `.toggle-password` style class (absolute, right: 12px, transparent, no border, cursor: pointer)
- [ ] 9.8 Add `.field-error` style class (flex, align-center, gap: 6px, `--status-disconnected` color, 13px)
- [ ] 9.9 Add `.toggle-mode-action` style class (dashed border, transparent background, `--accent` text, hover effects)
- [ ] 9.10 Add `.btn-secondary` style class for "返回" button (outlined style)

## 10. Mode Switch Animation

- [ ] 10.1 Add CSS transition to confirm password field (opacity, max-height)
- [ ] 10.2 Add `.hidden` class to hide confirm password field (opacity: 0, max-height: 0, overflow: hidden)
- [ ] 10.3 Apply conditional `.hidden` class to confirm password field based on `isRegister` state
- [ ] 10.4 Set transition duration to 200ms with ease-in-out timing
- [ ] 10.5 Add `@media (prefers-reduced-motion: reduce)` to disable transitions when requested

## 11. Responsive Design

- [ ] 11.1 Test login layout on mobile (375px width)
- [ ] 11.2 Test login layout on tablet (768px width)
- [ ] 11.3 Test login layout on desktop (1024px+ width)
- [ ] 11.4 Verify icons don't cause input field overflow on mobile
- [ ] 11.5 Adjust input padding if text overlaps with icons on small screens

## 12. Testing & Verification

- [ ] 12.1 Visual test: Verify brand icon displays correctly in gold color
- [ ] 12.2 Visual test: Verify title and subtitle text displays correctly
- [ ] 12.3 Visual test: Verify input icons (UserIcon, LockIcon) display inside input fields
- [ ] 12.4 Functional test: Click password toggle button, verify password shows/hides
- [ ] 12.5 Functional test: Submit empty form, verify inline errors appear below fields
- [ ] 12.6 Functional test: Start typing in field with error, verify error clears
- [ ] 12.7 Interaction test: Click mode toggle button, verify smooth animation between login/register
- [ ] 12.8 Interaction test: Verify confirm password field fades in/out when switching modes
- [ ] 12.9 Interaction test: Hover over mode toggle button, verify border color change
- [ ] 12.10 Accessibility test: Tab through form, verify focus states visible on inputs and buttons
- [ ] 12.11 Accessibility test: Verify error messages have role="alert" attribute
- [ ] 12.12 Accessibility test: Verify toggle buttons have aria-label when icons only
- [ ] 12.13 Cross-browser test: Open login in Chrome, Firefox, Safari (if available), verify consistent rendering
- [ ] 12.14 Form validation test: Enter username < 3 chars, verify error shows
- [ ] 12.15 Form validation test: Enter password < 6 chars, verify error shows
- [ ] 12.16 Form validation test: Enter mismatched passwords in register mode, verify error shows
- [ ] 12.17 Integration test: Complete login flow with valid credentials, verify success
- [ ] 12.18 Integration test: Complete registration flow with valid data, verify success
- [ ] 12.19 Run `pnpm build` to verify TypeScript compilation passes
- [ ] 12.20 Run `pnpm tauri dev` to verify app launches and login/register UI displays correctly

## 13. Documentation (Optional)

- [ ] 13.1 Update CHANGELOG.md (if project maintains one)
- [ ] 13.2 Add screenshots of improved login/register UI to documentation (if applicable)
