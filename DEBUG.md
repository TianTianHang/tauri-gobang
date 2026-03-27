# Debug Mode

To enable debug output, set the `TAURI_GOBANG_DEBUG` environment variable:

```bash
# Enable debug output
TAURI_GOBANG_DEBUG=1 pnpm tauri dev

# Or export it first
export TAURI_GOBANG_DEBUG=1
pnpm tauri dev

# Without debug output (default)
pnpm tauri dev
```

Debug output includes:
- AI engine initialization and path resolution
- Piskvork protocol commands (BOARD, TURN, BEGIN, etc.)
- Move coordinates and board state
- Engine response messages
- Thread lifecycle events
