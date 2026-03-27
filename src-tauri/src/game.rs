use serde::{Deserialize, Serialize};

pub const BOARD_SIZE: usize = 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cell {
    Empty,
    Black,
    White,
}

impl Cell {
    pub fn opposite(&self) -> Self {
        match self {
            Cell::Empty => Cell::Empty,
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    Playing,
    BlackWins,
    WhiteWins,
    Draw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveRecord {
    pub row: usize,
    pub col: usize,
    pub player: Cell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Vec<Vec<Cell>>,
    pub current_player: Cell,
    pub status: GameStatus,
    pub history: Vec<MoveRecord>,
    pub winner: Cell,
}

impl GameState {
    pub fn new() -> Self {
        let board = vec![vec![Cell::Empty; BOARD_SIZE]; BOARD_SIZE];
        GameState {
            board,
            current_player: Cell::Black,
            status: GameStatus::Playing,
            history: vec![],
            winner: Cell::Empty,
        }
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), String> {
        if self.status != GameStatus::Playing {
            return Err("Game is already over".to_string());
        }
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return Err("Position out of bounds".to_string());
        }
        if self.board[row][col] != Cell::Empty {
            return Err("Cell is already occupied".to_string());
        }

        self.board[row][col] = self.current_player;
        self.history.push(MoveRecord {
            row,
            col,
            player: self.current_player,
        });

        if self.check_win(row, col, self.current_player) {
            self.status = if self.current_player == Cell::Black {
                GameStatus::BlackWins
            } else {
                GameStatus::WhiteWins
            };
            self.winner = self.current_player;
        } else if self.history.len() == BOARD_SIZE * BOARD_SIZE {
            self.status = GameStatus::Draw;
        } else {
            self.current_player = self.current_player.opposite();
        }

        Ok(())
    }

    pub fn undo_move(&mut self) -> Result<(), String> {
        if self.history.is_empty() {
            return Err("No moves to undo".to_string());
        }
        if self.status != GameStatus::Playing {
            self.status = GameStatus::Playing;
            self.winner = Cell::Empty;
        }
        let last = self.history.pop().unwrap();
        self.board[last.row][last.col] = Cell::Empty;
        self.current_player = last.player;
        Ok(())
    }

    pub fn check_win(&self, row: usize, col: usize, player: Cell) -> bool {
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];
        for (dr, dc) in directions {
            let mut count = 1;
            for dir in [1i32, -1] {
                let mut r = row as i32 + dr * dir;
                let mut c = col as i32 + dc * dir;
                while r >= 0
                    && r < BOARD_SIZE as i32
                    && c >= 0
                    && c < BOARD_SIZE as i32
                    && self.board[r as usize][c as usize] == player
                {
                    count += 1;
                    r += dr * dir;
                    c += dc * dir;
                }
            }
            if count >= 5 {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn get_candidates(&self, range: usize) -> Vec<(usize, usize)> {
        let mut has_piece = false;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.board[row][col] != Cell::Empty {
                    has_piece = true;
                    break;
                }
            }
            if has_piece {
                break;
            }
        }
        if !has_piece {
            return vec![(7, 7)];
        }

        let mut candidate_set = std::collections::HashSet::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.board[row][col] != Cell::Empty {
                    for dr in -(range as i32)..=(range as i32) {
                        for dc in -(range as i32)..=(range as i32) {
                            let nr = row as i32 + dr;
                            let nc = col as i32 + dc;
                            if nr >= 0
                                && nr < BOARD_SIZE as i32
                                && nc >= 0
                                && nc < BOARD_SIZE as i32
                                && self.board[nr as usize][nc as usize] == Cell::Empty
                            {
                                candidate_set.insert((nr as usize, nc as usize));
                            }
                        }
                    }
                }
            }
        }
        candidate_set.into_iter().collect()
    }
}
