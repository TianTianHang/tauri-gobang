use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::{Duration, Instant};

use crate::game::GameState;

const SIDECAR_NAME: &str = "rapfi";

const BOARD_SIZE: usize = 15;

macro_rules! debugln {
    ($($arg:tt)*) => {
        if std::env::var("TAURI_GOBANG_DEBUG").is_ok() {
            eprintln!($($arg)*);
        }
    };
}

pub struct RapfiEngine {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    process: Child,
    timeout: Duration,
}

impl RapfiEngine {
    pub fn new(mut process: Child, timeout_ms: u64) -> Result<Self, String> {
        let stdin = process.stdin.take().ok_or("Failed to get stdin")?;
        let stdout = process.stdout.take().ok_or("Failed to get stdout")?;
        let stdout = BufReader::new(stdout);

        let mut engine = Self {
            stdin,
            stdout,
            process,
            timeout: Duration::from_millis(timeout_ms),
        };

        engine.send_command(&format!("START {}", BOARD_SIZE))?;
        engine.read_line()?;

        Ok(engine)
    }

    fn send_command(&mut self, cmd: &str) -> Result<(), String> {
        writeln!(self.stdin, "{}", cmd).map_err(|e| format!("Failed to send command: {}", e))?;
        self.stdin
            .flush()
            .map_err(|e| format!("Failed to flush stdin: {}", e))?;
        Ok(())
    }

    fn read_line(&mut self) -> Result<String, String> {
        let start = Instant::now();
        let mut line = String::new();

        loop {
            if start.elapsed() > self.timeout {
                return Err("Engine timeout".to_string());
            }

            line.clear();
            let bytes_read = self
                .stdout
                .read_line(&mut line)
                .map_err(|e| format!("Failed to read from engine: {}", e))?;

            if bytes_read == 0 {
                return Err("Engine closed connection".to_string());
            }

            let line = line.trim();
            if !line.is_empty() {
                return Ok(line.to_string());
            }
        }
    }

    fn get_move_from_state(&mut self, state: &GameState) -> Result<(usize, usize), String> {
        debugln!("📊 [AI] Board has {} moves", state.history.len());

        // Set timeout for this move (in milliseconds)
        let timeout_ms = self.timeout.as_millis();
        self.send_command(&format!("INFO timeout_turn {}", timeout_ms))?;
        debugln!("⏱️  [AI] Set timeout: {}ms", timeout_ms);

        // Check if this is the first move (AI plays first)
        if state.history.is_empty() {
            // AI plays first on empty board
            debugln!("📤 [AI] Sending: BEGIN");
            self.send_command("BEGIN")?;
        } else {
            // Reconstruct board state using BOARD command
            debugln!("📤 [AI] Sending: BOARD");
            self.send_command("BOARD")?;

            // Send all moves in chronological order
            // According to protocol: 1 = own stone (AI/black), 2 = opponent's stone (white)
            for (i, move_record) in state.history.iter().enumerate() {
                // Odd moves (0, 2, 4...) are black/AI stones (player 1)
                // Even moves (1, 3, 5...) are white/opponent stones (player 2)
                let player = if i % 2 == 0 { "1" } else { "2" };
                let move_str = format!("{},{},{}", move_record.col, move_record.row, player);
                self.send_command(&move_str)?;
                debugln!("📤 [AI]   Move {}: {}", i + 1, move_str);
            }

            debugln!("📤 [AI] Sending: DONE");
            self.send_command("DONE")?;
        }

        // Read response - engine will output MESSAGE lines, then final coordinates
        loop {
            let line = self.read_line()?;

            // Parse coordinate response (format: "col,row" or "x,y")
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(col), Ok(row)) = (
                    parts[0].trim().parse::<usize>(),
                    parts[1].trim().parse::<usize>(),
                ) {
                    if col < BOARD_SIZE && row < BOARD_SIZE {
                        debugln!("✅ [AI] Received move: col={}, row={}", col, row);
                        return Ok((row, col));
                    }
                }
            }

            // Skip debug/info messages (but print them for debugging)
            if line.starts_with("MESSAGE") || line.starts_with("ERROR") {
                debugln!("📥 [AI] Engine: {}", line);
                continue;
            }

            debugln!("📥 [AI] Received: {}", line);
        }
    }
}

