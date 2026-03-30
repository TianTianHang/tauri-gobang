use std::collections::HashMap;
use std::time::Instant;

use tokio::sync::mpsc;

pub type RoomMap = std::sync::Arc<tokio::sync::RwLock<HashMap<String, Room>>>;

#[derive(Debug, Clone, PartialEq)]
pub enum RoomStatus {
    Waiting,
    Playing,
    Ended,
}

pub struct Room {
    pub id: String,
    pub name: String,
    pub status: RoomStatus,
    pub host_id: String,
    pub player2_id: Option<String>,
    pub players: HashMap<String, PlayerConnection>,
    pub disconnected: Option<(String, Instant)>,
    pub timeout_handle: Option<tokio::task::JoinHandle<()>>,
}

pub struct PlayerConnection {
    pub user_id: String,
    pub username: String,
    pub sender: mpsc::Sender<String>,
}

impl Room {
    pub fn new(id: String, name: String, host_id: String, host_username: String, host_sender: mpsc::Sender<String>) -> Self {
        let mut players = HashMap::new();
        players.insert(
            host_id.clone(),
            PlayerConnection {
                user_id: host_id.clone(),
                username: host_username,
                sender: host_sender,
            },
        );
        Room {
            id,
            name,
            status: RoomStatus::Waiting,
            host_id,
            player2_id: None,
            players,
            disconnected: None,
            timeout_handle: None,
        }
    }

    pub fn add_player(&mut self, user_id: String, username: String, sender: mpsc::Sender<String>) {
        self.player2_id = Some(user_id.clone());
        self.players.insert(
            user_id.clone(),
            PlayerConnection {
                user_id,
                username,
                sender,
            },
        );
        self.status = RoomStatus::Playing;
    }

    pub fn is_participant(&self, user_id: &str) -> bool {
        self.host_id == user_id || self.player2_id.as_deref() == Some(user_id)
    }

    pub fn remove_player(&mut self, user_id: &str) {
        self.players.remove(user_id);
    }

    pub fn get_opponent_id(&self, user_id: &str) -> Option<String> {
        if self.host_id == user_id {
            self.player2_id.clone()
        } else if self.player2_id.as_deref() == Some(user_id) {
            Some(self.host_id.clone())
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    pub async fn send_to_player(&self, user_id: &str, msg: &str) -> bool {
        if let Some(player) = self.players.get(user_id) {
            player.sender.send(msg.to_string()).await.is_ok()
        } else {
            false
        }
    }

    pub async fn send_to_opponent(&self, user_id: &str, msg: &str) -> bool {
        if let Some(opp_id) = self.get_opponent_id(user_id) {
            self.send_to_player(&opp_id, msg).await
        } else {
            false
        }
    }

    pub async fn broadcast(&self, msg: &str) {
        for player in self.players.values() {
            let _ = player.sender.send(msg.to_string()).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_room(host_id: &str, host_username: &str) -> (Room, mpsc::Receiver<String>) {
        let (tx, rx) = mpsc::channel(32);
        let room = Room::new(
            "room-1".to_string(),
            "Test Room".to_string(),
            host_id.to_string(),
            host_username.to_string(),
            tx,
        );
        (room, rx)
    }

    #[test]
    fn test_room_new() {
        let (room, _rx) = make_room("host-1", "Alice");
        assert_eq!(room.status, RoomStatus::Waiting);
        assert_eq!(room.host_id, "host-1");
        assert!(room.player2_id.is_none());
        assert_eq!(room.players.len(), 1);
        assert!(room.disconnected.is_none());
    }

    #[test]
    fn test_add_player() {
        let (mut room, _rx) = make_room("host-1", "Alice");
        let (tx2, _rx2) = mpsc::channel(32);
        room.add_player("player-2".to_string(), "Bob".to_string(), tx2);

        assert_eq!(room.status, RoomStatus::Playing);
        assert_eq!(room.player2_id, Some("player-2".to_string()));
        assert_eq!(room.players.len(), 2);
    }

    #[test]
    fn test_is_participant() {
        let (room, _rx) = make_room("host-1", "Alice");
        assert!(room.is_participant("host-1"));
        assert!(!room.is_participant("other-user"));
    }

    #[test]
    fn test_is_participant_with_player2() {
        let (mut room, _rx) = make_room("host-1", "Alice");
        let (tx2, _rx2) = mpsc::channel(32);
        room.add_player("player-2".to_string(), "Bob".to_string(), tx2);

        assert!(room.is_participant("host-1"));
        assert!(room.is_participant("player-2"));
        assert!(!room.is_participant("other-user"));
    }

    #[test]
    fn test_get_opponent_id() {
        let (mut room, _rx) = make_room("host-1", "Alice");
        let (tx2, _rx2) = mpsc::channel(32);
        room.add_player("player-2".to_string(), "Bob".to_string(), tx2);

        assert_eq!(room.get_opponent_id("host-1"), Some("player-2".to_string()));
        assert_eq!(room.get_opponent_id("player-2"), Some("host-1".to_string()));
        assert_eq!(room.get_opponent_id("unknown"), None);
    }

    #[test]
    fn test_remove_player() {
        let (mut room, _rx) = make_room("host-1", "Alice");
        let (tx2, _rx2) = mpsc::channel(32);
        room.add_player("player-2".to_string(), "Bob".to_string(), tx2);

        room.remove_player("player-2");
        assert_eq!(room.players.len(), 1);
        assert!(!room.is_empty());
    }

    #[test]
    fn test_is_empty() {
        let (mut room, _rx) = make_room("host-1", "Alice");
        assert!(!room.is_empty());
        room.remove_player("host-1");
        assert!(room.is_empty());
    }

    #[tokio::test]
    async fn test_send_to_player() {
        let (room, mut rx) = make_room("host-1", "Alice");
        let sent = room.send_to_player("host-1", "hello").await;
        assert!(sent);
        let msg = rx.recv().await;
        assert_eq!(msg, Some("hello".to_string()));
    }

    #[tokio::test]
    async fn test_send_to_player_nonexistent() {
        let (room, _rx) = make_room("host-1", "Alice");
        let sent = room.send_to_player("nonexistent", "hello").await;
        assert!(!sent);
    }

    #[tokio::test]
    async fn test_send_to_opponent() {
        let (mut room, _rx) = make_room("host-1", "Alice");
        let (tx2, mut rx2) = mpsc::channel(32);
        room.add_player("player-2".to_string(), "Bob".to_string(), tx2);

        let sent = room.send_to_opponent("host-1", "move").await;
        assert!(sent);
        let msg = rx2.recv().await;
        assert_eq!(msg, Some("move".to_string()));
    }

    #[tokio::test]
    async fn test_broadcast() {
        let (mut room, mut rx1) = make_room("host-1", "Alice");
        let (tx2, mut rx2) = mpsc::channel(32);
        room.add_player("player-2".to_string(), "Bob".to_string(), tx2);

        room.broadcast("game_start").await;
        assert_eq!(rx1.recv().await, Some("game_start".to_string()));
        assert_eq!(rx2.recv().await, Some("game_start".to_string()));
    }
}
