# Patches Directory

This directory contains patches applied to third-party dependencies to enable Android PIE builds.

## Structure

```
patches/
└── rapfi-android/
    ├── 0001-fix-pthread-linking.patch  # Fix pthread linking on Android
    ├── 0002-fix-siwrite-undefined.patch # Fix S_IWRITE macro on Android
    └── README.md                       # Detailed patch documentation
```

## Applying Patches

The build script `scripts/build-android-rapfi.sh` automatically applies these patches before building.

To apply manually:

```bash
cd third-party/rapfi
git am ../../patches/rapfi-android/0001-fix-pthread-linking.patch
git am ../../patches/rapfi-android/0002-fix-siwrite-undefined.patch
```

Or use the helper script:

```bash
bash scripts/apply-rapfi-patches.sh
```

## Reverting Patches

To revert patches:

```bash
cd third-party/rapfi
git restore Rapfi/CMakeLists.txt Rapfi/external/zip/src/zip.c
```

Or use the helper script:

```bash
bash scripts/revert-rapfi-patches.sh
```

## Why These Patches Are Needed

### Android PIE Requirement

Android 5.0+ (API 21+) requires all native executables to be PIE (Position-Independent Executable).
The original rapfi build configuration generated non-PIE static binaries that Android refuses to execute.

### pthread Linking Issue

Android NDK doesn't support explicit `-lpthread` linking because pthread functionality is built into libc.
Trying to link pthread on Android causes: "cannot find -lpthread" error.

### S_IWRITE Macro Issue

Android NDK only defines `S_IWUSR` but not the `S_IWRITE` alias used by portable code.
This causes: "use of undeclared identifier 'S_IWRITE'" error in zip.c.

## Verification

After building, verify PIE binaries with:

```bash
file src-tauri/binaries/rapfi-*-linux-android | grep pie
readelf -h src-tauri/binaries/rapfi-*-linux-android | grep "Type: DYN"
```

Expected output:
- `ELF 64-bit LSB pie executable`
- `Type: DYN (Position-Independent Executable file)`
