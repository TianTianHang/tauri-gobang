# Capability: Server Deployment

## ADDED Requirements

### Requirement: Single binary distribution
The system SHALL be distributed as a single executable binary file.

#### Scenario: Build single binary
- **WHEN** the server is compiled with `cargo build --release`
- **THEN** a single executable file `gobang-server` is produced
- **AND** the binary contains all server logic
- **AND** no external files are required to run the server

#### Scenario: Binary runs standalone
- **WHEN** a user executes the binary
- **THEN** the server starts without requiring any additional setup
- **AND** all dependencies are statically linked or embedded

### Requirement: Automatic data directory creation
The system SHALL automatically create the data directory on first run.

#### Scenario: Create data directory on Linux/macOS
- **WHEN** the server runs for the first time on Linux or macOS
- **THEN** the directory `~/.gobang-server/` is created
- **AND** a message is displayed: "✓ 数据目录已创建: ~/.gobang-server/"

#### Scenario: Create data directory on Windows
- **WHEN** the server runs for the first time on Windows
- **THEN** the directory `%APPDATA%/gobang-server/` is created
- **AND** a message is displayed indicating the directory location

#### Scenario: Use existing data directory
- **WHEN** the server runs and the data directory already exists
- **THEN** the existing directory is used
- **AND** no creation message is displayed

### Requirement: Automatic database initialization
The system SHALL automatically initialize the SQLite database on first run.

#### Scenario: Create database file
- **WHEN** the server runs and the database file does not exist
- **THEN** the server creates `database.db` in the data directory
- **AND** executes the database migration SQL script
- **AND** displays: "✓ 数据库已创建: {path}"

#### Scenario: Run database migrations
- **WHEN** the database is initialized
- **THEN** all tables (users, rooms, games) are created
- **AND** all indexes are created
- **AND** the migration is logged

#### Scenario: Use existing database
- **WHEN** the server runs and the database file exists
- **THEN** the existing database is used
- **AND** no migration is run
- **AND** the server connects to the existing database

### Requirement: Default configuration
The system SHALL provide sensible default configuration without requiring a config file.

#### Scenario: Start with default configuration
- **WHEN** the server starts without a configuration file
- **THEN** the server uses default values:
  - `server_host`: "0.0.0.0"
  - `server_port`: 3001
  - `database_path`: "{data_dir}/database.db"
  - `log_level`: "info"
  - `log_path`: "{data_dir}/server.log"
  - `reconnect_timeout_seconds`: 30
  - `password_min_length`: 6

#### Scenario: Display startup information
- **WHEN** the server starts
- **THEN** the following is displayed:
  - "✓ 服务器启动在: ws://{host}:{port}"
  - "✓ 按 Ctrl+C 停止服务器"

### Requirement: Optional configuration file
The system SHALL support an optional TOML configuration file for custom settings.

#### Scenario: Generate default config file
- **WHEN** the server runs for the first time
- **THEN** a default `config.toml` is created in the data directory
- **AND** the file contains all default configuration values with comments

#### Scenario: Load custom configuration
- **WHEN** a custom `config.toml` exists in the data directory
- **THEN** the server loads the configuration from the file
- **AND** overrides the default values with the custom values

#### Scenario: Invalid configuration handling
- **WHEN** the configuration file contains invalid values
- **THEN** the server displays an error message describing the issue
- **AND** falls back to default values for the invalid entries
- **OR** exits with an error for critical configuration errors

### Requirement: Daemon mode (Linux/macOS)
The system SHALL support running as a background daemon process.

#### Scenario: Start in daemon mode
- **WHEN** the server is started with `--daemon` flag
- **THEN** the server forks to the background
- **AND** detaches from the terminal
- **AND** displays:
  - "✓ 服务器已在后台运行"
  - "✓ PID: {process_id}"
  - "✓ 日志: {log_path}"

#### Scenario: Daemon logging
- **WHEN** the server runs in daemon mode
- **THEN** all stdout and stderr are redirected to the log file
- **AND** the log file is created if it doesn't exist
- **AND** log entries include timestamps and log levels

