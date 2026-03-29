import { useRef, useCallback } from "react";
import { BOARD_SIZE } from "../types/game";

interface TouchPreviewOptions {
  padding: number;
  cellSize: number;
  disabled: boolean;
  onCellClick: (row: number, col: number) => void;
}

export function useTouchPreview({
  padding,
  cellSize,
  disabled,
  onCellClick,
}: TouchPreviewOptions) {
  const hoverRef = useRef<{ row: number; col: number } | null>(null);
  const rafRef = useRef<number>(0);

  const getCellFromPixel = useCallback(
    (x: number, y: number) => {
      const col = Math.round((x - padding) / cellSize);
      const row = Math.round((y - padding) / cellSize);
      if (row < 0 || row >= BOARD_SIZE || col < 0 || col >= BOARD_SIZE) return null;
      const cx = padding + col * cellSize;
      const cy = padding + row * cellSize;
      const dist = Math.sqrt((x - cx) ** 2 + (y - cy) ** 2);
      if (dist > cellSize * 0.48) return null;
      return { row, col };
    },
    [padding, cellSize]
  );

  const handleTouchStart = useCallback(
    (e: React.TouchEvent<HTMLCanvasElement>) => {
      if (disabled) return;
      e.preventDefault();
      const touch = e.touches[0];
      const canvas = e.currentTarget;
      const rect = canvas.getBoundingClientRect();
      const scaleX = canvas.width / rect.width;
      const scaleY = canvas.height / rect.height;
      const x = (touch.clientX - rect.left) * scaleX;
      const y = (touch.clientY - rect.top) * scaleY;
      const cell = getCellFromPixel(x, y);
      hoverRef.current = cell;
    },
    [disabled, getCellFromPixel]
  );

  const handleTouchMove = useCallback(
    (e: React.TouchEvent<HTMLCanvasElement>) => {
      if (disabled) return;
      e.preventDefault();
      cancelAnimationFrame(rafRef.current);
      rafRef.current = requestAnimationFrame(() => {
        const touch = e.touches[0];
        const canvas = e.currentTarget;
        const rect = canvas.getBoundingClientRect();
        const scaleX = canvas.width / rect.width;
        const scaleY = canvas.height / rect.height;
        const x = (touch.clientX - rect.left) * scaleX;
        const y = (touch.clientY - rect.top) * scaleY;
        const cell = getCellFromPixel(x, y);
        hoverRef.current = cell;
      });
    },
    [disabled, getCellFromPixel]
  );

  const handleTouchEnd = useCallback(
    (e: React.TouchEvent<HTMLCanvasElement>) => {
      if (disabled) return;
      e.preventDefault();
      const cell = hoverRef.current;
      if (cell) {
        onCellClick(cell.row, cell.col);
      }
      hoverRef.current = null;
    },
    [disabled, onCellClick]
  );

  return { hoverRef, handleTouchStart, handleTouchMove, handleTouchEnd, getCellFromPixel };
}
