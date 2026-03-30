# AGENTS.md

## Project Overview

Tauri 2 + React 19 + TypeScript desktop and Android application (Gobang/Five-in-a-Row game).
Frontend in `src/`, Rust backend in `src-tauri/src/`. Communication via Tauri `invoke` IPC and Tauri events.
Android support via `android_rapfi.rs` module (gated with `#[cfg(target_os = "android")]`).

**Server Architecture**: Independent Rust server in `server/` for online multiplayer.
Uses axum + SQLite + WebSocket for room management, user authentication, and game message relay.

## Build & Run Commands

**Client (Tauri Desktop/Android):**
```bash
pnpm dev              # Start Vite dev server (port 1420)
pnpm build            # Type-check (tsc) + build frontend
pnpm preview          # Preview production build
pnpm tauri dev        # Full Tauri desktop app in dev mode
pnpm tauri build      # Build production desktop app
pnpm tauri android dev  # Full Tauri Android app in dev mode
pnpm tauri build --target android  # Build production Android APK
```

**Server (Online Multiplayer):**
```bash
cd server
cargo run                         # Start server (default: port 3001)
cargo run -- --port 8080          # Custom port
cargo run -- --daemon             # Background mode (Unix only)
cargo test                        # Run all tests (unit + integration)
cargo build --release             # Build production binary
./build.sh                        # Cross-platform build script
```

## Nix开发环境 (推荐)

```bash
nix develop                   # 进入Android开发环境
nix flake update              # 更新依赖
nix flake check               # 验证配置
```

**首次使用（带direnv）：**
```bash
# 安装direnv（如果还没有）
# Ubuntu/Debian: sudo apt-get install direnv
# Arch: sudo pacman -S direnv
# Fedora: sudo dnf install direnv

# 配置shell hook（一次性）
# Bash: echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
# Zsh: echo 'eval "$(direnv hook zsh)"' >> ~/.zshrc

# 允许.envrc
direnv allow

# 之后每次进入目录自动激活环境
```

## Android模拟器管理

```bash
# 创建虚拟设备(AVD)
bash scripts/create-android-avd.sh

# 列出所有AVD
emulator -list-avds

# 启动模拟器
emulator -avd tauri-gobang-avd

# 使用快照加速启动
emulator -avd tauri-gobang-avd -snapshot quickboot

# 无窗口模式（CI/CD）
emulator -avd tauri-gobang-avd -no-window -no-audio

# 删除AVD
avdmanager delete avd -n tauri-gobang-avd
```

**ADB调试：**
```bash
# 查看连接的设备
adb devices

# 安装APK
adb install app-debug.apk

# 查看日志
adb logcat | grep -i tauri

# 查看设备信息
adb shell getprop ro.product.model
```

详细说明: [docs/ANDROID_EMULATOR_GUIDE.md](docs/ANDROID_EMULATOR_GUIDE.md)

**Rust backend:**
```bash
cd src-tauri && cargo check              # Type-check Rust code
cd src-tauri && cargo build              # Compile Rust backend
cd src-tauri && cargo test               # Run all Rust tests
cd src-tauri && cargo test --test ai     # Run specific test module (when added)
```

## Testing

**Rust (Server & Tauri Backend):**
```bash
cd server && cargo test              # Run all server tests (unit + integration)
cd src-tauri && cargo test           # Run all Tauri backend tests
cargo test --test <test_name>        # Run specific test
```

**Frontend:**
```bash
pnpm test                            # Run frontend tests (vitest)
pnpm test:watch                      # Watch mode for development
```

Tests located in:
- **Server**: `#[cfg(test)]` modules in `server/src/*.rs` (auth.rs, room.rs, protocol/, main.rs integration tests)
- **Frontend**: `src/__tests__/*.test.ts`

## Linting & Type-Checking

No linter/formatter configured. TypeScript compiler (`tsc`) is the type-checker.
```bash
pnpm build            # Runs tsc — verify types pass before committing
cd src-tauri && cargo clippy   # Rust lint (when configured)
```

## Package Manager

Use **pnpm** exclusively. Do not use npm or yarn.

## Project Structure

```
src/
  main.tsx                  # Entry point, React.StrictMode
  App.tsx                   # Root orchestrator (state, IPC, view routing)
  App.css                   # Global styles + CSS variables + dark mode
  types/game.ts             # All shared TypeScript types/interfaces
  components/
    MainMenu.tsx + .css     # Mode selection menu
    GameBoard.tsx + .css    # Canvas-based board rendering
    GameInfo.tsx + .css     # Game status, controls, undo/restart
    NetworkSetup.tsx + .css # Host/join TCP connection form
src-tauri/src/
  main.rs                   # Thin entry point
  lib.rs                    # All Tauri commands + run(), shared state management
  game.rs                   # GameState, board logic, win checking
  ai.rs                     # Minimax AI with alpha-beta pruning
  network.rs                # TCP networking, Tauri event emission
  rapfi.rs                  # Rapfi engine wrapper, path resolution
  android_rapfi.rs          # Android binary extraction (cfg-gated)
server/
  Cargo.toml                # Server dependencies
  src/
    main.rs                 # Entry point, CLI args, HTTP routes
    auth.rs                 # Password hashing, session management
    db.rs                   # SQLite connection, queries
    room.rs                 # Room state, in-memory store
    ws.rs                   # WebSocket handler, message relay
    types.rs                # API request/response types
    protocol/
      game.rs               # WebSocket message enums
  migrations/init.sql       # Database schema
  deploy/
    gobang-server.service   # systemd service template
  build.sh                  # Cross-platform build script
  README.md                 # Server documentation
```

