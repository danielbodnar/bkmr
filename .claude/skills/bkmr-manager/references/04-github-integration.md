# GitHub Integration and Stars Import

This document covers importing GitHub starred repositories into bkmr with comprehensive metadata and semantic search.

## Overview

Import your GitHub stars into bkmr to create a searchable, AI-powered knowledge base of curated repositories. This integration maps GitHub metadata to bkmr's tagging and search system.

## Workflow Overview

```
GitHub API → Filter & Transform → bkmr Import → Enable Embeddings → Fetch READMEs
```

## Metadata Mapping Strategy

### GitHub Fields → bkmr Tags

```
Topics:       [rust, cli, bookmark]    → rust,cli,bookmark
Language:     Rust                      → language-rust
Repo Name:    sysid/bkmr               → repo-bkmr
Owner:        sysid                     → author-sysid
Stars:        100+                      → popular
Updated:      2025                      → active-2025
Archived:     false                     → status-active
```

### Tag Prefixes for Organization

Use consistent prefixes for categorization:

| Prefix | Purpose | Examples |
|--------|---------|----------|
| `language-*` | Programming language | `language-rust`, `language-python` |
| `author-*` | Repository owner | `author-sysid`, `author-microsoft` |
| `repo-*` | Repository name | `repo-bkmr`, `repo-kubernetes` |
| `status-*` | Maintenance status | `status-active`, `status-archived` |
| `stars-*` | Popularity tier | `stars-100`, `stars-1k`, `stars-10k` |
| `year-*` | Last update year | `year-2025`, `year-2024` |

### Example Complete Mapping

```json
{
  "full_name": "sysid/bkmr",
  "description": "Fast bookmark and snippet manager",
  "html_url": "https://github.com/sysid/bkmr",
  "homepage": "https://sysid.github.io/bkmr-reborn/",
  "topics": ["rust", "cli", "bookmark", "snippet"],
  "language": "Rust",
  "stargazers_count": 127,
  "updated_at": "2025-10-15T10:30:00Z",
  "archived": false
}
```

**Maps to bkmr:**
```bash
bkmr add https://github.com/sysid/bkmr \
  rust,cli,bookmark,snippet,language-rust,repo-bkmr,author-sysid,status-active,stars-100,year-2025 \
  --title "sysid/bkmr - Fast bookmark manager" \
  --description "Fast bookmark and snippet manager | Homepage: https://sysid.github.io/bkmr-reborn/"
```

## Step-by-Step Import Process

### Step 1: Fetch GitHub Stars

Use the GitHub API to fetch starred repositories:

```bash
# Authenticate with GitHub CLI
gh auth status

# Fetch stars (via script)
./scripts/import-github-stars.sh <github-username>
```

**Script behavior:**
1. Fetches all starred repos via GitHub API
2. Filters by:
   - Updated in current year (2025)
   - Not archived
3. Extracts metadata:
   - Topics, language, name, owner
   - Description, homepage, URL
   - Stars count, update date
4. Generates bkmr import commands

### Step 2: Import to bkmr

The script generates import commands:

```bash
# Output saved to: github-stars-import.sh
chmod +x github-stars-import.sh

# Review before importing
head -20 github-stars-import.sh

# Execute import
./github-stars-import.sh

# Or import individually
bkmr add https://github.com/user/repo \
  topic1,topic2,language-rust,author-user \
  --title "user/repo - Description"
```

### Step 3: Enable Semantic Search

```bash
# Mark all GitHub stars as embeddable
for id in $(bkmr search -t github --np); do
    bkmr set-embeddable "$id" --enable
done

# Verify count
bkmr search -t github --json | jq '[.[] | select(.embeddable == true)] | length'

# Generate embeddings
bkmr --openai backfill

# Monitor progress
bkmr --openai backfill --dry-run  # Check what needs processing
```

### Step 4: Fetch and Embed READMEs

```bash
# Fetch README files from starred repos
./scripts/fetch-readme-embeddings.sh

# Or manually for specific repos
./scripts/fetch-readme-embeddings.sh --filter language-rust --limit 50
```

**Script behavior:**
1. Queries bkmr for GitHub star bookmarks
2. Fetches README.md from each repository
3. Converts to markdown content
4. Creates new bkmr bookmark with:
   - Content: README markdown
   - Tags: Original tags + `readme` + `_md_`
   - Title: "[Repo Name] README"
   - Linked to original bookmark
5. Generates embeddings with `--openai`

## Filtering and Querying Strategies

### By Language

