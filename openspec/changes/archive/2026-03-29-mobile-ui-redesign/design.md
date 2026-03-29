## Context

### 当前架构分析

**现有组件结构**：
```
App.tsx (根组件)
├─ MainMenu.tsx - 主菜单
├─ GameBoard.tsx - Canvas 棋盘渲染
├─ GameInfo.tsx - 游戏信息面板
└─ NetworkSetup.tsx - 网络设置

布局：
- game-page: flex column (mobile) / row (desktop)
- GameInfo 占据大量垂直空间（~300px）
- GameBoard 固定 620px 尺寸
```

**现有问题**：
1. GameBoard 使用固定常量（`CELL_SIZE = 40`, `CANVAS_SIZE = 620`）
2. 无触控事件处理（仅 mouse events）
3. GameInfo 组件耦合度高（包含多种操作）
4. 布局响应式有限（仅 flex-direction 变化）

### 技术约束

- **框架**：React 19 + TypeScript（严格模式）
- **构建**：Vite + Tauri
- **样式**：CSS Modules（无 Tailwind）
- **平台**：Desktop (macOS/Windows/Linux) + Android (Tauri Mobile)
- **无外部 UI 库**（保持轻量）

---

## Goals / Non-Goals

**Goals:**
1. 移动端棋盘占据最大可用空间（至少 50% 屏幕高度）
2. 所有触控目标符合 WCAG AAA 标准（44x44px 最小）
3. 菜单系统不干扰游戏主界面
4. 保持桌面端用户体验不退化
5. 支持 iOS Safari 和 Android Chrome 的触控 API
6. 深色/浅色模式对比度达到 WCAG AAA（7:1+）

**Non-Goals:**
1. 不添加手势操作（如双指缩放、滑动等）
2. 不使用第三方 UI 库（如 Material-UI, Ant Design）
3. 不修改核心游戏逻辑（GameState, AI, Network）
4. 不支持横屏特殊优化（横屏使用竖屏逻辑）
5. 不添加音效（本次仅触觉反馈）

---

## Decisions

### 决策 1: 动态棋盘尺寸计算策略

**选择**：混合方案（CSS 布局 + JS 渲染）

**方案对比**：

| 方案 | 优点 | 缺点 | 决策 |
|------|------|------|------|
| A. 纯 CSS (`aspect-ratio: 1`) | 简单，浏览器原生 | Canvas 内部渲染不同步，模糊 | ❌ |
| B. 纯 JS (每次 resize 计算) | 精确控制 | 频繁重绘，性能开销 | ❌ |
| C. 混合（CSS 布局 + JS 初始化） | 布局灵活，渲染清晰 | 需同步两个系统 | ✅ |

**实现方案**：
```tsx
// useBoardSize Hook
function useBoardSize() {
  const [size, setSize] = useState({ canvas: 0, cell: 0, padding: 0 });

  const calculate = useCallback(() => {
    const statusBarH = 44;
    const padding = 32;
    const availableH = window.innerHeight - statusBarH - padding;
    const availableW = window.innerWidth - 32;

    const canvasSize = Math.min(availableW, availableH);
    const paddingPx = canvasSize * 0.05;
    const cellSize = (canvasSize - paddingPx * 2) / 14;

    setSize({
      canvas: Math.floor(canvasSize),
      cell: Math.floor(cellSize),
      padding: Math.floor(paddingPx)
    });
  }, []);

  useEffect(() => {
    calculate();
    const handleResize = debounce(calculate, 100);
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, [calculate]);

  return size;
}

// CSS 配合
.game-board {
  width: 100%;
  height: 100%;
  max-width: calc(100vh - 140px);
  max-height: calc(100vh - 140px);
  aspect-ratio: 1 / 1;
}
```

**理由**：
- CSS 处理响应式布局（自适应容器）
- JS 处理 Canvas 渲染分辨率（确保清晰度）
- 防抖避免频繁重绘

---

### 决策 2: 菜单触发器位置

**选择**：右上角固定位置

