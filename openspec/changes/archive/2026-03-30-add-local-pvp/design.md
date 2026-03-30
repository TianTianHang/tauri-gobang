## Context

当前应用支持三种游戏模式：
- `ai`: 人机对战（玩家 vs AI）
- `online_host` / `online_client`: 联机对战（通过 WebSocket）

主菜单 UI 只有 2 个按钮，视觉上不够充实。本地双人是五子棋最经典的对战方式，适合朋友聚会场景，实现成本低，无需网络连接。

## Goals / Non-Goals

**Goals:**
- 在主菜单添加第三个"本地对战"按钮
- 实现双人同屏轮流下棋的核心逻辑
- 适配状态栏和菜单抽屉显示本地模式信息
- 保持与现有 AI 和联机模式的代码复用

**Non-Goals:**
- 不涉及后端修改（本地模式纯前端逻辑）
- 不实现计时器、悔棋确认等额外功能（保持简单）
- 不支持本地排位赛或战绩记录

## Decisions

### 1. GameMode 扩展
**Decision:** 在 `GameMode` 类型中新增 `"local_pvp"` 选项

**Rationale:**
- 保持类型安全，避免魔法字符串
- 与现有 `ai`, `online_host`, `online_client` 模式一致
- 路由逻辑可以根据 mode 分支处理

### 2. 图标选择：UserGroupIcon
**Decision:** 使用 Heroicons 风格的两个人形并排 SVG 图标

**Rationale:**
- 与现有 `RobotIcon`（人机）和 `GlobeIcon`（联机）风格统一
- 视觉语义清晰：两个人 = 双人对战
- 实现简单，无需引入外部图标库

### 3. 状态栏显示策略
**Decision:** 本地模式显示"⚫ 黑方回合" / "⚪ 白方回合"，不显示 AI 思考和对手信息

**Rationale:**
- 本地模式无需 AI，不存在"AI 思考中"状态
- 双人同屏，无需显示对手名字
- 清晰的轮次提示有助于面对面游戏

**Implementation:**
```tsx
// StatusBar.tsx
{mode === "local_pvp" && (
  <span className="player-turn">
    {gameState.current_player === Cell.Black ? "⚫ 黑方回合" : "⚪ 白方回合"}
  </span>
)}
```

### 4. 菜单抽屉条件渲染
**Decision:** 使用条件渲染隐藏难度选择和认输按钮

**Rationale:**
- 本地模式无 AI，难度选择无意义
- 面对面认输很尴尬，可直接商量或停止游戏
- 保持其他功能（悔棋、新游戏、返回菜单）可用

**Implementation:**
```tsx
{mode !== "local_pvp" && (
  <DifficultySelector ... />
)}
{mode === "online_host" || mode === "online_client" && (
  <SurrenderButton ... />
)}
```

### 5. 游戏逻辑复用
**Decision:** 本地模式复用现有 `GameState`、`make_move` 等核心逻辑

**Rationale:**
- 五子棋规则一致（落子、胜负判定）
- 无需修改 Rust 后端代码
- 仅在前端控制轮次和权限

**Key Difference:**
- AI 模式：玩家落子 → 触发 `ai_move_start`
- 本地模式：黑方落子 → 等待白方落子 → 循环
- 联机模式：本地落子 → 发送 WebSocket 消息 → 等待对手消息

## Risks / Trade-offs

### Risk 1: 悔棋功能可能引起争议
**Description:** 本地对战时，一方点击悔棋，对方可能不同意

**Mitigation:**
- 当前实现：直接撤销 2 步（双方各一步）
- 信任玩家之间会口头商量
- 未来可扩展为双方确认机制（不在此次范围）

### Risk 2: 主菜单按钮布局变化
**Description:** 从 2 个按钮增加到 3 个，可能在移动端显示拥挤

**Mitigation:**
- 现有 CSS 已使用垂直堆叠布局，无需改动
- 按钮高度和间距已在 MainMenu.css 中定义
- 响应式设计已覆盖（移动端和桌面端）

### Trade-off: 功能简洁 vs 功能丰富
**Decision:** 本次实现最基础的本地对战功能，不添加计时器、悔棋确认等

**Rationale:**
- 快速交付核心价值
- 降低实现复杂度
- 可根据用户反馈迭代

## Migration Plan

### 部署步骤
1. 更新 `src/types/game.ts`：添加 `local_pvp` 到 `GameMode` 类型
2. 更新 `src/components/Icons.tsx`：添加 `UserGroupIcon`
3. 更新 `src/components/MainMenu.tsx`：添加第三个按钮和 `onLocalPlay` handler
4. 更新 `src/components/MainMenu.css`：确认 3 按钮布局正常
5. 更新 `src/App.tsx`：添加 `local_pvp` 路由和 `handleLocalPlay` handler
6. 更新 `src/components/StatusBar.tsx`：添加本地模式轮次显示
7. 更新 `src/components/MenuDrawer.tsx`：条件隐藏难度和认输

### 回滚策略
- 所有修改在前端，无数据库或后端变更
- 如有问题，通过 git revert 即可回滚

## Open Questions

无（设计明确，可立即实施）