```bash
# Find all Rust projects
bkmr search -t language-rust

# Find Rust CLI tools
bkmr search -t language-rust,cli

# Fuzzy find Python projects
bkmr search --fzf -t language-python
```

### By Topic

```bash
# Find all projects tagged with "kubernetes"
bkmr search -t kubernetes

# Combined topics
bkmr search -t docker,container

# Semantic search for topics
bkmr --openai sem-search "container orchestration"
```

### By Author

```bash
# Find all repos by specific author
bkmr search -t author-microsoft

# Multiple authors
bkmr search -T author-google,author-facebook
```

### By Status

```bash
# Active projects (updated recently)
bkmr search -t status-active

# By year
bkmr search -t year-2025
```

### By Popularity

```bash
# Popular projects (100+ stars)
bkmr search -t stars-100

# Highly popular (1000+ stars)
bkmr search -t stars-1k

# Mega popular (10k+ stars)
bkmr search -t stars-10k
```

### Semantic Queries

```bash
# Find authentication libraries
bkmr --openai sem-search "authentication and authorization libraries"

# Find CLI frameworks
bkmr --openai sem-search "command line interface frameworks"

# Find async patterns
bkmr --openai sem-search "asynchronous programming patterns"

# Cross-language searches
bkmr --openai sem-search "dependency injection patterns"
# Returns examples from Rust, Python, TypeScript, etc.
```

## README Embedding Strategy

### Why Embed READMEs?

READMEs contain:
- Project purpose and goals
- Architecture and design
- Usage examples and patterns
- API documentation
- Best practices

Embedding READMEs enables semantic search across project documentation.

### README Linking

```bash
# Original star bookmark
ID: 100 | sysid/bkmr | Tags: rust,cli,language-rust
URL: https://github.com/sysid/bkmr

# README bookmark (linked)
ID: 101 | sysid/bkmr README | Tags: rust,cli,language-rust,readme,_md_
Content: [Full README markdown]
Link: parent_id=100
```

**Benefits:**
- Semantic search finds implementation details
- Both URL and README searchable
- Linked bookmarks for context
- README rendered in browser

### Selective README Fetching

```bash
# Fetch READMEs for specific subset
./scripts/fetch-readme-embeddings.sh \
  --filter language-rust \
  --limit 100

# Fetch for popular repos only
./scripts/fetch-readme-embeddings.sh \
  --filter stars-1k \
  --limit 50

# Fetch for specific topics
./scripts/fetch-readme-embeddings.sh \
  --filter cli,terminal \
  --limit 75
```

## Maintenance and Updates

### Re-Import Updated Stars

```bash
# Fetch latest stars
./scripts/import-github-stars.sh <username> --update

# Updates:
# ✅ New stars added
# ✅ Removed stars (unstars) deleted
# ✅ Updated metadata (description, topics)
# ✅ Star count updated
```

### Refresh READMEs

```bash
# Re-fetch READMEs for updated repos
./scripts/fetch-readme-embeddings.sh --force

# Only update changed READMEs
./scripts/fetch-readme-embeddings.sh --update
```

### Cleanup Archived/Deleted Repos

```bash
# Find and remove archived repos
bkmr search -t status-archived
bkmr delete $(bkmr search -t status-archived --np)

# Find repos with broken links (manual check)
bkmr search -t github | grep "404"
```

## Advanced GitHub Workflows

### Track Repository Updates

```bash
# Add custom tag for review queue
bkmr update -t needs-review $(bkmr search -t year-2025,status-active --np)

# Review and mark as reviewed
bkmr search --fzf -t needs-review
# After review: bkmr update -t reviewed -n needs-review <id>
```

### Categorize by Project Type

```bash
# Tag by type
bkmr update -t type-library <id>
bkmr update -t type-tool <id>
bkmr update -t type-framework <id>

# Find by type
bkmr search -t type-library,language-rust
```

### Create Reading Lists

```bash
# Tag for learning queue
bkmr update -t to-read,priority-high <id>

# Find learning materials
bkmr search --fzf -t to-read
```

### Extract Learning Resources

```bash
# Find tutorials and guides
bkmr --openai sem-search "beginner guide" -t language-python

# Find architecture examples
bkmr --openai sem-search "clean architecture implementation"

# Find testing resources
bkmr --openai sem-search "testing strategies and patterns"
```

## Integration with Development Workflow

### IDE Integration

```bash
# Generate shell stubs for quick access
source <(bkmr search --shell-stubs)

# Access repos as commands
repo-bkmr  # Opens https://github.com/sysid/bkmr
```

### Quick Reference