**方案对比**：

| 位置 | 优点 | 缺点 | 决策 |
|------|------|------|------|
| 右上角 | 符合习惯，不遮挡棋盘 | 需要伸手够（大屏手机） | ✅ |
| 右下角浮动 | 拇指友好（单手操作） | 遮挡棋盘角落 | ❌ |
| 底部中央 | iOS 风格，易触达 | 与棋盘分离，不协调 | ❌ |
| 左上角 | 返回按钮习惯位置 | 与返回功能混淆 | ❌ |

**实现**：
```tsx
<div className="status-bar">
  <div className="status-text">...</div>
  <button
    className="menu-btn"
    onClick={onMenuOpen}
    aria-label="打开游戏菜单"
  >
    <MenuIcon />
  </button>
</div>
```

**理由**：
- 大多数应用的菜单习惯位置（符合心理模型）
- 不遮挡棋盘主要区域
- 与状态栏对齐，视觉平衡

---

### 决策 3: 触觉反馈使用策略

**选择**：保守使用，仅在关键操作时反馈

**反馈场景**：

| 操作 | 震动模式 | 理由 |
|------|---------|------|
| 落子 | 10ms | 轻微确认，不频繁 |
| 获胜 | `[20, 50, 20]` | 成功庆祝，有明显节奏 |
| 打开菜单 | 5ms | 极轻微，系统反馈感 |
| 失败/错误 | 50ms | 单次震动，警示 |

**不使用场景**：
- ❌ Hover 预览（过于频繁）
- ❌ 按钮悬停（非移动端）
- ❌ 每个 touchmove（性能问题）

**实现**：
```tsx
function handlePiecePlace() {
  if (navigator.vibrate) {
    navigator.vibrate(10);
  }
  // 落子逻辑
}

function handleWin() {
  if (navigator.vibrate) {
    navigator.vibrate([20, 50, 20]);
  }
  // 胜利逻辑
}
```

**理由**：
- 避免过度使用导致疲劳
- 优先级：成功确认 > 系统反馈 > 警示
- 静默失败（不支持震动 API 的设备）

---

### 决策 4: 状态栏信息密度

**选择**：最小化显示（仅游戏状态 + AI 指示）

**方案对比**：

| 信息密度 | 显示内容 | 优点 | 缺点 | 决策 |
|---------|---------|------|------|------|
| 极简 | 仅当前落子方 | 最简洁 | 信息不足 | ❌ |
| **平衡** | 状态 + AI 指示 | 信息够用，不拥挤 | 需打开菜单看详情 | ✅ |
| 详细 | 状态 + 步数 + 时长 | 一目了然 | 挤压棋盘空间 | ❌ |

**实现**：
```tsx
<div className="status-bar">
  <div className="status-text">
    {gameState.status === 'playing'
      ? `● ${gameState.current_player === Cell.Black ? '黑' : '白'}棋落子`
      : getStatusText(gameState)}
    {aiThinking && <span className="thinking-indicator">🤔</span>}
  </div>
  <button className="menu-btn">...</button>
</div>
```

**详细信息放菜单**：
```tsx
// MenuDrawer.tsx
<div className="menu-section">
  <h3>ℹ️ 对局信息</h3>
  <p>当前步数: {gameState.history.length}</p>
  <p>对局时长: {formatDuration(duration)}</p>
</div>
```

**理由**：
- 游戏过程中最关心的是"轮到谁了"
- 步数、时长等次要信息不需要常驻显示
- 保持界面简洁，专注棋盘

---

### 决策 5: Canvas 渲染优化策略

**选择**：保持现有重绘逻辑，添加 touch preview

**现有逻辑**：
```tsx
// 每次 hover 重绘整个棋盘
const handleMouseMove = (e) => {
  const cell = getCellFromPixel(x, y);
  if (cell !== hoverRef.current) {
    hoverRef.current = cell;
    draw(); // 重绘所有棋子 + hover 预览
  }
};
```

**优化方案（暂不实施）**：