impl Drop for RapfiEngine {
    fn drop(&mut self) {
        let _ = self.send_command("END");
        let _ = self.process.wait();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

fn get_binaries_dir() -> Result<PathBuf, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?
        .parent()
        .map(PathBuf::from)
        .unwrap_or_default();

    debugln!("🔍 [AI DEBUG] Looking for binaries directory (config.toml):");

    // Try multiple possible locations for the binaries directory
    let possible_dirs = vec![
        exe_dir.join("binaries"),
        exe_dir.join("..").join("binaries"),
        PathBuf::from("./binaries"),
        PathBuf::from("../binaries"),
        exe_dir.join("../../src-tauri/binaries"),
        exe_dir.clone(),
    ];

    for (i, dir) in possible_dirs.iter().enumerate() {
        let config_exists = dir.join("config.toml").exists();
        debugln!(
            "   [{}] Checking: {} - has config: {}",
            i,
            dir.display(),
            config_exists
        );
        if config_exists {
            debugln!("   ✅ FOUND binaries dir: {}", dir.display());
            return Ok(dir.clone());
        }
    }

    // If config not found, try to use exe_dir as fallback
    debugln!(
        "   ⚠️  Config not found, using exe_dir as fallback: {}",
        exe_dir.display()
    );
    Ok(exe_dir)
}

fn get_bundled_engine_path() -> Result<String, String> {
    // Get target triple at runtime (from Cargo build env)
    // If not available (e.g., when not built by Cargo), use compile-time cfg
    let target = std::env::var("TARGET").unwrap_or_else(|_| {
        // Fallback to compile-time detection
        if cfg!(target_os = "windows") {
            if cfg!(target_arch = "x86_64") {
                "x86_64-pc-windows-msvc".to_string()
            } else {
                "i686-pc-windows-msvc".to_string()
            }
        } else if cfg!(target_os = "macos") {
            if cfg!(target_arch = "aarch64") {
                "aarch64-apple-darwin".to_string()
            } else {
                "x86_64-apple-darwin".to_string()
            }
        } else if cfg!(target_os = "linux") {
            if cfg!(target_arch = "x86_64") {
                "x86_64-unknown-linux-gnu".to_string()
            } else {
                "aarch64-unknown-linux-gnu".to_string()
            }
        } else {
            "unknown".to_string()
        }
    });

    let platform_engine_name = format!("{}-{}", SIDECAR_NAME, target);
    let simple_engine_name = if cfg!(target_os = "windows") {
        format!("{}.exe", SIDECAR_NAME)
    } else {
        SIDECAR_NAME.to_string()
    };

    // Try to find sidecar using Tauri's sidecar resolution
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?
        .parent()
        .map(PathBuf::from)
        .unwrap_or_default();

    debugln!("🔍 [AI DEBUG] Looking for Rapfi engine:");
    debugln!("   exe_dir: {}", exe_dir.display());
    debugln!("   platform_engine_name: {}", platform_engine_name);
    debugln!("   simple_engine_name: {}", simple_engine_name);

    // Common sidecar locations (try platform-specific name first, then simple name)
    let paths: Vec<PathBuf> = vec![
        // Platform-specific name (Tauri sidecar convention)
        exe_dir.join(&platform_engine_name),
        exe_dir.join("binaries").join(&platform_engine_name),
        exe_dir
            .join("..")
            .join("binaries")
            .join(&platform_engine_name),
        PathBuf::from(format!("./binaries/{}", platform_engine_name)),
        // Simple name as fallback
        exe_dir.join(&simple_engine_name),
        exe_dir.join("binaries").join(&simple_engine_name),
        exe_dir
            .join("..")
            .join("binaries")
            .join(&simple_engine_name),
        PathBuf::from(format!("./binaries/{}", simple_engine_name)),
    ];

    for (i, path) in paths.iter().enumerate() {
        debugln!(
            "   [{}] Checking: {} - exists: {}",
            i,
            path.display(),
            path.exists()
        );
        if path.exists() {
            debugln!("   ✅ FOUND: {}", path.display());
            let canonical = path
                .canonicalize()
                .map_err(|e| format!("Failed to canonicalize path: {}", e))?;
            debugln!("   📌 Canonicalized to: {}", canonical.display());
            return Ok(canonical.to_string_lossy().to_string());
        }
    }

    Err(format!(
        "Rapfi engine not found. Please download from https://github.com/dhbloo/rapfi/releases and place in src-tauri/binaries/ directory.",
    ))
}

pub fn get_rapfi_move<R: tauri::Runtime>(
    state: &GameState,
    difficulty: Difficulty,
    _app: tauri::AppHandle<R>,
) -> Result<(usize, usize), String> {
    let timeout = match difficulty {
        Difficulty::Easy => 500,
        Difficulty::Medium => 1500,
        Difficulty::Hard => 3000,
    };

    debugln!(
        "🎮 [AI DEBUG] Starting AI move calculation, difficulty: {:?}",
        difficulty
    );

    let engine_binary = get_bundled_engine_path()?;
    let binaries_dir = get_binaries_dir()?;

    debugln!("🚀 [AI DEBUG] Launching engine:");
    debugln!("   engine_binary: {}", engine_binary);
    debugln!("   working_dir: {}", binaries_dir.display());

    let process = Command::new(&engine_binary)
        .arg("--mode")
        .arg("gomocup")
        .current_dir(&binaries_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| {
            debugln!("❌ [AI DEBUG] Failed to spawn engine: {}", e);
            format!("Failed to start engine: {}", e)
        })?;

    debugln!("✅ [AI DEBUG] Engine spawned successfully");

    let mut engine = RapfiEngine::new(process, timeout)?;
    engine.get_move_from_state(state)
}
