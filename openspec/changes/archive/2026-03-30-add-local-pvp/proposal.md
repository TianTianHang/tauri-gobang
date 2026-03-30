## Why

当前主菜单只有"人机对战"和"联机对战"两个按钮，视觉上显得空旷，且缺少最传统、最基础的本地双人对战模式。本地双人（同屏对战）是五子棋最经典的对战方式，适合朋友聚会时面对面游戏，无需网络连接，零门槛体验。

## What Changes

- **主菜单新增第三个按钮**："本地对战"，位于"人机对战"和"联机对战"之间
- **新增游戏模式** `local_pvp`：双人同屏轮流下棋，无需 AI 和网络
- **新增图标组件** `UserGroupIcon`：两个人形并排的 SVG 图标（Heroicons 风格）
- **类型定义扩展**：`GameMode` 类型添加 `"local_pvp"` 选项
- **状态栏适配**：本地模式显示"黑方回合" / "白方回合"，不显示 AI 思考状态
- **菜单抽屉优化**：本地模式隐藏难度选择和认输按钮（无需 AI，无需认输）
- **CSS 调整**：主菜单按钮布局从 2 列改为 3 列垂直堆叠

## Capabilities

### New Capabilities

- `local-pvp`: 本地双人对战模式，支持同屏双人轮流下棋，独立于 AI 和联机模式

### Modified Capabilities

无（现有 capabilities 的 spec-level 行为不变）

## Impact

**前端代码：**
- `src/components/MainMenu.tsx` - 添加本地对战按钮
- `src/components/MainMenu.css` - 调整按钮布局
- `src/components/Icons.tsx` - 添加 UserGroupIcon
- `src/App.tsx` - 添加 local_pvp 路由逻辑和状态处理
- `src/components/StatusBar.tsx` - 本地模式的轮次提示
- `src/components/MenuDrawer.tsx` - 条件隐藏难度和认输
- `src/types/game.ts` - 扩展 GameMode 类型

**后端：**
无（本地模式纯前端逻辑，复用现有 GameState 和 move validation）

**依赖：**
无新增依赖
