# Nix环境快速参考

## 一分钟启动

```bash
# 首次使用
direnv allow
nix flake update

# 进入目录自动激活
cd tauri-gobang/

# 开发
pnpm tauri android dev
```

## Android模拟器

```bash
# 创建虚拟设备
bash scripts/create-android-avd.sh

# 启动模拟器
emulator -avd tauri-gobang-avd

# 列出AVD
emulator -list-avds

# 使用快照加速
emulator -avd tauri-gobang-avd -snapshot quickboot
```

## 常用命令

```bash
# Nix环境
nix develop              # 进入环境
nix flake update         # 更新依赖
nix flake check          # 验证配置

# direnv
direnv allow            # 允许.envrc
direnv reload           # 重新加载
direnv status           # 查看状态

# 验证环境
echo $ANDROID_NDK_HOME  # 检查NDK
rustc --version         # 检查Rust
gradle --version        # 检查Gradle
```

## 环境变量

| 变量 | 说明 |
|------|------|
| `ANDROID_HOME` | SDK路径 |
| `ANDROID_NDK_HOME` | NDK路径 |
| `ANDROID_SDK_ROOT` | SDK根路径 |
| `GRADLE_OPTS` | aapt2配置 |

## 故障排除

| 问题 | 解决 |
|------|------|
| direnv未加载 | `direnv reload` |
| 环境变量为空 | `nix develop` |
| Gradle失败 | 检查 `ANDROID_NDK_HOME` |
| API限流 | 等待1小时后重试 |

## 完整文档

- [实施总结](NIX_SETUP_SUMMARY.md)
- [AGENTS.md](../AGENTS.md#nix开发环境)
- [README.md](../README.md#快速开始-nix---推荐)
