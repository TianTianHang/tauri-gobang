# Mobile Game UI Specification

移动优先的五子棋游戏界面系统，提供沉浸式游戏体验和优化的触控交互。

## ADDED Requirements

### Requirement: Dynamic Board Sizing

棋盘必须根据设备屏幕尺寸动态调整大小，最大化显示区域同时保持可玩性。

#### Scenario: Mobile Portrait Mode
- **WHEN** 应用在竖屏移动设备上运行（屏幕宽度 < 768px）
- **THEN** 棋盘应占据尽可能多的垂直空间，同时保持正方形比例
- **AND** 棋盘宽度不应超过 `window.innerWidth - 32px`（左右各 16px padding）
- **AND** 棋盘高度不应超过 `window.innerHeight - 140px`（减去状态栏和 padding）
- **AND** 最终尺寸取上述两个约束的较小值
- **AND** 单个格子（cell）尺寸应在 22-26px 范围内

#### Scenario: Desktop Mode
- **WHEN** 应用在桌面设备或宽屏设备上运行（屏幕宽度 ≥ 768px）
- **THEN** 棋盘固定最大尺寸为 600px
- **AND** 棋盘在屏幕中央显示
- **AND** 单个格子尺寸为 40px（保持现有体验）

#### Scenario: Orientation Change
- **WHEN** 设备旋转（横竖屏切换）
- **THEN** 棋盘应重新计算尺寸并重新渲染
- **AND** 重新计算应在 100ms 内完成（防抖处理）
- **AND** 游戏状态（棋盘内容）完全保留

#### Scenario: Window Resize
- **WHEN** 浏览器窗口尺寸变化
- **THEN** 棋盘应平滑调整尺寸（使用 CSS transition）
- **AND** 调整不应触发完整的游戏状态重置
- **AND** Canvas 分辨率应更新以保持清晰度（考虑 DPR）

---

### Requirement: Touch-Optimized Interaction

系统必须提供专为触控优化的交互体验，包括触摸预览、触觉反馈和准确的点击检测。

#### Scenario: Touch Preview
- **WHEN** 用户触摸棋盘上的某个位置（`touchstart` 事件）
- **THEN** 系统应显示半透明的棋子预览（opacity 0.35）
- **AND** 预览应跟随手指移动（`touchmove` 事件）
- **AND** 预览应在手指离开时消失（`touchend`）
- **AND** 预览应使用当前玩家的棋子颜色

#### Scenario: Touch Placement Confirmation
- **WHEN** 用户在棋盘有效位置完成点击操作
- **THEN** 系统应在该位置落子
- **AND** 提供轻微触觉反馈（`navigator.vibrate(10)`，持续 10ms）
- **AND** 棋子应以淡入动画出现（300ms ease-out）
- **AND** 最后一手位置应有红色标记（4px 红点）

#### Scenario: Invalid Touch Handling
- **WHEN** 用户触摸棋盘外的位置或已占用格子
- **THEN** 系统不应落子
- **AND** 不应提供触觉反馈
- **AND** 不应显示预览

#### Scenario: Touch Feedback
- **WHEN** 用户成功落子
- **THEN** 提供轻微触觉反馈（10ms 震动）
- **WHEN** 用户获胜
- **THEN** 提供成功震动模式（`[20, 50, 20]`）
- **WHEN** 用户打开菜单
- **THEN** 提供极轻微反馈（5ms 震动）
- **WHEN** 设备不支持震动 API
- **THEN** 静默失败，不影响其他功能

#### Scenario: Tap Delay Elimination
- **WHEN** 任何交互元素被点击
- **THEN** 不应有 300ms 延迟
- **AND** 使用 `touch-action: manipulation` CSS 属性
- **AND** 交互应立即响应（< 100ms）

---

### Requirement: Status Bar Display

顶部状态栏必须清晰显示当前游戏状态和关键信息，同时保持最小化。

