# Rapfi Android Build Patches

This directory contains patches needed to build rapfi for Android with PIE (Position-Independent Executable) support.

## Patch Description

Android 5.0+ (API 21+) requires all executables to be PIE (Position-Independent Executable). The rapfi source code has two compatibility issues that prevent building PIE binaries for Android:

1. **pthread linking issue**: Android NDK doesn't support explicit `-lpthread` linking
2. **S_IWRITE undefined**: Android NDK doesn't define `S_IWRITE` macro

## Applying Patches

When building rapfi from source, apply these patches:

```bash
cd third-party/rapfi
git am ../patches/rapfi-android/0001-fix-pthread-linking.patch
git am ../patches/rapfi-android/0002-fix-siwrite-undefined.patch
```

Or manually apply with:

```bash
cd third-party/rapfi
patch -p1 < ../patches/rapfi-android/0001-fix-pthread-linking.patch
patch -p1 < ../patches/rapfi-android/0002-fix-siwrite-undefined.patch
```

## Patch Details

### 0001-fix-pthread-linking.patch

**File**: `Rapfi/CMakeLists.txt`  
**Line**: 549  
**Change**: Add `NOT ANDROID` condition to skip pthread linking on Android

```diff
-    elseif(NOT EMSCRIPTEN)
+    elseif(NOT EMSCRIPTEN AND NOT ANDROID)
```

**Reason**: Android's pthread functionality is built into libc, explicit linking fails.

### 0002-fix-siwrite-undefined.patch

**File**: `Rapfi/external/zip/src/zip.c`  
**Lines**: 16-17  
**Change**: Add S_IWRITE definition if not already defined

```c
#ifndef S_IWRITE
#define S_IWRITE S_IWUSR
#endif
```

**Reason**: Android NDK only defines `S_IWUSR`, not the `S_IWRITE` alias.

## Building PIE Binaries

After applying patches, use these CMake flags to generate PIE binaries:

```bash
-DANDROID_STL=c++_shared  # Required for PIE (not c++_static)
```

See `scripts/build-android-rapfi.sh` for complete build script.

## Verification

Verify PIE binaries with:

```bash
file rapfi-*-linux-android | grep pie
readelf -h rapfi-*-linux-android | grep "Type: DYN"
```

Expected output:
- `ELF 64-bit LSB pie executable`
- `Type: DYN (Position-Independent Executable file)`
