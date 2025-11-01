#!/usr/bin/env bash
# fetch-readme-embeddings.sh - Fetch README files from GitHub stars and embed in bkmr
# Usage: ./fetch-readme-embeddings.sh [options]
#
# Options:
#   --filter TAGS       Filter by tags (e.g., "language-rust,cli")
#   --limit N           Process at most N repositories (default: all)
#   --force             Re-fetch even if README already exists
#   --no-embed          Skip embedding generation (faster for testing)
#   --dry-run           Show what would be processed without executing
#
# Requirements:
#   - gh (GitHub CLI) authenticated
#   - jq for JSON processing
#   - bkmr installed with BKMR_DB_URL set
#   - OPENAI_API_KEY set (unless --no-embed)

set -euo pipefail

# Configuration
FILTER_TAGS="${FILTER_TAGS:-}"
LIMIT="${LIMIT:-}"
FORCE="${FORCE:-false}"
NO_EMBED="${NO_EMBED:-false}"
DRY_RUN="${DRY_RUN:-false}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Usage
usage() {
    cat << EOF
Usage: $0 [options]

Fetch README files from GitHub starred repositories and import to bkmr with embeddings.

Options:
    --filter TAGS       Filter by bkmr tags (e.g., "language-rust")
    --limit N           Process at most N repositories
    --force             Re-fetch even if README bookmark exists
    --no-embed          Skip embedding generation
    --dry-run           Preview without executing

Examples:
    $0
    $0 --filter language-rust --limit 50
    $0 --filter stars-1k --no-embed
    $0 --dry-run

Requirements:
    - gh auth login
    - export BKMR_DB_URL=~/.config/bkmr/bkmr.db
    - export OPENAI_API_KEY=sk-... (unless --no-embed)
EOF
    exit 1
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --filter)
            FILTER_TAGS="$2"
            shift 2
            ;;
        --limit)
            LIMIT="$2"
            shift 2
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --no-embed)
            NO_EMBED=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --help|-h)
            usage
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            usage
            ;;
    esac
done

# Verify dependencies
for cmd in gh jq bkmr; do
    if ! command -v "$cmd" &> /dev/null; then
        echo -e "${RED}Error: $cmd is not installed${NC}"
        exit 1
    fi
done

# Verify GitHub authentication
if ! gh auth status &> /dev/null; then
    echo -e "${RED}Error: Not authenticated with GitHub${NC}"
    echo "Run: gh auth login"
    exit 1
fi

# Verify BKMR_DB_URL
if [[ -z "${BKMR_DB_URL:-}" ]]; then
    echo -e "${RED}Error: BKMR_DB_URL not set${NC}"
    echo "Run: export BKMR_DB_URL=~/.config/bkmr/bkmr.db"
    exit 1
fi

# Verify OpenAI API key (unless no-embed)
if [[ "$NO_EMBED" == "false" ]] && [[ -z "${OPENAI_API_KEY:-}" ]]; then
    echo -e "${YELLOW}Warning: OPENAI_API_KEY not set${NC}"
    echo "Embeddings will not be generated. Use --no-embed to suppress this warning."
    NO_EMBED=true
fi

echo -e "${GREEN}Fetching GitHub starred repositories from bkmr...${NC}"

# Build tag filter
tag_filter_args=""
if [[ -n "$FILTER_TAGS" ]]; then
    tag_filter_args="-t $FILTER_TAGS"
    echo -e "${BLUE}Filtering by tags: $FILTER_TAGS${NC}"
fi

# Fetch GitHub bookmarks from bkmr
github_bookmarks=$(bkmr search $tag_filter_args -t github --json)
total=$(echo "$github_bookmarks" | jq length)

echo -e "${GREEN}Found $total GitHub bookmarks${NC}"

# Apply limit
if [[ -n "$LIMIT" ]]; then
    github_bookmarks=$(echo "$github_bookmarks" | jq --argjson limit "$LIMIT" '.[:$limit]')
    count=$(echo "$github_bookmarks" | jq length)
    echo -e "${BLUE}Limited to $count repositories${NC}"
else
    count=$total
fi

[[ "$count" -eq 0 ]] && { echo -e "${YELLOW}No repositories to process${NC}"; exit 0; }

# Process each repository
processed=0
skipped=0
failed=0

