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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_message_move_serialization() {
        let msg = GameMessage::Move { row: 7, col: 8 };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: GameMessage = serde_json::from_str(&json).unwrap();
        match parsed {
            GameMessage::Move { row, col } => {
                assert_eq!(row, 7);
                assert_eq!(col, 8);
            }
            _ => panic!("expected Move"),
        }
    }

    #[test]
    fn test_game_message_restart_request() {
        let msg = GameMessage::RestartRequest;
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"restart_request\""));
        let parsed: GameMessage = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, GameMessage::RestartRequest));
    }

    #[test]
    fn test_server_message_game_start() {
        let msg = ServerMessage::GameStart {
            black_player: "Alice".to_string(),
            white_player: "Bob".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: ServerMessage = serde_json::from_str(&json).unwrap();
        match parsed {
            ServerMessage::GameStart {
                black_player,
                white_player,
            } => {
                assert_eq!(black_player, "Alice");
                assert_eq!(white_player, "Bob");
            }
            _ => panic!("expected GameStart"),
        }
    }

    #[test]
    fn test_server_message_opponent_disconnected() {
        let msg = ServerMessage::OpponentDisconnected {
            username: "Bob".to_string(),
            can_reconnect: true,
            timeout_seconds: 30,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: ServerMessage = serde_json::from_str(&json).unwrap();
        match parsed {
            ServerMessage::OpponentDisconnected {
                username,
                can_reconnect,
                timeout_seconds,
            } => {
                assert_eq!(username, "Bob");
                assert!(can_reconnect);
                assert_eq!(timeout_seconds, 30);
            }
            _ => panic!("expected OpponentDisconnected"),
        }
    }

    #[test]
    fn test_server_message_game_ended() {
        let msg = ServerMessage::GameEnded {
            winner: Some("Alice".to_string()),
            reason: "opponent_disconnected".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: ServerMessage = serde_json::from_str(&json).unwrap();
        match parsed {
            ServerMessage::GameEnded { winner, reason } => {
                assert_eq!(winner, Some("Alice".to_string()));
                assert_eq!(reason, "opponent_disconnected");
            }
            _ => panic!("expected GameEnded"),
        }
    }

    #[test]
    fn test_deserialize_from_client_json() {
        let json = r#"{"type":"move","row":3,"col":5}"#;
        let msg: GameMessage = serde_json::from_str(json).unwrap();
        match msg {
            GameMessage::Move { row, col } => {
                assert_eq!(row, 3);
                assert_eq!(col, 5);
            }
            _ => panic!("expected Move"),
        }
    }
}
