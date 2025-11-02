#!/usr/bin/env bash
# import-github-stars.sh - Import GitHub starred repositories to bkmr
# Usage: ./import-github-stars.sh <github-username> [options]
#
# Options:
#   --year YEAR          Filter repos updated in YEAR (default: current year)
#   --include-archived   Include archived repositories
#   --update            Update existing bookmarks
#   --output FILE       Output file (default: github-stars-import.sh)
#   --dry-run           Show what would be imported without executing
#
# Requirements:
#   - gh (GitHub CLI) authenticated
#   - jq for JSON processing
#   - bkmr installed and configured

set -euo pipefail

# Configuration
GITHUB_USER="${1:-}"
FILTER_YEAR="${FILTER_YEAR:-$(date +%Y)}"
INCLUDE_ARCHIVED="${INCLUDE_ARCHIVED:-false}"
UPDATE_MODE="${UPDATE_MODE:-false}"
OUTPUT_FILE="${OUTPUT_FILE:-github-stars-import.sh}"
DRY_RUN="${DRY_RUN:-false}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Usage
usage() {
    cat << EOF
Usage: $0 <github-username> [options]

Options:
    --year YEAR          Filter repos updated in YEAR (default: $FILTER_YEAR)
    --include-archived   Include archived repositories
    --update            Update existing bookmarks
    --output FILE       Output file (default: $OUTPUT_FILE)
    --dry-run           Show what would be imported without executing

Examples:
    $0 danielbodnar
    $0 danielbodnar --year 2025 --output stars-2025.sh
    $0 danielbodnar --include-archived --update

Requirements:
    - gh CLI authenticated: gh auth login
    - jq: brew install jq or apt install jq
    - bkmr: cargo install bkmr
EOF
    exit 1
}

# Parse arguments
[[ -z "$GITHUB_USER" ]] && usage
shift
while [[ $# -gt 0 ]]; do
    case $1 in
        --year)
            FILTER_YEAR="$2"
            shift 2
            ;;
        --include-archived)
            INCLUDE_ARCHIVED=true
            shift
            ;;
        --update)
            UPDATE_MODE=true
            shift
            ;;
        --output)
            OUTPUT_FILE="$2"
            shift 2
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

[[ -z "$GITHUB_USER" ]] && usage

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

echo -e "${GREEN}Fetching starred repositories for $GITHUB_USER...${NC}"

# Fetch starred repositories
stars_json=$(gh api "/users/$GITHUB_USER/starred" \
    --paginate \
    --jq '[.[] | {
        full_name,
        name,
        owner: .owner.login,
        description,
        html_url,
        homepage,
        topics,
        language,
        stargazers_count,
        updated_at,
        archived,
        pushed_at
    }]')

echo -e "${GREEN}Fetched $(echo "$stars_json" | jq length) starred repositories${NC}"

# Filter repositories
filtered_json=$(echo "$stars_json" | jq --arg year "$FILTER_YEAR" --argjson include_archived "$INCLUDE_ARCHIVED" '
    [.[] |
        select(
            (.updated_at | startswith($year)) and
            (if $include_archived then true else .archived == false end)
        )
    ]
')

count=$(echo "$filtered_json" | jq length)
echo -e "${GREEN}Filtered to $count repositories (year: $FILTER_YEAR, include_archived: $INCLUDE_ARCHIVED)${NC}"

if [[ "$count" -eq 0 ]]; then
    echo -e "${YELLOW}No repositories match the filter criteria${NC}"
    exit 0
fi

# Generate bkmr import commands
echo -e "${GREEN}Generating bkmr import commands...${NC}"

cat > "$OUTPUT_FILE" << 'SCRIPT_HEADER'
#!/usr/bin/env bash
# Generated GitHub stars import script
# Generated: $(date)
# Source: import-github-stars.sh

set -euo pipefail

echo "Importing GitHub starred repositories to bkmr..."

SCRIPT_HEADER

# Generate import command for each repo
echo "$filtered_json" | jq -r '.[] |
    @json "\(.)"
' | while read -r repo_json; do
    # Extract fields
    full_name=$(echo "$repo_json" | jq -r '.full_name')
    name=$(echo "$repo_json" | jq -r '.name')
    owner=$(echo "$repo_json" | jq -r '.owner')
    description=$(echo "$repo_json" | jq -r '.description // empty')
    html_url=$(echo "$repo_json" | jq -r '.html_url')
    homepage=$(echo "$repo_json" | jq -r '.homepage // empty')
    topics=$(echo "$repo_json" | jq -r '.topics[]' | tr '\n' ',' | sed 's/,$//')
    language=$(echo "$repo_json" | jq -r '.language // empty')
    stars=$(echo "$repo_json" | jq -r '.stargazers_count')
    updated_at=$(echo "$repo_json" | jq -r '.updated_at')
    archived=$(echo "$repo_json" | jq -r '.archived')

    # Build tags
    tags="github"
    [[ -n "$topics" ]] && tags="$tags,$topics"
    [[ -n "$language" ]] && tags="$tags,language-${language,,}"  # Lowercase
    tags="$tags,repo-${name}"
    tags="$tags,author-${owner}"

    # Add status tag
    if [[ "$archived" == "true" ]]; then
        tags="$tags,status-archived"
    else
        tags="$tags,status-active"
    fi

    # Add year tag
    year=$(echo "$updated_at" | cut -d'-' -f1)
    tags="$tags,year-$year"

    # Add stars tier tag
    if [[ "$stars" -gt 10000 ]]; then
        tags="$tags,stars-10k"
    elif [[ "$stars" -gt 1000 ]]; then
        tags="$tags,stars-1k"
    elif [[ "$stars" -gt 100 ]]; then
        tags="$tags,stars-100"
    fi

    # Build description
    desc_text="$description"
    [[ -n "$homepage" ]] && desc_text="$desc_text | Homepage: $homepage"
    desc_text="$desc_text | Stars: $stars | Updated: ${updated_at:0:10}"

    # Generate bkmr command
    cat >> "$OUTPUT_FILE" << EOF

# $full_name
echo "Importing: $full_name"
bkmr add "$html_url" \\
  $tags \\
  --title "$full_name - ${description:0:60}" \\
  --description "$desc_text" || echo "Failed to import $full_name"

EOF
done

# Add summary
cat >> "$OUTPUT_FILE" << 'SCRIPT_FOOTER'

echo "Import complete!"
echo "Total bookmarks: $(bkmr search -t github --np | wc -l)"
echo ""
echo "Next steps:"
echo "  1. Enable embeddings: for id in \$(bkmr search -t github --np); do bkmr set-embeddable \$id --enable; done"
echo "  2. Backfill embeddings: bkmr --openai backfill"
echo "  3. Fetch READMEs: ./fetch-readme-embeddings.sh"
SCRIPT_FOOTER

chmod +x "$OUTPUT_FILE"

echo -e "${GREEN}Generated import script: $OUTPUT_FILE${NC}"
echo -e "${GREEN}Generated $count import commands${NC}"
echo ""
echo "Next steps:"
echo "  1. Review: cat $OUTPUT_FILE | less"
echo "  2. Execute: ./$OUTPUT_FILE"
echo "  3. Or execute now: bash $OUTPUT_FILE"
echo ""

if [[ "$DRY_RUN" == "false" ]]; then
    read -p "Execute import now? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${GREEN}Executing import...${NC}"
        bash "$OUTPUT_FILE"
    fi
else
    echo -e "${YELLOW}Dry-run mode: Generated script but not executing${NC}"
fi
