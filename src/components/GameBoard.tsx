import { useEffect, useRef, useCallback } from "react";
import { Cell, GameState, BOARD_SIZE } from "../types/game";
import "./GameBoard.css";

const CELL_SIZE = 40;
const PADDING = 30;
const PIECE_RADIUS = CELL_SIZE * 0.43;
const CANVAS_SIZE = CELL_SIZE * (BOARD_SIZE - 1) + PADDING * 2;
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
}

function GameBoard({ gameState, onCellClick, disabled, lastMove }: GameBoardProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const hoverRef = useRef<{ row: number; col: number } | null>(null);

  const getCellFromPixel = useCallback(
    (x: number, y: number): { row: number; col: number } | null => {
      const col = Math.round((x - PADDING) / CELL_SIZE);
      const row = Math.round((y - PADDING) / CELL_SIZE);
      if (row < 0 || row >= BOARD_SIZE || col < 0 || col >= BOARD_SIZE) return null;
      const cx = PADDING + col * CELL_SIZE;
      const cy = PADDING + row * CELL_SIZE;
      const dist = Math.sqrt((x - cx) ** 2 + (y - cy) ** 2);
      if (dist > CELL_SIZE * 0.48) return null;
      return { row, col };
    },
    []
  );

  const draw = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    ctx.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);

    const bgGrad = ctx.createLinearGradient(0, 0, CANVAS_SIZE, CANVAS_SIZE);
    bgGrad.addColorStop(0, "#DEB887");
    bgGrad.addColorStop(1, "#D2A96A");
    ctx.fillStyle = bgGrad;
    ctx.fillRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);

    ctx.strokeStyle = "#5C4033";
    ctx.lineWidth = 1;

    for (let i = 0; i < BOARD_SIZE; i++) {
      const pos = PADDING + i * CELL_SIZE;
      ctx.beginPath();
      ctx.moveTo(PADDING, pos);
      ctx.lineTo(PADDING + (BOARD_SIZE - 1) * CELL_SIZE, pos);
      ctx.stroke();

      ctx.beginPath();
      ctx.moveTo(pos, PADDING);
      ctx.lineTo(pos, PADDING + (BOARD_SIZE - 1) * CELL_SIZE);
      ctx.stroke();
    }

    ctx.fillStyle = "#5C4033";
    for (const [sr, sc] of STAR_POINTS) {
      ctx.beginPath();
      ctx.arc(PADDING + sc * CELL_SIZE, PADDING + sr * CELL_SIZE, 4, 0, Math.PI * 2);
      ctx.fill();
    }

    for (let row = 0; row < BOARD_SIZE; row++) {
      for (let col = 0; col < BOARD_SIZE; col++) {
        const cell = gameState.board[row][col];
        if (cell === Cell.Empty) continue;
        drawPiece(ctx, row, col, cell);
      }
    }

    if (lastMove) {
      const cx = PADDING + lastMove.col * CELL_SIZE;
      const cy = PADDING + lastMove.row * CELL_SIZE;
      ctx.fillStyle = "#FF4444";
      ctx.beginPath();
      ctx.arc(cx, cy, 4, 0, Math.PI * 2);
      ctx.fill();
    }

    if (hoverRef.current && !disabled && gameState.status === "playing") {
      const { row, col } = hoverRef.current;
      if (gameState.board[row][col] === Cell.Empty) {
        drawPiece(
          ctx,
          row,
          col,
          gameState.current_player,
          0.35
        );
      }
    }
  }, [gameState, disabled, lastMove]);

  useEffect(() => {
    draw();
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
    [getCellFromPixel, draw]
  );

  const handleMouseLeave = useCallback(() => {
    hoverRef.current = null;
    draw();
  }, [draw]);

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
        onCellClick(cell.row, cell.col);
      }
    },
    [disabled, getCellFromPixel, onCellClick]
  );

  return (
    <canvas
      ref={canvasRef}
      width={CANVAS_SIZE}
      height={CANVAS_SIZE}
      className="game-board"
      onMouseMove={handleMouseMove}
      onMouseLeave={handleMouseLeave}
      onClick={handleClick}
    />
  );
}

function drawPiece(
  ctx: CanvasRenderingContext2D,
  row: number,
  col: number,
  player: Cell,
  alpha: number = 1
) {
  const cx = PADDING + col * CELL_SIZE;
  const cy = PADDING + row * CELL_SIZE;
  const r = PIECE_RADIUS;

  ctx.save();
  ctx.globalAlpha = alpha;

  ctx.shadowColor = "rgba(0,0,0,0.4)";
  ctx.shadowBlur = 4;
  ctx.shadowOffsetX = 2;
  ctx.shadowOffsetY = 2;

  const grad = ctx.createRadialGradient(
    cx - r * 0.3,
    cy - r * 0.3,
    r * 0.1,
    cx,
    cy,
    r
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
  ctx.arc(cx, cy, r, 0, Math.PI * 2);
  ctx.fill();

  ctx.restore();
}

export default GameBoard;
