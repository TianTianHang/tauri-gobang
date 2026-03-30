mod auth;
mod db;
mod logging;
mod protocol;
mod room;
mod types;
mod ws;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::body::Body;
use clap::Parser;
use serde_json::json;
use std::path::PathBuf;
use std::time::Instant;
use tower_http::cors::CorsLayer;
use axum::middleware::Next;
use axum::response::Response;
use axum::http::Request;

use auth::Sessions;
use room::{Room, RoomMap};
use types::*;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Parser)]
#[command(name = "gobang-server", version, about = "Gobang game server")]
struct Cli {
    /// Server host
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Server port
    #[arg(short, long, default_value_t = 3001)]
    port: u16,

    /// Database path (relative to data dir)
    #[arg(long, default_value = "database.db")]
    database: String,

    /// Data directory path
    #[arg(long, default_value = "")]
    data_dir: String,

    /// Run as daemon (fork to background)
    #[arg(long)]
    daemon: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,gobang_server=debug".parse().unwrap()),
        )
        .init();

    let data_dir = if cli.data_dir.is_empty() {
        dirs_home()?.join(".gobang-server")
    } else {
        PathBuf::from(&cli.data_dir)
    };

    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
        println!("\x1b[32m\u{2713}\x1b[0m 数据目录已创建: {}", data_dir.display());
    }

    let db_path = data_dir.join(&cli.database).to_string_lossy().to_string();
    let pool = db::create_pool(&db_path).await?;
    db::init_database(&pool).await?;
    println!("\x1b[32m\u{2713}\x1b[0m 数据库已连接: {}", db_path);

    let config_path = data_dir.join("config.toml");
    if !config_path.exists() {
        let default_config = format!(
            r#"# Gobang Server Configuration
server_host = "{}"
server_port = {}
database_path = "database.db"
log_level = "info"
reconnect_timeout_seconds = 30
password_min_length = 6
"#,
            cli.host, cli.port
        );
        std::fs::write(&config_path, default_config)?;
        println!("\x1b[32m\u{2713}\x1b[0m 配置文件已创建: {}", config_path.display());
    }

    if cli.daemon {
        daemonize()?;
    }

    let sessions: Sessions = Arc::new(RwLock::new(std::collections::HashMap::new()));
    let rooms: RoomMap = Arc::new(RwLock::new(std::collections::HashMap::new()));

    let app_state = ws::AppState {
        db: pool,
        sessions,
        rooms,
    };

    let app = Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/rooms", get(list_rooms).post(create_room))
        .route("/api/rooms/{room_id}/join", post(join_room))
        .route("/game/{room_id}", get(ws::ws_handler))
        .layer(CorsLayer::permissive())
        .layer(axum::middleware::from_fn(logging_middleware))
        .with_state(app_state);

    let addr = format!("{}:{}", cli.host, cli.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("\x1b[32m\u{2713}\x1b[0m 服务器启动在: ws://{}:{}", cli.host, cli.port);
    println!("\x1b[32m\u{2713}\x1b[0m 按 Ctrl+C 停止服务器");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    println!("\x1b[32m\u{2713}\x1b[0m 服务器已停止");

    Ok(())
}

fn dirs_home() -> anyhow::Result<PathBuf> {
    if cfg!(target_os = "windows") {
        if let Some(appdata) = std::env::var_os("APPDATA") {
            Ok(PathBuf::from(appdata).join("gobang-server"))
        } else {
            anyhow::bail!("APPDATA environment variable not set")
        }
    } else {
        Ok(dirs_home_unix())
    }
}

fn dirs_home_unix() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

