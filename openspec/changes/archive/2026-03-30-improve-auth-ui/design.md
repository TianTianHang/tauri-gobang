## Context

当前登录/注册界面（LoginScreen 组件）功能完整但视觉呈现较为基础。缺少品牌元素、表单交互辅助、密码可见性切换和友好的错误提示。

现有设计系统：
- 金色主题 (`--accent: #D4AF37`)
- Noto Sans SC 字体
- 圆角卡片（16px）
- 状态颜色（红/绿/橙）
- 已有 BlackStoneIcon 可复用

**约束：**
- 纯前端变更，不修改后端 API
- 不引入新的外部依赖
- 保持表单验证逻辑不变
- 保持响应式设计（移动端 + 桌面端）

## Goals / Non-Goals

**Goals:**
- 提升品牌识别度（棋子图标 + 欢迎文案）
- 改善表单可用性（图标指示、密码切换）
- 优化错误反馈（内联错误提示）
- 提升视觉吸引力（友好的文案、按钮样式）

**Non-Goals:**
- 不添加新功能（如社交登录、忘记密码等）
- 不修改登录/注册的业务逻辑
- 不改变金色主题和整体设计语言
- 不引入新的字体或图标库

## Decisions

### 1. 品牌图标选择
**Decision:** 复用现有的 BlackStoneIcon 而非创建新图标

**Rationale:**
- BlackStoneIcon 已在代码库中，代表五子棋的核心元素
- 无需新增 SVG 代码
- 与游戏中的棋子视觉一致，建立品牌关联

**Implementation:**
```tsx
<div className="login-brand">
  <BlackStoneIcon className="brand-icon" />
</div>
```

**Alternative considered:**
- 创建新的 Logo 图标 → 增加设计和维护成本
- 使用 Emoji（♟️）→ 渲染不一致，不够专业

### 2. 输入框图标布局
**Decision:** 使用绝对定位将图标放在输入框内部左侧

**Rationale:**
- 节省空间，图标不占用额外行
- 符合现代表单设计模式（Material Design 3, iOS 15+）
- 图标作为字段类型的视觉提示

**Technical approach:**
```css
.input-group {
  position: relative;
}

.input-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
}

.input-group input {
  padding-left: 40px; /* Space for icon */
}
```

**Alternative considered:**
- 图标在左侧，输入框在右侧（flex 布局）→ 增加布局复杂度
- 图标在输入框上方 → 占用更多垂直空间

### 3. 密码可见性切换实现
**Decision:** 使用 React state 控制 input type，不使用第三方库

**Rationale:**
- 简单的功能，无需引入依赖
- 性能好，无额外 JavaScript
- 完全控制样式和行为

**Implementation:**
```tsx
const [showPassword, setShowPassword] = useState(false);
const [showConfirmPassword, setShowConfirmPassword] = useState(false);

<input
  type={showPassword ? "text" : "password"}
  ...
/>
<button
  type="button"
  onClick={() => setShowPassword(!showPassword)}
>
  {showPassword ? <EyeOffIcon /> : <EyeIcon />}
</button>
```

**Security consideration:**
- 默认隐藏密码（type="password"）
- 切换到可见是用户主动操作
- 符合安全最佳实践

### 4. 错误提示显示策略
**Decision:** 从顶部全局错误改为字段下方内联错误

**Rationale:**
- 更接近用户关注点（正在填写的字段）
- 减少眼球移动距离
- 符合 WCAG 无障碍指南（错误关联到输入）

**Implementation approach:**
```tsx
{usernameError && (
  <div className="field-error" role="alert">
    <ExclamationCircleIcon />
    <span>{usernameError}</span>
  </div>
)}
```

**Trade-off:**
- 需要为每个字段单独管理 error state
- 增加少量代码复杂度

**Mitigation:**
- 使用统一错误管理对象：`{ username: string, password: string, confirmPassword: string }`
- 在验证时设置对应字段的错误

### 5. 模式切换按钮样式
**Decision:** 虚线边框按钮而非纯文字链接

**Rationale:**
- 更高的可点击性（更明显的按钮样式）
- 视觉上更突出，引导用户注册
- 虚线边框暗示"可选操作"，与实线主按钮区分

**CSS approach:**
```css
.toggle-mode-action {
  border: 1px dashed var(--border);
  border-radius: 8px;
  background: transparent;
  color: var(--accent);
}

.toggle-mode-action:hover {
  background: var(--bg-secondary);
  border-color: var(--accent);
}
```

