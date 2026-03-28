# AI 引擎集成完成 ✅

内置 AI 已移除，应用现在完全使用 **Rapfi** 五子棋引擎。

## 📦 快速开始

### 1. 下载 Rapfi 引擎

```bash
cd src-tauri

# Linux/macOS
./download-sidecar.sh

# Windows
download-sidecar.bat
```

### 2. 构建应用

```bash
pnpm tauri build
```

### 3. 使用

```typescript
// 自动使用打包的引擎
const result = await invoke('ai_move', {
  state: gameState,
  difficulty: 'hard',
  enginePath: null  // null = 自动查找
});
```

## 📊 架构变更

| 项目 | 之前 | 现在 |
|------|------|------|
| AI 实现 | 内置 Negamax (683行) | Rapfi 引擎 |
| 代码量 | 683 行 + 包装器 | 219 行包装器 |
| AI 强度 | ~1500 ELO | **2000+ ELO** |
| 算法 | Negamax + Alpha-Beta | Alpha-Beta + NNUE |
| 依赖 | 无 | 外部二进制 |

## 🎯 优势

✅ **简化** - 删除 683 行复杂算法代码
✅ **强化** - AI 水平提升 500+ ELO
✅ **专业** - 使用比赛验证的引擎
✅ **兼容** - API 保持不变，前端无需修改

## 🤖 Android 支持

Android 平台通过从 APK assets 中提取 rapfi 二进制文件来支持 AI 功能，无需额外安装引擎。

**支持的架构：** `aarch64` (ARM64)、`x86_64`

**工作原理：**
1. 首次启动时自动检测设备架构
2. 从 `assets/binaries/` 提取对应架构的 rapfi 二进制到应用缓存目录
3. 设置可执行权限 (chmod 755)
4. 后续启动通过缓存检查跳过提取
5. 提取失败时回退到默认路径查找（向后兼容）

**相关文件：**
- `src-tauri/src/android_rapfi.rs` — Android 二进制提取模块（`#[cfg(target_os = "android")]`）
- `src-tauri/binaries/rapfi-aarch64-linux-android` — ARM64 Android 二进制
- `src-tauri/binaries/rapfi-x86_64-linux-android` — x86_64 Android 二进制

## 📚 文档

- [完整集成指南](docs/SIDECAR_INTEGRATION.md)
- [变更说明](docs/BUILTIN_AI_REMOVED.md)
