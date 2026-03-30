#!/usr/bin/env bash
# Nix环境验证脚本

set -e

echo "🔍 验证Nix Android开发环境配置"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 检查文件是否存在
echo "📁 检查文件..."
files=("flake.nix" ".envrc" ".gitignore")
for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    echo "  ✅ $file"
  else
    echo "  ❌ $file 缺失"
    exit 1
  fi
done
echo ""

# 检查git状态
echo "📊 Git状态..."
if git diff --cached --name-only | grep -q "flake.nix"; then
  echo "  ✅ flake.nix 已暂存"
else
  echo "  ⚠️  flake.nix 未暂存"
fi
echo ""

# 检查direnv
echo "🔧 direnv状态..."
if command -v direnv &> /dev/null; then
  echo "  ✅ direnv 已安装: $(direnv --version | head -n1)"
else
  echo "  ❌ direnv 未安装"
  echo "     安装: Ubuntu/Debian: sudo apt-get install direnv"
  echo "           Arch: sudo pacman -S direnv"
  exit 1
fi
echo ""

# 检查direnv hook
echo "🔗 direnv hook..."
if [ -n "$BASH_VERSION" ]; then
  if grep -q "direnv hook bash" ~/.bashrc 2>/dev/null || grep -q "direnv hook bash" ~/.bash_profile 2>/dev/null; then
    echo "  ✅ Bash hook已配置"
  else
    echo "  ⚠️  Bash hook未配置"
    echo "     添加: echo 'eval \"\$(direnv hook bash)\"' >> ~/.bashrc"
  fi
elif [ -n "$ZSH_VERSION" ]; then
  if grep -q "direnv hook zsh" ~/.zshrc 2>/dev/null; then
    echo "  ✅ Zsh hook已配置"
  else
    echo "  ⚠️  Zsh hook未配置"
    echo "     添加: echo 'eval \"\$(direnv hook zsh)\"' >> ~/.zshrc"
  fi
fi
echo ""

# 检查.envrc是否被允许
echo "🔐 .envrc权限..."
if direnv status | grep -q "Allowed"; then
  echo "  ✅ .envrc 已允许"
else
  echo "  ⚠️  .envrc 未允许"
  echo "     运行: direnv allow"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "✅ 配置文件验证完成！"
echo ""
echo "📝 下一步:"
echo "   1. 如GitHub API限流，等待1小时后运行: nix flake update"
echo "   2. 允许direnv: direnv allow"
echo "   3. 进入环境: nix develop (或重新进入目录触发direnv)"
echo "   4. 验证环境: echo \$ANDROID_NDK_HOME"
echo ""
