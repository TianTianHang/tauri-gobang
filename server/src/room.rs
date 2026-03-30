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