#### Scenario: In-Game Status Display
- **WHEN** 游戏进行中
- **THEN** 状态栏显示 "● 黑棋落子" 或 "○ 白棋落子"
- **AND** 文本应居中对齐
- **AND** 字体大小为 16px，字重 600
- **AND** 使用 `var(--text-primary)` 颜色

#### Scenario: AI Thinking Indicator
- **WHEN** AI 正在思考下一步
- **THEN** 状态栏应显示脉冲动画的思考指示器（🤔 emoji 或圆点）
- **AND** 动画应每 1.5秒循环一次（1s 活跃，0.5s 淡出）
- **AND** 使用 `var(--accent)` 颜色（金色）

#### Scenario: Game Over Status
- **WHEN** 游戏结束（一方获胜或平局）
- **THEN** 状态栏显示结果文本：
  - "● 黑棋胜！" 或 "○ 白棋胜！" 或 "平局！"
- **AND** 联机模式下显示 "🎉 你赢了！" 或 "你输了..."
- **AND** AI 思考指示器应停止

#### Scenario: Status Bar Dimensions
- **WHEN** 在任何设备上
- **THEN** 状态栏高度固定为 44px
- **AND** 左右内边距为 16px（移动端）或 24px（桌面端）
- **AND** 底部有 1px 边框（`var(--border)`）
- **AND** 高度符合最小触控目标标准

---

### Requirement: Menu Drawer System

抽屉式菜单必须提供对所有游戏操作的访问，同时保持游戏界面的简洁性。

#### Scenario: Menu Trigger
- **WHEN** 用户点击右上角的菜单按钮（≡ 图标）
- **THEN** 菜单抽屉应从右侧滑入
- **AND** 滑入动画应为 300ms ease-out
- **AND** 背景应显示半透明遮罩（opacity 0.5）
- **AND** 提供触觉反馈（5ms 震动）
- **AND** 菜单按钮尺寸为 44x44px（符合 WCAG AAA）

#### Scenario: Menu Content Organization
- **WHEN** 菜单打开
- **THEN** 菜单应包含以下部分（从上到下）：
  1. 标题栏：游戏设置 + 关闭按钮（✕）
  2. 游戏操作：新游戏、悔棋（2列网格）、重新开始（联机模式）
  3. 难度选择：简单/中等/困难（AI 模式，单选按钮）
  4. 对局信息：当前步数、对局时长
  5. 返回主菜单按钮（危险操作，红色边框）
- **AND** 每个部分应有清晰的标题和分割线

#### Scenario: Menu Button Standards
- **WHEN** 菜单中的任何按钮显示
- **THEN** 按钮最小高度为 48px（超过触控最小标准）
- **AND** 按钮之间至少有 8px 间距
- **AND** 主要操作按钮（新游戏）使用金色背景（`var(--accent)`）
- **AND** 危险操作（返回主菜单）使用红色边框（`var(--danger)`）
- **AND** 禁用状态的按钮 opacity 为 0.4

#### Scenario: Menu Dismissal
- **WHEN** 用户点击半透明背景遮罩
- **THEN** 菜单应滑出屏幕（300ms ease-in）
- **AND** 遮罩应淡出
- **AND** 焦点应返回到棋盘
- **WHEN** 用户点击关闭按钮（✕）
- **THEN** 同上行为
- **WHEN** 用户按下 ESC 键
- **THEN** 同上行为

#### Scenario: Menu Responsive Width
- **WHEN** 在移动设备上（屏幕宽度 < 768px）
- **THEN** 菜单宽度为 320px 或屏幕宽度的 80%（取较小值）
- **WHEN** 在桌面设备上（屏幕宽度 ≥ 768px）
- **THEN** 菜单宽度为 400px

---

### Requirement: Visual Design Consistency

界面必须在深色和浅色模式下保持一致的视觉质量和可读性。

