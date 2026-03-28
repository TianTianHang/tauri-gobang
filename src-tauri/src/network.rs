use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

macro_rules! debugln {
    ($($arg:tt)*) => {
        #[cfg(target_os = "android")]
        eprintln!($($arg)*);
        #[cfg(not(target_os = "android"))]
        if std::env::var("TAURI_GOBANG_DEBUG").is_ok() {
            eprintln!($($arg)*);
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NetworkMessage {
    Join { color: String },
    Move { row: usize, col: usize },
    UndoRequest,
    UndoAccept,
    UndoReject,
    RestartRequest,
    RestartAccept,
    RestartReject,
    Chat { message: String },
    GameOver { winner: String },
    Disconnect,
}

pub struct NetworkState {
    pub listener: Option<TcpListener>,
    pub stream: Option<TcpStream>,
    pub is_host: bool,
    pub connected: Arc<AtomicBool>,
}

impl NetworkState {
    pub fn new() -> Self {
        NetworkState {
            listener: None,
            stream: None,
            is_host: false,
            connected: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Default for NetworkState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_local_ip() -> Result<String, String> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
    socket.connect("8.8.8.8:80").map_err(|e| e.to_string())?;
    let local_addr = socket.local_addr().map_err(|e| e.to_string())?;
    Ok(local_addr.ip().to_string())
}

pub fn host_game(
    app: AppHandle,
    state: &Arc<Mutex<NetworkState>>,
    port: u16,
) -> Result<String, String> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).map_err(|e| e.to_string())?;

    let local_ip = get_local_ip()?;
    let bind_port = listener.local_addr().map_err(|e| e.to_string())?.port();
    let display_addr = format!("{}:{}", local_ip, bind_port);

    {
        let mut ns = state.lock().map_err(|e| e.to_string())?;
        ns.listener = Some(listener);
        ns.is_host = true;
        ns.connected.store(false, Ordering::SeqCst);
    }

    let state_clone = Arc::clone(state);
    let connected = Arc::clone(&state.lock().map_err(|e| e.to_string())?.connected);

    thread::spawn(move || {
        let ns = state_clone.lock().unwrap();
        let listener = ns.listener.as_ref().unwrap().try_clone().unwrap();
        drop(ns);

        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    let connected = Arc::clone(&connected);
                    connected.store(true, Ordering::SeqCst);
                    let _ = app.emit("network:opponent_joined", addr.to_string());

                    let app_clone = app.clone();
                    let conn = Arc::clone(&connected);
                    handle_connection(stream, app_clone, conn);
                    break;
                }
                Err(e) => {
                    debugln!("Accept error: {}", e);
                    break;
                }
            }
        }
    });

    Ok(display_addr)
}

pub fn join_game(
    app: AppHandle,
    state: &Arc<Mutex<NetworkState>>,
    ip: &str,
    port: u16,
) -> Result<(), String> {
    let addr = format!("{}:{}", ip, port);
    let stream = TcpStream::connect(&addr).map_err(|e| format!("连接失败: {}", e))?;
    stream.set_nonblocking(false).map_err(|e| e.to_string())?;
    stream.set_nodelay(true).map_err(|e| e.to_string())?;

    {
        let mut ns = state.lock().map_err(|e| e.to_string())?;
        ns.stream = Some(stream.try_clone().map_err(|e| e.to_string())?);
        ns.is_host = false;
        ns.connected.store(true, Ordering::SeqCst);
    }

    let app_clone = app.clone();
    let conn = Arc::clone(&state.lock().map_err(|e| e.to_string())?.connected);
    handle_connection(stream, app_clone, conn);

    Ok(())
}

fn handle_connection(stream: TcpStream, app: AppHandle, connected: Arc<AtomicBool>) {
    let mut reader = BufReader::new(stream);

    thread::spawn(move || {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    connected.store(false, Ordering::SeqCst);
                    let _ = app.emit("network:disconnected", "对手已断开连接");
                    break;
                }
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    match serde_json::from_str::<NetworkMessage>(trimmed) {
                        Ok(msg) => {
                            let event = match msg {
                                NetworkMessage::Move { .. } => "network:opponent_moved",
                                NetworkMessage::UndoRequest => "network:undo_request",
                                NetworkMessage::UndoAccept => "network:undo_accept",
                                NetworkMessage::UndoReject => "network:undo_reject",
                                NetworkMessage::RestartRequest => "network:restart_request",
                                NetworkMessage::RestartAccept => "network:restart_accept",
                                NetworkMessage::RestartReject => "network:restart_reject",
                                NetworkMessage::Chat { .. } => "network:chat",
                                NetworkMessage::GameOver { .. } => "network:game_over",
                                NetworkMessage::Join { .. } => "network:opponent_joined",
                                NetworkMessage::Disconnect => {
                                    connected.store(false, Ordering::SeqCst);
                                    "network:disconnected"
                                }
                            };
                            let _ = app.emit(event, trimmed);
                        }
                        Err(e) => {
                            debugln!("Failed to parse message: {} - {}", trimmed, e);
                        }
                    }
                }
                Err(e) => {
                    connected.store(false, Ordering::SeqCst);
                    let _ = app.emit("network:disconnected", format!("连接错误: {}", e));
                    break;
                }
            }
        }
    });
}

pub fn send_message(state: &Arc<Mutex<NetworkState>>, msg: &NetworkMessage) -> Result<(), String> {
    let ns = state.lock().map_err(|e| e.to_string())?;

    let stream = if ns.is_host {
        let listener = ns.listener.as_ref().ok_or("Not hosting")?;
        listener
            .incoming()
            .next()
            .ok_or("No active connection")?
            .map_err(|e| e.to_string())?
    } else {
        ns.stream
            .as_ref()
            .ok_or("Not connected")?
            .try_clone()
            .map_err(|e| e.to_string())?
    };

    let mut writer = stream.try_clone().map_err(|e| e.to_string())?;
    let json = serde_json::to_string(msg).map_err(|e| e.to_string())?;
    writeln!(writer, "{}", json).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn disconnect(state: &Arc<Mutex<NetworkState>>) -> Result<(), String> {
    let mut ns = state.lock().map_err(|e| e.to_string())?;
    ns.connected.store(false, Ordering::SeqCst);

    if let Some(ref listener) = ns.listener {
        let _ = listener.set_nonblocking(true);
    }

    ns.stream = None;
    ns.listener = None;

    Ok(())
}

pub fn is_connected(state: &Arc<Mutex<NetworkState>>) -> bool {
    if let Ok(ns) = state.lock() {
        ns.connected.load(Ordering::SeqCst)
    } else {
        false
    }
}
