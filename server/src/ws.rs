use std::time::{Duration, Instant};

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Query, State, WebSocketUpgrade};
use futures_util::{SinkExt, StreamExt};
use crate::protocol::game::{GameMessage, ServerMessage};
use serde::Deserialize;
use tokio::sync::mpsc;

use crate::auth::Sessions;
use crate::db;
use crate::room::{RoomMap, RoomStatus};

#[derive(Deserialize)]
pub struct WsParams {
    token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<WsParams>,
    room_id: axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state, params.token, room_id.0))
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub sessions: Sessions,
    pub rooms: RoomMap,
}

async fn handle_ws(socket: WebSocket, state: AppState, token: String, room_id: String) {
    let user_id = match crate::auth::verify_token(&state.sessions, &token).await {
        Some(id) => id,
        None => {
            tracing::warn!("ws: invalid token");
            return;
        }
    };

    let username = match db::find_user_by_id(&state.db, &user_id).await {
        Ok(Some(u)) => u.username,
        _ => {
            tracing::warn!("ws: user {} not found", user_id);
            return;
        }
    };

    let mut rooms = state.rooms.write().await;
    let room = match rooms.get_mut(&room_id) {
        Some(r) => r,
        None => {
            tracing::warn!("ws: room {} not found", room_id);
            return;
        }
    };

    if !room.is_participant(&user_id) {
        tracing::warn!("ws: user {} not in room {}", user_id, room_id);
        return;
    }

    let is_reconnect = room.disconnected.is_some()
        && room.disconnected.as_ref().unwrap().0 == user_id;

    if is_reconnect {
        let elapsed = room.disconnected.as_ref().unwrap().1.elapsed();
        if elapsed > Duration::from_secs(30) {
            tracing::warn!("ws: reconnect timeout for user {}", user_id);
            return;
        }

        if let Some(handle) = room.timeout_handle.take() {
            handle.abort();
        }
        room.disconnected = None;

        let (ws_tx, mut ws_rx) = socket.split();
        let (tx, mut rx) = mpsc::channel::<String>(32);

        room.remove_player(&user_id);
        room.players.insert(
            user_id.clone(),
            crate::room::PlayerConnection {
                user_id: user_id.clone(),
                username: username.clone(),
                sender: tx.clone(),
            },
        );

        let reconnected_msg = ServerMessage::PlayerReconnected {
            username: username.clone(),
        };
        let reconnected_json = serde_json::to_string(&reconnected_msg).unwrap();

        if let Some(opp_id) = room.get_opponent_id(&user_id) {
            room.send_to_player(&opp_id, &reconnected_json).await;
        }
        room.send_to_player(&user_id, &reconnected_json).await;

        drop(rooms);

        let user_id_clone = user_id.clone();
        tokio::spawn(async move {
            let mut ws_tx = ws_tx;
            while let Some(msg) = rx.recv().await {
                if ws_tx.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
        });

        forward_messages(&state, room_id, user_id_clone, &mut ws_rx, None).await;
    } else {
        let room_status = room.status.clone();
        let (ws_tx, mut ws_rx) = socket.split();
        let (tx, mut rx) = mpsc::channel::<String>(32);

        room.remove_player(&user_id);
        room.players.insert(
            user_id.clone(),
            crate::room::PlayerConnection {
                user_id: user_id.clone(),
                username: username.clone(),
                sender: tx.clone(),
            },
        );

        if room_status == RoomStatus::Waiting && room.players.len() >= 2 {
            room.status = RoomStatus::Playing;

            let host_username = room
                .players
                .get(&room.host_id)
                .map(|p| p.username.clone())
                .unwrap_or_default();
            let joiner_username = room
                .player2_id
                .as_ref()
                .and_then(|id| room.players.get(id))
                .map(|p| p.username.clone())
                .unwrap_or_default();

            let start_msg = ServerMessage::GameStart {
                black_player: host_username.clone(),
                white_player: joiner_username.clone(),
            };
            let start_json = serde_json::to_string(&start_msg).unwrap();
            room.broadcast(&start_json).await;

            let opponent_msg = ServerMessage::OpponentJoined {
                username: joiner_username,
            };
            let opponent_json = serde_json::to_string(&opponent_msg).unwrap();
            room.send_to_player(&room.host_id, &opponent_json).await;
        }

        drop(rooms);

        let user_id_clone = user_id.clone();
        tokio::spawn(async move {
            let mut ws_tx = ws_tx;
            while let Some(msg) = rx.recv().await {
                if ws_tx.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
        });

        forward_messages(&state, room_id, user_id_clone, &mut ws_rx, None).await;
    }
}

async fn forward_messages(
    state: &AppState,
    room_id: String,
    user_id: String,
    ws_rx: &mut futures_util::stream::SplitStream<WebSocket>,
    _game_state: Option<()>,
) {
    while let Some(msg) = ws_rx.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let text_str: &str = &text;

                match serde_json::from_str::<GameMessage>(text_str) {
                    Ok(_) => {
                        let rooms = state.rooms.read().await;
                        if let Some(room) = rooms.get(&room_id) {
                            if let Some(opp_id) = room.get_opponent_id(&user_id) {
                                room.send_to_player(&opp_id, text_str).await;
                            }
                        }
                    }
                    Err(_) => {
                        tracing::debug!("ws: unknown message format: {}", text_str);
                    }
                }
            }
            Ok(Message::Close(_)) => {
                tracing::info!("ws: user {} closed connection", user_id);
                break;
            }
            Err(e) => {
                tracing::warn!("ws: error for user {}: {}", user_id, e);
                break;
            }
            _ => {}
        }
    }

    handle_disconnect(state, room_id, user_id).await;
}