| 方案 | 复杂度 | 性能提升 | 决策 |
|------|--------|---------|------|
| A. 分层渲染（3 个 canvas） | 高 | 显著 | ❌ 过度设计 |
| B. 脏矩形重绘 | 中 | 中等 | ❌ 实现复杂 |
| C. 离屏 canvas 缓存 | 中 | 中等 | ❌ 增加内存 |
| D. 保持现状 | 低 | - | ✅ 先优化，有问题再改 |

**理由**：
- 当前棋子数量少（最多 225 个），重绘开销可接受
- 移动端 Canvas 性能通常足够
- 遵循"过早优化是万恶之源"原则

**待优化触发条件**：
- 帧率 < 30fps
- CPU 占用 > 80%
- 用户反馈卡顿

---

### 决策 6: 菜单动画实现方式

**选择**：CSS Transition（而非 Framer Motion）

**方案对比**：

| 方案 | 优点 | 缺点 | 决策 |
|------|------|------|------|
| CSS Transition | 原生，零依赖，性能好 | 功能有限 | ✅ |
| Framer Motion | 功能强大，手势支持 | 增加依赖，包体积 | ❌ |
| Web Animations API | 精确控制 | 兼容性，代码冗长 | ❌ |

**实现**：
```css
.menu-drawer {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  width: min(320px, 80vw);
  transform: translateX(100%);
  transition: transform 300ms ease-out;
  z-index: 1000;
}

.menu-drawer.open {
  transform: translateX(0);
}

.menu-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.5);
  opacity: 0;
  pointer-events: none;
  transition: opacity 300ms;
  z-index: 999;
}

.menu-overlay.open {
  opacity: 1;
  pointer-events: auto;
}
```

**理由**：
- 简单场景不需要复杂动画库
- CSS Transition 由浏览器优化（GPU 加速）
- 保持依赖最小化

---

### 决策 7: 深色模式实现方式

**选择**：保持现有 `prefers-color-scheme` 媒体查询

**现有实现**：
```css
@media (prefers-color-scheme: dark) {
  :root {
    --text-primary: #E5E5E5;
    --bg-primary: #0A0A0A;
    ...
  }
}
```

**方案对比**：

| 方案 | 优点 | 缺点 | 决策 |
|------|------|------|------|
| 系统偏好 (`prefers-color-scheme`) | 零配置，自动 | 用户无法手动切换 | ✅ |
| 手动切换按钮 | 用户可控 | 需要存储状态（localStorage） | ❌ |
| 混合（自动 + 手动） | 灵活性最高 | 实现复杂 | ❌ 过度设计 |

**理由**：
- 遵循系统偏好是现代应用标准
- 减少用户决策负担
- 简化实现

**优化**：
- 改进深色模式配色（纯黑 `#000000` 而非 `#0A0A0A`）
- 提高对比度（达到 WCAG AAA）

---

## Risks / Trade-offs

### 风险 1: 移动端性能问题

**风险描述**：
- Canvas 重绘在低端 Android 设备上可能卡顿
- Touch events 频繁触发导致性能问题
- Resize 计算在某些设备上延迟明显

**缓解措施**：
1. 使用 `requestAnimationFrame` 节流 touchmove 事件
2. Resize 防抖（100ms）
3. 添加性能监控（帧率、CPU 占用）
4. 必要时提供"低性能模式"选项

**触发条件**：
- 设备基准测试 < 20fps
- 用户反馈卡顿

**应急方案**：
- 禁用动画（`prefers-reduced-motion`）
- 简化渲染（去除阴影、渐变）
- 减少 preview 更新频率

---

### 风险 2: 触控精度不足

**风险描述**：
- 小屏手机（iPhone SE）棋子仅 22px
- 手指覆盖面积 ~30-40px，难以精确点击
- 可能误触相邻格子

**缓解措施**：
1. 触摸预览（半透明棋子确认位置）
2. 触觉反馈（确认感）
3. 落子后 3 秒内可撤销（悔棋友好）
4. 视觉辅助（最后一手红点标记）

