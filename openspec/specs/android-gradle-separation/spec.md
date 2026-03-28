# android-gradle-separation Specification

## Purpose
TBD - created by archiving change android-gradle-separation. Update Purpose after archive.
## Requirements
### Requirement: Custom Gradle logic lives outside gen/
The system SHALL keep all custom Android build configuration (packaging options, binary copy tasks, dependency configuration) in a file under `src-tauri/android/` that is tracked by git and separate from Tauri-generated files.

#### Scenario: Custom Gradle file exists in src-tauri/android/
- **WHEN** a developer inspects the project structure
- **THEN** `src-tauri/android/app-custom.gradle.kts` exists and contains packagingOptions, copyRapfiBinaries task, and preBuild dependency

#### Scenario: gen/android/app/build.gradle.kts references custom file
- **WHEN** a developer opens `gen/android/app/build.gradle.kts`
- **THEN** the file contains an `apply(from = "../../android/app-custom.gradle.kts")` line
- **AND** the file does NOT contain inline packagingOptions, copyRapfiBinaries task, or preBuild dependency

### Requirement: Build workflow unchanged after separation
The system SHALL produce identical build output whether custom logic is inline in `gen/android/app/build.gradle.kts` or separated via `apply from`.

#### Scenario: pnpm tauri android dev works
- **WHEN** a developer runs `pnpm tauri android dev`
- **THEN** Gradle executes successfully
- **AND** copyRapfiBinaries task runs
- **AND** APK contains librapfi.so and libc++_shared.so in jniLibs

#### Scenario: pnpm tauri android build works
- **WHEN** a developer runs `pnpm tauri android build`
- **THEN** release APK is built successfully
- **AND** rapfi AI engine is functional on device

### Requirement: Clone-init-dev workflow supported
The system SHALL allow a fresh clone to build Android APK after running `tauri android init` without manual file editing.

#### Scenario: Fresh clone with existing gen/ files
- **WHEN** a developer clones the repo (gen/ files tracked in git)
- **AND** runs `pnpm tauri android dev`
- **THEN** the build succeeds without manual configuration

#### Scenario: Fresh clone after tauri android init
- **WHEN** a developer clones the repo
- **AND** the existing `gen/android/app/build.gradle.kts` from git contains the apply line
- **AND** `tauri android init` does NOT overwrite it (verified behavior)
- **THEN** the build succeeds without manual configuration

### Requirement: MainActivity.kt contains no dead code
The system SHALL remove unused JNI methods from `MainActivity.kt`.

#### Scenario: No unused JNI methods
- **WHEN** a developer reads `MainActivity.kt`
- **THEN** the file contains only `onCreate` override with `enableEdgeToEdge()` and `super.onCreate()`
- **AND** the file does NOT contain `getNativeLibraryDir()` or `getRapfiExecutablePath()` methods

#### Scenario: Android rapfi path resolution still works
- **WHEN** AI engine starts on Android
- **THEN** `android_rapfi.rs` resolves `librapfi.so` path via `/proc/self/maps` parsing
- **AND** AI functions correctly without JNI helper methods

