## Why

当前项目使用手动克隆的rapfi源码（`third-party/rapfi.tmp/`），导致：
- 干净clone后缺失rapfi源码，需要手动下载
- 补丁维护虽然完善但需要手动应用
- 权重文件版本不清晰，与Networks子模块未同步
- `.gitignore`配置矛盾（忽略`src-tauri/binaries/`但文件被git跟踪）

这些问题阻碍了新贡献者的快速上手，增加了维护成本，且权重文件更新流程不明确。现在是规范化管理的最佳时机。

## What Changes

- **Git子模块化rapfi源码**：添加`third-party/rapfi`为Git子模块，替代手动克隆的`rapfi.tmp/`
- **修复.gitignore配置**：移除对`src-tauri/binaries/`的过度忽略，或精确控制忽略规则
- **创建权重同步脚本**：`scripts/sync-weights-from-networks.sh`从Networks子模块同步最新权重
- **创建源码setup脚本**：`scripts/setup-rapfi-source.sh`自动化补丁应用和子模块初始化
- **更新README文档**：添加完整的clone-init-dev流程说明
- **CI/CD集成**：在CI流程中添加rapfi源码和补丁应用步骤

## Capabilities

### New Capabilities
- `rapfi-source-management`: 规范化rapfi源码的获取、初始化和补丁管理
- `weight-files-sync`: 权重文件从Networks子模块到binaries/的自动化同步

### Modified Capabilities
（无。此变更主要改进内部构建流程，不改变外部规格需求）

## Impact

- **代码结构**：
  - 新增：`third-party/rapfi/`（Git子模块）
  - 删除：`third-party/rapfi.tmp/`（手动克隆，可选择性移除）
  - 修改：`.gitmodules`（新增子模块配置）
  - 修改：`.gitignore`（调整binaries/忽略规则）

- **构建脚本**：
  - 新增：`scripts/sync-weights-from-networks.sh`
  - 新增：`scripts/setup-rapfi-source.sh`
  - 保留：`scripts/build-android-rapfi.sh`、`scripts/apply-rapfi-patches.sh`、`scripts/revert-rapfi-patches.sh`

- **文档**：
  - 修改：`README.md`（添加子模块初始化步骤）
  - 修改：`src-tauri/android/README.md`（更新构建流程）
  - 可选更新：`docs/PATCH_MAINTENANCE.md`（补充子模块说明）

- **CI/CD**：
  - 需要在CI配置中添加`git submodule update --init`步骤
  - 需要运行补丁应用脚本

- **开发者体验**：
  - 新clone命令：`git clone --recursive <repo>`
  - 现有开发者需运行一次：`git submodule update --init --recursive`

- **依赖**：
  - 无新增外部依赖
  - Networks子模块（rapfi的子模块）将自动初始化
