# Example Configuration for Rapfi Engine

## Linux/macOS
```bash
# Download Rapfi engine
wget https://github.com/dhbloo/rapfi/releases/download/250615/rapfi-cli-250615-linux-x64.tar.gz
tar -xzf rapfi-cli-250615-linux-x64.tar.gz
chmod +x rapfi

# Download NNUE networks (optional but recommended for better performance)
wget https://github.com/dhbloo/rapfi-networks/releases/latest/download/networks.zip
unzip networks.zip

# Place in the engines directory
mkdir -p ~/.local/share/tauri-gobang/engines
mv rapfi ~/.local/share/tauri-gobang/engines/
mv *.nnue ~/.local/share/tauri-gobang/engines/
```

## Windows
```powershell
# Download from https://github.com/dhbloo/rapfi/releases
# Extract the ZIP file
# Place rapfi.exe in: C:\Users\YourName\AppData\Local\tauri-gobang\engines
# Also place .nnue files in the same directory
```

## Usage in Application

```typescript
// Using Rapfi engine
const result = await invoke<AiMoveResult>('ai_move_rapfi', {
  state: currentGameState,
  difficulty: 'hard',
  enginePath: '/home/user/.local/share/tauri-gobang/engines/rapfi'
  // or on Windows: 'C:\\Users\\YourName\\AppData\\Local\\tauri-gobang\\engines\\rapfi.exe'
});
```

## Configuration Options

### Difficulty Levels
- `easy`: 500ms thinking time
- `medium`: 1500ms thinking time
- `hard`: 3000ms thinking time (may use NNUE)

### Engine Paths
You can store the engine anywhere. Common locations:
- Linux: `~/.local/share/tauri-gobang/engines/rapfi`
- macOS: `~/Library/Application Support/tauri-gobang/engines/rapfi`
- Windows: `%LOCALAPPDATA%\tauri-gobang\engines\rapfi.exe`

### Alternative Strong Engines

If you want to try other strong Gomoku engines that support Piskvork protocol:

1. **Carbon-Gomoku** (older but strong)
   - Download: https://github.com/gomoku/Carbon-Gomoku
   - Compile and place the executable

2. **Other Gomocup engines**
   - Visit: http://gomocup.org/
   - Download engines from the tournament
   - Ensure they support Piskvork protocol

Note: The current implementation is optimized for Rapfi. Other engines may need protocol adjustments.
