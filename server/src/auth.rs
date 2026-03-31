use std::collections::HashMap;
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tokio::sync::RwLock;
use uuid::Uuid;

pub type Sessions = Arc<RwLock<HashMap<String, String>>>;

pub async fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("password hashing failed: {}", e))
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| format!("password hash parsing failed: {}", e))?;
    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash_and_verify_password() {
        let password = "test_password_123";
        let hash = hash_password(password).await.unwrap();
        assert!(verify_password(password, &hash).await.unwrap());
        assert!(!verify_password("wrong_password", &hash).await.unwrap());
    }

    #[tokio::test]
    async fn test_different_passwords_produce_different_hashes() {
        let hash1 = hash_password("password1").await.unwrap();
        let hash2 = hash_password("password2").await.unwrap();
        assert_ne!(hash1, hash2);
    }

    #[tokio::test]
    async fn test_same_password_produces_different_hashes() {
        let password = "same_password";
        let hash1 = hash_password(password).await.unwrap();
        let hash2 = hash_password(password).await.unwrap();
        assert_ne!(hash1, hash2);
        assert!(verify_password(password, &hash1).await.unwrap());
        assert!(verify_password(password, &hash2).await.unwrap());
    }

    #[test]
    fn test_generate_token_returns_unique_values() {
        let t1 = generate_token();
        let t2 = generate_token();
        assert_ne!(t1, t2);
        assert_eq!(t1.len(), 36);
    }

    #[test]
    fn test_generate_id_returns_unique_values() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
    }

    #[tokio::test]
    async fn test_session_store_and_verify() {
        let sessions: Sessions = Arc::new(RwLock::new(HashMap::new()));
        let token = generate_token();
        let user_id = "user-123".to_string();

        store_session(&sessions, token.clone(), user_id.clone()).await;
        let result = verify_token(&sessions, &token).await;
        assert_eq!(result, Some(user_id));
    }

    #[tokio::test]
    async fn test_verify_invalid_token_returns_none() {
        let sessions: Sessions = Arc::new(RwLock::new(HashMap::new()));
        let result = verify_token(&sessions, "nonexistent-token").await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_remove_session() {
        let sessions: Sessions = Arc::new(RwLock::new(HashMap::new()));
        let token = generate_token();
        let user_id = "user-123".to_string();

        store_session(&sessions, token.clone(), user_id).await;
        assert!(verify_token(&sessions, &token).await.is_some());

        remove_session(&sessions, &token).await;
        assert!(verify_token(&sessions, &token).await.is_none());
    }
}
