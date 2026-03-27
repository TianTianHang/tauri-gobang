# Bundled AI Engine Integration

This project now supports bundling the **Rapfi** Gomoku engine directly into the Tauri application.

## Quick Setup

### 1. Download Rapfi Engine

**Linux:**
```bash
cd src-tauri
./download-engine.sh
```

**Windows:**
```cmd
cd src-tauri
download-engine.bat
```

**macOS:**
```bash
cd src-tauri
# Download manually from: https://github.com/dhbloo/rapfi/releases
# Place the binary in: binaries/rapfi
chmod +x binaries/rapfi
```

### 2. Build the Application

```bash
pnpm tauri build
```

The build script will automatically:
- Check if Rapfi binary exists in `src-tauri/binaries/`
- Copy it to the build output directory
- Warn if the engine is missing

### 3. Using the Bundled Engine

Once bundled, you can use the external AI without specifying the engine path:

```typescript
// Use bundled engine (if available)
const result = await invoke<AiMoveResult>('ai_move_rapfi', {
  state: gameState,
  difficulty: 'hard',
  enginePath: null  // Will use bundled engine
});

// Or specify custom path
const result = await invoke<AiMoveResult>('ai_move_rapfi', {
  state: gameState,
  difficulty: 'hard',
  enginePath: '/path/to/custom/rapfi'
});
```

## File Structure

```
src-tauri/
├── binaries/              # External AI engines
│   ├── rapfi             # Linux/macOS binary
│   └── rapfi.exe         # Windows binary
├── build.rs              # Build script (bundles the engine)
├── download-engine.sh    # Linux/macOS download script
├── download-engine.bat   # Windows download script
└── src/
    └── rapfi.rs          # Rapfi integration code
```

## Build System Details

### build.rs

The `build.rs` script:
1. Checks for platform-specific Rapfi binary in `binaries/`
2. Copies it to Cargo's OUT_DIR during build
3. Provides warnings if the engine is missing

### tauri.conf.json

The `externalBin` configuration tells Tauri to bundle external binaries:
```json
"externalBin": [
  {
    "name": "rapfi",
    "path": "binaries/rapfi"
  }
]
```

## Platform-Specific Notes

### Linux
- Binary: `src-tauri/binaries/rapfi`
- Must be executable: `chmod +x binaries/rapfi`
- Dependencies: glibc 2.27+

### macOS
- Binary: `src-tauri/binaries/rapfi`
- Must be executable: `chmod +x binaries/rapfi`
- May need code signing for distribution

### Windows
- Binary: `src-tauri/binaries/rapfi.exe`
- No additional permissions needed
- May need to exclude from antivirus scanning

## Download Scripts

### download-engine.sh (Linux/macOS)

Automatically downloads:
- Latest Rapfi CLI for Linux x64
- Extracts and sets permissions
- Provides user-friendly output

### download-engine.bat (Windows)

Automatically downloads:
- Latest Rapfi CLI for Windows x64
- Extracts the ZIP file
- Provides user-friendly output

## Fallback Behavior

If the Rapfi engine is not bundled:
1. The application will still work with the built-in Rust AI
2. `ai_move_rapfi` command will return an error
3. Users can still specify a custom engine path at runtime

## Optional: NNUE Networks

For optimal performance, download NNUE evaluation networks:

1. Visit: https://github.com/dhbloo/rapfi-networks/releases
2. Download the latest network files
3. Place `.nnue` files in `src-tauri/binaries/`

The engine will automatically load these if available.

## Verification

To verify the engine is bundled:

```bash
# After build
ls -lh src-tauri/target/release/bundle/

# Or check the application resources
./src-tauri/target/release/tauri-gobang --help
```

## Troubleshooting

### Engine Not Found Error

**Cause:** Rapfi binary not in `src-tauri/binaries/`

**Solution:**
```bash
cd src-tauri
./download-engine.sh  # or download-engine.bat on Windows
```

### Permission Denied (Linux/macOS)

**Cause:** Binary not executable

**Solution:**
```bash
chmod +x src-tauri/binaries/rapfi
```

### Engine Fails to Start

**Cause:** Platform mismatch or missing dependencies

**Solution:**
- Ensure you downloaded the correct platform binary
- Check Rapfi's dependencies for your OS
- Try running the binary directly to test

### Build Warnings

**Warning:** `Rapfi engine not found`

**Impact:** Application still works, but external AI unavailable

**Solution:** Download the engine or ignore if using built-in AI

## Performance Comparison

| Feature | Built-in AI | Bundled Rapfi |
|---------|-------------|---------------|
| Speed | Fast | Medium |
| Strength | Medium | Strong (2000+ ELO) |
| Size | Small | +5-10 MB |
| Dependencies | None | None (self-contained) |
| Network | No | Optional NNUE |

## License Notes

- **Rapfi Engine**: GPL-3.0
- **This Integration**: Same as parent project
- Bundling GPL software in a commercial application may have implications
- Consider offering the engine as an optional download for commercial use

## Further Reading

- [Rapfi GitHub](https://github.com/dhbloo/rapfi)
- [Rapfi Networks](https://github.com/dhbloo/rapfi-networks)
- [Tauri External Binaries](https://v2.tauri.app/distribute/bundle/#external-binaries)
