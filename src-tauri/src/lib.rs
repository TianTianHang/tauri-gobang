#[cfg(target_os = "android")]
mod android_rapfi;
mod game;
mod network;
mod rapfi;

use game::GameState;
use network::NetworkState;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::Emitter;

macro_rules! debugln {
    ($($arg:tt)*) => {
        if std::env::var("TAURI_GOBANG_DEBUG").is_ok() {
            eprintln!($($arg)*);
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MoveResult {
    state: GameState,
    ai_thinking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AiMoveResult {
    row: usize,
    col: usize,
    state: GameState,
}

#[tauri::command]
fn new_game() -> GameState {
    GameState::new()
}

#[tauri::command]
fn make_move(state: GameState, row: usize, col: usize) -> Result<MoveResult, String> {
    let mut s = state;
    s.make_move(row, col)?;

    let ai_thinking = s.status == game::GameStatus::Playing;
    Ok(MoveResult {
        state: s,
        ai_thinking,
    })
}

#[tauri::command]
fn ai_move_start(
    state: GameState,
    difficulty: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let diff = match difficulty.as_str() {
        "easy" => rapfi::Difficulty::Easy,
        "medium" => rapfi::Difficulty::Medium,
        "hard" => rapfi::Difficulty::Hard,
        _ => return Err(format!("Unknown difficulty: {}", difficulty)),
    };

    let app_clone = app.clone();

    std::thread::spawn(move || {
        debugln!("🧠 [AI] Background thread started, difficulty: {:?}", diff);

        let result = rapfi::get_rapfi_move(&app_clone, &state, diff);

        match result {
            Ok((row, col)) => {
                debugln!("✅ [AI] AI calculated move: ({}, {})", row, col);
                let mut new_state = state;
                match new_state.make_move(row, col) {
                    Ok(_) => {
                        let move_result = AiMoveResult {
                            row,
                            col,
                            state: new_state,
                        };
                        debugln!("📤 [AI] Emitting ai:move_completed event");
                        let _ = app_clone.emit("ai:move_completed", move_result);
                    }
                    Err(e) => {
                        debugln!("❌ [AI] Move failed: {}", e);
                        let _ = app_clone.emit("ai:move_error", format!("Move failed: {}", e));
                    }
                }
            }
            Err(e) => {
                debugln!("❌ [AI] AI calculation failed: {}", e);
                let _ = app_clone.emit("ai:move_error", format!("AI error: {}", e));
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn undo_move(state: GameState) -> Result<MoveResult, String> {
    let mut s = state;
    s.undo_move()?;
    Ok(MoveResult {
        state: s,
        ai_thinking: false,
    })
}

#[tauri::command]
fn undo_two_moves(state: GameState) -> Result<MoveResult, String> {
    let mut s = state;
    if s.history.len() >= 2 {
        s.undo_move()?;
        s.undo_move()?;
    } else {
        s.undo_move()?;
    }
    Ok(MoveResult {
        state: s,
        ai_thinking: false,
    })
}

#[tauri::command]
fn network_host(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<Mutex<NetworkState>>>,
    port: u16,
) -> Result<String, String> {
    network::host_game(app, &state.inner(), port)
}

#[tauri::command]
fn network_join(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<Mutex<NetworkState>>>,
    ip: String,
    port: u16,
) -> Result<(), String> {
    network::join_game(app, &state.inner(), &ip, port)
}

#[tauri::command]
fn network_send_move(
    state: tauri::State<'_, Arc<Mutex<NetworkState>>>,
    row: usize,
    col: usize,
) -> Result<(), String> {
    network::send_message(&state.inner(), &network::NetworkMessage::Move { row, col })
}

#[tauri::command]
fn network_send_undo_request(
    state: tauri::State<'_, Arc<Mutex<NetworkState>>>,
) -> Result<(), String> {
    network::send_message(&state.inner(), &network::NetworkMessage::UndoRequest)
}

#[tauri::command]
fn network_send_undo_accept(
    state: tauri::State<'_, Arc<Mutex<NetworkState>>>,
) -> Result<(), String> {
    network::send_message(&state.inner(), &network::NetworkMessage::UndoAccept)
}

#[tauri::command]
fn network_send_undo_reject(
    state: tauri::State<'_, Arc<Mutex<NetworkState>>>,
) -> Result<(), String> {
    network::send_message(&state.inner(), &network::NetworkMessage::UndoReject)
}

#[tauri::command]
fn network_send_restart_request(
    state: tauri::State<'_, Arc<Mutex<NetworkState>>>,
) -> Result<(), String> {
    network::send_message(&state.inner(), &network::NetworkMessage::RestartRequest)
}

#[tauri::command]
fn network_send_restart_accept(
    state: tauri::State<'_, Arc<Mutex<NetworkState>>>,
) -> Result<(), String> {
    network::send_message(&state.inner(), &network::NetworkMessage::RestartAccept)
}

#[tauri::command]
fn network_disconnect(state: tauri::State<'_, Arc<Mutex<NetworkState>>>) -> Result<(), String> {
    network::disconnect(&state.inner())
}

#[tauri::command]
fn network_is_connected(state: tauri::State<'_, Arc<Mutex<NetworkState>>>) -> bool {
    network::is_connected(&state.inner())
}

#[tauri::command]
fn get_local_ip() -> Result<String, String> {
    network::get_local_ip()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(Arc::new(Mutex::new(NetworkState::new())))
        .invoke_handler(tauri::generate_handler![
            new_game,
            make_move,
            ai_move_start,
            undo_move,
            undo_two_moves,
            network_host,
            network_join,
            network_send_move,
            network_send_undo_request,
            network_send_undo_accept,
            network_send_undo_reject,
            network_send_restart_request,
            network_send_restart_accept,
            network_disconnect,
            network_is_connected,
            get_local_ip,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
