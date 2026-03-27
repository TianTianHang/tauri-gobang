import { Cell, GameState, GameStatus, Difficulty, DIFFICULTY_LABELS, GameMode } from "../types/game";
import "./GameInfo.css";
import { BackIcon } from "./Icons";

interface GameInfoProps {
  gameState: GameState;
  difficulty: Difficulty;
  onDifficultyChange: (d: Difficulty) => void;
  onUndo: () => void;
  onNewGame: () => void;
  onBackToMenu: () => void;
  aiThinking: boolean;
  mode: GameMode;
  myColor?: Cell;
  onUndoRequest?: () => void;
  onRestartRequest?: () => void;
  undoRequested?: boolean;
  restartRequested?: boolean;
  onAcceptUndo?: () => void;
  onRejectUndo?: () => void;
  onAcceptRestart?: () => void;
  onRejectRestart?: () => void;
}

function GameInfo({
  gameState,
  difficulty,
  onDifficultyChange,
  onUndo,
  onNewGame,
  onBackToMenu,
  aiThinking,
  mode,
  myColor,
  onUndoRequest,
  onRestartRequest,
  undoRequested,
  restartRequested,
  onAcceptUndo,
  onRejectUndo,
  onAcceptRestart,
  onRejectRestart,
}: GameInfoProps) {
  const statusText = getStatusText(gameState, mode, myColor);
  const isOnline = mode === "online_host" || mode === "online_client";
  const isMyTurn =
    !isOnline ||
    gameState.current_player === myColor ||
    gameState.status !== GameStatus.Playing;

  return (
    <div className="game-info">
      <div className="game-info-header">
        <button className="btn-back" onClick={onBackToMenu}>
          <BackIcon className="icon" />
          <span>返回</span>
        </button>
        <span className="game-mode-badge">
          {mode === "ai" ? "人机对战" : isOnline ? "联机对战" : ""}
        </span>
      </div>

      <div className={`game-status ${gameState.status}`}>
        {aiThinking && <span className="thinking-dot"></span>}
        <span>{statusText}</span>
      </div>

      <div className="game-info-row">
        <span className="info-label">步数</span>
        <span className="info-value">{gameState.history.length}</span>
      </div>

      {mode === "ai" && (
        <div className="difficulty-selector">
          <span className="info-label">难度</span>
          <div className="difficulty-buttons">
            {(Object.keys(DIFFICULTY_LABELS) as Difficulty[]).map((d) => (
              <button
                key={d}
                className={`diff-btn ${d === difficulty ? "active" : ""}`}
                onClick={() => onDifficultyChange(d)}
                disabled={aiThinking}
              >
                {DIFFICULTY_LABELS[d]}
              </button>
            ))}
          </div>
        </div>
      )}

      {undoRequested && (
        <div className="request-dialog">
          <p>对方请求悔棋</p>
          <div className="request-buttons">
            <button className="btn-accept" onClick={onAcceptUndo}>
              同意
            </button>
            <button className="btn-reject" onClick={onRejectUndo}>
              拒绝
            </button>
          </div>
        </div>
      )}

      {restartRequested && (
        <div className="request-dialog">
          <p>对方请求重新开始</p>
          <div className="request-buttons">
            <button className="btn-accept" onClick={onAcceptRestart}>
              同意
            </button>
            <button className="btn-reject" onClick={onRejectRestart}>
              拒绝
            </button>
          </div>
        </div>
      )}

      <div className="game-buttons">
        {mode === "ai" && (
          <button onClick={onUndo} disabled={gameState.history.length === 0 || aiThinking}>
            悔棋
          </button>
        )}
        {isOnline && onUndoRequest && (
          <button onClick={onUndoRequest} disabled={!isMyTurn || undoRequested}>
            悔棋请求
          </button>
        )}
        {isOnline && onRestartRequest && (
          <button onClick={onRestartRequest} disabled={restartRequested}>
            重新开始
          </button>
        )}
        <button onClick={onNewGame}>新游戏</button>
      </div>
    </div>
  );
}

function getStatusText(state: GameState, mode: GameMode, myColor?: Cell): string {
  const isOnline = mode === "online_host" || mode === "online_client";

  switch (state.status) {
    case GameStatus.Playing:
      if (isOnline && myColor) {
        return state.current_player === myColor ? "轮到你落子" : "等待对手落子...";
      }
      return state.current_player === Cell.Black ? "● 黑棋落子" : "○ 白棋落子";
    case GameStatus.BlackWins:
      if (isOnline && myColor) {
        return myColor === Cell.Black ? "🎉 你赢了！" : "你输了...";
      }
      return "● 黑棋胜！";
    case GameStatus.WhiteWins:
      if (isOnline && myColor) {
        return myColor === Cell.White ? "🎉 你赢了！" : "你输了...";
      }
      return "○ 白棋胜！";
    case GameStatus.Draw:
      return "平局！";
  }
}

export default GameInfo;
