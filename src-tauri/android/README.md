# Android Custom Gradle Configuration

This directory contains custom Android build configuration separate from Tauri-generated files.

## Overview

Tauri CLI generates Android project files in `gen/android/`. To keep custom logic separate and maintainable, we use Gradle's `apply from` mechanism to reference external configuration files.

## Structure

```
src-tauri/android/
├── app-custom.gradle.kts  # Custom build logic (referenced by gen/android/app/build.gradle.kts)
└── README.md               # This file
```

## How It Works

### Relative Path Derivation

The `src-tauri/gen/android/app/build.gradle.kts` file references this custom config:

```kotlin
apply(from = "../../../android/app-custom.gradle.kts")
```

**Path breakdown:**
- Starting point: `src-tauri/gen/android/app/build.gradle.kts`
- `../../../` goes up three levels: `src-tauri/gen/android/app/` → `src-tauri/gen/android/` → `src-tauri/gen/` → `src-tauri/`
- `android/app-custom.gradle.kts` then resolves to `src-tauri/android/app-custom.gradle.kts`

Why this works:
- `gen/` is under `src-tauri/`, so going up three levels from `gen/android/app/` reaches `src-tauri/`
- From `src-tauri/`, `android/app-custom.gradle.kts` is directly accessible

### What's Inside app-custom.gradle.kts

1. **Packaging Options** - APK packaging configuration:
   - `useLegacyPackaging = true` - Native libraries in jniLibs instead of AAB
   - `doNotStrip` - Prevents redundant stripping of rapfi binaries

2. **copyRapfiBinaries Task** - Copies rapfi AI engine binaries:
   - From: `src-tauri/binaries/rapfi-{arch}`
   - To: `gen/android/app/src/main/jniLibs/{abi}/librapfi.so`
   - Runs before compilation via `preBuild` dependency

3. **preBuild Hook** - Ensures binaries are copied before build starts

## Clone-Init-Dev Workflow

For a fresh clone of this repository:

1. **Clone the repo** - All files including `gen/android/app/build.gradle.kts` are present
2. **Initialize Android (if needed)**:
   ```bash
   pnpm tauri android init
   ```
   - This command generates `gen/android/` if missing
   - It **will not overwrite** existing `build.gradle.kts` (verified Tauri behavior)
   - The `apply(from = ...)` line remains intact
3. **Run dev build**:
   ```bash
   pnpm tauri android dev
   ```
   - Gradle processes `app/build.gradle.kts`
   - Applies our custom config from `src-tauri/android/app-custom.gradle.kts`
   - Copies rapfi binaries to jniLibs
   - Builds and installs the APK

## Why This Pattern?

| Concern | Solution |
|---------|----------|
| Tauri template changes isolated | Custom logic in `src-tauri/android/`, not mixed in `gen/` |
| Git-tracked configuration | `src-tauri/android/` is committed, `gen/` is selective |
| Easy updates | `tauri android init` updates `gen/` without touching our custom files |
| Clear separation | New developers can see what's Tauri vs. what's custom |

## Caution: Deleting gen/

If you manually delete `gen/` and run `pnpm tauri android init`, the `apply(from = ...)` line in `build.gradle.kts` will be lost. **Do not delete `gen/`** — it contains the reference to our custom config.

If you accidentally delete it:
1. Check out `gen/android/app/build.gradle.kts` from git
2. Re-run `pnpm tauri android init`

## Troubleshooting

**Build fails with "file not found: ../../android/app-custom.gradle.kts"**
- Ensure `src-tauri/android/app-custom.gradle.kts` exists
- Verify you haven't moved the directory structure

**Rapfi AI doesn't work on device**
- Check that `copyRapfiBinaries` task ran: `./gradlew copyRapfiBinaries --info`
- Verify binaries exist: `ls -la gen/android/app/src/main/jniLibs/*/librapfi.so`

**Tauri CLI upgrade breaks build**
- Run `git diff gen/android/` to see what changed
- If `apply(from = ...)` was removed, re-add it to `gen/android/app/build.gradle.kts`
