use std::time::{Duration, Instant};

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Query, State, WebSocketUpgrade};
use futures_util::{SinkExt, StreamExt};
use crate::protocol::game::{GameMessage, ServerMessage};
use serde::Deserialize;
use tokio::sync::mpsc;

use crate::auth::Sessions;
use crate::db;
use crate::logging;
use crate::protocol::game::ErrorMessage;
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
    // Note: Using query parameter for token instead of Authorization header
    // because not all browsers support custom headers in WebSocket handshake.
    // This is the recommended approach for WebSocket authentication.
    ws.on_upgrade(move |socket| handle_ws(socket, state, params.token, room_id.0))
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub sessions: Sessions,
    pub rooms: RoomMap,
}

async fn handle_ws(socket: WebSocket, state: AppState, token: String, room_id: String) {
    let connection_id = logging::generate_request_id();
    let token_hash = &token[..8.min(token.len())];

    tracing::info!(
        connection_id = %connection_id,
        room_id = %room_id,
        token_hash = %token_hash,
        "websocket connection attempt"
    );

    let user_id = match crate::auth::verify_token(&state.sessions, &token).await {
        Some(id) => id,
        None => {
            tracing::warn!(
                connection_id = %connection_id,
                "websocket connection failed: invalid token"
            );
            let (mut sender, _): (futures_util::stream::SplitSink<WebSocket, Message>, _) = socket.split();
            let error = ErrorMessage::InvalidToken { message: "invalid token".to_string() };
            if let Ok(json) = serde_json::to_string(&error) {
                let _ = sender.send(Message::Text(json.into())).await;
            }
            let _ = sender.close().await;
            return;
        }
    };

    let username = match db::find_user_by_id(&state.db, &user_id).await {
        Ok(Some(u)) => u.username,
        _ => {
            tracing::warn!(
                connection_id = %connection_id,
                user_id = %user_id,
                "websocket connection failed: user not found"
            );
            let (mut sender, _): (futures_util::stream::SplitSink<WebSocket, Message>, _) = socket.split();
            let error = ErrorMessage::UserNotFound { message: "user not found".to_string() };
            if let Ok(json) = serde_json::to_string(&error) {
                let _ = sender.send(Message::Text(json.into())).await;
            }
            let _ = sender.close().await;
            return;
        }
    };

    tracing::info!(
        connection_id = %connection_id,
        user_id = %user_id,
        username = %username,
        "websocket token verification successful"
    );

    let mut rooms = state.rooms.write().await;
    let room = match rooms.get_mut(&room_id) {
        Some(r) => {
            tracing::info!(
                connection_id = %connection_id,
                room_id = %room_id,
                user_id = %user_id,
                "websocket room validation successful"
            );
            r
        }
        None => {
            tracing::warn!(
                connection_id = %connection_id,
                room_id = %room_id,
                "websocket connection failed: room not found"
            );
            let (mut sender, _): (futures_util::stream::SplitSink<WebSocket, Message>, _) = socket.split();
            let error = ErrorMessage::RoomNotFound { message: "room not found".to_string() };
            if let Ok(json) = serde_json::to_string(&error) {
                let _ = sender.send(Message::Text(json.into())).await;
            }
            let _ = sender.close().await;
            return;
        }
    };

    if !room.is_participant(&user_id) {
        tracing::warn!(
            connection_id = %connection_id,
            user_id = %user_id,
            room_id = %room_id,
            "websocket connection failed: not a participant"
        );
        let (mut sender, _): (futures_util::stream::SplitSink<WebSocket, Message>, _) = socket.split();
        let error = ErrorMessage::NotParticipant { message: "not a room participant".to_string() };
        if let Ok(json) = serde_json::to_string(&error) {
            let _ = sender.send(Message::Text(json.into())).await;
        }
        let _ = sender.close().await;
        return;
    }

    let is_reconnect = room.disconnected.is_some()
        && room.disconnected.as_ref().map(|d| &d.0) == Some(&user_id);

    if is_reconnect {
        let elapsed = room.disconnected.as_ref().unwrap().1.elapsed();
        if elapsed > Duration::from_secs(30) {
            tracing::warn!(
                connection_id = %connection_id,
                user_id = %user_id,
                room_id = %room_id,
                "websocket reconnect timeout"
            );
            return;
        }

        if room.reconnect_in_progress.is_some() {
            tracing::warn!(
                connection_id = %connection_id,
                room_id = %room_id,
                "websocket reconnection already in progress"
            );
            return;
        }

        room.reconnect_in_progress = Some(user_id.clone());

        tracing::info!(
            connection_id = %connection_id,
            user_id = %user_id,
            room_id = %room_id,
            time_since_disconnect_ms = elapsed.as_millis(),
            "websocket player reconnected"
        );

        if let Some(handle) = room.timeout_handle.take() {
            handle.abort();
        }
        room.disconnected = None;
        room.reconnect_in_progress = None;

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

        // Send game state sync to reconnected player
        if let Some((moves, current_player)) = room.get_current_game_state() {
            let sync_msg = ServerMessage::GameStateSync {
                moves: moves.into_iter().map(|m| crate::protocol::game::MoveRecordSync {
                    row: m.row,
                    col: m.col,
                }).collect(),
                current_player,
            };
            let sync_json = serde_json::to_string(&sync_msg).unwrap();
            room.send_to_player(&user_id, &sync_json).await;
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

        forward_messages(&state, room_id, user_id_clone, &mut ws_rx).await;
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

        if room_status == RoomStatus::Playing && room.players.len() >= 2 {
            let all_connected = room.players.values().all(|p| !p.sender.is_closed());
            if all_connected {
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

        forward_messages(&state, room_id, user_id_clone, &mut ws_rx).await;
    }
}

async fn forward_messages(
    state: &AppState,
    room_id: String,
    user_id: String,
    ws_rx: &mut futures_util::stream::SplitStream<WebSocket>,
) {
    let start = Instant::now();
    while let Some(msg) = ws_rx.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let text_str: &str = &text;

                match serde_json::from_str::<GameMessage>(text_str) {
                    Ok(game_msg) => {
                        let mut rooms = state.rooms.write().await;
                        if let Some(room) = rooms.get_mut(&room_id) {
                            if let Some(opp_id) = room.get_opponent_id(&user_id) {
                                room.send_to_player(&opp_id, text_str).await;
                            }
                            if let GameMessage::Move { row, col } = game_msg {
                                room.add_move(row as u8, col as u8);
                                tracing::debug!(
                                    room_id = %room_id,
                                    user_id = %user_id,
                                    row,
                                    col,
                                    "game move"
                                );
                            }
                        }
                    }
                    Err(_) => {
                        tracing::debug!(
                            room_id = %room_id,
                            user_id = %user_id,
                            "unknown message format"
                        );
                    }
                }
            }
            Ok(Message::Close(_)) => {
                let duration = start.elapsed();
                tracing::info!(
                    room_id = %room_id,
                    user_id = %user_id,
                    duration_ms = duration.as_millis(),
                    "websocket connection closed normally"
                );
                break;
            }
            Err(e) => {
                let duration = start.elapsed();
                tracing::warn!(
                    room_id = %room_id,
                    user_id = %user_id,
                    error = %e,
                    duration_ms = duration.as_millis(),
                    "websocket connection closed with error"
                );
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
        tracing::warn!(
            room_id = %room_id,
            user_id = %user_id,
            "player disconnected from non-playing room"
        );
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
        tracing::warn!(
            room_id = %room_id,
            "second disconnect in room, cleaning up"
        );
        if let Some(handle) = room.timeout_handle.take() {
            handle.abort();
        }
        if !room.is_empty() {
            let end_msg = ServerMessage::GameEnded {
                winner: None,
                reason: "both_disconnected".to_string(),
            };
            let end_json = serde_json::to_string(&end_msg).unwrap();
            room.broadcast(&end_json).await;
        }
        room.status = RoomStatus::Ended;
        if let Err(e) = db::update_room_status(&state.db, &room_id, "ended").await {
            tracing::error!(error = %e, "failed to update room status");
        }
        rooms.remove(&room_id);
        return;
    }

    let opponent_id = room.get_opponent_id(&user_id);
    room.disconnected = Some((user_id.clone(), Instant::now(), opponent_id.clone()));

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

        let is_reconnect_handled = room.disconnected.is_none();

        if is_reconnect_handled {
            return;
        }

        let still_disconnected = room
            .disconnected
            .as_ref()
            .map(|(uid, _, _)| uid == &disconnected_user_id)
            .unwrap_or(false);

        if !still_disconnected {
            tracing::warn!(room_id = %room_id_clone, "timeout task: inconsistent state");
            rooms.remove(&room_id_clone);
            return;
        }

        let winner_id = room
            .disconnected
            .as_ref()
            .and_then(|(_, _, winner)| winner.clone())
            .or_else(|| room.get_opponent_id(&disconnected_user_id));

        if winner_id.is_none() {
            tracing::error!(room_id = %room_id_clone, "timeout: no winner determinable");
            rooms.remove(&room_id_clone);
            return;
        }

        tracing::info!(
            room_id = %room_id_clone,
            winner_id = ?winner_id,
            "timeout triggered, game ended"
        );

        let end_msg = ServerMessage::GameEnded {
            winner: winner_id.clone(),
            reason: "opponent_disconnected".to_string(),
        };
        let end_json = serde_json::to_string(&end_msg).unwrap();
        room.broadcast(&end_json).await;

        room.status = RoomStatus::Ended;
        if let Err(e) = db::update_room_status(&state_clone.db, &room_id_clone, "ended").await {
            tracing::error!(error = %e, "failed to update room status");
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
