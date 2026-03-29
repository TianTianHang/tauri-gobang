# Mobile UI Redesign - Implementation Tasks

## 1. 项目初始化与设置

- [x] 1.1 创建新组件文件结构
  - `src/components/StatusBar.tsx` + `StatusBar.css`
  - `src/components/MenuDrawer.tsx` + `MenuDrawer.css`
  - `src/hooks/useBoardSize.ts`
  - `src/hooks/useTouchPreview.ts`
  - `src/hooks/useHapticFeedback.ts`

- [x] 1.2 更新现有组件导入
  - 更新 `App.tsx` 导入路径
  - 添加必要的 React hooks 导入

## 2. 动态棋盘尺寸系统

- [x] 2.1 实现 `useBoardSize` Hook
  - 计算 `canvasSize`, `cellSize`, `padding`
  - 添加 `useEffect` 监听 resize 事件
  - 实现防抖（100ms）
  - 添加 `@ts-expect-error` 或类型定义

- [x] 2.2 重构 `GameBoard.tsx`
  - 移除固定常量（`CELL_SIZE`, `PADDING`, `CANVAS_SIZE`）
  - 使用 `useBoardSize` 获取动态尺寸
  - 更新 `canvas` 元素的 `width` 和 `height` 属性
  - 更新 `drawPiece` 函数使用动态尺寸

- [x] 2.3 更新 `GameBoard.css`
  - 移除固定尺寸样式
  - 添加响应式样式：
    - `.game-board { width: 100%; height: 100%; max-width: calc(100vh - 140px); aspect-ratio: 1 / 1; }`
  - 保持 `touch-action: none`

- [x] 2.4 测试动态尺寸
  - 测试 iPhone SE (375px)
  - 测试 iPhone 12 (390px)
  - 测试桌面端 (≥768px)
  - 验证 resize 事件触发重新计算

## 3. 触摸交互系统

- [x] 3.1 实现 `useTouchPreview` Hook
  - 处理 `touchstart` 事件（显示预览）
  - 处理 `touchmove` 事件（更新预览位置，使用 `requestAnimationFrame` 节流）
  - 处理 `touchend` 事件（隐藏预览）
  - 调用 `onCellClick` 回调

- [x] 3.2 实现 `useHapticFeedback` Hook
  - 创建 `vibrate` 函数（检查 `navigator.vibrate` 支持）
  - 定义震动模式常量：
    - `PLACE: 10` (落子)
    - `WIN: [20, 50, 20]` (获胜)
    - `MENU_OPEN: 5` (打开菜单)
    - `ERROR: 50` (错误)
  - 添加静默失败逻辑

- [x] 3.3 集成触摸事件到 `GameBoard.tsx`
  - 添加 `onTouchStart`, `onTouchMove`, `onTouchEnd` 处理器
  - 使用 `useTouchPreview` Hook
  - 在落子成功时调用 `hapticFeedback.place()`
  - 保留现有鼠标事件（桌面端兼容）

- [x] 3.4 优化触摸事件性能
  - 实现 `requestAnimationFrame` 节流
  - 添加 `touch-action: manipulation` CSS
  - 测试低端设备性能

- [x] 3.5 添加触摸预览视觉
  - 更新 `draw` 函数：
    - 检查 `hoverRef.current` 是否存在
    - 绘制半透明棋子（`globalAlpha = 0.35`）
  - 确保 `touchend` 时清除预览

## 4. 状态栏组件

- [x] 4.1 创建 `StatusBar.tsx` 组件
  - Props: `gameState`, `aiThinking`, `mode`, `myColor`
  - 实现 `getStatusText` 函数（从 GameInfo 迁移）
  - 添加 AI 思考指示器（脉冲动画）
  - 集成菜单按钮

- [x] 4.2 创建 `StatusBar.css`
  - 布局：`display: flex; justify-content: space-between; height: 44px;`
  - 状态文本样式：16px, font-weight 600
  - AI 指示器动画：`@keyframes pulse`
  - 菜单按钮：44x44px, cursor-pointer, hover 状态

- [x] 4.3 集成到 `App.tsx`
  - 替换 GameInfo 的状态显示部分
  - 传递必要的 props
  - 调整布局（从 row 改为 column）

## 5. 菜单抽屉系统

- [x] 5.1 创建 `MenuDrawer.tsx` 组件
  - Props: `isOpen`, `onClose`, `gameState`, `difficulty`, `onDifficultyChange`, `onUndo`, `onNewGame`, `onBackToMenu`, `mode`, ...
  - 实现菜单内容结构：
    - 标题栏（游戏设置 + 关闭按钮）
    - 游戏操作部分（新游戏、悔棋、重新开始）
    - 难度选择（AI 模式）
    - 对局信息（步数、时长）
    - 返回主菜单按钮
  - 实现 `handleOverlayClick` 和 `handleClose`

