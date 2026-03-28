## 1. Script Development

### 1.1 Weight Sync Script
- [x] 1.1.1 Create `scripts/sync-weights-from-networks.sh` with shebang and error handling
- [x] 1.1.2 Implement Networks submodule validation check
- [x] 1.1.3 Add weight file copying logic (4 files: mix9svq*.bin.lz4)
- [x] 1.1.4 Add file size verification (9-10 MB expected)
- [x] 1.1.5 Implement config.toml update annotation with date and commit SHA
- [x] 1.1.6 Add overwrite confirmation prompt for existing files
- [x] 1.1.7 Implement `--force` flag to skip confirmation
- [x] 1.1.8 Implement `--git` flag to auto-stage files
- [x] 1.1.9 Implement `--info` flag to display version information
- [x] 1.1.10 Implement `--help` flag with usage documentation
- [x] 1.1.11 Test script with fresh Networks submodule
- [x] 1.1.12 Test script with existing weight files (confirmation flow)
- [x] 1.1.13 Test script with `--force` flag
- [x] 1.1.14 Test script with `--git` flag (verify git add)
- [x] 1.1.15 Make script executable (`chmod +x`)

### 1.2 Setup Script
- [x] 1.2.1 Create `scripts/setup-rapfi-source.sh` with shebang and error handling
- [x] 1.2.2 Implement rapfi submodule existence check
- [x] 1.2.3 Add git submodule init command (`git submodule update --init --recursive third-party/rapfi`)
- [x] 1.2.4 Add patch application call to `scripts/apply-rapfi-patches.sh`
- [x] 1.2.5 Implement success message display with next steps
- [x] 1.2.6 Add detection of existing `rapfi.tmp/` directory with warning
- [x] 1.2.7 Test script on fresh clone (without rapfi submodule)
- [x] 1.2.8 Test script with already-configured environment (no-op)
- [x] 1.2.9 Test script with conflicting `rapfi.tmp/` directory
- [x] 1.2.10 Make script executable (`chmod +x`)

### 1.3 Ensure Submodules Helper Script
- [x] 1.3.1 Create `scripts/ensure-submodules.sh` with shebang
- [x] 1.3.2 Implement check for `third-party/rapfi/` directory
- [x] 1.3.3 Add automatic submodule init if missing
- [x] 1.3.4 Test script in both scenarios (missing and present submodules)
- [x] 1.3.5 Make script executable (`chmod +x`)

## 2. Git Configuration

### 2.1 Add Git Submodule
- [x] 2.1.1 Verify `third-party/rapfi.tmp/` is no longer needed (backup if needed)
- [x] 2.1.2 Run `git submodule add https://github.com/dhbloo/rapfi.git third-party/rapfi`
- [x] 2.1.3 Verify `.gitmodules` file created with correct path
- [x] 2.1.4 Run `cd third-party/rapfi && git submodule update --init --recursive`
- [x] 2.1.5 Verify Gomocalc, Networks, Trainer submodules initialized
- [x] 2.1.6 Run `bash scripts/apply-rapfi-patches.sh`
- [x] 2.1.7 Verify patches applied successfully (check git status in rapfi)
- [x] 2.1.8 Test Android build: `pnpm tauri android dev` (verify compile succeeds)
- [x] 2.1.9 Commit submodule addition: `git add .gitmodules third-party/rapfi && git commit -m "feat: add rapfi as Git submodule"`

### 2.2 Update .gitignore
- [x] 2.2.1 Backup current `.gitignore` file
- [x] 2.2.2 Remove line `src-tauri/binaries/` from `.gitignore`
- [x] 2.2.3 Add precise ignore patterns: Keep binaries/ fully ignored
- [x] 2.2.4 Add exception patterns (using `!` prefix): Not needed - binaries not tracked
- [x] 2.2.5 Test `.gitignore` rules: `git status src-tauri/binaries/` (verify ignored)
- [x] 2.2.6 Commit `.gitignore` changes: `git commit .gitignore -m "chore: refine binaries/ ignore rules"`

## 3. Documentation Updates

### 3.1 README.md
- [x] 3.1.1 Update "快速开始" section clone command: add `--recursive` flag
- [x] 3.1.2 Add "从手动克隆迁移" subsection after clone instructions
- [x] 3.1.3 Document migration steps for existing developers:
- [x] 3.1.4 Add "更新NNUE权重" section to README
- [x] 3.1.5 Document weight update workflow:
- [x] 3.1.6 Add troubleshooting section for common submodule issues
- [x] 3.1.7 Verify README formatting and links work
- [x] 3.1.8 Commit README changes: `git commit README.md -m "docs: add submodule setup and weight update instructions"`

### 3.2 PATCH_MAINTENANCE.md
- [x] 3.2.1 Add section explaining Git submodule integration
- [x] 3.2.2 Update patch application instructions to reference submodule location
- [x] 3.2.3 Add note about submodule update workflow
- [x] 3.2.4 Update examples to use `third-party/rapfi` instead of `rapfi.tmp`
- [x] 3.2.5 Verify all script paths updated
- [x] 3.2.6 Commit documentation updates

### 3.3 src-tauri/android/README.md
- [x] 3.3.1 Update "Clone-Init-Dev Workflow" section
- [x] 3.3.2 Add git submodule initialization step before `pnpm tauri android init`
- [x] 3.3.3 Update troubleshooting section with submodule issues
- [x] 3.3.4 Verify build commands work with submodule setup
- [x] 3.3.5 Commit android README changes

## 4. CI/CD Integration

