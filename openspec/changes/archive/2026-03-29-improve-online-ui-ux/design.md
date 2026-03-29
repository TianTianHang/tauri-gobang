## Context

当前联机模式的UI存在两个关键问题：
1. **信息流断裂** - NetworkSetup组件显示IP后，用户进入游戏界面（StatusBar/GameInfo），IP信息丢失，导致用户需要记住或在界面间切换
2. **情感反馈不足** - 游戏结束状态仅在GameInfo组件的小字文本中显示，没有模态框、动画或视觉冲击力

**技术约束**：
- Tauri 2 + React 19 + TypeScript，无外部UI库
- 现有组件结构：StatusBar（顶部状态）、GameInfo（侧边信息）、GameBoard（棋盘）
- 已有Tauri事件系统用于网络事件
- 支持桌面和Android平台

**Stakeholders**：
- 联机对战用户（主要）
- 移动端用户（需要触摸友好）

## Goals / Non-Goals

**Goals:**
1. 在游戏界面持久化显示连接信息（IP、端口、连接状态），消除信息断裂
2. 提供视觉冲击力强的游戏结束反馈（全屏模态框 + 动画）
3. 保持现有组件架构，最小化重构
4. 支持深色模式和响应式设计
5. 无障碍友好（ARIA、键盘导航、对比度）

**Non-Goals:**
- 不修改核心游戏逻辑（GameState、网络协议）
- 不添加新的外部依赖（使用现有React hooks和Tauri APIs）
- 不修改现有游戏规则或AI行为
- 不实现重播、录像、观战等高级功能（留待后续）

## Decisions

### 1. 连接信息显示位置选择

**决策**：在StatusBar中添加连接信息区域，位于现有状态文本和菜单按钮之间

**理由**：
- StatusBar是游戏界面的顶部全局区域，用户视线自然停留
- 相比GameInfo（侧边栏），StatusBar在移动端也始终可见
- 最小化对现有布局的影响（只需插入一个flex子元素）

**替代方案**：
- 方案A：在GameInfo中添加 - ❌ 移动端侧边栏可能折叠或不在视口内
- 方案B：新增独立的ConnectionBar组件 - ❌ 增加复杂度和垂直空间占用
- 方案C：在StatusBar和GameInfo中都显示 - ❌ 信息重复，维护成本高

### 2. 模态框实现方式

**决策**：在GameInfo组件中添加条件渲染的全屏模态框，当`gameState.status !== GameStatus.Playing`时显示

**理由**：
- GameInfo已有游戏状态逻辑，是触发模态框的自然位置
- 全屏固定定位覆盖层，不影响现有布局
- 使用React条件渲染，无需额外状态管理

**技术细节**：
```tsx
{gameState.status !== GameStatus.Playing && (
  <div className="game-result-modal victory|defeat">
    {/* 模态框内容 */}
  </div>
)}
```

**替代方案**：
- 方案A：独立的Modal组件 + 状态管理 - ❌ 增加复杂度，过度工程化
- 方案B：使用第三方库（react-modal） - ❌ 不符合"无外部依赖"约束
- 方案C：在GameBoard上叠加 - ❌ GameBoard应专注于游戏渲染，不应混入UI逻辑

### 3. 动画实现方式

**决策**：使用CSS动画（@keyframes）+ CSS过渡，避免JS动画库

**理由**：
- 性能更好（浏览器原生优化）
- 代码更轻量（无额外依赖）
- 易于测试和调试

**动画设计**：
- 模态框淡入 + 缩放：`opacity: 0 → 1`, `transform: scale(0.95) → scale(1)`，300ms ease-out
- 连接状态点脉冲：用于"连接中"状态，复用现有pulse动画

**替代方案**：
- 方案A：使用Framer Motion - ❌ 引入60KB+依赖，违反约束
- 方案B：Web Animations API - ⚠️ 功能强大但代码冗长，CSS动画更简洁

### 4. IP复制功能实现

**决策**：使用Tauri的`writeText` API（clipboard plugin）或浏览器`navigator.clipboard.writeText`

**理由**：
- Tauri 2内置clipboard支持
- 降级方案：浏览器API（桌面环境）
- 添加"已复制！"临时提示，使用`setTimeout`在2秒后清除

**技术细节**：
```tsx
const handleCopyIp = async () => {
  const text = `${localIp}:${port}`;
  try {
    await invoke('write_text', { text }); // Tauri
    // or navigator.clipboard.writeText(text);
    setCopyFeedback('已复制！');
    setTimeout(() => setCopyFeedback(null), 2000);
  } catch (e) {
    console.error('Copy failed:', e);
  }
};
```

**替代方案**：
- 方案A：不提供复制功能，用户手动记忆 - ❌ 用户体验差
- 方案B：使用第三方copy-to-clipboard库 - ❌ 不必要的依赖

