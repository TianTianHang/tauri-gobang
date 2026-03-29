## Why

当前的五子棋游戏在移动设备上的用户体验存在明显问题：

1. **棋盘尺寸不合适**：固定 620px 宽度在手机屏幕上需要大幅缩放，导致棋子过小（~18-22px），难以精确点击
2. **交互方式不匹配移动端**：依赖 hover 预览，但移动设备不支持悬停，导致落子缺乏视觉确认
3. **操作按钮占用过多空间**：GameInfo 面板占据大量垂直空间，挤压棋盘显示区域
4. **布局不够沉浸**：非游戏元素分散注意力，特别是在小屏幕上

移动设备是五子棋游戏的重要平台（Tauri 支持 Android），改善移动端体验将显著提升用户参与度。

## What Changes

### 核心变更

1. **重新设计移动端布局**：
   - 棋盘居中并最大化，占据尽可能多的屏幕空间
   - 将所有非必要操作移至二级菜单（右上角抽屉式菜单）
   - 添加精简的顶部状态栏（44px 高度）

2. **优化棋盘尺寸策略**：
   - 实现动态尺寸计算：根据屏幕尺寸自动调整棋盘大小
   - 保持棋盘正方形比例，取可用空间的较小边
   - 确保 cell 尺寸在 22-26px 范围内（可接受的点击精度）

3. **改进移动端交互**：
   - 添加触摸预览（touch preview）替代 hover 预览
   - 实现触觉反馈（haptic feedback）确认落子
   - 优化触摸事件处理（消除 300ms 延迟）

4. **创建菜单抽屉组件**：
   - 右上角触发按钮（44x44px 触控目标）
   - 从右侧滑入的抽屉式菜单
   - 包含所有游戏操作（新游戏、悔棋、难度、返回等）

5. **统一状态显示**：
   - 顶部状态栏显示当前游戏状态和 AI 思考指示
   - 减少信息密度，保持界面简洁

### 技术实现要点

- Canvas 尺寸动态计算（基于 `window.innerHeight` 和 `window.innerWidth`）
- CSS 响应式布局（Flexbox + `aspect-ratio`）
- Touch Events API（`touchstart`, `touchmove`, `touchend`）
- Vibration API（`navigator.vibrate`）
- 菜单动画（CSS Transform + Transition）

## Capabilities

### New Capabilities

- `mobile-game-ui`: 移动优先的游戏界面系统，涵盖动态棋盘尺寸、触控优化、菜单抽屉、状态栏、触觉反馈

### Modified Capabilities

无（本次变更仅涉及 UI/UX 改进，不影响核心游戏逻辑和功能需求）

## Impact

### 代码影响

**新增组件**：
- `src/components/StatusBar.tsx` + `.css` - 顶部状态栏
- `src/components/MenuDrawer.tsx` + `.css` - 抽屉式菜单
- `src/hooks/useBoardSize.ts` - 棋盘尺寸计算 Hook
- `src/hooks/useTouchPreview.ts` - 触摸预览 Hook

**修改组件**：
- `src/App.tsx` - 重构布局逻辑，集成新组件
- `src/components/GameBoard.tsx` - 添加动态尺寸、触摸事件处理
- `src/components/GameInfo.tsx` - 简化功能，部分操作迁移至菜单
- `src/App.css` - 更新全局布局样式

**移除**：
- 无（保留现有组件，仅重构）

### API 变更

无 Tauri 命令变更（纯前端改进）

### 依赖变更

无需新增外部依赖（使用 React Hooks 和 Web APIs）

### 用户体验影响

**正面影响**：
- 移动端棋盘可玩性显著提升（棋子更大，点击更准确）
- 更沉浸的游戏体验（界面简洁，专注棋盘）
- 更流畅的交互（触觉反馈、触摸预览）

**潜在风险**：
- 桌面用户需要适应新布局（菜单位置变化）
- 首次使用可能需要探索菜单位置（可通过引导解决）

### 性能影响

- Canvas 重绘逻辑需要优化（避免频繁重绘）
- Resize 事件需要防抖处理
- 整体性能影响轻微（主要是布局计算）

### 可访问性

- 改进：触控目标符合 WCAG AAA 标准（44x44px 最小）
- 改进：颜色对比度符合 WCAG AAA（深色/浅色模式）
- 需添加：键盘导航支持（菜单和棋盘）
- 需添加：屏幕阅读器 ARIA 标签