**权衡**：
- 棋盘最大化 vs 点击精度
- 选择：优先棋盘大小，接受轻微精度损失（通过预览补偿）

---

### 风险 3: 桌面用户体验退化

**风险描述**：
- 菜单从侧边栏移至右上角抽屉
- 原有 GameInfo 信息隐藏在菜单中
- 桌面用户可能感到不便

**缓解措施**：
1. 保持桌面端棋盘尺寸（600px）
2. 菜单宽度更大（400px）
3. 考虑桌面端保留部分 GameInfo 内容（如步数）
4. 首次使用引导高亮菜单位置

**监控指标**：
- 桌面端菜单打开频率
- 平均游戏时长（是否因操作不便而减少）
- 用户反馈

---

### 风险 4: 可访问性覆盖不全

**风险描述**：
- 键盘导航可能不完整
- 屏幕阅读器 ARIA 标签缺失
- 焦点管理不当（菜单打开后焦点位置）

**缓解措施**：
1. 添加完整的 ARIA 属性（见 spec.md）
2. 键盘导航测试（Tab、方向键、ESC）
3. 焦点陷阱（菜单打开时焦点在菜单内）
4. 使用 `useRef` 管理焦点

**测试清单**：
- [ ] Tab 键顺序正确
- [ ] 焦点可见（3px outline）
- [ ] ESC 关闭菜单
- [ ] 方向键在棋盘上移动光标
- [ ] 屏幕阅读器正确朗读

---

### 权衡 1: 简洁性 vs 信息可见性

**权衡**：
- 将 GameInfo 内容移至菜单 → 界面简洁
- 但步数、时长等信息需要打开菜单才能看到

**选择**：
- 优先简洁性（沉浸式体验）
- 次要信息移至菜单（符合"渐进式披露"原则）

**理由**：
- 游戏过程中用户最关心的是棋盘和轮次
- 步数、时长是次要信息（不需要常驻）
- 菜单操作成本低（点击一次）

---

### 权衡 2: 动画流畅性 vs 开发复杂度

**权衡**：
- 简单 CSS transition → 功能有限
- Framer Motion → 功能强大，但增加依赖

**选择**：
- 使用 CSS Transition（本次）
- 必要时未来升级到动画库

**理由**：
- 当前需求简单（滑入/滑出、淡入淡出）
- CSS Transition 完全够用
- 保持依赖最小化

---

### 权衡 3: 触觉反馈 vs 电池消耗

**权衡**：
- 频繁震动 → 更好的反馈
- 但消耗电池，可能惹人厌烦

**选择**：
- 保守使用（仅在关键操作）
- 最短震动时长（5-10ms）

**理由**：
- 过度触觉反馈会导致疲劳
- 移动设备电池敏感
- "少即是多"

---

## 测试策略

### 单元测试
- `useBoardSize` Hook 尺寸计算逻辑
- `useTouchPreview` 事件处理
- MenuDrawer 组件状态管理

### 集成测试
- 菜单打开/关闭流程
- 棋盘落子 → 触觉反馈 → UI 更新
- Resize 事件触发重绘

### 视觉回归测试
- 深色/浅色模式截图对比
- 不同设备尺寸（375px, 768px, 1024px）
- 动画帧率检查

### 手动测试
- 真机测试（iPhone SE, iPhone 12, Android 中端机）
- 触控精度测试（连续落子 100 次，误触率 < 2%）
- 性能测试（Chrome DevTools Performance 面板）
- 可访问性测试（屏幕阅读器、键盘导航）

---

## 未来迭代方向

### Phase 2（可选）
- 添加手势操作（双指缩放棋盘）
- 横屏优化（左右布局）
- 音效系统（落子声、胜利音效）
- 在线对战对手头像显示

### Phase 3（探索）
- AI 对局分析模式（显示最佳落子位置）
- 悔棋历史时间轴
- 游戏回放功能
- 云端对局记录同步
