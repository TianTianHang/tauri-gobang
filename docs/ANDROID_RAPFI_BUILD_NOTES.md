# Android Rapfi Build Notes

## Build Summary

Successfully built PIE (Position-Independent Executable) binaries for both Android architectures:

- **aarch64-linux-android**: 24MB (ARM64 devices)
- **x86_64-linux-android**: 24MB (x86_64 emulators)

Both binaries are verified PIE type:
- ELF Type: `DYN (Position-Independent Executable file)`
- File command: `ELF 64-bit LSB pie executable`
- Dynamic linker: `/system/bin/linker64`

## Build Date

March 28, 2026

## Build Command

```bash
bash scripts/build-android-rapfi.sh
```

## Source Modifications

Three source code patches were applied to enable PIE builds on Android:

1. **CMakeLists.txt** - Skip pthread linking on Android (line 549)
2. **zip.c** - Define S_IWRITE macro for Android NDK (lines 16-17)
3. **build script** - Use `c++_shared` STL instead of `c++_static`

All patches are stored in `patches/rapfi-android/` for reproducibility.

## Verification Commands

```bash
# Check PIE type
file src-tauri/binaries/rapfi-*-linux-android | grep pie

# Check ELF header
readelf -h src-tauri/binaries/rapfi-aarch64-linux-android | grep Type

# Check dynamic dependencies
readelf -d src-tauri/binaries/rapfi-aarch64-linux-android | grep NEEDED
```

## Next Steps

These binaries will be:
1. Packaged into Tauri app bundle via `tauri.conf.json` `bundle.resources`
2. Extracted to app cache directory at runtime by `android_rapfi.rs`
3. Executed via shell wrapper to bypass SELinux restrictions
