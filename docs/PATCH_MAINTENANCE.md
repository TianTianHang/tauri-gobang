# Patch Maintenance Guide

This document explains how to maintain and apply patches for the rapfi Android build.

## Overview

The rapfi source code requires two patches to build PIE (Position-Independent Executable) binaries for Android:

1. **pthread linking fix** - `CMakeLists.txt` line 549
2. **S_IWRITE macro fix** - `zip.c` lines 16-17

These patches are stored in `patches/rapfi-android/` and automatically applied during build.

## Quick Start

### Building Android Binaries

```bash
# 1. Clone rapfi source (if not already done)
git submodule update --init --recursive third-party/rapfi

# 2. Clone Networks submodule
cd third-party/rapfi/Rapfi/external/
git clone https://github.com/dhbloo/rapfi-networks.git Networks

# 3. Build (patches are applied automatically)
cd ../../..
bash scripts/build-android-rapfi.sh
```

### Manual Patch Management

```bash
# Apply patches
bash scripts/apply-rapfi-patches.sh

# Revert patches
bash scripts/revert-rapfi-patches.sh

# Check patch status
cd third-party/rapfi
git status Rapfi/CMakeLists.txt Rapfi/external/zip/src/zip.c
```

## Patch Files

### 0001-fix-pthread-linking.patch

**Purpose**: Fix pthread linking error on Android

**File**: `third-party/rapfi/Rapfi/CMakeLists.txt`

**Change**:
```diff
-    elseif(NOT EMSCRIPTEN)
+    elseif(NOT EMSCRIPTEN AND NOT ANDROID)
```

**Reason**: Android doesn't support explicit `-lpthread` linking

**Error without patch**:
```
cannot find -lpthread
```

### 0002-fix-siwrite-undefined.patch

**Purpose**: Fix S_IWRITE undefined error on Android

**File**: `third-party/rapfi/Rapfi/external/zip/src/zip.c`

**Change**:
```c
#ifndef S_IWRITE
#define S_IWRITE S_IWUSR
#endif
```

**Reason**: Android NDK only defines `S_IWUSR`, not `S_IWRITE`

**Error without patch**:
```
error: use of undeclared identifier 'S_IWRITE'
```

## Build Script Integration

The build script `scripts/build-android-rapfi.sh` automatically:
1. Applies patches via `apply-rapfi-patches.sh`
2. Builds for ARM64 and x86_64 with `c++_shared` STL
3. Copies binaries to `src-tauri/binaries/`

## Recreating Patches

If you need to update patches (e.g., after upstream changes):

```bash
cd third-party/rapfi

# Make your changes
# ... edit files ...

# Generate patches
git diff Rapfi/CMakeLists.txt > ../patches/rapfi-android/0001-fix-pthread-linking.patch
git diff Rapfi/external/zip/src/zip.c > ../patches/rapfi-android/0002-fix-siwrite-undefined.patch
```

## Verification

After building, verify PIE binaries:

```bash
# Check file type
file src-tauri/binaries/rapfi-*-linux-android

# Expected: ELF 64-bit LSB pie executable

# Check ELF header
readelf -h src-tauri/binaries/rapfi-aarch64-linux-android | grep Type

# Expected: Type: DYN (Position-Independent Executable file)

# Check dynamic dependencies
readelf -d src-tauri/binaries/rapfi-aarch64-linux-android | grep NEEDED

# Expected: libm.so, libc++_shared.so, libdl.so, libc.so
```

## Troubleshooting

### Build fails with "cannot find -lpthread"

**Solution**: Ensure patch 0001 is applied
```bash
bash scripts/apply-rapfi-patches.sh
```

### Build fails with "S_IWRITE undeclared"

**Solution**: Ensure patch 0002 is applied
```bash
bash scripts/apply-rapfi-patches.sh
```

### Binary is not PIE type

**Solution**: Ensure build script uses `-DANDROID_STL=c++_shared`

Check `scripts/build-android-rapfi.sh` lines 33 and 51:
```bash
-DANDROID_STL=c++_shared  # Must NOT be c++_static
```

### Patches already applied

The build script checks if patches are already applied and skips re-applying.

To force re-apply:
```bash
cd third-party/rapfi
git restore Rapfi/CMakeLists.txt Rapfi/external/zip/src/zip.c
cd ../..
bash scripts/apply-rapfi-patches.sh
```

## References

- Original issue: x86_64 binary was non-PIE (1.9MB static EXEC)
- Solution: Rebuild as PIE (24MB dynamic DYN) with `c++_shared`
- Documentation: `docs/ANDROID_RAPFI_BUILD_NOTES.md`
- Patch details: `patches/rapfi-android/README.md`