#### Scenario: Dark Mode Color Scheme
- **WHEN** 系统处于深色模式
- **THEN** 背景色应使用纯黑 `#000000`（OLED 优化）
- **AND** 次级背景使用 `#121212`
- **AND** 浮层/卡片使用 `#1E1E1E`
- **AND** 主文本使用 `#FFFFFF`（对比度 21:1，WCAG AAA）
- **AND** 次要文本使用 `#A0A0A0`（对比度 12.6:1，WCAG AAA）
- **AND** 边框使用 `#2A2A2A`
- **AND** 品牌强调色使用金色 `#D4AF37`

#### Scenario: Light Mode Color Scheme
- **WHEN** 系统处于浅色模式
- **THEN** 背景色应使用白色 `#FFFFFF`
- **AND** 次级背景使用 `#F5F5F5`
- **AND** 主文本使用深蓝黑 `#0C4A6E`（对比度 12.1:1，WCAG AAA）
- **AND** 次要文本使用 `#475569`（对比度 9.8:1，WCAG AAA）
- **AND** 边框使用 `#E5E5E5`
- **AND** 保持金色强调色 `#D4AF37`

#### Scenario: Board Appearance
- **WHEN** 棋盘渲染
- **THEN** 木纹背景应保持渐变效果（浅木色 `#DEB887` 到深木色 `#D2A96A`）
- **AND** 棋盘线使用深棕色 `#5C4033`
- **AND** 黑棋使用深灰到黑色渐变（`#555555` 到 `#111111`）
- **AND** 白棋使用白色到浅灰渐变（`#FFFFFF` 到 `#CCCCCC`）
- **AND** 棋子应有阴影效果（`rgba(0,0,0,0.4)`，2px 偏移）
- **AND** 移动端应减少阴影强度（更扁平）

#### Scenario: Animation Performance
- **WHEN** 任何动画执行
- **THEN** 动画持续时间应在 150-500ms 范围内
- **AND** 使用 CSS transition 或 transform（优于 position 或 width）
- **AND** 尊重用户的 `prefers-reduced-motion` 偏好设置
- **AND** 复杂动画（如菜单滑入）应为 300ms
- **AND** 简单交互（如按钮悬停）应为 150-200ms

---

### Requirement: Accessibility Support

系统必须符合 WCAG AAA 可访问性标准，支持键盘导航和屏幕阅读器。

#### Scenario: Keyboard Navigation
- **WHEN** 用户使用键盘操作
- **THEN** Tab 键应按以下顺序导航：
  1. 菜单按钮
  2. 菜单内的按钮（网格顺序）
  3. 棋盘（按 Tab 聚焦后，方向键移动光标）
- **AND** 焦点元素应有清晰的视觉指示（3px outline，`var(--accent)` 颜色）
- **AND** ESC 键应关闭打开的菜单
- **AND** Enter/Space 键应激活焦点按钮

#### Scenario: Screen Reader Support
- **WHEN** 屏幕阅读器访问应用
- **THEN** 菜单按钮应有 `aria-label="打开游戏菜单"`
- **AND** 菜单抽屉应有 `role="dialog"` 和 `aria-modal="true"`
- **AND** 棋盘应有 `role="grid"` 和 `aria-label="五子棋棋盘，15行15列"`
- **AND** 棋盘应有动态 `aria-description`（如"当前黑棋落子，共23步"）
- **AND** 所有图标应有 `aria-hidden="true"`（纯装饰性）

#### Scenario: Touch Target Compliance
- **WHEN** 任何交互元素在移动设备上显示
- **THEN** 触控目标最小尺寸为 44x44px（WCAG AAA 推荐）
- **AND** 相邻触控目标之间至少有 8px 间距
- **AND** 触控目标应足够大，不需要精确手指定位

#### Scenario: Color Contrast Compliance
- **WHEN** 任何文本或交互元素显示
- **THEN** 所有文本对比度应至少为 7:1（WCAG AAA）
- **AND** 重要交互元素（按钮）对比度应至少为 7:1
- **AND** 颜色不应作为唯一的视觉指示器（配合图标或文本）
