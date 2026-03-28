## ADDED Requirements

### Requirement: Git submodule initialization
The system SHALL provide automated initialization of the rapfi Git submodule to ensure source code is available after cloning.

#### Scenario: Fresh clone with --recursive flag
- **WHEN** developer runs `git clone --recursive <repository-url>`
- **THEN** the `third-party/rapfi/` directory SHALL be populated with rapfi source code
- **AND** all rapfi submodules (Gomocalc, Networks, Trainer) SHALL be initialized

#### Scenario: Existing clone without submodules
- **WHEN** developer runs `git clone <repository-url>` without `--recursive`
- **AND** then runs `git submodule update --init --recursive`
- **THEN** the `third-party/rapfi/` directory SHALL be populated
- **AND** all submodules SHALL be initialized

### Requirement: Patch application automation
The system SHALL provide automated script to apply Android build patches to the rapfi submodule.

#### Scenario: Apply patches via script
- **WHEN** developer runs `bash scripts/apply-rapfi-patches.sh`
- **THEN** patch `0001-fix-pthread-linking.patch` SHALL be applied to `third-party/rapfi/Rapfi/CMakeLists.txt`
- **AND** patch `0002-fix-siwrite-undefined.patch` SHALL be applied to `third-party/rapfi/Rapfi/external/zip/src/zip.c`
- **AND** script SHALL exit with status 0 if patches applied successfully

#### Scenario: Patches already applied
- **WHEN** developer runs `bash scripts/apply-rapfi-patches.sh` on already-patched source
- **THEN** script SHALL detect existing patches
- **AND** script SHALL display "Patches already applied or modified files detected" message
- **AND** script SHALL exit gracefully without error

#### Scenario: Missing rapfi source
- **WHEN** developer runs `bash scripts/apply-rapfi-patches.sh` without rapfi submodule initialized
- **THEN** script SHALL display error message "rapfi source not found at third-party/rapfi"
- **AND** script SHALL suggest running `git submodule update --init --recursive`
- **AND** script SHALL exit with non-zero status

### Requirement: One-time setup script
The system SHALL provide a unified setup script that initializes rapfi submodule and applies patches.

#### Scenario: First-time setup
- **WHEN** developer runs `bash scripts/setup-rapfi-source.sh` on fresh clone
- **THEN** script SHALL check if `third-party/rapfi/` exists
- **AND** if not exists, SHALL run `git submodule update --init --recursive third-party/rapfi`
- **AND** THEN SHALL run `bash scripts/apply-rapfi-patches.sh`
- **AND** script SHALL display success message with next steps

#### Scenario: Setup on already configured environment
- **WHEN** developer runs `bash scripts/setup-rapfi-source.sh` with rapfi already set up
- **THEN** script SHALL detect existing setup
- **AND** script SHALL display "Rapfi source already configured" message
- **AND** script SHALL exit without re-running setup

### Requirement: Submodule update workflow
The system SHALL support updating rapfi submodule to latest upstream version.

#### Scenario: Update rapfi to latest upstream
- **WHEN** developer runs `cd third-party/rapfi && git pull origin main`
- **THEN** rapfi source SHALL be updated to latest commit
- **AND** developer MAY need to re-apply patches if upstream changes conflict
- **AND** developer SHALL run `bash scripts/apply-rapfi-patches.sh` to verify patches still apply

#### Scenario: Update rapfi Networks submodule
- **WHEN** developer runs `cd third-party/rapfi && git submodule update --remote Networks`
- **THEN** Networks submodule SHALL be updated to latest commit on its main branch
- **AND** developer MAY run weight sync script to copy new weights to `src-tauri/binaries/`

### Requirement: Patch revert capability
The system SHALL provide script to revert rapfi patches to restore original upstream code.

#### Scenario: Revert applied patches
- **WHEN** developer runs `bash scripts/revert-rapfi-patches.sh`
- **THEN** changes to `third-party/rapfi/Rapfi/CMakeLists.txt` SHALL be reverted
- **AND** changes to `third-party/rapfi/Rapfi/external/zip/src/zip.c` SHALL be reverted
- **AND** files SHALL match upstream HEAD state

#### Scenario: Revert with no patches applied
- **WHEN** developer runs `bash scripts/revert-rapfi-patches.sh` on clean upstream code
- **THEN** script SHALL display "No patches to revert (files are clean)" message
- **AND** script SHALL exit gracefully

### Requirement: CI/CD integration
The system SHALL include rapfi submodule initialization and patch application in CI/CD pipeline.

#### Scenario: CI pipeline on fresh clone
- **WHEN** CI runner clones repository with default settings
- **THEN** CI pipeline SHALL run `git submodule update --init --recursive`
- **AND** CI pipeline SHALL run `bash scripts/apply-rapfi-patches.sh`
- **AND** CI pipeline SHALL verify patches applied successfully

#### Scenario: CI pipeline cache optimization
- **WHEN** CI runner uses cached Git submodules
- **THEN** CI pipeline SHALL skip re-downloading rapfi source if unchanged
- **AND** CI pipeline SHALL still verify patches are applied

### Requirement: Developer migration from manual clone
The system SHALL support developers who previously used `third-party/rapfi.tmp/`.

#### Scenario: Developer has rapfi.tmp directory
- **WHEN** developer with existing `third-party/rapfi.tmp/` runs `bash scripts/setup-rapfi-source.sh`
- **THEN** script SHALL detect conflict with `rapfi.tmp/` directory
- **AND** script SHALL display warning message suggesting removal of old directory
- **AND** script SHALL continue setup if developer confirms

#### Scenario: Remove old rapfi.tmp directory
- **WHEN** developer deletes `third-party/rapfi.tmp/` after submodule setup
- **THEN** system SHALL continue to function normally using `third-party/rapfi/`
- **AND** no build scripts SHALL reference `rapfi.tmp/` path
