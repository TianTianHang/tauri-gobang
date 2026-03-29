import { useState, useEffect, useCallback } from "react";
import { BOARD_SIZE } from "../types/game";

interface BoardSize {
  canvas: number;
  cell: number;
  padding: number;
}

function debounce(fn: () => void, ms: number) {
  let timer: ReturnType<typeof setTimeout>;
  return () => {
    clearTimeout(timer);
    timer = setTimeout(fn, ms);
  };
}

export function useBoardSize(): BoardSize {
  const [size, setSize] = useState<BoardSize>({ canvas: 0, cell: 0, padding: 0 });

  const calculate = useCallback(() => {
    const statusBarH = 44;
    const paddingOuter = 16;
    const availableH = window.innerHeight - statusBarH - paddingOuter * 2;
    const availableW = window.innerWidth - paddingOuter * 2;
    const canvasSize = Math.min(availableW, availableH, 600);
    const padding = Math.floor(canvasSize * 0.05);
    const cell = (canvasSize - padding * 2) / (BOARD_SIZE - 1);

    setSize({
      canvas: Math.floor(canvasSize),
      cell: Math.floor(cell * 10) / 10,
      padding,
    });
  }, []);

  useEffect(() => {
    calculate();
    const handleResize = debounce(calculate, 100);
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, [calculate]);

  return size;
}
