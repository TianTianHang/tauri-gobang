## ADDED Requirements

### Requirement: Brand identity display
The system SHALL display brand elements at the top of login/register screen to create visual identity.

#### Scenario: Display brand icon and welcome text on login
- **WHEN** user views login screen
- **THEN** system displays BlackStoneIcon (48x48px) in gold color
- **AND** system displays title "欢迎回到五子棋"
- **AND** system displays subtitle "登录以开始对战"

#### Scenario: Display brand icon and title on registration
- **WHEN** user switches to registration mode
- **THEN** system displays BlackStoneIcon (48x48px) in gold color
- **AND** system displays title "创建五子棋账号"
- **AND** system displays subtitle "加入全球玩家社区"

---

### Requirement: Input field icon indicators
The system SHALL display contextual icons inside input fields to improve visual recognition.

#### Scenario: Display user icon in username field
- **WHEN** username input field is rendered
- **THEN** system displays UserIcon (18x18px) on the left side of input
- **AND** icon uses `--text-secondary` color
- **AND** icon is positioned 12px from left edge
- **AND** input field has padding-left: 40px to accommodate icon

#### Scenario: Display lock icon in password field
- **WHEN** password input field is rendered
- **THEN** system displays LockIcon (18x18px) on the left side of input
- **AND** icon uses `--text-secondary` color
- **AND** icon is positioned 12px from left edge
- **AND** input field has padding-left: 40px to accommodate icon

---

### Requirement: Password visibility toggle
The system SHALL provide a toggle button to show/hide password text.

#### Scenario: Show password by default, hide on toggle click
- **WHEN** password field is first rendered
- **THEN** input type is "password" (dots/bullets)
- **AND** system displays EyeOffIcon button on the right side
- **AND** when user clicks the toggle button, input type changes to "text"
- **AND** button icon changes to EyeIcon
- **AND** password becomes visible

#### Scenario: Hide password when toggle clicked again
- **WHEN** password is visible and user clicks toggle button
- **THEN** input type changes to "password"
- **AND** button icon changes to EyeOffIcon
- **AND** password becomes hidden (dots/bullets)

#### Scenario: Toggle button styling
- **WHEN** password visibility toggle is displayed
- **THEN** button is positioned 12px from right edge
- **AND** button has transparent background
- **AND** button has no border
- **AND** button color is `--text-secondary`
- **AND** button changes to `--text-primary` on hover
- **AND** button has cursor: pointer

---

### Requirement: Inline error messages
The system SHALL display validation error messages inline below the relevant form fields instead of at the top of the form.

#### Scenario: Display inline error with icon
- **WHEN** validation fails (username length, password length, password mismatch)
- **THEN** system displays error message below the relevant field
- **AND** error message includes ExclamationCircleIcon (16x16px)
- **AND** error message uses `--status-disconnected` color (red)
- **AND** error message has font size 13px
- **AND** icon and text are horizontally aligned with 6px gap

#### Scenario: Remove inline error when user starts typing
- **WHEN** user starts typing in the field that has an error
- **THEN** system clears the inline error message
- **AND** system removes error styling from the input field

---

### Requirement: Friendly mode switch button
The system SHALL provide a button-style toggle between login and registration modes instead of a plain text link.

#### Scenario: Display "立即注册" button on login screen
- **WHEN** user is on login screen
- **THEN** system displays a button with text "立即注册"
- **AND** button has dashed border using `--border` color
- **AND** button text uses `--accent` color (gold)
- **AND** button has font weight 600
- **AND** button background is transparent
- **AND** button changes background to `--bg-secondary` on hover
- **AND** button border changes to `--accent` on hover

#### Scenario: Display "去登录" button on registration screen
- **WHEN** user switches to registration mode
- **THEN** system displays a button with text "去登录"
- **AND** button uses same styling as "立即注册" button

#### Scenario: Switch to registration mode when button clicked
- **WHEN** user clicks "立即注册" button
- **THEN** system switches to registration mode
- **AND** form title changes to "创建五子棋账号"
- **AND** "确认密码" field becomes visible
- **AND** button text changes to "去登录"

---

### Requirement: Improved placeholder text
The system SHALL display friendly, action-oriented placeholder text in input fields.

#### Scenario: Username field placeholder
- **WHEN** username field is displayed
- **THEN** placeholder text is "输入你的用户名" instead of "3-20 个字符"

#### Scenario: Password field placeholder on login
- **WHEN** password field is displayed in login mode
- **THEN** placeholder text is "输入你的密码"

#### Scenario: Password field placeholder on registration
- **WHEN** password field is displayed in registration mode
- **THEN** placeholder text is "至少 6 个字符"

#### Scenario: Confirm password field placeholder
- **WHEN** confirm password field is displayed
- **THEN** placeholder text is "再次输入密码"

---

### Requirement: Secondary action button styling
The system SHALL style the "返回" (Back) button as a secondary action with outlined style.

#### Scenario: Display back button as secondary style
- **WHEN** login/register form is displayed
- **THEN** "返回" button has outlined style
- **AND** button background is transparent
- **AND** button has 1px solid border using `--border` color
- **AND** button text color is `--text-secondary`
- **AND** button has smaller padding than primary action button

---

### Requirement: Mode transition animation
The system SHALL animate the transition between login and registration modes for smooth user experience.

#### Scenario: Animate height change when switching modes
- **WHEN** user clicks mode toggle button
- **THEN** form container smoothly animates height change
- **AND** "确认密码" field fades in when switching to registration
- **AND** "确认密码" field fades out when switching to login
- **AND** animation duration is 200-300ms
- **AND** animation uses ease-in-out timing function

#### Scenario: Respect reduced motion preference
- **WHEN** user has `prefers-reduced-motion: reduce` enabled
- **THEN** all mode transition animations are disabled
- **AND** mode changes happen instantly without animation
