#!/bin/bash
# 启动联机测试环境 - 服务器 + 2个客户端实例

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# 日志目录
LOG_DIR="$PROJECT_ROOT/logs"
mkdir -p "$LOG_DIR"

# PID 存储
PIDS=()

# 清理函数
cleanup() {
    echo -e "\n${YELLOW}🛑 正在停止所有服务...${NC}"

    # 杀掉所有后台进程
    for pid in "${PIDS[@]}"; do
        if kill -0 "$pid" 2>/dev/null; then
            kill "$pid" 2>/dev/null || true
            echo -e "${GREEN}  ✓ 已停止 PID $pid${NC}"
        fi
    done

    # 清理可能遗留的 cargo run 进程
    pkill -f "gobang-server" 2>/dev/null || true
    pkill -f "tauri dev" 2>/dev/null || true

    echo -e "${GREEN}✓ 所有服务已停止${NC}"
    echo -e "${CYAN}💾 日志文件保存在: $LOG_DIR${NC}"
    exit 0
}

# 捕获退出信号
trap cleanup INT TERM

echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo -e "${BLUE}       🎮 Gobang 联机测试环境启动器${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo ""

# 检查依赖
echo -e "${YELLOW}🔍 检查依赖...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ 未找到 cargo，请先安装 Rust${NC}"
    exit 1
fi
if ! command -v pnpm &> /dev/null; then
    echo -e "${RED}✗ 未找到 pnpm，请先安装 pnpm${NC}"
    exit 1
fi
echo -e "${GREEN}✓ 依赖检查通过${NC}"
echo ""

# 清理旧的日志文件（可选）
echo -e "${YELLOW}🗑️  清理旧日志...${NC}"
rm -f "$LOG_DIR"/*.log
echo -e "${GREEN}✓ 日志目录已清理${NC}"
echo ""

# 步骤 1: 启动服务器
echo -e "${YELLOW}📡 步骤 1/3: 启动服务器...${NC}"
cd "$PROJECT_ROOT/server"
cargo run > "$LOG_DIR/server.log" 2>&1 &
SERVER_PID=$!
PIDS+=("$SERVER_PID")
echo -e "${GREEN}  ✓ 服务器已启动 (PID: $SERVER_PID)${NC}"

# 等待服务器就绪
echo -e "${YELLOW}  ⏳ 等待服务器就绪...${NC}"
for i in {1..30}; do
    if curl -s http://localhost:3001/api/rooms > /dev/null 2>&1; then
        echo -e "${GREEN}  ✓ 服务器已就绪！${NC}"
        break
    fi
    if [ $i -eq 30 ]; then
        echo -e "${RED}✗ 服务器启动超时，请查看日志: $LOG_DIR/server.log${NC}"
        cleanup
    fi
    sleep 1
done
echo ""

# 步骤 2: 启动客户端 1（端口 1420）
echo -e "${YELLOW}🎮 步骤 2/3: 启动客户端实例 1 (玩家 1, 端口 1420)...${NC}"
cd "$PROJECT_ROOT"
VITE_DEV_PORT=1420 pnpm tauri dev > "$LOG_DIR/client1.log" 2>&1 &
CLIENT1_PID=$!
PIDS+=("$CLIENT1_PID")
echo -e "${GREEN}  ✓ 客户端 1 已启动 (PID: $CLIENT1_PID)${NC}"
sleep 3
echo ""

# 步骤 3: 启动客户端 2（端口 1422）
echo -e "${YELLOW}🎮 步骤 3/3: 启动客户端实例 2 (玩家 2, 端口 1422)...${NC}"
VITE_DEV_PORT=1422 pnpm tauri dev > "$LOG_DIR/client2.log" 2>&1 &
CLIENT2_PID=$!
PIDS+=("$CLIENT2_PID")
echo -e "${GREEN}  ✓ 客户端 2 已启动 (PID: $CLIENT2_PID)${NC}"
echo ""

# 显示测试指南
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo -e "${GREEN}🎉 测试环境已就绪！${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}📋 测试步骤：${NC}"
echo ""
echo -e "${BLUE}  【玩家 1 - 客户端 1】${NC}"
echo -e "    1. 点击 ${GREEN}\"联机对战\"${NC}"
echo -e "    2. 使用用户名 ${GREEN}\"player1\"${NC} 和任意密码注册/登录"
echo -e "    3. 点击 ${GREEN}\"创建房间\"${NC}，输入房间名（如：测试房间）"
echo -e "    4. 等待对手加入..."
echo ""
echo -e "${BLUE}  【玩家 2 - 客户端 2】${NC}"
echo -e "    1. 点击 ${GREEN}\"联机对战\"${NC}"
echo -e "    2. 使用用户名 ${GREEN}\"player2\"${NC} 和任意密码注册/登录"
echo -e "    3. 在房间列表中找到 ${GREEN}\"测试房间\"${NC}，点击 ${GREEN}\"加入\"${NC}"
echo -e "    4. 游戏自动开始！开始对战吧！🎯"
echo ""
echo -e "${YELLOW}📊 服务状态：${NC}"
echo -e "    • 服务器: ${GREEN}http://localhost:3001${NC}"
echo -e "    • 客户端 1: ${GREEN}http://localhost:1420${NC} (默认端口)"
echo -e "    • 客户端 2: ${GREEN}http://localhost:1422${NC} (备用端口)"
echo ""
echo -e "${YELLOW}📁 日志文件：${NC}"
echo -e "    • 服务器日志: ${CYAN}tail -f $LOG_DIR/server.log${NC}"
echo -e "    • 客户端 1 日志: ${CYAN}tail -f $LOG_DIR/client1.log${NC}"
echo -e "    • 客户端 2 日志: ${CYAN}tail -f $LOG_DIR/client2.log${NC}"
echo ""
echo -e "${YELLOW}💡 调试技巧：${NC}"
echo -e "    • 查看服务器日志: ${CYAN}tail -f $LOG_DIR/server.log${NC}"
echo -e "    • 客户端调试: 在应用中按 ${GREEN}F12${NC} 打开开发者工具"
echo -e "    • 查看所有日志: ${CYAN}ls -la $LOG_DIR/${NC}"
echo ""
echo -e "${RED}按 Ctrl+C 停止所有服务${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo ""

# 保持脚本运行
wait
