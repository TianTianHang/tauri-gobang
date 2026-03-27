#!/bin/bash
# Android SDK 安装脚本 - Arch Linux
# 用于设置 Tauri Android 开发环境

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查是否以 root 权限运行
check_not_root() {
    if [ "$EUID" -eq 0 ]; then
        log_error "请不要使用 root 权限运行此脚本"
        log_info "使用 yay 安装 AUR 包需要普通用户权限"
        exit 1
    fi
}

# 检查 yay 是否安装
check_yay() {
    if ! command -v yay &> /dev/null; then
        log_error "yay 未安装"
        log_info "请先安装 yay: sudo pacman -S --needed base-devel git && cd /tmp && git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si"
        exit 1
    fi
    log_success "yay 已安装"
}

# 检查已安装的包
check_installed() {
    log_info "检查已安装的 Android SDK 组件..."
    pacman -Q | grep -i android || echo "尚未安装 Android 相关包"
}

# 安装基础 SDK 组件
install_base_sdk() {
    log_info "安装 Android SDK 基础组件..."

    yay -S --needed --noconfirm android-sdk

    log_success "android-sdk 安装完成"
}

# 安装 build-tools
install_build_tools() {
    log_info "安装 Android SDK Build Tools..."

    yay -S --needed --noconfirm android-sdk-build-tools

    # 验证安装
    if [ -d "/opt/android-sdk/build-tools" ]; then
        log_success "build-tools 安装完成"
        ls -la /opt/android-sdk/build-tools/
    else
        log_warn "build-tools 目录未找到，但包可能已安装"
    fi
}

# 安装 platform
install_platform() {
    log_info "安装 Android Platform..."

    # 尝试安装 android-platform-35，如果失败则尝试 34
    if yay -Si android-platform-35 &> /dev/null; then
        log_info "安装 android-platform-35..."
        yay -S --needed --noconfirm android-platform-35
    elif yay -Si android-platform-34 &> /dev/null; then
        log_info "安装 android-platform-34..."
        yay -S --needed --noconfirm android-platform-34
    else
        log_error "找不到可用的 android-platform 包"
        return 1
    fi

    # 验证安装
    if [ -d "/opt/android-sdk/platforms" ]; then
        log_success "platform 安装完成"
        ls -la /opt/android-sdk/platforms/
    else
        log_warn "platforms 目录未找到"
    fi
}

# 安装系统镜像
install_system_image() {
    log_info "安装 Android 系统镜像（可能需要较长时间，镜像约 1GB）..."

    # 尝试安装 Android 34/35 的 x86_64 系统镜像
    if yay -Si android-google-apis-playstore-x86-64-system-image-35 &> /dev/null; then
        log_info "安装 Android 35 x86_64 系统镜像（with Google Play）..."
        yay -S --needed --noconfirm android-google-apis-playstore-x86-64-system-image-35
    elif yay -Si android-google-apis-playstore-x86-64-system-image-34 &> /dev/null; then
        log_info "安装 Android 34 x86_64 系统镜像（with Google Play）..."
        yay -S --needed --noconfirm android-google-apis-playstore-x86-64-system-image-34
    else
        log_info "安装通用的 x86_64 系统镜像..."
        yay -S --needed --noconfirm android-google-apis-playstore-x86-64-system-image
    fi

    # 验证安装
    if [ -d "/opt/android-sdk/system-images" ]; then
        log_success "系统镜像安装完成"
        ls -la /opt/android-sdk/system-images/
    else
        log_warn "system-images 目录未找到"
    fi
}

# 安装 android-udev 规则
install_udev_rules() {
    log_info "安装 android-udev 规则（用于 USB 设备识别）..."

    yay -S --needed --noconfirm android-udev

    log_success "android-udev 安装完成"
    log_warn "需要重新加载 udev 规则: sudo udevadm control --reload-rules"
}

# 配置环境变量
configure_environment() {
    log_info "配置环境变量..."

    ANDROID_ENV="# Android SDK Environment
export ANDROID_HOME=/opt/android-sdk
export ANDROID_SDK_ROOT=\$ANDROID_HOME
export PATH=\$PATH:\$ANDROID_HOME/emulator
export PATH=\$PATH:\$ANDROID_HOME/platform-tools
export PATH=\$PATH:\$ANDROID_HOME/cmdline-tools/latest/bin
"

    # 检测使用的 shell
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.bashrc"
    else
        SHELL_CONFIG="$HOME/.profile"
    fi

    # 检查是否已经配置过
    if grep -q "ANDROID_HOME=/opt/android-sdk" "$SHELL_CONFIG" 2>/dev/null; then
        log_warn "环境变量已存在于 $SHELL_CONFIG"
    else
        echo "" >> "$SHELL_CONFIG"
        echo "$ANDROID_ENV" >> "$SHELL_CONFIG"
        log_success "环境变量已添加到 $SHELL_CONFIG"
    fi

    # 导出当前会话的环境变量
    export ANDROID_HOME=/opt/android-sdk
    export ANDROID_SDK_ROOT=$ANDROID_HOME
    export PATH=$PATH:$ANDROID_HOME/emulator
    export PATH=$PATH:$ANDROID_HOME/platform-tools
    export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin

    log_success "当前会话环境变量已设置"
}

