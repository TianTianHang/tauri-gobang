#!/bin/bash
set -e  # Exit on error
set -u  # Exit on undefined variable
set -o pipefail  # Exit on pipe failure

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RAPFI_DIR="$PROJECT_ROOT/third-party/rapfi"
PATCH_SCRIPT="$SCRIPT_DIR/apply-rapfi-patches.sh"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to display usage
usage() {
    cat << EOF
Usage: $(basename "$0") [OPTIONS]

Setup rapfi source code by initializing Git submodule and applying patches.

OPTIONS:
    --help     Display this help message

EXAMPLES:
    $(basename "$0")              # Setup rapfi submodule and apply patches

This script will:
1. Initialize the rapfi Git submodule at third-party/rapfi
2. Apply Android build patches
3. Display next steps

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

# Function to log info
info_msg() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# Function to log warning
warning_msg() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help)
            usage
            ;;
        *)
            error_exit "Unknown option: $1. Use --help for usage information."
            ;;
    esac
done

# Function to check if rapfi submodule exists
check_rapfi_submodule() {
    if [ -d "$RAPFI_DIR" ]; then
        # Check if it's a valid Git directory
        if [ -f "$RAPFI_DIR/.git" ] || [ -d "$RAPFI_DIR/.git" ]; then
            return 0  # Exists and is a Git repo
        fi
    fi
    return 1  # Does not exist or is not a Git repo
}

# Function to check for conflicting rapfi.tmp directory
check_rapfi_tmp_conflict() {
    local rapfi_tmp_dir="$PROJECT_ROOT/third-party/rapfi.tmp"

    if [ -d "$rapfi_tmp_dir" ]; then
        warning_msg "Detected existing directory: $rapfi_tmp_dir"
        echo ""
        echo "The old manual clone directory may conflict with the new Git submodule."
        echo "Consider removing it after setup is complete:"
        echo "  rm -rf $rapfi_tmp_dir"
        echo ""
        read -p "Press Enter to continue setup..."
    fi
}

# Function to initialize rapfi submodule
init_rapfi_submodule() {
    info_msg "Initializing rapfi Git submodule..."

    if check_rapfi_submodule; then
        info_msg "Rapfi submodule already exists at $RAPFI_DIR"
        return 0
    fi

    cd "$PROJECT_ROOT"

    # Initialize and update the submodule recursively
    git submodule update --init --recursive third-party/rapfi

    if [ ! -d "$RAPFI_DIR" ]; then
        error_exit "Failed to initialize rapfi submodule"
    fi

    success_msg "Rapfi submodule initialized successfully"
}

# Function to apply rapfi patches
apply_rapfi_patches() {
    info_msg "Applying Android build patches..."

    if [ ! -f "$PATCH_SCRIPT" ]; then
        error_exit "Patch script not found: $PATCH_SCRIPT"
    fi

    bash "$PATCH_SCRIPT"

    success_msg "Patches applied successfully"
}

# Function to display next steps
display_next_steps() {
    echo ""
    echo "=========================================="
    success_msg "Rapfi source setup complete!"
    echo "=========================================="
    echo ""
    echo "Next steps:"
    echo "  1. Build Android rapfi binary:"
    echo "     bash scripts/build-android-rapfi.sh"
    echo ""
    echo "  2. Update NNUE weights (optional):"
    echo "     bash scripts/sync-weights-from-networks.sh"
    echo ""
    echo "  3. Build and run the app:"
    echo "     pnpm tauri android dev"
    echo ""
}

# Main execution
main() {
    echo "Rapfi Source Setup"
    echo "=================="
    echo ""

    # Check for conflicting rapfi.tmp directory
    check_rapfi_tmp_conflict

    # Check if already configured
    if check_rapfi_submodule; then
        info_msg "Rapfi source already configured at $RAPFI_DIR"
        echo ""
        read -p "Re-apply patches? (y/N) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            apply_rapfi_patches
        fi
    else
        # Initialize submodule
        init_rapfi_submodule

        # Apply patches
        apply_rapfi_patches

        # Display next steps
        display_next_steps
    fi
}

# Run main function
main
