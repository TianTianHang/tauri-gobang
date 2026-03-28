#!/bin/bash
set -e  # Exit on error
set -u  # Exit on undefined variable
set -o pipefail  # Exit on pipe failure

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
NETWORKS_DIR="$PROJECT_ROOT/third-party/rapfi/Networks/mix9svq"
BINARIES_DIR="$PROJECT_ROOT/src-tauri/binaries"
CONFIG_FILE="$BINARIES_DIR/config.toml"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default options
FORCE=false
GIT_STAGE=false
INFO_ONLY=false

# Function to display usage
usage() {
    cat << EOF
Usage: $(basename "$0") [OPTIONS]

Sync NNUE weight files from rapfi Networks submodule to src-tauri/binaries/

OPTIONS:
    --force    Skip confirmation when overwriting existing files
    --git      Stage updated files for git commit after sync
    --info     Display current version information only
    --help     Display this help message

EXAMPLES:
    $(basename "$0")              # Sync with confirmation prompt
    $(basename "$0") --force      # Sync without prompt
    $(basename "$0") --git        # Sync and stage files for commit
    $(basename "$0") --info       # Show version information

EOF
    exit 0
}

# Function to log error and exit
error_exit() {
    echo -e "${RED}ERROR: $1${NC}" >&2
    exit 1
}

# Function to log success message
success_msg() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Function to log warning
warning_msg() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --force)
            FORCE=true
            shift
            ;;
        --git)
            GIT_STAGE=true
            shift
            ;;
        --info)
            INFO_ONLY=true
            shift
            ;;
        --help)
            usage
            ;;
        *)
            error_exit "Unknown option: $1. Use --help for usage information."
            ;;
    esac
done

# Function to validate Networks submodule
validate_networks_submodule() {
    if [ ! -d "$NETWORKS_DIR" ]; then
        error_exit "Networks submodule not initialized at $NETWORKS_DIR\n\nPlease run:\n  cd third-party/rapfi && git submodule update --init --recursive"
    fi

    # Check if Networks submodule is properly initialized (has .git file or directory)
    if [ ! -f "$NETWORKS_DIR/../.git" ] && [ ! -d "$NETWORKS_DIR/../.git" ]; then
        # Check parent directory for .git file indicating submodule
        if [ ! -f "$PROJECT_ROOT/third-party/rapfi/.git" ]; then
            error_exit "Rapfi submodule not initialized\n\nPlease run:\n  git submodule update --init --recursive third-party/rapfi"
        fi
    fi

    success_msg "Networks submodule validated at $NETWORKS_DIR"
}

# Function to get Networks submodule commit SHA
get_networks_sha() {
    if [ -d "$PROJECT_ROOT/third-party/rapfi/.git" ]; then
        cd "$PROJECT_ROOT/third-party/rapfi"
        git submodule status -- Networks | awk '{print $1}' | sed 's/^-//'
    else
        cd "$PROJECT_ROOT"
        git submodule status -- third-party/rapfi/Networks 2>/dev/null | awk '{print $1}' | sed 's/^-//' || echo "unknown"
    fi
}

# Weight files to copy
declare -a WEIGHT_FILES=(
    "mix9svqfreestyle_bsmix.bin.lz4"
    "mix9svqrenju_bs15_black.bin.lz4"
    "mix9svqrenju_bs15_white.bin.lz4"
    "mix9svqstandard_bs15.bin.lz4"
)

# Function to copy weight files
copy_weight_files() {
    local copied_count=0
    local total_size_bytes=0

    echo "Copying weight files from $NETWORKS_DIR to $BINARIES_DIR"

    for weight_file in "${WEIGHT_FILES[@]}"; do
        local src_file="$NETWORKS_DIR/$weight_file"
        local dest_file="$BINARIES_DIR/$weight_file"

        if [ ! -f "$src_file" ]; then
            error_exit "Weight file not found: $src_file"
        fi

        # Check if destination file exists
        if [ -f "$dest_file" ] && [ "$FORCE" != true ]; then
            warning_msg "File already exists: $weight_file"
        fi

        cp "$src_file" "$dest_file"
        local file_size=$(stat -c%s "$dest_file" 2>/dev/null || stat -f%z "$dest_file" 2>/dev/null)
        local size_mb=$(echo "scale=2; $file_size / 1024 / 1024" | bc)

        # Verify file size (9-10 MB expected range)
        local size_mb_int=$(echo "$size_mb" | cut -d. -f1)
        if [ "$size_mb_int" -lt 9 ] || [ "$size_mb_int" -gt 10 ]; then
            warning_msg "File size $size_mb MB is outside expected range (9-10 MB)"
        fi

        total_size_bytes=$((total_size_bytes + file_size))
        echo "  ✓ $weight_file ($size_mb MB)"
        copied_count=$((copied_count + 1))
    done

    local total_size_mb=$(echo "scale=2; $total_size_bytes / 1024 / 1024" | bc)
    success_msg "Copied $copied_count weight files ($total_size_mb MB)"
}

