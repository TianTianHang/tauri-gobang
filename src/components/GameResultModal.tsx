import type { ReactNode } from "react";
import { useEffect, useRef } from "react";
import { Cell, GameState, GameMode, GameStatus } from "../types/game";
import { TrophyIcon, XCircleIcon, HandshakeIcon, BlackStoneIcon, WhiteStoneIcon } from "./Icons";
import "./GameResultModal.css";

interface GameResultModalProps {
  gameState: GameState;
  mode: GameMode;
  myColor?: Cell;
  gameDuration: number;
  onNewGame: () => void;
  onBackToMenu: () => void;
}

function GameResultModal({
  gameState,
  mode,
  myColor,
  gameDuration,
  onNewGame,
  onBackToMenu,
}: GameResultModalProps) {
  const modalRef = useRef<HTMLDivElement>(null);
  const resultType = getResultType(gameState, mode, myColor);

  const configs = {
    victory: {
      icon: <TrophyIcon className="result-icon-svg" />,
      title: "你赢了！",
      subtitle: getWinnerText(gameState),
    },
    defeat: {
      icon: <XCircleIcon className="result-icon-svg" />,
      title: "你输了...",
      subtitle: getEncouragingText(gameState),
    },
    draw: {
      icon: <HandshakeIcon className="result-icon-svg" />,
      title: "平局！",
      subtitle: "旗鼓相当！",
    },
  };

  const config = configs[resultType];
  const ariaLabel = `游戏结束，${config.title}`;

  useEffect(() => {
    const modal = modalRef.current;
    if (!modal) return;

    const firstBtn = modal.querySelector<HTMLButtonElement>(".result-btn");
    firstBtn?.focus();

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onNewGame();
        return;
      }

      if (e.key === "Tab") {
        const focusable = modal.querySelectorAll<HTMLElement>(
          'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );
        if (focusable.length === 0) return;

        const first = focusable[0];
        const last = focusable[focusable.length - 1];

        if (e.shiftKey) {
          if (document.activeElement === first) {
            e.preventDefault();
            last.focus();
          }
        } else {
          if (document.activeElement === last) {
            e.preventDefault();
            first.focus();
          }
        }
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [onNewGame]);

  const minutes = Math.floor(gameDuration / 60);
  const seconds = gameDuration % 60;
  const durationText = `${minutes}:${seconds.toString().padStart(2, "0")}`;

  return (
    <div
      className="game-result-overlay"
      role="dialog"
      aria-modal="true"
      aria-label={ariaLabel}
      ref={modalRef}
    >
      <div className={`game-result-modal ${resultType}`}>
        <div className="result-icon">{config.icon}</div>
        <h2 className="result-title">{config.title}</h2>
        <p className="result-subtitle">{config.subtitle}</p>
        <div className="result-stats">
          <div className="stat-item">
            <span className="stat-value">{gameState.history.length}</span>
            <span className="stat-label">步</span>
          </div>
          <div className="stat-item">
            <span className="stat-value">{durationText}</span>
            <span className="stat-label">用时</span>
          </div>
        </div>
        <div className="result-actions">
          <button className="result-btn result-btn-primary" onClick={onNewGame} aria-label="再来一局">
            再来一局
          </button>
          <button className="result-btn result-btn-secondary" onClick={onBackToMenu} aria-label="返回菜单">
            返回菜单
          </button>
        </div>
      </div>
    </div>
  );
}

function getResultType(
  gameState: GameState,
  mode: GameMode,
  myColor?: Cell
): "victory" | "defeat" | "draw" {
  if (gameState.status === GameStatus.Draw) return "draw";

  const isOnline = mode === "online_host" || mode === "online_client";

  if (isOnline && myColor) {
    if (gameState.status === GameStatus.BlackWins) {
      return myColor === Cell.Black ? "victory" : "defeat";
    }
    return myColor === Cell.White ? "victory" : "defeat";
  }

  if (mode === "ai" || mode === "local_pvp") {
    return gameState.status === GameStatus.BlackWins ? "victory" : "defeat";
  }

  return "draw";
}

function getWinnerText(gameState: GameState): ReactNode {
  if (gameState.status === GameStatus.BlackWins) return <><BlackStoneIcon className="stone-icon" /> 黑棋获胜</>;
  if (gameState.status === GameStatus.WhiteWins) return <><WhiteStoneIcon className="stone-icon" /> 白棋获胜</>;
  return "";
}

function getEncouragingText(gameState: GameState): ReactNode {
  if (gameState.status === GameStatus.BlackWins) return <><BlackStoneIcon className="stone-icon" /> 黑棋获胜，再接再厉！</>;
  if (gameState.status === GameStatus.WhiteWins) return <><WhiteStoneIcon className="stone-icon" /> 白棋获胜，再接再厉！</>;
  return "";
}

export default GameResultModal;
