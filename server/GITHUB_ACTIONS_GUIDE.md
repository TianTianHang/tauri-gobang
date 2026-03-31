# GitHub Actions 构建指南

## 📦 概述

本项目使用 GitHub Actions 在 Ubuntu 18.04 环境中自动编译 gobang-server，确保生成的二进制文件兼容 Ubuntu 16.04+ 系统。

## 🚀 自动构建工作流

### 1. Server Build (主要工作流)

**触发条件：**
- 推送代码到 `main` 或 `master` 分支（修改 `server/` 目录时）
- 创建/更新 Pull Request
- 手动触发（可在 GitHub 网页操作）

**构建环境：**
- Ubuntu 18.04 (glibc 2.27)
- Rust stable toolchain

**产物命名：**
```
gobang-server-ubuntu18.04-v<版本>-<commit-hash>
```

示例：
```
gobang-server-ubuntu18.04-v0.1.0-a1b2c3d
```

---

### 2. Server Release (发布工作流)

**触发条件：**
- 推送 tag: `server-v*` (例如 `server-v0.1.0`)

**功能：**
- 编译二进制文件
- 自动创建 GitHub Release
- 上传构建产物到 Release

---

## 📖 使用方法

### 方式一：自动构建 (推荐)

```bash
# 1. 修改 server/ 目录的代码
# 2. 提交并推送
git add server/
git commit -m "feat: 添加新功能"
git push origin main

# 3. GitHub Actions 自动触发构建
# 4. 等待构建完成 (~5-10 分钟)
# 5. 下载 artifact
```

### 方式二：手动触发

1. 访问 GitHub 仓库页面
2. 点击 "Actions" 标签
3. 选择 "Server Build (Ubuntu 18.04)" 工作流
4. 点击 "Run workflow" 按钮
5. 可选：输入自定义版本号
6. 点击绿色的 "Run workflow" 开始构建

### 方式三：创建 Release

```bash
# 1. 更新 Cargo.toml 中的版本号
vim server/Cargo.toml

# 2. 提交并推送
git add server/Cargo.toml
git commit -m "chore: bump version to 0.2.0"
git push origin main

# 3. 创建并推送 tag
git tag server-v0.2.0
git push origin server-v0.2.0

# 4. GitHub Actions 自动创建 Release 并上传二进制
```

---

## 📥 下载构建产物

### 从 Actions 页面下载

1. 访问 GitHub Actions 页面
2. 选择成功的构建运行
3. 滚动到页面底部的 "Artifacts" 区域
4. 点击下载文件（`.tar.gz` 格式）

### 从 Release 下载

1. 访问 GitHub Releases 页面
2. 选择对应的版本
3. 下载 `gobang-server` 二进制文件

### 解压 Artifact

```bash
# 下载后解压
tar xzf gobang-server-ubuntu18.04-v0.1.0-a1b2c3d.tar.gz

# 进入目录
cd gobang-server-ubuntu18.04-v0.1.0-a1b2c3d/

# 查看内容
ls -l
# gobang-server       ← 可执行文件
# BUILD_INFO.txt      ← 构建信息

# 测试运行
./gobang-server --version
```

---

## 🔍 兼容性说明

**编译环境：** Ubuntu 18.04 (glibc 2.27)

**兼容的系统：**
- ✅ Ubuntu 14.04+ (glibc 2.19+)
- ✅ Ubuntu 16.04+ (glibc 2.23+)
- ✅ Ubuntu 18.04+ (glibc 2.27+)
- ✅ Ubuntu 20.04 / 22.04 / 24.04
- ✅ Debian 8+ ( Jessie+ )
- ✅ CentOS 7+ (可能需要)

**不兼容的系统：**
- ❌ Ubuntu 12.04 及更早版本 (glibc < 2.15)
- ❌ Debian 7 及更早版本

---

## 🛠️ 本地构建 (不推荐)

如果你需要在本地构建（不使用 GitHub Actions）：

### 使用 Docker (推荐)

```bash
# 在 Ubuntu 18.04 容器中构建
docker run -it --rm \
  -v "$PWD:/app" \
  -w /app/server \
  ubuntu:18.04 \
  bash -c "
    apt-get update &&
    apt-get install -y curl &&
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y &&
    . ~/.cargo/env &&
    cargo build --release &&
    strip target/release/gobang-server
  "
```

### 在 Ubuntu 18.04 系统上

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 构建
cd server
cargo build --release
strip target/release/gobang-server
```

---

## 📊 构建时间

| 场景 | 时间 |
|------|------|
| 首次构建（无缓存） | ~8-10 分钟 |
| 缓存命中 | ~3-5 分钟 |
| 下载 artifact | ~1-2 分钟 |

---

## 🐛 故障排查

### 问题：构建失败

**检查：**
1. 查看Actions日志，确认错误信息
2. 检查 `server/Cargo.toml` 语法是否正确
3. 确认代码能本地编译通过

### 问题：下载的 artifact 损坏

**解决：**
```bash
# 验证压缩包完整性
tar tzf gobang-server-ubuntu18.04-v0.1.0-xxx.tar.gz

# 如果报错，重新下载或重新构建
```

### 问题：二进制无法运行

**检查：**
```bash
# 查看二进制信息
file gobang-server

# 检查动态链接
ldd gobang-server

# 查看 glibc 版本需求
objdump -T gobang-server | grep GLIBC | sort -u
```

---

## 📝 构建信息说明

每个构建产物包含 `BUILD_INFO.txt` 文件：

```
Gobang Server Build Information
================================
Version: 0.1.0
Commit: a1b2c3d
Build Date: 2026-03-31 11:00:00 UTC
Runner OS: Ubuntu 18.04
Rust Version: rustc 1.94.1
Cargo Version: cargo 1.94.1
Git Branch: main
GitHub Run: 42
```

---

## 🔗 相关链接

- **GitHub Actions**: https://github.com/TianTianHang/tauri-gobang/actions
- **Server 源码**: `server/` 目录
- **构建脚本**: `server/build.sh` (本地构建用)

---

## 💡 提示

1. **首次使用**：推送一个小改动来测试 workflow 是否正常工作
2. **定期更新**：GitHub Actions 会自动使用最新的 Rust 稳定版
3. **保留时间**：Artifacts 自动保留 90 天
4. **手动触发**：适合需要频繁构建测试的场景

---

**有问题？** 请在 GitHub Issues 中反馈。