```bash
# Alias for quick GitHub search
alias ghf='bkmr search --fzf -t github'

# Search for implementation examples
alias find-impl='bkmr --openai sem-search --limit 10 -t github'

# Usage:
ghf  # Interactive GitHub stars browser
find-impl "rate limiting implementation"
```

### Project Discovery

```bash
# Find projects by use case
bkmr --openai sem-search "REST API framework for Rust"
bkmr --openai sem-search "static site generator"
bkmr --openai sem-search "terminal UI library"

# Find by technology stack
bkmr search -t language-typescript,framework
bkmr search -t language-rust,web
```

## API Rate Limiting

### GitHub API Limits

- Unauthenticated: 60 requests/hour
- Authenticated: 5000 requests/hour

**Best practices:**
```bash
# Authenticate with GitHub CLI
gh auth login

# Use authenticated requests (automatic with gh CLI)
./scripts/import-github-stars.sh <username>

# Monitor rate limit
gh api rate_limit
```

### OpenAI API Limits

- Rate limits vary by tier
- Monitor usage in OpenAI dashboard

**Throttle requests:**
```bash
# Process in batches
for id in $(bkmr search -t github --np | head -100); do
    bkmr set-embeddable "$id" --enable
done

bkmr --openai backfill

# Wait between batches if needed
sleep 60
```

## Cost Estimation

### GitHub API
- Free with authentication
- No cost for star imports

### OpenAI Embeddings
- `text-embedding-ada-002`: $0.0001 per 1K tokens
- Average README: ~1000 tokens
- 1000 starred repos: ~$0.10 for embeddings

**Calculate:**
```bash
# Count stars to import
gh api user/starred --paginate | jq length

# Estimate: count * $0.0001
```

## Examples

### Complete Import Session

```bash
# 1. Fetch and import stars
./scripts/import-github-stars.sh danielbodnar

# 2. Enable embeddings
for id in $(bkmr search -t github,year-2025 --np); do
    bkmr set-embeddable "$id" --enable
done

# 3. Generate embeddings
bkmr --openai backfill

# 4. Fetch READMEs for top projects
./scripts/fetch-readme-embeddings.sh --filter stars-100 --limit 100

# 5. Search semantically
bkmr --openai sem-search "CLI tools for developers"
```

### Selective Import

```bash
# Import only Rust projects starred this year
./scripts/import-github-stars.sh danielbodnar \
  --filter language:Rust \
  --year 2025

# Import only specific topics
./scripts/import-github-stars.sh danielbodnar \
  --filter topic:kubernetes OR topic:docker
```

## Troubleshooting

### Authentication Failures

```bash
# Verify GitHub authentication
gh auth status

# Re-authenticate if needed
gh auth login

# Test API access
gh api user/starred --limit 1
```

### Import Script Errors

```bash
# Debug mode
bash -x ./scripts/import-github-stars.sh danielbodnar

# Check JSON output
cat github-stars-import.sh | head -20

# Validate format
bkmr add --help  # Verify syntax
```

### README Fetch Failures

```bash
# Check network connectivity
curl -I https://api.github.com

# Verify repo access
gh api repos/sysid/bkmr/readme

# Check for missing READMEs
# Some repos don't have README.md - script handles gracefully
```

## Best Practices

### 1. Regular Syncs

```bash
# Weekly update
./scripts/import-github-stars.sh <username> --update

# Cron job for automatic sync
0 0 * * 0 /path/to/import-github-stars.sh <username> --update
```

### 2. Selective Embedding

```bash
# Enable embeddings only for documentation-heavy repos
for id in $(bkmr search -t github,docs --np); do
    bkmr set-embeddable "$id" --enable
done

# Skip simple tools and utilities
```

### 3. Tag Hygiene

```bash
# Review and consolidate tags
bkmr tags | grep language-

# Bulk tag updates
for id in $(bkmr search -t github,javascript --np); do
    bkmr update -n javascript -t language-javascript "$id"
done
```

### 4. Curate Collections

```bash
# Create curated lists
bkmr update -t collection-rust-async $(bkmr search -t language-rust,async --np)
bkmr update -t collection-web-frameworks $(bkmr search -t web,framework --np)

# Find collections
bkmr search -t collection-rust-async
```

### 5. Backup Before Large Imports

```bash
# Backup database
cp ~/.config/bkmr/bkmr.db ~/.config/bkmr/bkmr_backup_$(date +%Y%m%d).db

# Import stars
./scripts/import-github-stars.sh <username>

# Verify
bkmr search -t github | wc -l
```

## Advanced Queries

### Cross-Reference Searches