# 创建 Android 虚拟设备（AVD）
create_avd() {
    log_info "准备创建 Android 虚拟设备..."

    # 等待系统镜像完全安装
    sleep 2

    # 查找可用的系统镜像
    log_info "查找可用的系统镜像..."
    if [ -d "/opt/android-sdk/system-images" ]; then
        find /opt/android-sdk/system-images -type d -name "x86_64" | head -5
    fi

    # 创建 AVD 目录
    mkdir -p "$HOME/.android/avd"

    # 尝试找到合适的系统镜像
    SYSTEM_IMAGE=""
    if [ -d "/opt/android-sdk/system-images/android-35/google_apis_playstore/x86_64" ]; then
        SYSTEM_IMAGE="system-images;android-35;google_apis_playstore;x86_64"
    elif [ -d "/opt/android-sdk/system-images/android-34/google_apis_playstore/x86_64" ]; then
        SYSTEM_IMAGE="system-images;android-34;google_apis_playstore;x86_64"
    elif [ -d "/opt/android-sdk/system-images/android-33/google_apis_playstore/x86_64" ]; then
        SYSTEM_IMAGE="system-images;android-33;google_apis_playstore;x86_64"
    fi

    if [ -z "$SYSTEM_IMAGE" ]; then
        log_warn "未找到合适的系统镜像，跳过 AVD 创建"
        log_info "请稍后手动创建 AVD: avdmanager create avd -n tauri_emulator -k <system-image> -d pixel_6"
        return 0
    fi

    log_info "使用系统镜像: $SYSTEM_IMAGE"

    # 创建 AVD
    if [ -x "/opt/android-sdk/cmdline-tools/latest/bin/avdmanager" ]; then
        /opt/android-sdk/cmdline-tools/latest/bin/avdmanager create avd \
            -n tauri_emulator \
            -k "$SYSTEM_IMAGE" \
            -d "pixel_6" \
            --force

        log_success "AVD 'tauri_emulator' 创建成功"
        log_info "启动模拟器命令: emulator -avd tauri_emulator"
    else
        log_warn "avdmanager 未找到，跳过 AVD 创建"
    fi
}

# 验证安装
verify_installation() {
    log_info "验证安装..."

    echo ""
    log_info "=== 已安装的包 ==="
    pacman -Q | grep -i android | grep -v debug

    echo ""
    log_info "=== Android SDK 目录结构 ==="
    if [ -d "/opt/android-sdk" ]; then
        ls -la /opt/android-sdk/
    else
        log_error "/opt/android-sdk 目录不存在"
    fi

    echo ""
    log_info "=== 环境变量 ==="
    echo "ANDROID_HOME: $ANDROID_HOME"
    echo "ANDROID_SDK_ROOT: $ANDROID_SDK_ROOT"

    echo ""
    log_info "=== 可用工具 ==="
    command -v emulator && echo "emulator: $(which emulator)"
    command -v adb && echo "adb: $(which adb)"
}

# 打印后续步骤
print_next_steps() {
    echo ""
    log_success "========================================"
    log_success "Android SDK 安装完成！"
    log_success "========================================"
    echo ""
    log_info "后续步骤："
    echo ""
    echo "1. 重新加载 shell 配置："
    echo "   source ~/.zshrc   # 如果使用 zsh"
    echo "   source ~/.bashrc  # 如果使用 bash"
    echo ""
    echo "2. 重新加载 udev 规则（如已安装）："
    echo "   sudo udevadm control --reload-rules"
    echo ""
    echo "3. 启动 Android 模拟器："
    echo "   emulator -avd tauri_emulator"
    echo ""
    echo "4. 在 Tauri 项目中运行："
    echo "   cd /data/projects/tauri-gobang"
    echo "   pnpm tauri android dev"
    echo ""
    log_warn "如果遇到问题，请检查："
    echo "  - 环境变量是否正确: echo \$ANDROID_HOME"
    echo "  - SDK 目录是否存在: ls -la /opt/android-sdk/"
    echo "  - 镜像是否下载完成: ls -la /opt/android-sdk/system-images/"
    echo ""
}

# 主函数
main() {
    echo ""
    log_success "========================================"
    log_success "Android SDK 安装脚本 - Arch Linux"
    log_success "========================================"
    echo ""

    check_not_root
    check_yay
    check_installed

    echo ""
    log_info "开始安装..."
    echo ""

    install_base_sdk
    install_build_tools
    install_platform
    install_system_image
    install_udev_rules
    configure_environment
    create_avd
    verify_installation

    print_next_steps
}

# 运行主函数
main