async fn handle_disconnect(state: &AppState, room_id: String, user_id: String) {
    let mut rooms = state.rooms.write().await;
    let room = match rooms.get_mut(&room_id) {
        Some(r) => r,
        None => return,
    };

    let _ = room.send_to_opponent(&user_id, "").await;

    if room.status != RoomStatus::Playing {
        room.remove_player(&user_id);
        if room.is_empty() {
            rooms.remove(&room_id);
        }
        return;
    }

    let disconnected_username = room
        .players
        .get(&user_id)
        .map(|p| p.username.clone())
        .unwrap_or_default();

    room.remove_player(&user_id);

    if room.disconnected.is_some() {
        tracing::info!("ws: second disconnect in room {}, cleaning up", room_id);
        if let Some(handle) = room.timeout_handle.take() {
            handle.abort();
        }
        let end_msg = ServerMessage::GameEnded {
            winner: None,
            reason: "both_disconnected".to_string(),
        };
        let end_json = serde_json::to_string(&end_msg).unwrap();
        room.broadcast(&end_json).await;
        if let Err(e) = db::update_room_status(&state.db, &room_id, "ended").await {
            tracing::error!("failed to update room status: {}", e);
        }
        rooms.remove(&room_id);
        return;
    }

    let opponent_id = room.get_opponent_id(&user_id);
    room.disconnected = Some((user_id.clone(), Instant::now()));

    let msg = ServerMessage::OpponentDisconnected {
        username: disconnected_username,
        can_reconnect: true,
        timeout_seconds: 30,
    };
    let msg_json = serde_json::to_string(&msg).unwrap();

    if let Some(ref opp_id) = opponent_id {
        room.send_to_player(opp_id, &msg_json).await;
    }

    let state_clone = state.clone();
    let room_id_clone = room_id.clone();
    let disconnected_user_id = user_id.clone();
    let timeout_handle = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(30)).await;

        let mut rooms = state_clone.rooms.write().await;
        let room = match rooms.get_mut(&room_id_clone) {
            Some(r) => r,
            None => return,
        };

        let still_disconnected = room
            .disconnected
            .as_ref()
            .map(|(uid, _)| uid == &disconnected_user_id)
            .unwrap_or(false);

        if !still_disconnected {
            return;
        }

        let winner_id = room
            .players
            .keys()
            .next()
            .cloned()
            .or_else(|| room.get_opponent_id(&disconnected_user_id));

        let end_msg = ServerMessage::GameEnded {
            winner: winner_id.clone(),
            reason: "opponent_disconnected".to_string(),
        };
        let end_json = serde_json::to_string(&end_msg).unwrap();
        room.broadcast(&end_json).await;

        if let Err(e) = db::update_room_status(&state_clone.db, &room_id_clone, "ended").await {
            tracing::error!("failed to update room status: {}", e);
        }

        let game_id = uuid::Uuid::new_v4().to_string();
        if let Err(e) = db::insert_game(
            &state_clone.db,
            &game_id,
            &room_id_clone,
            winner_id.as_deref(),
            "opponent_disconnected",
        )
        .await
        {
            tracing::error!("failed to insert game record: {}", e);
        }

        rooms.remove(&room_id_clone);
    });

    room.timeout_handle = Some(timeout_handle);
}
