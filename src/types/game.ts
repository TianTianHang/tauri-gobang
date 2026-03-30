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

export type GameMode =
  | "menu"
  | "ai"
  | "online_host"
  | "online_client"
  | "login"
  | "lobby"
  | "waiting";

export interface ConnectionInfo {
  ip: string;
  port: number;
}

export type ConnectionStatus = "disconnected" | "connecting" | "connected";

export const BOARD_SIZE = 15;

export const DIFFICULTY_LABELS: Record<Difficulty, string> = {
  easy: "简单",
  medium: "中等",
  hard: "困难",
};

export interface NetworkMessage {
  type: "move" | "restart_request" | "restart_accept" | "restart_reject" | "disconnect";
  row?: number;
  col?: number;
}

export interface ServerMessage {
  type: "game_start" | "opponent_joined" | "opponent_disconnected" | "player_reconnected" | "game_ended";
  black_player?: string;
  white_player?: string;
  username?: string;
  can_reconnect?: boolean;
  timeout_seconds?: number;
  winner?: string;
  reason?: string;
}

export interface AuthResponse {
  token: string;
  user_id: string;
  username: string;
}

export interface RoomListEntry {
  id: string;
  name: string;
  host_username: string;
  created_at: number;
}

export interface CreateRoomResponse {
  room_id: string;
  room_name: string;
  ws_url: string;
}

export interface JoinRoomResponse {
  room_id: string;
  host_username: string;
  ws_url: string;
}