- [x] 5.2 创建 `MenuDrawer.css`
  - 抽屉样式：
    - `position: fixed; top: 0; right: 0; bottom: 0; width: min(320px, 80vw);`
    - `transform: translateX(100%); transition: transform 300ms ease-out;`
    - `.open { transform: translateX(0); }`
  - 遮罩样式：
    - `position: fixed; inset: 0; background: rgba(0,0,0,0.5);`
    - `opacity: 0; pointer-events: none; transition: opacity 300ms;`
    - `.open { opacity: 1; pointer-events: auto; }`
  - 菜单部分样式：清晰的标题、分割线、间距
  - 按钮样式：最小 48px 高度，8px 间距，hover/active 状态
  - 响应式：桌面端宽度 400px

- [x] 5.3 集成到 `App.tsx`
  - 添加 `menuOpen` state
  - 添加 `handleMenuOpen` 和 `handleMenuClose` 函数
  - 在 `handleMenuOpen` 中调用 `hapticFeedback.menuOpen()`
  - 添加 ESC 键监听器关闭菜单

- [x] 5.4 迁移 GameInfo 功能到菜单
  - 将"新游戏"、"悔棋"等按钮移至菜单
  - 将"难度选择"移至菜单
  - 将"对局信息"（步数、时长）移至菜单
  - 简化 GameInfo 组件（保留核心逻辑，移除 UI）

## 6. 可访问性支持

- [x] 6.1 添加键盘导航
  - 菜单按钮：`tabIndex={0}`
  - 菜单内按钮：确保 Tab 顺序正确
  - 添加 ESC 键监听器关闭菜单
  - 添加 `:focus-visible` 样式（3px outline）

- [x] 6.2 添加 ARIA 属性
  - 菜单按钮：`aria-label="打开游戏菜单"`, `aria-expanded={menuOpen}`, `aria-controls="menu-drawer"`
  - 菜单抽屉：`role="dialog"`, `aria-modal="true"`, `aria-label="游戏设置菜单"`
  - 棋盘：`role="grid"`, `aria-label="五子棋棋盘，15行15列"`, `aria-description={...}`
  - 图标：`aria-hidden="true"`
  - 按钮：描述性 `aria-label`

- [x] 6.3 检查颜色对比度
  - 验证深色模式所有文本对比度 ≥ 7:1
  - 验证浅色模式所有文本对比度 ≥ 7:1
  - 使用对比度检查工具（如 WebAIM Contrast Checker）

- [x] 6.4 触控目标验证
  - 检查所有按钮 ≥ 44x44px
  - 检查相邻按钮间距 ≥ 8px
  - 使用 Chrome DevTools 检查触控目标

## 7. 样式更新

- [x] 7.1 更新 `App.css`
  - 添加全局 CSS 变量（深色/浅色模式）
  - 更新 `.game-page` 布局：
    - 移动端：`flex-direction: column`
    - 桌面端：保持 `flex-direction: row` 或改为 column
  - 添加 `.status-bar` 容器样式
  - 移除旧的 `.game-info` 相关样式（如果独立）

- [x] 7.2 更新颜色系统
  - 深色模式：
    - `--bg-primary: #000000`
    - `--bg-secondary: #121212`
    - `--bg-elevated: #1E1E1E`
    - `--text-primary: #FFFFFF`
    - `--text-secondary: #A0A0A0`
    - `--border: #2A2A2A`
  - 浅色模式：
    - `--bg-primary: #FFFFFF`
    - `--bg-secondary: #F5F5F5`
    - `--text-primary: #0C4A6E`
    - `--text-secondary: #475569`
    - `--border: #E5E5E5`
  - 保留棋盘和棋子颜色（现有）

- [x] 7.3 添加动画样式
  - AI 思考指示器：`@keyframes pulse`
  - 菜单滑入/滑出：`transition: transform 300ms`
  - 落子标记淡入：`@keyframes fadeIn`
  - 按钮悬停：`transition: background-color 200ms, border-color 200ms`
  - 添加 `@media (prefers-reduced-motion: reduce)` 禁用动画

## 8. 组件重构与清理

- [x] 8.1 简化 `GameInfo.tsx`
  - 移除 UI 渲染部分（已迁移至 MenuDrawer）
  - 保留核心逻辑（如果需要）
  - 或完全移除（如果不再需要）