### 5. 样式架构

**决策**：遵循现有CSS模式，使用CSS自定义属性（CSS变量）支持深色模式

**理由**：
- 与现有代码库一致（App.css定义了--text-primary等变量）
- 无需重构现有样式系统
- 深色模式通过`@media (prefers-color-scheme: dark)`切换

**新增CSS变量**（如需要）：
```css
:root {
  --status-connected: #22C55E;  /* 绿色 */
  --status-connecting: #F59E0B; /* 黄色 */
  --status-disconnected: #EF4444; /* 红色 */
  --modal-bg-glass: rgba(255, 255, 255, 0.8);
  --modal-victory-gradient: linear-gradient(135deg, #D4AF37 0%, #F59E0B 100%);
}
```

**替代方案**：
- 方案A：使用CSS-in-JS（styled-components） - ❌ 与现有CSS文件模式冲突
- 方案B：使用Tailwind CSS - ❌ 需要重构所有现有样式，成本高

### 6. 图标选择

**决策**：使用SVG内联图标（Heroicons风格），不使用emoji

**理由**：
- 更专业、一致性更好
- 支持深色模式的颜色定制
- 无障碍友好（可添加aria-label）

**图标来源**：
- 胜利：Heroicons `trophy` 或 `check-circle`
- 失败：Heroicons `x-circle` 或自定义灰心图标
- 连接状态：CSS实现的圆点（更轻量）

**替代方案**：
- 方案A：使用emoji - ❌ 不同平台渲染不一致，不专业
- 方案B：引入icon库（react-icons） - ⚠️ 可用但增加了依赖，内联SVG更轻量

## Risks / Trade-offs

### Risk 1: 模态框在低端设备上动画卡顿
**缓解措施**：
- 使用CSS动画（GPU加速）
- 添加`@media (prefers-reduced-motion: reduce)`禁用动画
- 测试时使用Chrome DevTools CPU throttling

### Risk 2: 剪贴板权限被拒绝
**缓解措施**：
- 捕获错误，静默失败（不阻断用户流程）
- 在开发环境中测试不同平台（Windows/macOS/Linux/Android）

### Risk 3: StatusBar在移动端空间不足
**缓解措施**：
- 响应式设计：小屏幕隐藏"房间:"/"已连接:"文字，只显示IP
- 或使用缩写：`192.168...:5555`

### Risk 4: 深色模式下玻璃拟态效果不明显
**缓解措施**：
- 使用更高的不透明度：`rgba(0, 0, 0, 0.8)`（深色模式）
- 测试两种模式下的对比度

### Risk 5: 模态框覆盖层与现有MenuDrawer冲突
**缓解措施**：
- 确保模态框的z-index高于MenuDrawer
- 或在模态框显示时禁用菜单按钮

## Migration Plan

### 阶段1: StatusBar连接信息显示
1. 在App.tsx中保存`localIp`和`port`到state
2. 修改StatusBar.tsx，添加连接信息UI
3. 添加StatusBar.css样式
4. 测试IP复制功能

### 阶段2: 游戏结果模态框
1. 在GameInfo.tsx中添加模态框JSX结构
2. 添加GameInfo.css模态框样式（包括动画）
3. 添加SVG图标
4. 测试胜利/失败触发和模态框显示
5. 测试"再来一局"和"返回菜单"按钮功能

### 阶段3: 响应式和深色模式优化
1. 调整移动端布局（小屏幕IP显示）
2. 测试深色模式下的视觉效果
3. 验证对比度符合4.5:1标准
4. 添加ARIA标签和键盘导航测试

### 阶段4: 性能和无障碍
1. 使用Lighthouse测试性能和可访问性
2. 修复发现的问题
3. 在Android设备上测试触摸交互

### Rollback策略
- 每个阶段独立提交，可随时回滚
- 所有改动为UI增强，不影响核心游戏逻辑
- 如果模态框问题严重，可先禁用动画或整体回滚

## Open Questions

1. **游戏统计信息范围** - 模态框中应显示哪些统计？
   - 当前想法：步数、用时
   - **待决策**：是否显示"平均思考时间"、"悔棋次数"等高级统计？

2. **IP显示格式** - 移动端是否需要完全隐藏IP？
   - 当前想法：缩写显示（如`192.168...:5555`）
   - **待决策**：还是隐藏IP，只显示"已连接"状态？

3. **模态框按钮行为** - "再来一局"是否自动重置游戏？
   - 当前想法：调用`onNewGame`，相当于"新游戏"按钮
   - **待决策**：联机模式下"再来一局"是否需要重新匹配？

4. **连接状态检测** - 如何实时检测连接断开？
   - 当前想法：依赖现有的`network:disconnected`事件
   - **待决策**：是否需要心跳机制？（可能超出本次范围）
