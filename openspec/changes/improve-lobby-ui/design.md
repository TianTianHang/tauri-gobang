## Context

当前联机大厅（RoomList 组件）功能完整但视觉呈现较为基础。房间卡片缺少状态反馈，时间显示使用绝对格式，空状态体验枯燥，操作按钮缺少视觉层次。

现有设计系统：
- 金色主题 (`--accent: #D4AF37`)
- Noto Sans SC 字体
- 圆角卡片（16px, 12px, 10px）
- 浅色/深色模式支持

**约束：**
- 纯前端变更，不修改后端 API
- 不引入新的外部依赖（使用现有 Heroicons 风格）
- 保持响应式设计（移动端 + 桌面端）
- 保持现有功能逻辑不变

## Goals / Non-Goals

**Goals:**
- 提升房间卡片的视觉识别度（状态指示器、人数显示）
- 改善空状态的用户体验
- 建立操作按钮的视觉层次（主操作 vs 次要操作）
- 统一图标风格，提升视觉一致性

**Non-Goals:**
- 不添加新功能（如快速匹配、游客模式等）
- 不修改房间创建/加入的业务逻辑
- 不改变金色主题和整体设计语言
- 不引入新的字体或图标库

## Decisions

### 1. 状态指示器设计
**Decision:** 使用彩色圆点（8x8px）+ CSS 变量系统

**Rationale:**
- 简洁直观，不占用过多空间
- 使用 CSS 变量便于统一管理和未来主题扩展
- 颜色语义符合通用惯例：🟢 绿色=可加入，🔴 红色=满员，🟠 橙色=等待中

**Alternatives considered:**
- 文字标签（"可加入"）→ 太占空间，移动端拥挤
- 边框颜色 → 不够醒目
- 图标（✓/✗）→ 语义不如颜色直观

**Implementation:**
```css
:root {
  --room-available: #22C55E;
  --room-full: #EF4444;
  --room-waiting: #F59E0B;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--room-available);
}
```

### 2. 相对时间格式化
**Decision:** 前端使用 `Intl.RelativeTimeFormat` API 格式化时间

**Rationale:**
- 原生 API，无需额外库
- 自动本地化（中文："3分钟前"，英文："3 minutes ago"）
- 更符合用户习惯（"多久前" vs "14:30"）

**Fallback strategy:**
- < 1小时: "X分钟前"
- < 24小时: "X小时前"
- ≥ 24小时: 绝对日期 "MM-DD HH:mm"

**Implementation:**
```typescript
function formatRelativeTime(timestamp: number): string {
  const now = Date.now();
  const diff = Math.floor((now - timestamp * 1000) / 1000); // seconds

  if (diff < 3600) {
    const minutes = Math.floor(diff / 60);
    return `${minutes}分钟前`;
  }
  if (diff < 86400) {
    const hours = Math.floor(diff / 3600);
    return `${hours}小时前`;
  }
  // Fallback to absolute time
  return new Date(timestamp * 1000).toLocaleString("zh-CN", {
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}
```

### 3. 图标实现方式
**Decision:** 直接在 `Icons.tsx` 中添加 SVG 图标组件

**Rationale:**
- 项目已有 Icons.tsx，保持一致性
- Heroicons 风格与现有图标统一
- 无需引入新依赖
- SVG 性能好，可自定义颜色

**New icons to add:**
- `StatusDotIcon` - 状态指示点（带颜色参数）
- `UserIcon` - 用户图标
- `ClockIcon` - 时钟图标
- `PlusIcon` - 加号图标

### 4. 空状态设计
**Decision:** 使用 Emoji（🏠）+ 友好文案，不添加插图图片

**Rationale:**
- Emoji 轻量，无需加载图片
- 与当前简洁风格一致
- 减少资源加载和打包体积

**Alternative considered:**
- SVG 插画 → 增加代码量和维护成本
- 外部图片 → 加载延迟，可能失败

**Text content:**
```
🏠
大厅空空如也...
创建一个房间等待朋友吧
```

### 5. 按钮层次设计
**Decision:** 创建房间使用实心金色按钮，次要操作使用描边按钮

**Rationale:**
- 符合 Material Design 和 iOS HIG 的按钮层次规范
- 引导用户执行主要操作（创建房间）
- 保持视觉平衡

**Implementation:**
```css
/* Primary - Create Room */
.btn-primary {
  background: var(--accent);
  color: var(--text-on-accent);
  width: 100%;
  padding: 14px 20px;
}

/* Secondary - Refresh, Logout */
.btn-secondary {
  background: transparent;
  border: 1px solid var(--border);
  color: var(--text-secondary);
  padding: 10px 16px;
}
```

### 6. CSS 变量扩展
**Decision:** 在 `App.css` 中添加房间状态相关的 CSS 变量

**Rationale:**
- 统一管理颜色值
- 便于未来主题扩展（如添加自定义主题）
- 保持与现有 CSS 变量系统一致

**New variables:**
```css
:root {
  /* Room status colors */
  --room-available: #22C55E;
  --room-full: #EF4444;
  --room-waiting: #F59E0B;
}

@media (prefers-color-scheme: dark) {
  :root {
    /* Dark mode can use same colors - already high contrast */
  }
}
```

## Risks / Trade-offs

### Risk 1: 相对时间在不同时区可能不准确
**Description:** 用户设备时间不准或时区设置错误可能导致相对时间错误

**Mitigation:**
- 时间戳来自服务器，是 UTC 时间
- 前端计算相对时间，基于客户端当前时间
- 对于 ≥ 24小时的情况回退到绝对日期，减少误判

### Risk 2: 移动端空间拥挤
**Description:** 添加状态指示器、人数、图标后，房间卡片可能在移动端显得拥挤

**Mitigation:**
- 使用小尺寸图标（14-16px）
- 保持图标内联，不占用额外行
- 调整 `flex` 布局，确保文本截断（`text-overflow: ellipsis`）
- 在移动端减少 padding（已有响应式 CSS）

### Trade-off: Emoji vs SVG 图标
**Decision:** 空状态使用 Emoji 🏠 而非自定义 SVG

**Reason:**
- 快速实现，无需设计资源
- 轻量，不增加代码量
- 潜在问题：不同系统显示效果不一致（Android vs iOS）

**Acceptable because:**
- 空状态是低频页面
- Emoji 在现代设备上渲染效果已足够好
- 可在未来替换为 SVG 插画（非阻塞）

## Migration Plan

### 部署步骤
1. 更新 `App.css`：添加房间状态 CSS 变量
2. 更新 `Icons.tsx`：添加 4 个新图标组件
3. 更新 `RoomList.tsx`：
   - 导入新图标
   - 添加 `formatRelativeTime` 函数
   - 渲染状态指示器、人数、图标
   - 优化空状态渲染
4. 更新 `RoomList.css`：
   - 添加 `.status-dot`, `.player-count` 等样式
   - 调整按钮层次样式
   - 优化空状态样式

### 回滚策略
- 所有修改在前端，无数据迁移
- 如有视觉问题，通过 git revert 即可回滚
- 不影响功能使用

## Open Questions

无（设计明确，可立即实施）
