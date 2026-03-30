use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameMessage {
    Move { row: usize, col: usize },
    RestartRequest,
    RestartAccept,
    RestartReject,
    Disconnect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    GameStart {
        black_player: String,
        white_player: String,
    },
    OpponentJoined {
        username: String,
    },
    OpponentDisconnected {
        username: String,
        can_reconnect: bool,
        timeout_seconds: u64,
    },
    PlayerReconnected {
        username: String,
    },
    GameEnded {
        winner: Option<String>,
        reason: String,
    },
}