#[cfg(unix)]
fn daemonize() -> anyhow::Result<()> {
    use std::process::{Command, Stdio};

    let self_path = std::env::current_exe()?;

    let args: Vec<String> = std::env::args().collect();
    let filtered_args: Vec<String> = args
        .iter()
        .filter(|a| *a != "--daemon")
        .cloned()
        .collect();

    Command::new(&self_path)
        .args(&filtered_args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()?;

    println!("\x1b[32m\u{2713}\x1b[0m 服务器已在后台运行");
    std::process::exit(0);
}

#[cfg(not(unix))]
fn daemonize() -> anyhow::Result<()> {
    anyhow::bail!("daemon mode is only supported on Unix systems")
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
}

async fn logging_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let request_id = logging::generate_request_id();
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let start = Instant::now();

    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        "request started"
    );

    let response = next.run(request).await;
    let duration = start.elapsed();

    let status = response.status();

    if duration.as_secs_f64() > 1.0 {
        tracing::warn!(
            request_id = %request_id,
            method = %method,
            path = %path,
            status = %status,
            duration_ms = duration.as_millis(),
            "slow request"
        );
    } else {
        tracing::info!(
            request_id = %request_id,
            method = %method,
            path = %path,
            status = %status,
            duration_ms = duration.as_millis(),
            "request completed"
        );
    }

    response
}

fn extract_token(headers: &axum::http::HeaderMap, query: &std::collections::HashMap<String, String>) -> Option<String> {
    if let Some(auth_header) = headers.get("Authorization") {
        let auth_str = auth_header.to_str().ok()?;
        if auth_str.starts_with("Bearer ") {
            return Some(auth_str[7..].to_string());
        }
    }
    query.get("token").cloned()
}

async fn register(
    State(state): State<ws::AppState>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    let username = req.username.trim().to_string();
    if username.len() < 3 || username.len() > 20 {
        tracing::warn!(username = %username, "registration failed: username length invalid");
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "用户名长度需在 3-20 个字符之间"})),
        );
    }

    if req.password.len() < 6 {
        tracing::warn!(username = %username, "registration failed: password too short");
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "密码至少需要 6 个字符"})),
        );
    }

    let existing = db::find_user_by_username(&state.db, &username).await;
    match existing {
        Ok(Some(_)) => {
            tracing::warn!(username = %username, "registration failed: username already exists");
            (
                StatusCode::CONFLICT,
                Json(json!({"error": "用户名已存在"})),
            )
        }
        Ok(None) => {
            let user_id = auth::generate_id();
            let hash = match auth::hash_password(&req.password).await {
                Ok(h) => h,
                Err(e) => {
                    tracing::error!(error = %e, "registration failed: password hashing error");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e})),
                    );
                }
            };

            if let Err(e) = db::insert_user(&state.db, &user_id, &username, &hash).await {
                tracing::error!(error = %e, user_id = %user_id, "registration failed: database error");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("注册失败: {}", e)})),
                );
            }

            let token = auth::generate_token();
            auth::store_session(&state.sessions, token.clone(), user_id.clone()).await;

            tracing::info!(user_id = %user_id, username = %username, "user registered successfully");

            (
                StatusCode::CREATED,
                Json(json!({
                    "token": token,
                    "user_id": user_id,
                    "username": username,
                })),
            )
        }
        Err(e) => {
            tracing::error!(error = %e, "registration failed: database error");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("数据库错误: {}", e)})),
            )
        }
    }
}

async fn login(
    State(state): State<ws::AppState>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    match db::find_user_by_username(&state.db, req.username.trim()).await {
        Ok(Some(user)) => {
            match auth::verify_password(&req.password, &user.password_hash).await {
                Ok(true) => {
                    let token = auth::generate_token();
                    auth::store_session(&state.sessions, token.clone(), user.id.clone()).await;

                    tracing::info!(user_id = %user.id, username = %user.username, "user logged in successfully");

                    (
                        StatusCode::OK,
                        Json(json!({
                            "token": token,
                            "user_id": user.id,
                            "username": user.username,
                        })),
                    )
                }
                Ok(false) => {
                    tracing::warn!(username = %user.username, "login failed: invalid password");
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({"error": "用户名或密码错误"})),
                    )
                }
                Err(e) => {
                    tracing::error!(error = %e, "login failed: password verification error");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e})),
                    )
                }
            }
        }
        Ok(None) => {
            tracing::warn!(username = %req.username.trim(), "login failed: user not found");
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "用户名或密码错误"})),
            )
        }
        Err(e) => {
            tracing::error!(error = %e, "login failed: database error");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("数据库错误: {}", e)})),
            )
        }
    }
}

