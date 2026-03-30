# Android模拟器使用指南

## 快速开始

### 1. 创建虚拟设备(AVD)

```bash
# 确保在Nix环境中
nix develop

# 运行创建脚本
bash scripts/create-android-avd.sh
```

### 2. 启动模拟器

```bash
# 列出可用的AVD
emulator -list-avds

# 启动模拟器
emulator -avd tauri-gobang-avd

# 使用快照加速启动（首次运行后）
emulator -avd tauri-gobang-avd -snapshot quickboot
```

### 3. 运行应用

```bash
# 在另一个终端窗口
pnpm tauri android dev
```

## 模拟器配置

### 默认配置

| 选项 | 值 | 说明 |
|------|-----|------|
| 名称 | tauri-gobang-avd | AVD名称 |
| 设备 | Pixel 5 | 模拟设备型号 |
| API级别 | 35 | Android 14 |
| ABI | x86_64 | 64位x86（适合桌面） |
| 系统镜像 | Google APIs | 包含Google服务 |

### 自定义配置

编辑 `~/.android/avd/tauri-gobang-avd.avd/config.ini`:

```ini
# 内存配置
hw.ramSize=4096
vm.heapSize=512

# 存储配置
disk.dataPartition.size=8g

# 显示配置
hw.lcd.density=420
hw.lcd.height=2340
hw.lcd.width=1080

# 网络配置
hw.gpu.enabled=yes
hw.gpu.mode=auto

# 其他
hw.camera.back=emulated
hw.camera.front=emulated
hw.audioInput=yes
```

## 常用命令

### AVD管理

```bash
# 列出所有AVD
emulator -list-avds

# 删除AVD
avdmanager delete avd -n tauri-gobang-avd

# 查看AVD详细信息
avdmanager list avd
```

### 模拟器启动选项

```bash
# 基本启动
emulator -avd tauri-gobang-avd

# 无窗口模式（CI/CD）
emulator -avd tauri-gobang-avd -no-window -no-audio

# 使用快照（快速启动）
emulator -avd tauri-gobang-avd -snapshot quickboot

# 调试模式
emulator -avd tauri-gobang-avd -show-kernel

# 指定端口
emulator -avd tauri-gobang-avd -ports 5554,5555

# 禁用启动动画
emulator -avd tauri-gobang-avd -no-boot-anim
```

### ADB调试

```bash
# 查看连接的设备
adb devices

# 安装APK
adb install app-debug.apk

# 卸载应用
adb uninstall com.tiantian.tauri_gobang

# 查看日志
adb logcat | grep -i tauri

# 进入shell
adb shell

# 截图
adb shell screencap -p /sdcard/screen.png
adb pull /sdcard/screen.png

# 录屏
adb shell screenrecord /sdcard/demo.mp4
adb pull /sdcard/demo.mp4
```

## 性能优化

### 启动加速

```bash
# 1. 使用快照
emulator -avd tauri-gobang-avd -snapshot quickboot

# 2. 禁用启动动画
emulator -avd tauri-gobang-avd -no-boot-anim -gpu auto

# 3. 增加内存
# 编辑 config.ini: hw.ramSize=6144
```

### 运行时性能

```bash
# 启用GPU加速
emulator -avd tauri-gobang-avd -gpu auto

# 禁用音频（如果不需要）
emulator -avd tauri-gobang-avd -no-audio

# 使用多核CPU
emulator -avd tauri-gobang-avd -cores 4
```

## 故障排除

### 模拟器启动慢

**原因**: 首次启动需要初始化系统

**解决**:
- 使用快照: `emulator -avd tauri-gobang-avd -snapshot quickboot`
- 禁用启动动画: `-no-boot-anim`
- 增加内存和CPU

### 模拟器卡顿

**原因**: GPU未启用或资源不足

**解决**:
```bash
# 启用GPU
emulator -avd tauri-gobang-avd -gpu auto

# 检查配置
grep hw.gpu ~/.android/avd/tauri-gobang-avd.avd/config.ini
```

### HAXM/KVM问题

**检查**:
```bash
# Linux: 检查KVM
ls /dev/kvm
kvm-ok

# 确保用户在kvm组
sudo usermod -a -G kvm $USER
```

### 系统镜像缺失

**错误**: `ERROR: No system images installed`

**解决**:
```bash
# 检查 flake.nix 配置
# - platformVersions = [ "35" ]
# - includeSystemImages = true
# - systemImageTypes = [ "google_apis" ]
# - abiVersions = [ "x86_64" ]

# 重新进入环境
nix develop
```

### ADB连接失败

**检查**:
```bash
# 查看设备状态
adb devices -l

# 重启ADB
adb kill-server
adb start-server

# 检查端口
netstat -tuln | grep 5555
```

## 高级用法

### 多设备测试

```bash
# 启动多个模拟器实例
emulator -avd tauri-gobang-avd -port 5554
emulator -avd tauri-gobang-avd -port 5556

# 指定目标设备
adb -s emulator-5554 install app.apk
adb -s emulator-5556 install app.apk
```

### 自动化测试

```bash
# 启动模拟器（无窗口）
emulator -avd tauri-gobang-avd -no-window -no-audio &

# 等待启动完成
adb wait-for-device

# 运行测试
pnpm test:e2e

# 关闭模拟器
adb -s emulator-5554 emu kill
```

### 快照管理

```bash
# 创建快照
emulator -avd tauri-gobang-avd -snapshot save clean

# 加载快照
emulator -avd tauri-gobang-avd -snapshot load clean

# 列出快照
# (需要通过AVD GUI管理)
```

## 参考资源

- [Android模拟器文档](https://developer.android.com/studio/run/emulator)
- [AVD管理](https://developer.android.com/studio/command-line/avdmanager)
- [命令行选项](https://developer.android.com/studio/run/emulator-commandline)