# Function to update config.toml with sync information
update_config_annotation() {
    local networks_sha=$(get_networks_sha)
    local current_date=$(date +%Y-%m-%d)
    local annotation="# Weights updated: $current_date from Networks@$networks_sha"

    if [ ! -f "$CONFIG_FILE" ]; then
        error_exit "Config file not found: $CONFIG_FILE"
    fi

    # Check if annotation already exists
    if grep -q "^# Weights updated:" "$CONFIG_FILE"; then
        # Replace existing annotation
        sed -i.bak "s/^# Weights updated:.*/$annotation/" "$CONFIG_FILE"
        rm -f "${CONFIG_FILE}.bak"
        success_msg "Updated weight version annotation in config.toml"
    else
        # Add new annotation at the beginning of the file
        local temp_file=$(mktemp)
        echo "$annotation" > "$temp_file"
        cat "$CONFIG_FILE" >> "$temp_file"
        mv "$temp_file" "$CONFIG_FILE"
        success_msg "Added weight version annotation to config.toml"
    fi

    echo "  $annotation"
}

# Function to check for existing weight files
check_existing_files() {
    local existing_files=()

    for weight_file in "${WEIGHT_FILES[@]}"; do
        if [ -f "$BINARIES_DIR/$weight_file" ]; then
            existing_files+=("$weight_file")
        fi
    done

    if [ ${#existing_files[@]} -gt 0 ] && [ "$FORCE" != true ]; then
        echo ""
        warning_msg "The following weight files already exist:"
        for file in "${existing_files[@]}"; do
            echo "  - $file"
        done
        echo ""
        read -p "Do you want to overwrite them? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Sync cancelled."
            exit 0
        fi
    fi
}

# Function to stage files for git commit
stage_git_files() {
    local networks_sha=$(get_networks_sha)

    echo ""
    cd "$PROJECT_ROOT"
    git add src-tauri/binaries/*.bin.lz4
    git add "$CONFIG_FILE"
    success_msg "Weight files staged for git commit"

    echo ""
    echo "Weight files staged. Commit with:"
    echo "  git commit -m \"chore: update NNUE weights from Networks@$networks_sha\""
}

# Function to display version information
display_info() {
    echo "Weight Files Version Information"
    echo "==============================="
    echo ""

    # Display Networks submodule SHA
    local networks_sha=$(get_networks_sha)
    echo "Networks submodule commit: $networks_sha"

    # Display last sync date from config.toml
    if [ -f "$CONFIG_FILE" ]; then
        local last_sync=$(grep "^# Weights updated:" "$CONFIG_FILE" || echo "No sync information found")
        echo "Last sync: $last_sync"
    else
        echo "Config file not found: $CONFIG_FILE"
    fi

    echo ""

    # Display current weight files
    echo "Current weight files in $BINARIES_DIR:"
    for weight_file in "${WEIGHT_FILES[@]}"; do
        if [ -f "$BINARIES_DIR/$weight_file" ]; then
            local file_size=$(stat -c%s "$BINARIES_DIR/$weight_file" 2>/dev/null || stat -f%z "$BINARIES_DIR/$weight_file" 2>/dev/null)
            local size_mb=$(echo "scale=2; $file_size / 1024 / 1024" | bc)
            echo "  ✓ $weight_file ($size_mb MB)"
        else
            echo "  ✗ $weight_file (missing)"
        fi
    done

    exit 0
}

# Main execution
main() {
    # Handle --info flag
    if [ "$INFO_ONLY" = true ]; then
        validate_networks_submodule
        display_info
    fi

    echo "NNUE Weight Files Sync"
    echo "====================="
    echo ""

    # Validate Networks submodule
    validate_networks_submodule

    # Check for existing files
    check_existing_files

    # Copy weight files
    copy_weight_files

    # Update config.toml annotation
    update_config_annotation

    # Stage files for git if --git flag is set
    if [ "$GIT_STAGE" = true ]; then
        stage_git_files
    fi

    echo ""
    success_msg "Weight files sync complete!"
}

# Run main function
main