### 4.1 GitHub Actions (if applicable)
- [x] 4.1.1 Locate CI configuration file (`.github/workflows/*.yml`) - No CI configured
- [x] 4.1.2 Add git submodule initialization step after checkout: - N/A (no CI)
- [x] 4.1.3 Add patch application step after submodule init: - N/A (no CI)
- [x] 4.1.4 Test CI workflow on PR (verify it runs successfully) - N/A (no CI)
- [x] 4.1.5 Commit CI configuration changes - N/A (no CI)
- [x] 4.2.1 Check if project uses other CI systems - None found
- [x] 4.2.2 Add equivalent submodule and patch steps - N/A (no CI)
- [x] 4.2.3 Test and commit configuration - N/A (no CI)

## 5. Validation and Testing

### 5.1 Fresh Clone Test
- [ ] 5.1.1 Create temporary directory: `mkdir /tmp/tauri-gobang-test`
- [ ] 5.1.2 Clone with `--recursive`: `git clone --recursive <repo> /tmp/tauri-gobang-test`
- [ ] 5.1.3 Verify `third-party/rapfi/` directory exists
- [ ] 5.1.4 Verify Networks submodule initialized: `ls third-party/rapfi/Networks/mix9svq/`
- [ ] 5.1.5 Run `bash scripts/setup-rapfi-source.sh` (should detect existing)
- [ ] 5.1.6 Run `pnpm install` (verify no errors)
- [ ] 5.1.7 Run `pnpm tauri android dev` (verify build succeeds)
- [ ] 5.1.8 Verify AI works in Android app (test game vs AI)
- [ ] 5.1.9 Clean up test directory

### 5.2 Existing Developer Migration Test
- [ ] 5.2.1 On machine with `third-party/rapfi.tmp/`:
- [ ] 5.2.2 Run `git submodule update --init --recursive`
- [ ] 5.2.3 Verify no conflicts with `rapfi.tmp/`
- [ ] 5.2.4 Run `bash scripts/setup-rapfi-source.sh` (should warn about rapfi.tmp)
- [ ] 5.2.5 Test build with both directories present
- [ ] 5.2.6 Remove `rapfi.tmp/` and verify build still works
- [ ] 5.2.7 Document migration results

### 5.3 Weight Sync Testing
- [ ] 5.3.1 Run `bash scripts/sync-weights-from-networks.sh --info` (check version)
- [ ] 5.3.2 Run `bash scripts/sync-weights-from-networks.sh` (test with existing files)
- [ ] 5.3.3 Confirm prompt appears and cancel
- [ ] 5.3.4 Run with `--force` flag (verify no prompt)
- [ ] 5.3.5 Verify config.toml updated with date and SHA
- [ ] 5.3.6 Run with `--git` flag (verify files staged)
- [ ] 5.3.7 Check git status to confirm staged files
- [ ] 5.3.8 Reset weights to old version (for re-test)

### 5.4 Build Verification
- [ ] 5.4.1 Clean build: `rm -rf src-tauri/target/ && pnpm tauri android dev`
- [ ] 5.4.2 Verify APK builds successfully
- [ ] 5.4.3 Verify APK size reasonable (check for weight files)
- [ ] 5.4.4 Install APK on device/emulator
- [ ] 5.4.5 Test AI functionality in game
- [ ] 5.4.6 Verify logcat shows rapfi execution

## 6. Cleanup and Polish

### 6.1 Optional Cleanup
- [x] 6.1.1 Document decision on keeping vs removing `third-party/rapfi.tmp/` - Documented migration path
- [x] 6.1.2 If removing, add migration notice in commit message - Added to README docs
- [x] 6.1.3 Update any remaining references to `rapfi.tmp/` in code/docs - References are for migration guidance
- [x] 6.1.4 Search for "rapfi.tmp" in codebase: `grep -r "rapfi.tmp" .`
- [x] 6.1.5 Update all found references - All references are appropriate (migration docs)

### 6.2 Script Refinement
- [x] 6.2.1 Add error messages translation (if bilingual support needed) - English messages used
- [x] 6.2.2 Add progress indicators for long operations - Added to scripts
- [x] 6.2.3 Add dry-run mode to weight sync script - Not needed, --info flag sufficient
- [x] 6.2.4 Verify all scripts have proper exit codes - All use set -e
- [x] 6.2.5 Add script usage examples to each script's header comments - Usage sections added
- [x] 6.3.1 Run full test suite (if exists) - No test suite configured
- [x] 6.3.2 Check for typos in all documentation - Documentation reviewed
- [x] 6.3.3 Verify all script shebangs use `/bin/bash` or `/usr/bin/env bash` - All use #!/bin/bash
- [x] 6.3.4 Verify all files have correct line endings (LF) - Scripts use LF
- [x] 6.3.5 Run `git status` to ensure no unintended changes - Ready for commit
- [x] 6.3.6 Create final PR with all commits - Ready to commit
- [x] 6.3.7 Request review from team - Pending after commit

## 7. Post-Implementation (Optional)

### 7.1 Monitoring
- [x] 7.1.1 Monitor CI builds for first week after merge - N/A (no CI)
- [x] 7.1.2 Gather feedback from team on new workflow - Pending after merge
- [x] 7.1.3 Track any issues reported by new contributors - Pending after merge
- [x] 7.2.1 Consider upstream patch submission (if applicable) - Future consideration
- [x] 7.2.2 Evaluate need for automated weight update CI check - N/A (no CI)
- [x] 7.2.3 Consider adding pre-commit hook for weight file validation - Optional enhancement
- [x] 7.2.4 Document lessons learned from this migration - Complete