### Requirement: Systemd service integration (Linux)
The system SHALL support integration with systemd for service management.

#### Scenario: Install systemd service file
- **WHEN** an administrator runs the install script
- **THEN** a systemd service file is created at `~/.config/systemd/user/gobang-server.service`
- **AND** the service file contains:
  - Description of the service
  - ExecStart pointing to the binary
  - Restart=always for auto-restart
  - After=network.target

#### Scenario: Start service via systemd
- **WHEN** an administrator runs `systemctl --user start gobang-server`
- **THEN** the server starts as a user service
- **AND** runs in the background
- **AND** can be managed with systemctl commands

#### Scenario: Enable service on boot
- **WHEN** an administrator runs `systemctl --user enable gobang-server`
- **THEN** the server is configured to start automatically on login
- **AND** the service persists across reboots (for linger-enabled users)

### Requirement: Cross-platform builds
The system SHALL support building for multiple platforms.

#### Scenario: Build for Linux x86_64
- **WHEN** the build script is run with target `x86_64-unknown-linux-gnu`
- **THEN** a Linux binary is produced that runs on standard Linux distributions

#### Scenario: Build for Windows x86_64
- **WHEN** the build script is run with target `x86_64-pc-windows-gnu`
- **THEN** a Windows `.exe` binary is produced

#### Scenario: Build for macOS x86_64
- **WHEN** the build script is run with target `x86_64-apple-darwin`
- **THEN** a macOS binary is produced for Intel Macs

#### Scenario: Build for macOS ARM64
- **WHEN** the build script is run with target `aarch64-apple-darwin`
- **THEN** a macOS binary is produced for Apple Silicon Macs

### Requirement: Graceful shutdown
The system SHALL handle shutdown signals gracefully.

#### Scenario: Shutdown on Ctrl+C
- **WHEN** the server receives SIGINT (Ctrl+C)
- **THEN** the server stops accepting new connections
- **AND** waits for active WebSocket connections to close (with timeout)
- **AND** closes the database connection
- **AND** displays "✓ 服务器已停止"

#### Scenario: Shutdown on SIGTERM
- **WHEN** the server receives SIGTERM (systemd stop)
- **THEN** the server performs graceful shutdown
- **AND** closes all resources properly

### Requirement: Port binding
The system SHALL bind to the configured port and handle binding errors.

#### Scenario: Successful port binding
- **WHEN** the server starts
- **THEN** it binds to the configured host and port
- **AND** displays the WebSocket URL

#### Scenario: Port already in use
- **WHEN** the configured port is already occupied
- **THEN** the server displays an error: "端口 {port} 已被占用"
- **AND** the server exits with a non-zero status code

### Requirement: Log management
The system SHALL manage log files appropriately.

#### Scenario: Create log file on startup
- **WHEN** the server starts
- **THEN** the log file is created if it doesn't exist
- **AND** new log entries are appended to the file

#### Scenario: Log rotation (optional)
- **WHEN** the log file exceeds a configured size limit
- **THEN** the server may rotate the log file
- **AND** creates a new log file with a timestamp
- **OR** delegates log rotation to external tools (logrotate)

### Requirement: Environment variable support
The system SHALL support overriding configuration via environment variables.

#### Scenario: Override port via environment variable
- **WHEN** the environment variable `GOBANG_SERVER_PORT` is set
- **THEN** the server uses the port from the environment variable
- **AND** this overrides both default and config file values

#### Scenario: Override database path via environment variable
- **WHEN** the environment variable `GOBANG_DATABASE_PATH` is set
- **THEN** the server uses the database path from the environment variable
- **AND** this overrides both default and config file values

### Requirement: Version information
The system SHALL provide version and build information.

#### Scenario: Display version
- **WHEN** the server is started with `--version` flag
- **THEN** the server displays:
  - Version number (e.g., "gobang-server 0.1.0")
  - Build information (commit hash, build date if available)
- **AND** exits immediately
