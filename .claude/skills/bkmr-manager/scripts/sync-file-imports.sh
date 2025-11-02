#!/usr/bin/env bash
# sync-file-imports.sh - Sync file-imported bookmarks
# Usage: ./sync-file-imports.sh <directory> <base-path-name>
#
# Example:
#   ./sync-file-imports.sh ~/scripts SCRIPTS_HOME
#   ./sync-file-imports.sh ~/docs DOCS_HOME

set -euo pipefail

IMPORT_DIR="${1:-}"
BASE_PATH="${2:-}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

usage() {
    cat << EOF
Usage: $0 <directory> <base-path-name>

Sync file-imported bookmarks from a directory.

Arguments:
    directory       Directory containing files to import/sync
    base-path-name  Base path name from bkmr config (e.g., SCRIPTS_HOME)

Options:
    --delete-missing  Delete bookmarks for missing files
    --dry-run        Preview changes without executing

Examples:
    $0 ~/scripts SCRIPTS_HOME
    $0 ~/docs DOCS_HOME --delete-missing
    $0 ~/repos GITHUB_REPOS --dry-run

Requirements:
    - bkmr installed
    - BKMR_DB_URL set
    - Base path configured in ~/.config/bkmr/config.toml
EOF
    exit 1
}

[[ -z "$IMPORT_DIR" || -z "$BASE_PATH" ]] && usage

# Verify bkmr
if ! command -v bkmr &> /dev/null; then
    echo -e "${RED}Error: bkmr is not installed${NC}"
    exit 1
fi

# Verify directory
if [[ ! -d "$IMPORT_DIR" ]]; then
    echo -e "${RED}Error: Directory not found: $IMPORT_DIR${NC}"
    exit 1
fi

# Verify BKMR_DB_URL
if [[ -z "${BKMR_DB_URL:-}" ]]; then
    echo -e "${RED}Error: BKMR_DB_URL not set${NC}"
    echo "Run: export BKMR_DB_URL=~/.config/bkmr/bkmr.db"
    exit 1
fi

# Parse additional options
DELETE_MISSING=false
DRY_RUN=false
shift 2

while [[ $# -gt 0 ]]; do
    case $1 in
        --delete-missing)
            DELETE_MISSING=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            usage
            ;;
    esac
done

echo -e "${GREEN}Syncing file imports from: $IMPORT_DIR${NC}"
echo -e "${BLUE}Base path: $BASE_PATH${NC}"

# Build command
cmd="bkmr import-files \"$IMPORT_DIR\" --base-path $BASE_PATH --update"

[[ "$DELETE_MISSING" == "true" ]] && cmd="$cmd --delete-missing"
[[ "$DRY_RUN" == "true" ]] && cmd="$cmd --dry-run"

echo -e "${BLUE}Executing: $cmd${NC}"
echo ""

# Execute
eval "$cmd"

# Summary
echo ""
echo -e "${GREEN}Sync complete!${NC}"
echo ""
echo "Statistics:"
echo "  Total bookmarks: $(bkmr search --np | wc -l)"
echo "  Imported files: $(bkmr search -t _imported_ --np | wc -l)"
echo "  Shell scripts: $(bkmr search -t _shell_ --np | wc -l)"
echo "  Markdown docs: $(bkmr search -t _md_ --np | wc -l)"