echo "$github_bookmarks" | jq -c '.[]' | while read -r bookmark; do
    # Extract bookmark data
    bookmark_id=$(echo "$bookmark" | jq -r '.id')
    url=$(echo "$bookmark" | jq -r '.url')
    title=$(echo "$bookmark" | jq -r '.title')
    tags=$(echo "$bookmark" | jq -r '.tags')

    # Extract owner/repo from URL
    if [[ "$url" =~ github\.com/([^/]+)/([^/]+) ]]; then
        owner="${BASH_REMATCH[1]}"
        repo="${BASH_REMATCH[2]}"
    else
        echo -e "${YELLOW}Skipping non-GitHub URL: $url${NC}"
        ((skipped++))
        continue
    fi

    echo -e "${BLUE}Processing $((processed + 1))/$count: $owner/$repo${NC}"

    # Check if README bookmark already exists (unless force)
    if [[ "$FORCE" == "false" ]]; then
        existing_readme=$(bkmr search -t readme --json | jq -r --arg repo "$repo" '.[] |
            select(.title | contains($repo) and contains("README")) |
            .id' | head -1)

        if [[ -n "$existing_readme" ]]; then
            echo -e "${YELLOW}  README already imported (ID: $existing_readme), skipping${NC}"
            ((skipped++))
            continue
        fi
    fi

    if [[ "$DRY_RUN" == "true" ]]; then
        echo -e "${YELLOW}  [DRY-RUN] Would fetch README from $owner/$repo${NC}"
        ((processed++))
        continue
    fi

    # Fetch README content
    echo "  Fetching README..."
    readme_content=$(gh api "repos/$owner/$repo/readme" \
        --jq '.content' 2>/dev/null | base64 --decode 2>/dev/null) || {
        echo -e "${YELLOW}  No README found or access denied${NC}"
        ((failed++))
        continue
    }

    # Check if README is empty
    if [[ -z "$readme_content" ]] || [[ $(echo "$readme_content" | wc -c) -lt 10 ]]; then
        echo -e "${YELLOW}  README is empty or too short, skipping${NC}"
        ((failed++))
        continue
    fi

    # Build tags for README bookmark
    readme_tags="$tags,readme,_md_,source-github"

    # Create title
    readme_title="[$owner/$repo] README"

    # Create description with link to parent
    readme_desc="README from $owner/$repo | Parent bookmark ID: $bookmark_id | Stars: $(echo "$bookmark" | jq -r '.description' | grep -oP 'Stars: \K\d+' || echo 'N/A')"

    # Import to bkmr
    echo "  Importing README to bkmr..."

    # Create temporary file for README content
    temp_readme=$(mktemp)
    echo "$readme_content" > "$temp_readme"

    # Import with or without embeddings
    if [[ "$NO_EMBED" == "true" ]]; then
        # Import without embeddings
        bkmr add "$temp_readme" \
            $readme_tags \
            --title "$readme_title" \
            --description "$readme_desc" \
            --type md || {
            echo -e "${RED}  Failed to import README${NC}"
            ((failed++))
            rm -f "$temp_readme"
            continue
        }
    else
        # Import with embeddings
        bkmr --openai add "$temp_readme" \
            $readme_tags \
            --title "$readme_title" \
            --description "$readme_desc" \
            --type md || {
            echo -e "${RED}  Failed to import README with embeddings${NC}"
            ((failed++))
            rm -f "$temp_readme"
            continue
        }

        # Enable embeddable flag
        new_id=$(bkmr search --json | jq -r --arg title "$readme_title" '.[] |
            select(.title == $title) | .id' | head -1)

        if [[ -n "$new_id" ]]; then
            bkmr set-embeddable "$new_id" --enable || echo -e "${YELLOW}  Warning: Could not enable embeddable${NC}"
        fi
    fi

    rm -f "$temp_readme"
    echo -e "${GREEN}  ✓ Imported README ($(echo "$readme_content" | wc -l) lines)${NC}"
    ((processed++))

    # Rate limiting - sleep between requests
    sleep 1
done

# Summary
echo ""
echo -e "${GREEN}================== Summary ==================${NC}"
echo "Total repositories: $count"
echo "Successfully processed: $processed"
echo "Skipped (already exists): $skipped"
echo "Failed: $failed"
echo ""

if [[ "$NO_EMBED" == "false" ]] && [[ "$processed" -gt 0 ]]; then
    echo -e "${BLUE}Embeddings were generated during import${NC}"
    echo "To backfill any missing embeddings:"
    echo "  bkmr --openai backfill"
fi

echo ""
echo "Search your imported READMEs:"
echo "  bkmr search -t readme"
echo "  bkmr search --fzf -t readme,language-rust"
echo "  bkmr --openai sem-search \"your query here\" -t readme"