- [x] 8.2 更新 `App.tsx`
  - 导入新组件（StatusBar, MenuDrawer）
  - 移除旧组件引用（GameInfo）
  - 更新状态管理（添加 `menuOpen`）
  - 调整事件处理器

- [x] 8.3 移除未使用的代码
  - 删除旧的 GameInfo 相关导入
  - 删除未使用的 CSS 类
  - 清理注释和 TODO

## 9. 测试

- [x] 9.1 单元测试
  - 测试 `useBoardSize` 尺寸计算
  - 测试 `useTouchPreview` 事件处理
  - 测试 `useHapticFeedback` 震动调用
  - 测试 StatusBar 渲染逻辑
  - 测试 MenuDrawer 状态管理

- [x] 9.2 集成测试
  - 测试菜单打开/关闭流程
  - 测试落子 → 触觉反馈 → UI 更新
  - 测试 resize 事件触发重绘
  - 测试联机模式请求对话框（如果保留）

- [x] 9.3 视觉回归测试
  - 截图对比深色/浅色模式
  - 测试不同设备尺寸（375px, 768px, 1024px, 1440px）
  - 检查动画流畅性（Chrome DevTools Performance）

- [x] 9.4 手动测试
  - 真机测试：
    - iPhone SE (小屏)
    - iPhone 12 (中屏)
    - Android 中端机
  - 触控精度测试：连续落子 100 次，误触率 < 2%
  - 性能测试：帧率 ≥ 30fps，CPU 占用 < 80%
  - 可访问性测试：
    - 键盘导航（Tab、方向键、ESC）
    - 屏幕阅读器（VoiceOver/TalkBack）
    - 颜色对比度验证

- [x] 9.5 浏览器兼容性测试
  - iOS Safari 13+
  - Android Chrome 80+
  - Desktop Chrome/Firefox/Safari/Edge

## 10. 文档与交付

- [x] 10.1 更新组件文档
  - 为新组件添加 JSDoc 注释
  - 更新 props 接口说明

- [x] 10.2 更新 README（如需要）
  - 添加移动端使用说明
  - 添加触控操作说明

- [x] 10.3 代码审查准备
  - 运行 `pnpm build` 确保 TypeScript 编译通过
  - 运行 `cd src-tauri && cargo check` 确保 Rust 代码无问题
  - 检查代码风格一致性

- [x] 10.4 创建 Release Notes（可选）
  - 总结变更内容
  - 列出改进和已知问题

---

## 任务优先级

### P0（必须完成，阻塞发布）
- 任务组 2：动态棋盘尺寸系统
- 任务组 3：触摸交互系统（3.1-3.3）
- 任务组 4：状态栏组件
- 任务组 5：菜单抽屉系统（5.1-5.3）

### P1（重要，应完成）
- 任务组 6：可访问性支持（6.1, 6.2）
- 任务组 7：样式更新（7.1, 7.2）
- 任务组 8：组件重构与清理

### P2（增强，可延后）
- 任务组 1：项目初始化（部分可在进行中完成）
- 任务组 3：触摸交互系统（3.4, 3.5）
- 任务组 6：可访问性支持（6.3, 6.4）
- 任务组 7：样式更新（7.3）
- 任务组 9：测试（9.1-9.3 可简化）

### P3（可选）
- 任务组 10：文档与交付
- 任务组 9：测试（9.4, 9.5）

---

## 估算工时

| 任务组 | 预估时间 | 依赖 |
|--------|---------|------|
| 1. 项目初始化 | 1h | - |
| 2. 动态棋盘尺寸 | 3h | 1 |
| 3. 触摸交互 | 4h | 1, 2 |
| 4. 状态栏 | 2h | 1 |
| 5. 菜单抽屉 | 5h | 1, 4 |
| 6. 可访问性 | 3h | 4, 5 |
| 7. 样式更新 | 2h | 2, 4, 5 |
| 8. 组件重构 | 2h | 5 |
| 9. 测试 | 4h | 全部 |
| 10. 文档 | 1h | 全部 |
| **总计** | **27h** | - |

**注意**：这是初步估算，实际时间可能因复杂度和调试而变化。

---

## 并行执行建议

可并行进行的任务组：
- 任务组 2 和 4（独立的组件）
- 任务组 3 和 5（触摸交互和菜单可同时开发）
- 任务组 6 和 7（可访问性和样式可同时进行）

建议顺序：
1. 任务组 1（初始化）
2. 任务组 2 + 4（并行）
3. 任务组 3 + 5（并行）
4. 任务组 6 + 7 + 8（并行）
5. 任务组 9（测试）
6. 任务组 10（文档）
