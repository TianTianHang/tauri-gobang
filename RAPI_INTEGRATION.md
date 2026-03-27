# Rapfi AI Integration

This project now supports using the **Rapfi** Gomoku engine, one of the strongest open-source Gomoku AIs available.

## What is Rapfi?

Rapfi is a powerful Gomoku/Renju playing engine that:
- Uses advanced alpha-beta search with NNUE (neural network) evaluation
- Achieves high ELO ratings in tournaments (2000+)
- Supports standard Piskvork protocol for easy integration
- Is actively maintained (latest release: June 2025)

## Setup Instructions

### 1. Download Rapfi Engine

Visit the [Rapfi releases page](https://github.com/dhbloo/rapfi/releases) and download the appropriate package for your platform:

- **Linux**: `rapfi-cli-250615-linux-x64.tar.gz` (or newer)
- **Windows**: `rapfi-cli-250615-win-x64.zip`
- **macOS**: Check the releases for available builds

Extract the archive and place the executable in a known location.

### 2. Configure the Engine Path

You need to specify the path to the Rapfi executable when calling the AI.

**Example paths:**
- Linux: `/path/to/rapfi` or `./rapfi`
- Windows: `C:\\path\\to\\rapfi.exe`
- macOS: `/path/to/rapfi`

### 3. Using Rapfi in Your Application

The application now has two AI move functions:

#### Built-in AI (Rust implementation):
```typescript
await invoke<AiMoveResult>('ai_move', {
  state: gameState,
  difficulty: 'hard'  // 'easy' | 'medium' | 'hard'
});
```

#### Rapfi AI (external engine):
```typescript
await invoke<AiMoveResult>('ai_move_rapfi', {
  state: gameState,
  difficulty: 'hard',  // 'easy' | 'medium' | 'hard'
  enginePath: '/path/to/rapfi'  // or 'C:\\path\\to\\rapfi.exe'
});
```

## Difficulty Levels

Both engines support three difficulty levels:

- **Easy**: Fast computation (500ms for Rapfi)
- **Medium**: Balanced speed (1500ms for Rapfi)
- **Hard**: Deep search (3000ms for Rapfi, may use NNUE evaluation)

## Performance Comparison

| Feature | Built-in AI | Rapfi |
|---------|-------------|-------|
| Algorithm | Negamax + Alpha-Beta | Alpha-Beta + NNUE |
| Strength | Medium | Strong (2000+ ELO) |
| Speed | Fast | Medium (configurable) |
| Dependencies | None (pure Rust) | External binary |
| Memory | Low | Medium (NNUE models) |

## Piskvork Protocol

Rapfi uses the Piskvork protocol (also known as Gomocup protocol) for communication. The integration handles:

- `START [size]` - Initialize the board
- `TURN [x],[y]` - Opponent's move
- `BEGIN` - AI starts first
- `BOARD` - Send current board state
- `INFO timeout_turn [ms]` - Set time limit
- `END` - Terminate engine

## Troubleshooting

### Engine Not Found
If you get "Failed to start engine" error:
- Check that the engine path is correct
- Ensure the executable has proper permissions (Linux/Mac: `chmod +x rapfi`)
- Verify the file is not corrupted (try running it manually)

### Timeout Errors
If the engine times out:
- Increase timeout in the difficulty mapping
- Check if Rapfi is waiting for NNUE model files
- Ensure sufficient system resources

### Protocol Errors
If you see "Invalid response format":
- Verify you're using the correct Rapfi executable
- Check that no other process is using the engine
- Try restarting the application

## Advanced Configuration

### NNUE Models
Rapfi requires neural network evaluation models for optimal performance. Download the latest networks from:
https://github.com/dhbloo/rapfi-networks

Place the `.nnue` files in the same directory as the Rapfi executable.

### Build from Source
If you want to build Rapfi from source:
```bash
git clone https://github.com/dhbloo/rapfi.git
cd rapfi/Rapfi
cmake --preset x64-clang-Native
cmake --build build/x64-clang-Native
```

See the [Rapfi README](https://github.com/dhbloo/rapfi) for detailed build instructions.

## References

- [Rapfi GitHub](https://github.com/dhbloo/rapfi)
- [Rapfi Networks](https://github.com/dhbloo/rapfi-networks)
- [Gomocup Tournament](http://gomocup.org/)
- [Piskvork Protocol](https://plastovicka.github.io/protocl2en.htm)

## License

Rapfi is licensed under GPL-3.0. This integration wrapper follows the same license as the parent project.
