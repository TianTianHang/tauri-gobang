## Context

**当前状态：**
- Rapfi源码位于`third-party/rapfi.tmp/`（手动克隆，非Git跟踪）
- 补丁维护在`patches/rapfi-android/`，格式规范但需手动应用
- 权重文件（`*.bin.lz4`）直接放在`src-tauri/binaries/`并被git跟踪
- `.gitignore`配置矛盾：第11行忽略`src-tauri/binaries/`，但该目录文件实际被跟踪
- Networks子模块（包含最新权重）存在于rapfi源码中但未利用

**约束条件：**
- 必须保持现有Android构建流程正常工作（Gradle copyRapfiBinaries任务）
- 不能破坏干净的clone-to-build流程
- 需要兼容现有开发者环境（已有本地文件的开发者）
- 权重文件总计约40MB，在git可接受范围内

**利益相关者：**
- 新贡献者：需要快速clone-init-build体验
- 维护者：需要清晰的权重更新流程
- CI/CD：需要自动化构建流程

## Goals / Non-Goals

**Goals：**
- 自动化rapfi源码获取（Git子模块替代手动克隆）
- 明确权重文件版本来源（从Networks子模块同步）
- 修复.gitignore配置矛盾
- 提供一键setup脚本（补丁应用 + 子模块初始化）
- 保持向后兼容（现有开发者可平滑迁移）

**Non-Goals：**
- 不修改rapfi引擎本身的行为或API
- 不改变Android运行时加载逻辑（`android_rapfi.rs`保持不变）
- 不引入新的外部依赖（除rapfi和Networks子模块）
- 不迁移到Git LFS（权重文件直接跟踪）

## Decisions

### 决策1：使用Git子模块而非手动克隆

**选择：** 添加`third-party/rapfi`为Git子模块

**理由：**
- ✅ 自动跟踪上游更新：`git submodule update --remote`
- ✅ 清晰的依赖关系：`.gitmodules`明确声明
- ✅ 团队协作友好：`git clone --recursive`自动初始化
- ✅ CI/CD友好：标准Git命令即可

**替代方案及拒绝理由：**
- 方案A：保持手动克隆 + setup脚本
  - ❌ 仍需手动下载，增加新贡献者门槛
- 方案B：使用git subtree
  - ❌ 合并上游更新复杂，子模块更适合第三方依赖
- 方案C：不跟踪源码，仅跟踪二进制
  - ❌ 丢失补丁应用历史，难以重建Android二进制

### 决策2：.gitignore精确控制而非全忽略

**选择：** 移除`src-tauri/binaries/`的完全忽略，改用精确模式

**当前配置：**
```
src-tauri/binaries/  # 忽略整个目录
```

**新配置：**
```
# 忽略开发时的可执行文件（不同平台）
src-tauri/binaries/rapfi
src-tauri/binaries/pbrain-rapfi-*

# 保留Android特定二进制（被git跟踪）
!src-tauri/binaries/rapfi-*-linux-android

# 保留配置和权重文件（被git跟踪）
!src-tauri/binaries/config.toml
!src-tauri/binaries/*.bin.lz4
```

**理由：**
- ✅ Android二进制被Gradle复制到jniLibs，必须版本控制
- ✅ 权重文件40MB总大小可接受，不应被忽略
- ✅ 避免未来意外忽略重要文件
- ✅ 清晰分离：开发二进制忽略，发布二进制跟踪

### 决策3：权重文件从Networks子模块同步

**选择：** 创建`scripts/sync-weights-from-networks.sh`从`third-party/rapfi/Networks/`复制到`src-tauri/binaries/`

**工作流程：**
```bash
# rapfi更新Networks子模块后
cd third-party/rapfi
git submodule update --remote Networks

# 同步到项目binaries/
cd ../..
bash scripts/sync-weights-from-networks.sh

# 提交更新的权重文件
git add src-tauri/binaries/*.bin.lz4
git commit -m "chore: update NNUE weights from rapfi Networks"
```

**理由：**
- ✅ 单一数据源：Networks子模块是官方权重仓库
- ✅ 版本可追溯：通过git commit知道权重更新时间
- ✅ 自动化：脚本替代手动复制，减少错误
- ✅ 可选操作：不强制每次构建都同步，灵活控制

**替代方案及拒绝理由：**
- 方案A：让rapfi直接从Networks子模块读取权重
  - ❌ 相对路径复杂（`../../../../third-party/rapfi/...`）
  - ❌ 跨平台路径处理麻烦
  - ❌ 增加运行时复杂度
- 方案B：使用Git LFS管理权重文件
  - ❌ GitHub LFS有存储限制（免费1GB）
  - ❌ 40MB总大小在可接受范围内，不急需LFS

### 决策4：保留补丁系统而非提交到上游

**选择：** 继续在本地维护补丁（`patches/rapfi-android/`），定期尝试合并到上游

**理由：**
- ✅ 快速迭代：不需等待上游review
- ✅ 针对性优化：补丁专门为本项目的Android构建优化
- ✅ 向后兼容：上游API变化时可快速调整
- ✅ 已有完整系统：apply/revert脚本、文档齐全

**风险缓解：**
- 定期检查上游是否有类似修复（每个季度）
- 在补丁comment中标注原因和上游issue链接
- 考虑未来提交PR到上游（如果补丁通用性强）

## Risks / Trade-offs

### 风险1：子模块增加clone复杂度

**风险：** 新贡献者可能忘记使用`--recursive`，导致clone后源码缺失

