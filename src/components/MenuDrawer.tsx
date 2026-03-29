import { useEffect, useRef } from "react";
import {
  Cell,
  GameState,
  GameMode,
  Difficulty,
  DIFFICULTY_LABELS,
} from "../types/game";
import "./MenuDrawer.css";

interface MenuDrawerProps {
  isOpen: boolean;
  onClose: () => void;
  gameState: GameState;
  difficulty: Difficulty;
  onDifficultyChange: (d: Difficulty) => void;
  onUndo: () => void;
  onNewGame: () => void;
  onBackToMenu: () => void;
  mode: GameMode;
  aiThinking: boolean;
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

function MenuDrawer({
  isOpen,
  onClose,
  gameState,
  difficulty,
  onDifficultyChange,
  onUndo,
  onNewGame,
  onBackToMenu,
  mode,
  aiThinking,
  myColor,
  onUndoRequest,
  onRestartRequest,
  undoRequested,
  restartRequested,
  onAcceptUndo,
  onRejectUndo,
  onAcceptRestart,
  onRejectRestart,
}: MenuDrawerProps) {
  const drawerRef = useRef<HTMLDivElement>(null);

  const isOnline = mode === "online_host" || mode === "online_client";
  const isMyTurn =
    !isOnline ||
    gameState.current_player === myColor ||
    gameState.status !== "playing";

  useEffect(() => {
    if (!isOpen) return;
    const handleKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    document.addEventListener("keydown", handleKey);
    drawerRef.current?.focus();
    return () => document.removeEventListener("keydown", handleKey);
  }, [isOpen, onClose]);

  return (
    <>
      <div
        className={`menu-overlay ${isOpen ? "open" : ""}`}
        onClick={onClose}
        aria-hidden="true"
      />
      <div
        ref={drawerRef}
        className={`menu-drawer ${isOpen ? "open" : ""}`}
        role="dialog"
        aria-modal="true"
        aria-label="游戏设置菜单"
        id="menu-drawer"
        tabIndex={-1}
      >
        <div className="drawer-header">
          <span className="drawer-title">游戏设置</span>
          <button
            className="drawer-close-btn"
            onClick={onClose}
            aria-label="关闭菜单"
          >
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" aria-hidden="true">
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </div>

        <div className="drawer-content">
          <div className="drawer-section">
            <h3 className="section-title">游戏操作</h3>
            <div className="action-grid">
              <button
                className="drawer-btn drawer-btn-primary"
                onClick={() => { onNewGame(); onClose(); }}
              >
                新游戏
              </button>
              {mode === "ai" && (
                <button
                  className="drawer-btn"
                  onClick={() => { onUndo(); onClose(); }}
                  disabled={gameState.history.length === 0 || aiThinking}
                >
                  悔棋
                </button>
              )}
              {isOnline && onUndoRequest && (
                <button
                  className="drawer-btn"
                  onClick={() => { onUndoRequest(); onClose(); }}
                  disabled={!isMyTurn || undoRequested}
                >
                  悔棋请求
                </button>
              )}
              {isOnline && onRestartRequest && (
                <button
                  className="drawer-btn"
                  onClick={() => { onRestartRequest(); onClose(); }}
                  disabled={restartRequested}
                >
                  重新开始
                </button>
              )}
            </div>
          </div>

          {undoRequested && (
            <div className="drawer-section request-section">
              <p className="request-text">对方请求悔棋</p>
              <div className="request-btns">
                <button className="drawer-btn drawer-btn-primary" onClick={() => { onAcceptUndo?.(); onClose(); }}>
                  同意
                </button>
                <button className="drawer-btn" onClick={() => { onRejectUndo?.(); onClose(); }}>
                  拒绝
                </button>
              </div>
            </div>
          )}

          {restartRequested && (
            <div className="drawer-section request-section">
              <p className="request-text">对方请求重新开始</p>
              <div className="request-btns">
                <button className="drawer-btn drawer-btn-primary" onClick={() => { onAcceptRestart?.(); onClose(); }}>
                  同意
                </button>
                <button className="drawer-btn" onClick={() => { onRejectRestart?.(); onClose(); }}>
                  拒绝
                </button>
              </div>
            </div>
          )}

          {mode === "ai" && (
            <div className="drawer-section">
              <h3 className="section-title">难度</h3>
              <div className="difficulty-btns">
                {(Object.keys(DIFFICULTY_LABELS) as Difficulty[]).map((d) => (
                  <button
                    key={d}
                    className={`drawer-btn diff-option ${d === difficulty ? "active" : ""}`}
                    onClick={() => onDifficultyChange(d)}
                    disabled={aiThinking}
                  >
                    {DIFFICULTY_LABELS[d]}
                  </button>
                ))}
              </div>
            </div>
          )}

          <div className="drawer-section">
            <h3 className="section-title">对局信息</h3>
            <div className="info-row">
              <span className="info-label">当前步数</span>
              <span className="info-value">{gameState.history.length}</span>
            </div>
            <div className="info-row">
              <span className="info-label">模式</span>
              <span className="info-value">
                {mode === "ai" ? "人机对战" : isOnline ? "联机对战" : ""}
              </span>
            </div>
          </div>

          <div className="drawer-section">
            <button
              className="drawer-btn drawer-btn-danger"
              onClick={() => { onBackToMenu(); onClose(); }}
            >
              返回主菜单
            </button>
          </div>
        </div>
      </div>
    </>
  );
}

export default MenuDrawer;
