## ADDED Requirements

### Requirement: Weight files synchronization from Networks submodule
The system SHALL provide automated script to copy weight files from rapfi Networks submodule to `src-tauri/binaries/` directory.

#### Scenario: Sync all weight files
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh`
- **THEN** script SHALL copy `mix9svqfreestyle_bsmix.bin.lz4` from `third-party/rapfi/Networks/mix9svq/` to `src-tauri/binaries/`
- **AND** script SHALL copy `mix9svqrenju_bs15_black.bin.lz4` to `src-tauri/binaries/`
- **AND** script SHALL copy `mix9svqrenju_bs15_white.bin.lz4` to `src-tauri/binaries/`
- **AND** script SHALL copy `mix9svqstandard_bs15.bin.lz4` to `src-tauri/binaries/`
- **AND** script SHALL display success message with list of copied files

#### Scenario: Verify file sizes after sync
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh`
- **THEN** each copied weight file SHALL be approximately 9-10 MB in size
- **AND** script SHALL display file sizes for verification

### Requirement: Networks submodule validation
The system SHALL validate that Networks submodule is initialized before attempting weight sync.

#### Scenario: Networks submodule not initialized
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh` without Networks submodule initialized
- **THEN** script SHALL display error message "Networks submodule not initialized"
- **AND** script SHALL suggest running `cd third-party/rapfi && git submodule update --init --recursive`
- **AND** script SHALL exit with non-zero status

#### Scenario: Networks submodule initialized
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh` with Networks submodule present
- **THEN** script SHALL locate weight files at `third-party/rapfi/Networks/mix9svq/*.bin.lz4`
- **AND** script SHALL proceed with synchronization

### Requirement: Config.toml update annotation
The system SHALL update `src-tauri/binaries/config.toml` with weight file version information after sync.

#### Scenario: Update config.toml with sync date
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh`
- **THEN** script SHALL update or add comment in `config.toml` with format:
  ```
  # Weights updated: YYYY-MM-DD from Networks@<commit-sha>
  ```
- **AND** `<commit-sha>` SHALL be the short SHA of Networks submodule HEAD

#### Scenario: Preserve existing config.toml content
- **WHEN** developer runs weight sync script
- **THEN** script SHALL preserve all existing configuration in `config.toml`
- **AND** script SHALL only modify or add the weight update annotation comment

### Requirement: Weight file overwrite confirmation
The system SHALL prompt for confirmation before overwriting existing weight files.

#### Scenario: Weight files already exist
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh` with existing weight files in `src-tauri/binaries/`
- **THEN** script SHALL display list of files to be overwritten
- **AND** script SHALL prompt developer for confirmation (y/n)
- **AND** if confirmed, script SHALL overwrite existing files
- **AND** if declined, script SHALL exit without changes

#### Scenario: Force overwrite flag
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh --force`
- **THEN** script SHALL overwrite existing weight files without prompting
- **AND** script SHALL display message "Overwriting existing weight files"

### Requirement: Git integration for weight updates
The system SHALL support staging updated weight files for git commit after sync.

#### Scenario: Auto-stage weight files after sync
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh` with `--git` flag
- **THEN** script SHALL run `git add src-tauri/binaries/*.bin.lz4` after copying files
- **AND** script SHALL display message suggesting:
  ```
  Weight files staged. Commit with:
  git commit -m "chore: update NNUE weights from Networks@<commit-sha>"
  ```

#### Scenario: Detect uncommitted changes before sync
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh --git` with uncommitted changes in `src-tauri/binaries/`
- **THEN** script SHALL detect uncommitted changes via `git status`
- **AND** script SHALL display warning about uncommitted changes
- **AND** script SHALL prompt whether to continue

### Requirement: Version tracking for weight files
The system SHALL maintain version information for weight files to enable traceability.

#### Scenario: Tag weight updates with Networks commit
- **WHEN** weight files are synchronized
- **THEN** system SHALL record Networks submodule commit SHA in `config.toml`
- **AND** developer SHALL be able to trace weight file version to specific Networks commit

#### Scenario: Display weight version information
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh --info`
- **THEN** script SHALL display current Networks submodule commit SHA
- **AND** script SHALL display date of last weight sync from `config.toml`
- **AND** script SHALL compare current and last-synced versions

### Requirement: Weight file integrity verification
The system SHALL verify that copied weight files are valid and not corrupted.

#### Scenario: Verify file sizes after sync
- **WHEN** weight files are copied to `src-tauri/binaries/`
- **THEN** script SHALL verify each file size is within expected range (9-10 MB)
- **AND** if file size is unexpected, script SHALL display warning message
- **AND** script SHALL exit with non-zero status if files appear corrupted

#### Scenario: Verify file count
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh`
- **THEN** script SHALL verify that exactly 4 weight files were copied
- **AND** script SHALL display total count of copied files

### Requirement: Documentation for weight update workflow
The system SHALL document the recommended workflow for updating weight files.

#### Scenario: README documentation
- **WHEN** developer reads `README.md` or `docs/` directory
- **THEN** documentation SHALL include section on "Updating NNUE Weights"
- **AND** documentation SHALL provide step-by-step workflow:
  1. Update Networks submodule: `cd third-party/rapfi && git submodule update --remote Networks`
  2. Sync weights: `bash scripts/sync-weights-from-networks.sh`
  3. Commit changes: `git add src-tauri/binaries/*.bin.lz4 && git commit -m "chore: update weights"`
- **AND** documentation SHALL explain when to update weights (e.g., after rapfi release)

#### Scenario: Script usage help
- **WHEN** developer runs `bash scripts/sync-weights-from-networks.sh --help`
- **THEN** script SHALL display usage information
- **AND** script SHALL list all available flags (`--force`, `--git`, `--info`, `--help`)
- **AND** script SHALL provide example commands
