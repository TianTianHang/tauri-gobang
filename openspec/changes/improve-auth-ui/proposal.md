## Why

当前登录/注册界面 UI 设计较为简陋，缺少品牌展示和视觉吸引力。表单字段缺少图标和交互反馈，错误提示显示在顶部不够直观，密码字段缺少可见性切换，登录/注册模式切换不够友好。这些问题影响用户的第一印象和注册转化率。

## What Changes

- **品牌区域**：添加棋子图标（BlackStoneIcon）和欢迎/副标题（"欢迎回到五子棋"/"创建五子棋账号"）
- **表单布局**：为每个输入框添加图标占位符（UserIcon、LockIcon），使用输入框组布局
- **密码可见性**：添加眼睛图标按钮，允许用户切换密码显示/隐藏
- **错误提示**：将顶部错误消息移到相关字段下方，使用内联错误样式（带图标）
- **切换按钮**：将底部的文字链接改为虚线边框按钮（"立即注册"/"去登录"）
- **次要操作**：返回按钮改为次要样式（描边按钮）
- **图标组件**：添加 EyeIcon、EyeOffIcon、LockIcon、ExclamationCircleIcon

## Capabilities

### New Capabilities

无（本次变更纯 UI 优化，不涉及新功能）

### Modified Capabilities

无（登录/注册功能行为不变，仅视觉呈现优化）

## Impact

**前端代码：**
- `src/components/LoginScreen.tsx` - 添加品牌区域、输入框组、密码切换、内联错误、状态管理
- `src/components/LoginScreen.css` - 添加品牌、输入框组、错误提示、切换按钮样式
- `src/components/Icons.tsx` - 添加 EyeIcon、EyeOffIcon、LockIcon、ExclamationCircleIcon

**后端：**
无（纯前端变更，登录/注册 API 调用逻辑不变）

**依赖：**
无新增依赖（使用现有 Heroicons 风格的 SVG 图标）
