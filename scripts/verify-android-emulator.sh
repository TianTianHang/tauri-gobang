#!/usr/bin/env bash
# Android模拟器环境验证脚本

set -e

echo "🔍 验证Android模拟器环境配置"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查函数
check_pass() {
  echo -e "${GREEN}✅ $1${NC}"
}

check_fail() {
  echo -e "${RED}❌ $1${NC}"
}

check_warn() {
  echo -e "${YELLOW}⚠️  $1${NC}"
}

# 1. 检查Nix环境
echo "📦 Nix环境检查..."
if [ -n "$IN_NIX_SHELL" ] || [ -n "$NIX_BUILD_TOP" ]; then
  check_pass "在Nix环境中"
else
  check_warn "不在Nix环境中"
  echo "   运行: nix develop"
fi
echo ""

# 2. 检查Android SDK
echo "🤖 Android SDK检查..."
if [ -n "$ANDROID_HOME" ]; then
  check_pass "ANDROID_HOME: $ANDROID_HOME"
else
  check_fail "ANDROID_HOME 未设置"
  exit 1
fi

if [ -n "$ANDROID_SDK_ROOT" ]; then
  check_pass "ANDROID_SDK_ROOT: $ANDROID_SDK_ROOT"
else
  check_fail "ANDROID_SDK_ROOT 未设置"
  exit 1
fi
echo ""

# 3. 检查Android NDK
echo "🔧 Android NDK检查..."
if [ -n "$ANDROID_NDK_HOME" ]; then
  check_pass "ANDROID_NDK_HOME: $ANDROID_NDK_HOME"

  if [ -d "$ANDROID_NDK_HOME" ]; then
    check_pass "NDK目录存在"

    if [ -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt" ]; then
      check_pass "NDK toolchains存在"
    else
      check_fail "NDK toolchains不存在"
    fi
  else
    check_fail "NDK目录不存在: $ANDROID_NDK_HOME"
  fi
else
  check_fail "ANDROID_NDK_HOME 未设置"
  exit 1
fi
echo ""

# 4. 检查模拟器
echo "📱 模拟器检查..."
if command -v emulator &> /dev/null; then
  check_pass "emulator命令可用"

  # 检查系统镜像
  API_LEVEL="35"
  ABI="x86_64"
  SYSTEM_IMAGE_DIR="$ANDROID_HOME/system-images/android-$API_LEVEL/google_apis/$ABI"

  if [ -d "$SYSTEM_IMAGE_DIR" ]; then
    check_pass "系统镜像存在: API $API_LEVEL ($ABI)"
  else
    check_fail "系统镜像缺失: $SYSTEM_IMAGE_DIR"
    echo "   请检查 flake.nix 配置"
    exit 1
  fi
else
  check_fail "emulator命令不可用"
  echo "   请确保在Nix环境中: nix develop"
  exit 1
fi
echo ""

# 5. 检查AVD管理工具
echo "🛠️  AVD管理工具检查..."
if command -v avdmanager &> /dev/null; then
  check_pass "avdmanager命令可用"
else
  check_fail "avdmanager命令不可用"
  exit 1
fi

if command -v adb &> /dev/null; then
  check_pass "adb命令可用"
else
  check_fail "adb命令不可用"
  exit 1
fi
echo ""

# 6. 检查KVM支持
echo "⚡ KVM虚拟化检查..."
if [ -e /dev/kvm ]; then
  check_pass "/dev/kvm 存在"

  if [ -r /dev/kvm ] && [ -w /dev/kvm ]; then
    check_pass "KVM设备可读写"
  else
    check_warn "KVM设备权限不足"
    echo "   运行: sudo usermod -a -G kvm \$USER"
    echo "   然后重新登录"
  fi
else
  check_warn "KVM不可用"
  echo "   模拟器会使用软件加速（较慢）"
fi
echo ""

# 7. 检查已创建的AVD
echo "📋 已创建的AVD..."
AVD_DIR="$HOME/.android/avd"
if [ -d "$AVD_DIR" ]; then
  AVD_COUNT=$(find "$AVD_DIR" -maxdepth 1 -name "*.avd" -type d 2>/dev/null | wc -l)
  if [ "$AVD_COUNT" -gt 0 ]; then
    check_pass "找到 $AVD_COUNT 个AVD"
    echo "   AVD列表:"
    find "$AVD_DIR" -maxdepth 1 -name "*.avd" -type d -exec basename {} .avd \; | sed 's/^/     • /'
  else
    check_warn "未找到AVD"
    echo "   运行: bash scripts/create-android-avd.sh"
  fi
else
  check_warn "AVD目录不存在: $AVD_DIR"
fi
echo ""

# 8. 检查连接的设备
echo "🔗 连接的设备..."
if command -v adb &> /dev/null; then
  DEVICE_COUNT=$(adb devices 2>/dev/null | grep -v "List of devices" | grep -c "device" || true)
  if [ "$DEVICE_COUNT" -gt 0 ]; then
    check_pass "找到 $DEVICE_COUNT 个设备"
    adb devices 2>/dev/null | grep "device" | sed 's/^/     /'
  else
    check_warn "没有连接的设备"
    echo "   • 启动模拟器: emulator -avd tauri-gobang-avd"
    echo "   • 或连接真实设备并启用USB调试"
  fi
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "✅ 验证完成！"
echo ""
echo "📝 下一步:"
if [ "$AVD_COUNT" -eq 0 ]; then
  echo "   1. 创建AVD: bash scripts/create-android-avd.sh"
  echo "   2. 启动模拟器: emulator -avd tauri-gobang-avd"
else
  echo "   1. 启动模拟器: emulator -avd <name>"
fi
echo "   2. 运行应用: pnpm tauri android dev"
echo ""