**缓解措施：**
- README.md醒目位置标注：
  ```bash
  git clone --recursive https://github.com/xxx/tauri-gobang.git
  ```
- 提供`scripts/ensure-submodules.sh`检查脚本：
  ```bash
  #!/bin/bash
  if [ ! -d "third-party/rapfi" ]; then
    echo "⚠️  Submodules not initialized. Running: git submodule update --init"
    git submodule update --init --recursive
  fi
  ```
- CI/CD中添加检查步骤，失败时提示完整clone命令

### 风险2：现有开发者本地环境冲突

**风险：** 已有`third-party/rapfi.tmp/`的开发者可能遇到路径冲突

**缓解措施：**
- 在README.md添加迁移指南：
  ```bash
  # 如果你有旧的rapfi.tmp目录，请执行：
  rm -rf third-party/rapfi.tmp  # 可选：备份
  git submodule update --init --recursive
  bash scripts/apply-rapfi-patches.sh
  ```
- setup脚本检测并提示冲突：
  ```bash
  if [ -d "third-party/rapfi.tmp" ]; then
    echo "⚠️  检测到旧的rapfi.tmp目录，建议删除后继续"
  fi
  ```

### 风险3：Networks子模块可能未初始化

**风险：** `third-party/rapfi/Networks/`可能未初始化，导致权重同步失败

**缓解措施：**
- sync-weights脚本添加检查：
  ```bash
  if [ ! -d "$NETWORKS_DIR" ]; then
    echo "❌ Networks子模块未初始化，运行："
    echo "   cd third-party/rapfi && git submodule update --init --recursive"
    exit 1
  fi
  ```
- setup-rapfi-source.sh自动初始化子模块：
  ```bash
  cd third-party/rapfi
  git submodule update --init --recursive
  ```

### 风险4：权重文件更新不及时

**风险：** 权重文件可能落后于Networks子模块最新版本

**缓解措施：**
- 在config.toml中添加注释标记更新日期：
  ```toml
  # Weights updated: 2025-06-10 from Networks@8a75d42
  ```
- CI/CD中添加检查（可选）：比较权重文件日期与Networks子模块commit日期
- 文档说明：权重更新不影响AI正确性，仅影响强度，可按需更新

### 权衡1：增加.gitmodules复杂度 vs 自动化收益

**选择：** 接受子模块的轻微复杂度，换取自动化跟踪上游的能力

**理由：**
- 子模块是Git标准功能，开发者熟悉
- `--recursive`是一次性成本，后续`git pull`自动处理
- 自动化收益远超复杂度成本：更新、review、协作都更简单

### 权衡2：权重文件git跟踪 vs LFS

**选择：** 直接git跟踪40MB权重文件

**理由：**
- 40MB总大小在可接受范围（GitHub单文件100MB限制）
- 避免LFS配置和带宽成本
- 简化clone流程（无需LFS客户端）

## Migration Plan

### 阶段1：准备变更（1天）

1. 创建`scripts/sync-weights-from-networks.sh`
2. 创建`scripts/setup-rapfi-source.sh`
3. 准备`.gitmodules`配置文件
4. 修改`.gitignore`配置

### 阶段2：添加子模块（1天）

1. 提交所有新脚本和配置更改
2. 添加rapfi为Git子模块：
   ```bash
   git submodule add https://github.com/dhbloo/rapfi.git third-party/rapfi
   cd third-party/rapfi
   git submodule update --init --recursive
   cd ../..
   ```
3. 应用补丁：
   ```bash
   bash scripts/apply-rapfi-patches.sh
   ```
4. 验证构建：`pnpm tauri android dev`

### 阶段3：更新文档（半天）

1. 更新`README.md`：
   - 在"快速开始"部分添加`git clone --recursive`
   - 添加"从手动克隆迁移"小节
2. 更新`docs/PATCH_MAINTENANCE.md`：补充子模块说明
3. 可选：更新`src-tauri/android/README.md`

### 阶段4：CI/CD集成（半天）

1. 在CI配置中添加子模块初始化：
   ```yaml
   - name: Checkout submodules
     run: git submodule update --init --recursive
   ```
2. 添加补丁应用步骤：
   ```yaml
   - name: Apply rapfi patches
     run: bash scripts/apply-rapfi-patches.sh
   ```

### 阶段5：清理旧文件（可选，1天）

1. 确认所有测试通过
2. 可选删除`third-party/rapfi.tmp/`（文档说明保留也可）
3. 更新所有引用`rapfi.tmp`的文档

### 回滚策略

如果出现严重问题：
1. 恢复`.gitmodules`：`git revert <commit>`
2. 恢复`.gitignore`：`git checkout HEAD~1 .gitignore`
3. 保留`third-party/rapfi/`作为参考（不删除）
4. 临时使用`rapfi.tmp/`重新构建Android二进制

## Open Questions

1. **Q: 是否需要在CI中自动同步权重文件？**
   - A: 暂不自动。权重更新不影响CI测试（AI功能可mock或skip），手动同步更可控。

2. **Q: Networks子模块更新频率如何？**
   - A: 根据rapfi release节奏（当前约每季度）。不需要每次rapfi commit都更新。

3. **Q: 是否需要为补丁创建upstream PR？**
   - A: 中期目标（3-6个月）。先确保本地补丁稳定，再评估通用性提交上游。

4. **Q: .gitignore中是否需要忽略`third-party/rapfi.tmp/`？**
   - A: 是的，添加避免意外提交旧目录：
      ```
      third-party/rapfi.tmp/
      ```