## TypeScript / React Style

- **Strict mode** enabled: `noUnusedLocals`, `noUnusedParameters`, `noFallthroughCasesInSwitch`
- **Imports**: Named for hooks/types, default for components, side-effect for CSS. Order: React hooks → external libs → local types → local components → CSS files
- **Quotes**: Double quotes
- **Semicolons**: Always
- **Indentation**: 2 spaces
- **Components**: Function declarations (`function App() { ... }`), never arrow functions
- **Props**: `interface ComponentProps { ... }` defined above the component
- **Exports**: `export default ComponentName;` at bottom of file
- **Event handlers**: `useCallback` for handlers passed as props; inline arrows for simple one-liners
- **State**: All state via `useState` in parent components, passed down as props (no external state library, no Context)
- **Canvas**: GameBoard uses raw HTML Canvas API with `useRef`
- **Tauri IPC**: Always type invoke return: `invoke<GameState>("new_game")`. Always wrap in `try/catch`
- **Tauri events**: Use `listen` from `@tauri-apps/api/event`, store `UnlistenFn` in `useRef` array, clean up in `useEffect`
- **Forms**: Plain HTML controlled inputs, no form library

## TypeScript Type Conventions

- **Enums**: String-valued enums for serde interop (`enum Cell { Empty = "empty", ... }`)
- **Interfaces**: For all object shapes (`interface GameState { ... }`)
- **Type aliases**: For union types (`type Difficulty = "easy" | "medium" | "hard"`)
- **Optional props**: `?` suffix (`lastMove?: { row: number; col: number } | null`)
- **Rust interop**: TypeScript enums mirror Rust enums via serde `rename_all = "snake_case"`. Some fields use snake_case to match Rust (e.g., `current_player`)

## CSS Conventions

- Co-located `.css` files per component (e.g., `GameBoard.css` next to `GameBoard.tsx`)
- CSS custom properties in `App.css` `:root` for theming (light/dark via `prefers-color-scheme`)
- Flat kebab-case class names with loose prefixes: `.game-`, `.menu-`, `.setup-`, `.btn-`
- No CSS modules, no Tailwind, no BEM

## File Naming

- Components: `PascalCase.tsx` (`GameBoard.tsx`)
- CSS files: `PascalCase.css` matching their component (`GameBoard.css`)
- Utilities/entry: `camelCase.ts` (`main.tsx`)
- Types directory: `types/` with `camelCase.ts` files
- Rust modules: `snake_case.rs` (`game.rs`, `network.rs`)

## Rust Style

- **Edition**: 2021, 4-space indent, default `rustfmt`
- **Naming**: `snake_case` for functions/variables, `PascalCase` for types
- **Serde**: `#[derive(Debug, Clone, Serialize, Deserialize)]` on all shared types; `#[serde(rename_all = "snake_case")]` on enums
- **Attributes**: `#[tauri::command]` for IPC handlers
- **Error handling**: `Result<T, String>` for commands; `.expect()` only for app startup; `.map_err(|e| e.to_string())` for error conversion
- **Shared state**: `Arc<Mutex<T>>` managed via `.manage()` in `tauri::Builder`
- **Concurrency**: `thread::spawn` for background tasks (no async/tokio)
- **Networking**: Raw TCP (`std::net`), line-delimited JSON protocol
- **Structure**: `lib.rs` for commands + `run()`, `main.rs` as thin entry

## Tauri IPC Pattern

1. Define Rust command: `#[tauri::command] fn my_command(state: State<...>) -> Result<T, String> { ... }`
2. Register in `lib.rs`: `.invoke_handler(tauri::generate_handler![my_command])`
3. Call from frontend: `import { invoke } from "@tauri-apps/api/core"; await invoke<T>("my_command", { arg })`
4. For push events (Rust→Frontend): `app.emit("event:name", payload)` in Rust; `listen("event:name", handler)` in frontend

## Common Pitfalls

- Dev server must run on port 1420 (Tauri hardcodes this for dev)
- Tauri commands must be registered in `generate_handler![]` or they will 404
- Use `@tauri-apps/api` v2 APIs (not v1)
- CSP is disabled (`null`) in `tauri.conf.json` — tighten before production
- TypeScript and Rust types must be kept in sync — serde serialization depends on matching enum variants and field names
- Network error messages are mixed Chinese/English — keep consistent with existing pattern
