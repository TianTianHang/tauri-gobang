#!/bin/bash
set -e  # Exit on error
set -u  # Exit on undefined variable
set -o pipefail  # Exit on pipe failure

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RAPFI_DIR="$PROJECT_ROOT/third-party/rapfi"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to display usage
usage() {
    cat << EOF
Usage: $(basename "$0") [OPTIONS]

Ensure Git submodules are initialized.

OPTIONS:
    --help     Display this help message

EXAMPLES:
    $(basename "$0")              # Check and initialize submodules if needed

This script will check if rapfi Git submodule is initialized,
and initialize it automatically if missing.

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

# Function to initialize submodules if missing
init_submodules() {
    if check_rapfi_submodule; then
        info_msg "Rapfi submodule already initialized"
        return 0
    fi

    info_msg "Rapfi submodule not found. Initializing..."
    cd "$PROJECT_ROOT"
    git submodule update --init --recursive

    if check_rapfi_submodule; then
        success_msg "Submodules initialized successfully"
    else
        error_exit "Failed to initialize submodules"
    fi
}

# Main execution
main() {
    echo "Checking Git submodules..."
    echo ""

    init_submodules

    echo ""
    success_msg "All submodules ready!"
}

# Run main function
main