async fn list_rooms(
    State(state): State<ws::AppState>,
    headers: axum::http::HeaderMap,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match extract_token(&headers, &params) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "缺少认证令牌"})),
            );
        }
    };

    match auth::verify_token(&state.sessions, &token).await {
        Some(_) => {}
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "无效的认证令牌"})),
            );
        }
    }

    match db::list_waiting_rooms(&state.db).await {
        Ok(rooms) => (StatusCode::OK, Json(json!({"rooms": rooms}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("获取房间列表失败: {}", e)})),
        ),
    }
}

async fn create_room(
    State(state): State<ws::AppState>,
    headers: axum::http::HeaderMap,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
    Json(req): Json<CreateRoomRequest>,
) -> impl IntoResponse {
    let token = match extract_token(&headers, &params) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "缺少认证令牌"})),
            );
        }
    };

    let user_id = match auth::verify_token(&state.sessions, &token).await {
        Some(id) => id,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "无效的认证令牌"})),
            );
        }
    };

    let username = match db::find_user_by_id(&state.db, &user_id).await {
        Ok(Some(u)) => u.username,
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "用户不存在"})),
            );
        }
    };

    let room_id = auth::generate_id();
    if let Err(e) = db::insert_room(&state.db, &room_id, &req.name, &user_id).await {
        tracing::error!(error = %e, "room creation failed: database error");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("创建房间失败: {}", e)})),
        );
    }

    let (tx, _rx) = tokio::sync::mpsc::channel::<String>(32);
    let room = Room::new(
        room_id.clone(),
        req.name.clone(),
        user_id.clone(),
        username,
        tx,
    );

    state.rooms.write().await.insert(room_id.clone(), room);

    tracing::info!(
        room_id = %room_id,
        room_name = %req.name,
        host_id = %user_id,
        "room created successfully"
    );

    (
        StatusCode::CREATED,
        Json(json!({
            "room_id": room_id,
            "room_name": req.name,
            "ws_url": format!("/game/{}", room_id)
        })),
    )
}