```bash
# Find repos by author and topic
bkmr search -t author-microsoft,language-typescript

# Find active Rust projects
bkmr search -t language-rust,status-active,year-2025

# Popular Python tools
bkmr search -t language-python,stars-1k
```

### Semantic Discovery

```bash
# Find authentication solutions
bkmr --openai sem-search "OAuth2 and JWT authentication libraries"

# Find testing frameworks
bkmr --openai sem-search "testing frameworks with mocking support"

# Find data processing tools
bkmr --openai sem-search "ETL and data pipeline tools"

# Find observability solutions
bkmr --openai sem-search "monitoring and logging infrastructure"
```

### Combined Filters

```bash
# Semantic search within language
bkmr --openai sem-search "web framework" -t language-rust

# Semantic search for popular projects
bkmr --openai sem-search "database client" -t stars-1k

# Fuzzy find with semantic pre-filter
bkmr --openai sem-search "CLI framework" --limit 20 | \
  xargs -I {} bkmr open {}
```

## Automation Examples

### Nightly Star Import

```bash
#!/bin/bash
# cron-github-sync.sh

GITHUB_USER="danielbodnar"
BKMR_DB="$HOME/.config/bkmr/bkmr.db"

export BKMR_DB_URL="$BKMR_DB"

# Backup database
cp "$BKMR_DB" "${BKMR_DB}.backup"

# Import new stars
/path/to/import-github-stars.sh "$GITHUB_USER" --update

# Enable embeddings for new entries
for id in $(bkmr search -t github,year-2025 --np | tail -50); do
    bkmr set-embeddable "$id" --enable
done

# Backfill embeddings
bkmr --openai backfill

# Log results
echo "$(date): Synced $(bkmr search -t github --np | wc -l) GitHub stars"
```

**Crontab entry:**
```cron
# Daily at 2 AM
0 2 * * * /path/to/cron-github-sync.sh >> /var/log/bkmr-sync.log 2>&1
```

### Curated Lists Generation

```bash
#!/bin/bash
# generate-curated-lists.sh

# Rust CLI tools collection
bkmr search -t language-rust,cli --np | while read id; do
    bkmr update -t collection-rust-cli "$id"
done

# Web frameworks collection
bkmr search -t framework,web --np | while read id; do
    bkmr update -t collection-web-frameworks "$id"
done

# Testing tools collection
bkmr --openai sem-search "testing frameworks" --limit 50 --np | while read id; do
    bkmr update -t collection-testing "$id"
done
```

### Export for Sharing

```bash
# Export GitHub stars collection
bkmr search -t github --json > github-stars-export.json

# Share with team
scp github-stars-export.json team-server:/shared/bookmarks/

# Import on another machine
cat github-stars-export.json | jq -r '.[] |
  "bkmr add \(.url) \(.tags | join(",")) --title \"\(.title)\""
' | bash
```

## Performance Optimization

### Batch Processing

```bash
# Process in chunks to avoid memory issues
total=$(bkmr search -t github --np | wc -l)
chunk_size=100

for ((i=0; i<total; i+=chunk_size)); do
    echo "Processing chunk $((i/chunk_size + 1))"
    bkmr search -t github --np | tail -n +$((i+1)) | head -n $chunk_size | while read id; do
        bkmr set-embeddable "$id" --enable
    done
    bkmr --openai backfill
    sleep 10  # Rate limiting
done
```

### Parallel Processing (Carefully)

```bash
# Parallel README fetches (respect rate limits)
bkmr search -t github --np | head -100 | \
  xargs -P 5 -I {} ./scripts/fetch-single-readme.sh {}

# Note: Built-in rate limiting in scripts
```

## Monitoring and Analytics

### Import Statistics

```bash
# Count by language
bkmr tags | grep ^language-

# Count by year
bkmr tags | grep ^year-

# Popular repos
bkmr search -t stars-1k --np | wc -l

# Total GitHub bookmarks
bkmr search -t github --np | wc -l
```

### Embedding Coverage

```bash
# Count embeddable bookmarks
bkmr search --json | jq '[.[] | select(.embeddable == true)] | length'

# Percentage with embeddings
total=$(bkmr search -t github --np | wc -l)
embedded=$(bkmr search --json -t github | jq '[.[] | select(.embeddable == true)] | length')
echo "Coverage: $((embedded * 100 / total))%"
```

### Quality Metrics

```bash
# Find bookmarks without descriptions
bkmr search --json | jq '.[] | select(.description == "") | .id'

# Find bookmarks with few tags
bkmr search --json | jq '.[] | select((.tags | split(",") | length) < 3) | .id'

# Find bookmarks needing review
bkmr search -n reviewed -t github
```