### 6. 状态管理优化
**Decision:** 为每个密码字段单独管理可见性状态

**Rationale:**
- 注册页面有两个密码字段（密码 + 确认密码）
- 用户可能需要分别查看
- 独立状态更灵活

**Implementation:**
```tsx
const [showPassword, setShowPassword] = useState(false);
const [showConfirmPassword, setShowConfirmPassword] = useState(false);
```

**Alternative considered:**
- 全局密码可见性状态 → 两个密码字段同步显示/隐藏，不够灵活

### 7. 动画实现
**Decision:** 使用 CSS transition + opacity 实现模式切换动画

**Rationale:**
- 轻量级，无需 JavaScript 动画库
- 性能好（GPU 加速）
- 自动支持 `prefers-reduced-motion`

**CSS approach:**
```css
.confirm-password-field {
  opacity: 1;
  max-height: 100px;
  transition: opacity 0.2s ease, max-height 0.2s ease;
}

.confirm-password-field.hidden {
  opacity: 0;
  max-height: 0;
  overflow: hidden;
}

@media (prefers-reduced-motion: reduce) {
  .confirm-password-field,
  .confirm-password-field.hidden {
    transition: none;
  }
}
```

### 8. 图标大小规范
**Decision:** 统一图标尺寸：输入框内 18px，品牌图标 48px，错误图标 16px

**Rationale:**
- 输入框内图标需要小巧不抢占焦点
- 品牌图标需要足够大以建立视觉识别
- 错误图标与文字对齐

**Size standard:**
```css
--icon-input: 18px;    /* UserIcon, LockIcon */
--icon-brand: 48px;    /* BlackStoneIcon */
--icon-error: 16px;    /* ExclamationCircleIcon */
--icon-toggle: 20px;   /* EyeIcon, EyeOffIcon */
```

## Risks / Trade-offs

### Risk 1: 内联错误增加状态管理复杂度
**Description:** 从单一 error state 改为字段级 error state，增加代码量

**Mitigation:**
- 使用对象结构统一管理：`errors: { username?: string, password?: string, ... }`
- 创建 `validateField(field)` 函数统一验证逻辑
- 在 `onChange` 时清除对应字段错误

### Risk 2: 图标在移动端可能导致输入框拥挤
**Description:** 小屏幕设备上，图标 + 文字可能显得拥挤

**Mitigation:**
- 图标尺寸保持小巧（18px）
- 增加输入框 padding-left（40px）留出足够空间
- 移动端已有响应式 CSS，保持 `max-width: 420px`
- 测试 375px 宽度设备（iPhone SE）

### Risk 3: 密码可见性切换可能被滥用
**Description:** 在公共场所查看密码可能导致隐私泄露

**Mitigation:**
- 默认隐藏密码，需要用户主动点击
- 仅在用户设备上显示，不通过网络传输
- 符合常见 UX 模式（iOS、Android、Web 标准）
- 可在未来添加"显示密码"警告提示（非阻塞）

### Trade-off: 虚线边框 vs 实线边框
**Decision:** 模式切换按钮使用虚线边框

**Reason:**
- 虚线暗示"次要/可选操作"
- 与实线主按钮（登录/注册）形成视觉层次
- 更符合内容层级的视觉语言

**Acceptable because:**
- 常见的 UI 模式（如"Notional"、"Stripe"等）
- 清晰的视觉区分
- 悬停时变为实线，提供反馈

## Migration Plan

### 部署步骤
1. 更新 `Icons.tsx`：添加 EyeIcon、EyeOffIcon、LockIcon、ExclamationCircleIcon
2. 更新 `LoginScreen.tsx`：
   - 导入新图标
   - 添加密码可见性 state
   - 添加字段级错误 state
   - 渲染品牌区域
   - 渲染输入框组（带图标）
   - 渲染密码切换按钮
   - 渲染内联错误
   - 更新模式切换按钮样式
3. 更新 `LoginScreen.css`：
   - 添加 `.login-brand`, `.brand-icon` 样式
   - 添加 `.login-title`, `.login-subtitle` 样式
   - 添加 `.input-group`, `.input-icon` 样式
   - 添加 `.toggle-password` 样式
   - 添加 `.field-error` 样式
   - 添加 `.toggle-mode-action` 样式
   - 添加模式切换动画（transition）

### 回滚策略
- 所有修改在前端，无数据迁移
- 如有视觉或功能问题，通过 git revert 即可回滚
- 不影响登录/注册功能使用

## Open Questions

无（设计明确，可立即实施）
