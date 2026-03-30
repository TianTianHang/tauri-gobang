use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

const MIGRATION_SQL: &str = include_str!("../migrations/init.sql");

pub async fn create_pool(database_path: &str) -> Result<SqlitePool> {
    std::fs::create_dir_all(std::path::Path::new(database_path).parent().unwrap())?;

    let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", database_path))?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .busy_timeout(std::time::Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub async fn init_database(pool: &SqlitePool) -> Result<()> {
    sqlx::query(MIGRATION_SQL).execute(pool).await?;
    tracing::info!("database migrations applied");
    Ok(())
}

pub async fn insert_user(
    pool: &SqlitePool,
    id: &str,
    username: &str,
    password_hash: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)",
    )
    .bind(id)
    .bind(username)
    .bind(password_hash)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<types::User>> {
    let row = sqlx::query_as::<_, types::User>(
        "SELECT id, username, password_hash, created_at FROM users WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn find_user_by_id(pool: &SqlitePool, id: &str) -> Result<Option<types::User>> {
    let row = sqlx::query_as::<_, types::User>(
        "SELECT id, username, password_hash, created_at FROM users WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn insert_room(
    pool: &SqlitePool,
    id: &str,
    name: &str,
    host_id: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO rooms (id, name, host_id, status) VALUES (?, ?, ?, 'waiting')",
    )
    .bind(id)
    .bind(name)
    .bind(host_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_room_by_id(pool: &SqlitePool, id: &str) -> Result<Option<types::RoomRecord>> {
    let row = sqlx::query_as::<_, types::RoomRecord>(
        "SELECT id, name, host_id, player2_id, status, created_at, ended_at FROM rooms WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn update_room_player2(pool: &SqlitePool, room_id: &str, player2_id: &str) -> Result<()> {
    sqlx::query("UPDATE rooms SET player2_id = ?, status = 'playing' WHERE id = ?")
        .bind(player2_id)
        .bind(room_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_room_status(pool: &SqlitePool, room_id: &str, status: &str) -> Result<()> {
        let ended_at = if status == "ended" {
            Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64)
        } else {
        None
    };
    if let Some(ts) = ended_at {
        sqlx::query("UPDATE rooms SET status = ?, ended_at = ? WHERE id = ?")
            .bind(status)
            .bind(ts)
            .bind(room_id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE rooms SET status = ? WHERE id = ?")
            .bind(status)
            .bind(room_id)
            .execute(pool)
            .await?;
    }
    Ok(())
}

pub async fn list_waiting_rooms(pool: &SqlitePool) -> Result<Vec<types::RoomListEntry>> {
    let rows = sqlx::query_as::<_, types::RoomListEntry>(
        "SELECT r.id, r.name, u.username as host_username, r.created_at \
         FROM rooms r JOIN users u ON r.host_id = u.id \
         WHERE r.status = 'waiting' ORDER BY r.created_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn insert_game(
    pool: &SqlitePool,
    id: &str,
    room_id: &str,
    winner_id: Option<&str>,
    reason: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO games (id, room_id, winner_id, reason) VALUES (?, ?, ?, ?)",
    )
    .bind(id)
    .bind(room_id)
    .bind(winner_id)
    .bind(reason)
    .execute(pool)
    .await?;
    Ok(())
}

mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
    pub struct User {
        pub id: String,
        pub username: String,
        pub password_hash: String,
        pub created_at: i64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
    pub struct RoomRecord {
        pub id: String,
        pub name: String,
        pub host_id: String,
        pub player2_id: Option<String>,
        pub status: String,
        pub created_at: i64,
        pub ended_at: Option<i64>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
    pub struct RoomListEntry {
        pub id: String,
        pub name: String,
        pub host_username: String,
        pub created_at: i64,
    }
}
