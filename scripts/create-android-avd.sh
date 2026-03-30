#!/usr/bin/env bash
# 创建Android虚拟设备(AVD)

set -e

echo "📱 创建Android虚拟设备"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 检查环境
if [ -z "$ANDROID_HOME" ]; then
  echo "❌ 错误: ANDROID_HOME 未设置"
  echo "   请先运行: nix develop"
  exit 1
fi

if [ -z "$ANDROID_NDK_HOME" ]; then
  echo "❌ 错误: ANDROID_NDK_HOME 未设置"
  echo "   请先运行: nix develop"
  exit 1
fi

# 检查avdmanager和emulator
if ! command -v avdmanager &> /dev/null; then
  echo "❌ 错误: avdmanager 未找到"
  echo "   请确保已在Nix环境中"
  exit 1
fi

if ! command -v emulator &> /dev/null; then
  echo "❌ 错误: emulator 未找到"
  echo "   请确保已在Nix环境中"
  exit 1
fi

# AVD配置
AVD_NAME="tauri-gobang-avd"
DEVICE="pixel_5"
API_LEVEL="35"
ABI="x86_64"
SYSTEM_IMAGE="system-images;android-$API_LEVEL;google_apis;$ABI"

echo "🔧 配置:"
echo "   名称: $AVD_NAME"
echo "   设备: $DEVICE"
echo "   API: $API_LEVEL"
echo "   ABI: $ABI"
echo ""

# 检查系统镜像是否存在
if [ ! -d "$ANDROID_HOME/system-images/android-$API_LEVEL/google_apis/$ABI" ]; then
  echo "❌ 错误: 系统镜像未找到"
  echo "   路径: $ANDROID_HOME/system-images/android-$API_LEVEL/google_apis/$ABI"
  echo ""
  echo "请检查 flake.nix 中的配置:"
  echo "   platformVersions = [ \"$API_LEVEL\" ]"
  echo "   systemImageTypes = [ \"google_apis\" ]"
  echo "   abiVersions = [ \"$ABI\" ]"
  exit 1
fi

# 检查AVD是否已存在
AVD_PATH="$HOME/.android/avd/$AVD_NAME.avd"
if [ -d "$AVD_PATH" ]; then
  echo "⚠️  AVD已存在: $AVD_NAME"
  echo ""
  read -p "是否删除并重新创建? (y/N): " -n 1 -r
  echo
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🗑️  删除现有AVD..."
    rm -rf "$AVD_PATH"
    rm -f "$HOME/.android/avd/$AVD_NAME.ini"
  else
    echo "❌ 取消创建"
    exit 0
  fi
fi

# 创建AVD
echo "📦 创建AVD..."
avdmanager create avd \
  -n "$AVD_NAME" \
  -k "$SYSTEM_IMAGE" \
  -d "$DEVICE" \
  --force

echo ""
echo "✅ AVD创建成功！"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 启动模拟器:"
echo "   emulator -avd $AVD_NAME"
echo ""
echo "📋 其他命令:"
echo "   emulator -list-avds              # 列出所有AVD"
echo "   emulator -avd $AVD_NAME -snapshot foo  # 使用快照加速启动"
echo ""
echo "⚙️  模拟器配置文件:"
echo "   ~/.android/avd/$AVD_NAME.avd/config.ini"
echo ""
echo "💡 提示:"
echo "   - 首次启动会较慢（需要初始化系统）"
echo "   - 可以用 -snapshot 参数创建快照，下次快速启动"
echo "   - 可以在 config.ini 中调整内存、分辨率等参数"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
