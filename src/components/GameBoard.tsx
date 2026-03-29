import { useEffect, useRef, useCallback } from "react";
import { Cell, GameState, BOARD_SIZE } from "../types/game";
import { useBoardSize } from "../hooks/useBoardSize";
import { useTouchPreview } from "../hooks/useTouchPreview";
import "./GameBoard.css";

const STAR_POINTS = [
  [3, 3],
  [3, 11],
  [7, 7],
  [11, 3],
  [11, 11],
];

interface GameBoardProps {
  gameState: GameState;
  onCellClick: (row: number, col: number) => void;
  disabled: boolean;
  lastMove?: { row: number; col: number } | null;
  onPiecePlaced?: () => void;
}

function GameBoard({ gameState, onCellClick, disabled, lastMove, onPiecePlaced }: GameBoardProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { canvas: canvasSize, cell: cellSize, padding } = useBoardSize();
  const pieceRadius = cellSize * 0.43;

  const {
    hoverRef,
    handleTouchStart,
    handleTouchMove,
    handleTouchEnd,
    getCellFromPixel,
  } = useTouchPreview({
    padding,
    cellSize,
    disabled,
    onCellClick: (row, col) => {
      if (disabled) return;
      if (gameState.board[row][col] !== Cell.Empty) return;
      onPiecePlaced?.();
      onCellClick(row, col);
    },
  });

  const draw = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas || canvasSize === 0) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    ctx.clearRect(0, 0, canvasSize, canvasSize);

    const bgGrad = ctx.createLinearGradient(0, 0, canvasSize, canvasSize);
    bgGrad.addColorStop(0, "#DEB887");
    bgGrad.addColorStop(1, "#D2A96A");
    ctx.fillStyle = bgGrad;
    ctx.fillRect(0, 0, canvasSize, canvasSize);

    ctx.strokeStyle = "#5C4033";
    ctx.lineWidth = 1;

    for (let i = 0; i < BOARD_SIZE; i++) {
      const pos = padding + i * cellSize;
      ctx.beginPath();
      ctx.moveTo(padding, pos);
      ctx.lineTo(padding + (BOARD_SIZE - 1) * cellSize, pos);
      ctx.stroke();

      ctx.beginPath();
      ctx.moveTo(pos, padding);
      ctx.lineTo(pos, padding + (BOARD_SIZE - 1) * cellSize);
      ctx.stroke();
    }

    ctx.fillStyle = "#5C4033";
    for (const [sr, sc] of STAR_POINTS) {
      ctx.beginPath();
      ctx.arc(padding + sc * cellSize, padding + sr * cellSize, Math.max(3, cellSize * 0.1), 0, Math.PI * 2);
      ctx.fill();
    }

    for (let row = 0; row < BOARD_SIZE; row++) {
      for (let col = 0; col < BOARD_SIZE; col++) {
        const cell = gameState.board[row][col];
        if (cell === Cell.Empty) continue;
        drawPiece(ctx, row, col, cell, 1, padding, cellSize, pieceRadius);
      }
    }

    if (hoverRef.current && !disabled && gameState.status === "playing") {
      const { row, col } = hoverRef.current;
      if (gameState.board[row][col] === Cell.Empty) {
        drawPiece(ctx, row, col, gameState.current_player, 0.35, padding, cellSize, pieceRadius);
      }
    }

    if (lastMove) {
      const cx = padding + lastMove.col * cellSize;
      const cy = padding + lastMove.row * cellSize;
      ctx.fillStyle = "#FF4444";
      ctx.beginPath();
      ctx.arc(cx, cy, Math.max(3, cellSize * 0.1), 0, Math.PI * 2);
      ctx.fill();
    }


  }, [canvasSize, cellSize, padding, pieceRadius, gameState, disabled, lastMove, hoverRef]);

  useEffect(() => {
    draw();
  }, [draw]);

  useEffect(() => {
    const handleResize = () => draw();
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, [draw]);

  const handleMouseMove = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      const canvas = canvasRef.current;
      if (!canvas) return;
      const rect = canvas.getBoundingClientRect();
      const scaleX = canvas.width / rect.width;
      const scaleY = canvas.height / rect.height;
      const x = (e.clientX - rect.left) * scaleX;
      const y = (e.clientY - rect.top) * scaleY;
      const cell = getCellFromPixel(x, y);
      const prev = hoverRef.current;
      if (cell?.row !== prev?.row || cell?.col !== prev?.col) {
        hoverRef.current = cell;
        draw();
      }
    },
    [getCellFromPixel, draw, hoverRef]
  );

  const handleMouseLeave = useCallback(() => {
    hoverRef.current = null;
    draw();
  }, [draw, hoverRef]);

  const handleClick = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      if (disabled) return;
      const canvas = canvasRef.current;
      if (!canvas) return;
      const rect = canvas.getBoundingClientRect();
      const scaleX = canvas.width / rect.width;
      const scaleY = canvas.height / rect.height;
      const x = (e.clientX - rect.left) * scaleX;
      const y = (e.clientY - rect.top) * scaleY;
      const cell = getCellFromPixel(x, y);
      if (cell) {
        if (gameState.board[cell.row][cell.col] !== Cell.Empty) return;
        onPiecePlaced?.();
        onCellClick(cell.row, cell.col);
      }
    },
    [disabled, getCellFromPixel, onCellClick, onPiecePlaced, gameState.board]
  );

  return (
    <div className="game-board-wrapper">
      <canvas
        ref={canvasRef}
        width={canvasSize}
        height={canvasSize}
        className="game-board"
        role="grid"
        aria-label="五子棋棋盘，15行15列"
        aria-description={`当前${gameState.current_player === Cell.Black ? "黑棋" : "白棋"}落子，共${gameState.history.length}步`}
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
        onClick={handleClick}
        onTouchStart={handleTouchStart}
        onTouchMove={handleTouchMove}
        onTouchEnd={handleTouchEnd}
      />
    </div>
  );
}

function drawPiece(
  ctx: CanvasRenderingContext2D,
  row: number,
  col: number,
  player: Cell,
  alpha: number,
  padding: number,
  cellSize: number,
  radius: number
) {
  const cx = padding + col * cellSize;
  const cy = padding + row * cellSize;

  ctx.save();
  ctx.globalAlpha = alpha;

  ctx.shadowColor = "rgba(0,0,0,0.4)";
  ctx.shadowBlur = 4;
  ctx.shadowOffsetX = 2;
  ctx.shadowOffsetY = 2;

  const grad = ctx.createRadialGradient(
    cx - radius * 0.3,
    cy - radius * 0.3,
    radius * 0.1,
    cx,
    cy,
    radius
  );

  if (player === Cell.Black) {
    grad.addColorStop(0, "#555555");
    grad.addColorStop(1, "#111111");
  } else {
    grad.addColorStop(0, "#FFFFFF");
    grad.addColorStop(1, "#CCCCCC");
  }

  ctx.fillStyle = grad;
  ctx.beginPath();
  ctx.arc(cx, cy, radius, 0, Math.PI * 2);
  ctx.fill();

  ctx.restore();
}

export default GameBoard;
