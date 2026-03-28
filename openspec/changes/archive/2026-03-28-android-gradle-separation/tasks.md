## 1. Create custom Gradle file

- [x] 1.1 Create `src-tauri/android/` directory
- [x] 1.2 Create `src-tauri/android/app-custom.gradle.kts` with packagingOptions (useLegacyPackaging + doNotStrip), copyRapfiBinaries task, and preBuild dependency hook
- [x] 1.3 Verify content matches original inline logic from `gen/android/app/build.gradle.kts` lines 53-130

## 2. Create documentation

- [x] 2.1 Create `src-tauri/android/README.md` explaining the separation pattern, relative path derivation, and clone-init-dev workflow

## 3. Simplify gen/android/app/build.gradle.kts

- [x] 3.1 Remove packagingOptions block (lines 53-61)
- [x] 3.2 Remove copyRapfiBinaries task and preBuild hook (lines 68-130)
- [x] 3.3 Add `apply(from = "../../android/app-custom.gradle.kts")` at end of file
- [x] 3.4 Verify resulting file is ~70 lines with Tauri template + 1 apply line

## 4. Simplify MainActivity.kt

- [x] 4.1 Remove `getNativeLibraryDir()` method from MainActivity.kt
- [x] 4.2 Remove `getRapfiExecutablePath()` method from MainActivity.kt
- [x] 4.3 Verify file contains only `onCreate` override

## 5. Verify

- [x] 5.1 Run `pnpm tauri android dev` and confirm build succeeds
- [x] 5.2 Verify rapfi AI works on Android device/emulator
- [x] 5.3 Verify `pnpm tauri android build` produces working release APK