async fn join_room(
    State(state): State<ws::AppState>,
    headers: axum::http::HeaderMap,
    axum::extract::Path(room_id): axum::extract::Path<String>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match extract_token(&headers, &params) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "缺少认证令牌"})),
            );
        }
    };

    let user_id = match auth::verify_token(&state.sessions, &token).await {
        Some(id) => id,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "无效的认证令牌"})),
            );
        }
    };

    let username = match db::find_user_by_id(&state.db, &user_id).await {
        Ok(Some(u)) => u.username,
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "用户不存在"})),
            );
        }
    };

    let mut rooms = state.rooms.write().await;
    let room = match rooms.get_mut(&room_id) {
        Some(r) => r,
        None => {
            tracing::warn!(room_id = %room_id, "join room failed: room not found");
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "房间不存在"})),
            );
        }
    };

    if room.host_id == user_id {
        tracing::warn!(room_id = %room_id, user_id = %user_id, "join room failed: cannot join own room");
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "无法加入自己创建的房间"})),
        );
    }

    if room.status != crate::room::RoomStatus::Waiting {
        tracing::warn!(room_id = %room_id, "join room failed: room not in waiting status");
        return (
            StatusCode::CONFLICT,
            Json(json!({"error": "房间不可加入"})),
        );
    }

    if let Err(e) = db::update_room_player2(&state.db, &room_id, &user_id).await {
        tracing::error!(error = %e, "join room failed: database error");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("加入房间失败: {}", e)})),
        );
    }

    let (tx, _rx) = tokio::sync::mpsc::channel::<String>(32);
    room.add_player(user_id.clone(), username.clone(), tx);

    tracing::info!(
        room_id = %room_id,
        user_id = %user_id,
        "player joined room successfully"
    );

    let host_username = room
        .players
        .get(&room.host_id)
        .map(|p| p.username.clone())
        .unwrap_or_default();

    (
        StatusCode::OK,
        Json(json!({
            "room_id": room_id,
            "host_username": host_username,
            "ws_url": format!("/game/{}", room_id)
        })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode, Method};
    use tower::util::ServiceExt;

    async fn create_test_app() -> Router {
        let pool = db::create_pool(":memory:").await.unwrap();
        db::init_database(&pool).await.unwrap();

        let sessions: Sessions = Arc::new(RwLock::new(std::collections::HashMap::new()));
        let rooms: RoomMap = Arc::new(RwLock::new(std::collections::HashMap::new()));

        let app_state = ws::AppState {
            db: pool,
            sessions,
            rooms,
        };

        Router::new()
            .route("/api/register", post(register))
            .route("/api/login", post(login))
            .route("/api/rooms", get(list_rooms).post(create_room))
            .route("/api/rooms/{room_id}/join", post(join_room))
            .layer(CorsLayer::permissive())
            .with_state(app_state)
    }

    async fn register_user(app: &Router, username: &str, password: &str) -> serde_json::Value {
        let body = format!(r#"{{"username":"{}","password":"{}"}}"#, username, password);
        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/register")
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&body_bytes).unwrap()
    }

    #[tokio::test]
    async fn test_register_success() {
        let app = create_test_app().await;
        let body = register_user(&app, "testuser", "password123").await;
        assert!(body.get("token").is_some());
        assert!(body.get("user_id").is_some());
        assert_eq!(body["username"], "testuser");
    }

    #[tokio::test]
    async fn test_register_validation_username_too_short() {
        let app = create_test_app().await;
        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/register")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"username":"ab","password":"password123"}"#))
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_register_validation_password_too_short() {
        let app = create_test_app().await;
        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/register")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"username":"testuser","password":"12345"}"#))
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_register_duplicate_username() {
        let app = create_test_app().await;
        register_user(&app, "testuser", "password123").await;
        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/register")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"username":"testuser","password":"password456"}"#))
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_login_success() {
        let app = create_test_app().await;
        register_user(&app, "testuser", "password123").await;

        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/login")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"username":"testuser","password":"password123"}"#))
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert!(json.get("token").is_some());
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let app = create_test_app().await;
        register_user(&app, "testuser", "password123").await;

        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/login")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"username":"testuser","password":"wrongpass"}"#))
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_rooms_unauthorized() {
        let app = create_test_app().await;
        let req = Request::builder()
            .method(Method::GET)
            .uri("/api/rooms")
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_rooms_invalid_token() {
        let app = create_test_app().await;
        let req = Request::builder()
            .method(Method::GET)
            .uri("/api/rooms?token=invalid")
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_create_and_list_rooms() {
        let app = create_test_app().await;
        let reg = register_user(&app, "alice", "password123").await;
        let token = reg["token"].as_str().unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("/api/rooms?token={}", token))
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"Test Room"}"#))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);

        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("/api/rooms?token={}", token))
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json["rooms"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_join_room_success() {
        let app = create_test_app().await;
        let reg1 = register_user(&app, "alice", "password123").await;
        let reg2 = register_user(&app, "bob", "password123").await;
        let token1 = reg1["token"].as_str().unwrap();
        let token2 = reg2["token"].as_str().unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("/api/rooms?token={}", token1))
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"Test Room"}"#))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let room: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let room_id = room["room_id"].as_str().unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("/api/rooms/{}/join?token={}", room_id, token2))
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_join_own_room_rejected() {
        let app = create_test_app().await;
        let reg = register_user(&app, "alice", "password123").await;
        let token = reg["token"].as_str().unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("/api/rooms?token={}", token))
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"Test Room"}"#))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let room: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let room_id = room["room_id"].as_str().unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("/api/rooms/{}/join?token={}", room_id, token))
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_join_nonexistent_room() {
        let app = create_test_app().await;
        let reg = register_user(&app, "alice", "password123").await;
        let token = reg["token"].as_str().unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("/api/rooms/nonexistent/join?token={}", token))
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
