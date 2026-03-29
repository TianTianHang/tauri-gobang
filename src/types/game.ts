export enum Cell {
  Empty = "empty",
  Black = "black",
  White = "white",
}

export enum GameStatus {
  Playing = "playing",
  BlackWins = "black_wins",
  WhiteWins = "white_wins",
  Draw = "draw",
}

export interface MoveRecord {
  row: number;
  col: number;
  player: Cell;
}

export interface GameState {
  board: Cell[][];
  current_player: Cell;
  status: GameStatus;
  history: MoveRecord[];
  winner: Cell;
}

export interface MoveResult {
  state: GameState;
  ai_thinking: boolean;
}

export interface AiMoveResult {
  row: number;
  state: GameState;
  col: number;
}

export type Difficulty = "easy" | "medium" | "hard";

export type ConnectionStatus = "connected" | "connecting" | "disconnected";

export interface ConnectionInfo {
  ip: string;
  port: number;
}

export type GameMode = "menu" | "ai" | "online_host" | "online_client" | "host_setup" | "join_setup";

export const BOARD_SIZE = 15;

export const DIFFICULTY_LABELS: Record<Difficulty, string> = {
  easy: "简单",
  medium: "中等",
  hard: "困难",
};
