use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

pub type Sessions = Arc<RwLock<HashMap<String, String>>>;

pub async fn hash_password(password: &str) -> Result<String, String> {
    bcrypt::hash(password, 12).map_err(|e| format!("password hashing failed: {}", e))
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    bcrypt::verify(password, hash).map_err(|e| format!("password verification failed: {}", e))
}

pub fn generate_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

pub async fn store_session(sessions: &Sessions, token: String, user_id: String) {
    sessions.write().await.insert(token, user_id);
}

pub async fn verify_token(sessions: &Sessions, token: &str) -> Option<String> {
    sessions.read().await.get(token).cloned()
}

pub async fn remove_session(sessions: &Sessions, token: &str) {
    sessions.write().await.remove(token);
}
