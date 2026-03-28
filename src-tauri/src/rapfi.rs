use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::{Duration, Instant};

use crate::game::GameState;
use tauri::path::BaseDirectory;
use tauri::Manager;

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
    pub fn new<R: tauri::Runtime>(
        app: &tauri::AppHandle<R>,
        timeout_ms: u64,
    ) -> Result<Self, String> {
        debugln!("🚀 [AI] Starting Rapfi engine");

        // 使用 PathResolver 查找 sidecar 二进制
        let engine_binary = get_engine_path(app)?;
        let binaries_dir = get_binaries_dir(app)?;

        debugln!("📁 [AI] Engine binary: {}", engine_binary.display());
        debugln!("📁 [AI] Working directory: {}", binaries_dir.display());

        // 启动进程
        let mut process = Command::new(&engine_binary)
            .arg("--mode")
            .arg("gomocup")
            .current_dir(&binaries_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| {
                debugln!("❌ [AI] Failed to spawn engine: {}", e);
                format!("Failed to start engine: {}", e)
            })?;

        debugln!("✅ [AI] Engine spawned successfully");

        // 分别提取 stdin 和 stdout 避免部分移动
        let stdin = process.stdin.take().ok_or("Failed to get stdin")?;
        let stdout = process.stdout.take().ok_or("Failed to get stdout")?;

        let mut engine = Self {
            stdin,
            stdout: BufReader::new(stdout),
            process,
            timeout: Duration::from_millis(timeout_ms),
        };

        // 发送 START 命令
        engine.send_command(&format!("START {}", BOARD_SIZE))?;
        let response = engine.read_line()?;
        debugln!("📥 [AI] START response: {}", response);

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

        // 设置超时
        let timeout_ms = self.timeout.as_millis();
        self.send_command(&format!("INFO timeout_turn {}", timeout_ms))?;
        debugln!("⏱️  [AI] Set timeout: {}ms", timeout_ms);

        // 检查是否先手
        if state.history.is_empty() {
            debugln!("📤 [AI] Sending: BEGIN");
            self.send_command("BEGIN")?;
        } else {
            // 重构棋盘状态
            debugln!("📤 [AI] Sending: BOARD");
            self.send_command("BOARD")?;

            // 按时间顺序发送所有移动
            for (i, move_record) in state.history.iter().enumerate() {
                let player = if i % 2 == 0 { "1" } else { "2" };
                let move_str = format!("{},{},{}", move_record.col, move_record.row, player);
                self.send_command(&move_str)?;
                debugln!("📤 [AI]   Move {}: {}", i + 1, move_str);
            }

            debugln!("📤 [AI] Sending: DONE");
            self.send_command("DONE")?;
        }

        // 读取响应
        loop {
            let line = self.read_line()?;

            // 解析坐标响应
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

            // 跳过调试/信息消息
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
        let _ = self.process.kill();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

fn get_engine_path<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Result<PathBuf, String> {
    #[cfg(target_os = "android")]
    {
        match crate::android_rapfi::extract_rapfi_binary(app) {
            Ok(path) => return Ok(path),
            Err(e) => {
                eprintln!("⚠️ [Android] Extraction failed, falling back: {}", e);
            }
        }
    }

    use std::env;

    // 1. 尝试从资源目录查找（打包后）
    if let Ok(path) = app.path().resolve("rapfi", BaseDirectory::Resource) {
        if path.exists() {
            debugln!("📁 [AI] Found rapfi in resources: {}", path.display());
            return Ok(path);
        }
    }

    // 2. 尝试从当前可执行文件目录查找（开发环境）
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let rapfi_path = exe_dir.join("rapfi");
            if rapfi_path.exists() {
                debugln!("📁 [AI] Found rapfi next to exe: {}", rapfi_path.display());
                return Ok(rapfi_path);
            }

            // Windows下尝试 .exe 扩展名
            #[cfg(target_os = "windows")]
            {
                let rapfi_exe = exe_dir.join("rapfi.exe");
                if rapfi_exe.exists() {
                    debugln!(
                        "📁 [AI] Found rapfi.exe next to exe: {}",
                        rapfi_exe.display()
                    );
                    return Ok(rapfi_exe);
                }
            }
        }
    }

    // 3. 开发环境 fallback
    let target = env::var("TARGET").unwrap_or_else(|_| {
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

    let dev_path = PathBuf::from("./src-tauri/binaries").join(format!("rapfi-{}", target));

    if dev_path.exists() {
        debugln!("📁 [AI] Using dev rapfi: {}", dev_path.display());
        return Ok(dev_path);
    }

    Err(format!(
        "Rapfi engine not found. Please ensure it's built or in src-tauri/binaries/"
    ))
}

fn get_binaries_dir<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Result<PathBuf, String> {
    // 1. 尝试从资源目录解析
    if let Ok(path) = app.path().resolve("binaries", BaseDirectory::Resource) {
        if path.exists() {
            debugln!(
                "📁 [AI] Found binaries dir in resources: {}",
                path.display()
            );
            return Ok(path);
        }
    }

    // 2. 尝试从当前可执行文件目录
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let binaries_dir = exe_dir.join("binaries");
            if binaries_dir.exists() {
                debugln!(
                    "📁 [AI] Found binaries dir next to exe: {}",
                    binaries_dir.display()
                );
                return Ok(binaries_dir);
            }
        }
    }

    // 3. 开发环境 fallback
    let dev_path = PathBuf::from("./src-tauri/binaries");
    if dev_path.exists() {
        debugln!("📁 [AI] Using dev binaries dir: {}", dev_path.display());
        return Ok(dev_path);
    }

    Err("binaries directory not found".to_string())
}

pub fn get_rapfi_move<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    state: &GameState,
    difficulty: Difficulty,
) -> Result<(usize, usize), String> {
    let timeout = match difficulty {
        Difficulty::Easy => 500,
        Difficulty::Medium => 1500,
        Difficulty::Hard => 3000,
    };

    debugln!(
        "🎮 [AI] Starting AI move calculation, difficulty: {:?}",
        difficulty
    );

    let mut engine = RapfiEngine::new(app, timeout)?;
    engine.get_move_from_state(state)
}
