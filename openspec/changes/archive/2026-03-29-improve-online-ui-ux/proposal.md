## Why

联机模式存在两个用户体验问题：
1. **创建房间后IP地址不可见** - 用户创建房间后进入游戏界面，无法再次查看自己的IP地址告诉对手，需要在两个界面间切换，认知负担高
2. **胜利/失败状态显示不明显** - 游戏结束时仅在状态栏显示小字文本，缺乏视觉冲击力和情感反馈，用户无法充分体验胜负结果

这些问题影响了联机对战的可用性和情感体验，降低了用户满意度。

## What Changes

### 界面改进
- **StatusBar 持久化连接信息显示**：
  - 在游戏界面顶部始终显示IP地址和端口
  - 添加连接状态指示器（绿/黄/红点）
  - 支持点击复制IP地址到剪贴板
  - 响应式布局，移动端自适应显示

- **游戏结果模态框**：
  - 全屏半透明覆盖层，玻璃拟态效果
  - 胜利：金色主题，大号图标和"你赢了！"文字
  - 失败：灰色主题，柔和的"你输了..."文字
  - 显示游戏统计（用时、步数等）
  - 提供"再来一局"和"返回菜单"操作按钮
  - 弹出动画效果（淡入 + 缩放）

### 交互优化
- **IP复制反馈** - 点击IP后显示"已复制！"提示，2秒后自动消失
- **模态框动画** - 300ms ease-out 动画，提升视觉冲击力
- **无障碍支持** - ARIA标签、键盘导航、4.5:1对比度

### 视觉设计
- **SVG图标** - 使用Heroicons/Lucide图标替代emoji，提升专业性
- **深色模式适配** - 所有新增UI支持深色主题
- **响应式设计** - 移动端触摸目标最小44x44px

## Capabilities

### New Capabilities
- `connection-info-display`: 在游戏界面顶部持久化显示联机连接信息（IP、端口、连接状态）
- `game-result-modal`: 游戏结束时的全屏模态框，提供明确的胜负反馈和后续操作

### Modified Capabilities
- `online-game-play`: 现有联机游戏体验增强，但不改变核心游戏逻辑

## Impact

### Affected Components
- **Frontend**:
  - `src/components/StatusBar.tsx` - 添加连接信息显示区域
  - `src/components/StatusBar.css` - 新增连接信息样式
  - `src/components/GameInfo.tsx` - 添加游戏结果模态框逻辑
  - `src/components/GameInfo.css` - 新增模态框样式
  - `src/App.tsx` - 传递IP/端口信息到StatusBar
  - `src/App.css` - 可能需要调整全局CSS变量

### Dependencies
- 无新增外部依赖
- 使用现有React hooks和Tauri APIs
- SVG图标可直接内联或使用Heroicons

### Backwards Compatibility
- **Non-breaking**: 所有改动为UI增强，不改变现有API或数据流
- **Progressive Enhancement**: 核心游戏功能不受影响，模态框为新增体验

### Testing Requirements
- 测试IP复制功能（剪贴板权限）
- 测试模态框在不同屏幕尺寸下的显示
- 测试胜利/失败状态的正确触发
- 测试深色模式下的视觉效果
- 测试动画性能（60fps）
