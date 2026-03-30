import { Cell, GameState, GameMode, GameStatus } from "../types/game";
import { BlackStoneIcon, WhiteStoneIcon } from "./Icons";
import "./StatusBar.css";

interface StatusBarProps {
  gameState: GameState;
  aiThinking: boolean;
  mode: GameMode;
  myColor?: Cell;
  onMenuOpen: () => void;
  menuOpen: boolean;
  opponentName?: string;
}

function StatusBar({ gameState, aiThinking, mode, myColor, onMenuOpen, menuOpen, opponentName }: StatusBarProps) {
  const statusContent = getStatusContent(gameState, mode, myColor);
  const isOnline = mode === "online_host" || mode === "online_client";

  return (
    <div className="status-bar">
      <div className="status-text">
        <span>{statusContent}</span>
        {aiThinking && <span className="thinking-indicator" aria-hidden="true">🤔</span>}
      </div>
      {isOnline && opponentName && (
        <div className="connection-info">
          <span className={`connection-dot connected`} />
          <span className="ip-prefix">vs {opponentName}</span>
        </div>
      )}
      <button
        className="menu-trigger-btn"
        onClick={onMenuOpen}
        aria-label="打开游戏菜单"
        aria-expanded={menuOpen}
        aria-controls="menu-drawer"
      >
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" aria-hidden="true">
          <line x1="3" y1="6" x2="21" y2="6" />
          <line x1="3" y1="12" x2="21" y2="12" />
          <line x1="3" y1="18" x2="21" y2="18" />
        </svg>
      </button>
    </div>
  );
}

function getStatusContent(state: GameState, mode: GameMode, myColor?: Cell): React.ReactNode {
  const isOnline = mode === "online_host" || mode === "online_client";

  switch (state.status) {
    case GameStatus.Playing:
      if (isOnline && myColor) {
        return state.current_player === myColor ? "轮到你落子" : "等待对手落子...";
      }
      return state.current_player === Cell.Black
        ? <><BlackStoneIcon className="stone-icon" /> 黑棋落子</>
        : <><WhiteStoneIcon className="stone-icon" /> 白棋落子</>;
    case GameStatus.BlackWins:
      if (isOnline && myColor) {
        return myColor === Cell.Black ? "🎉 你赢了！" : "你输了...";
      }
      return <><BlackStoneIcon className="stone-icon" /> 黑棋胜！</>;
    case GameStatus.WhiteWins:
      if (isOnline && myColor) {
        return myColor === Cell.White ? "🎉 你赢了！" : "你输了...";
      }
      return <><WhiteStoneIcon className="stone-icon" /> 白棋胜！</>;
    case GameStatus.Draw:
      return "平局！";
  }
}

export default StatusBar;
