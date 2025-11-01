# Advanced Workflows

This document covers power-user workflows and advanced bkmr techniques.

## Template Interpolation

Make content dynamic with Jinja2-style templates using `minijinja`.

### Date and Time

```bash
# Current date
bkmr add "Backup {{ current_date | strftime('%Y-%m-%d') }}" backup,_shell_

# Timestamp
bkmr add "Log {{ current_date | strftime('%Y%m%d_%H%M%S') }}" logging

# Relative dates (requires template logic)
bkmr add "Report from {{ current_date | strftime('%Y-%m-01') }}" reports
```

### Environment Variables

```bash
# User-specific URL
bkmr add "https://dashboard.company.com/users/{{ env('USER') }}" dashboard

# With fallback
bkmr add "API={{ env('API_KEY', 'default-key') }}" config,_env_

# Nested paths
bkmr add "{{ env('HOME') }}/.config/app/config.yml" config
```

### Shell Commands

```bash
# Git branch
bkmr add 'Branch: {{ "git branch --show-current" | shell }}' git,_snip_

# Hostname
bkmr add 'Host: {{ "hostname" | shell }}' system,_snip_

# IP address
bkmr add 'IP: {{ "hostname -I | cut -d\" \" -f1" | shell }}' network,_snip_

# Current directory
bkmr add 'PWD: {{ "pwd" | shell }}' system
```

### Conditional Logic

```bash
# Environment-based configuration
bkmr add '{% if env("ENVIRONMENT") == "production" %}
export DB_URL=prod-db.example.com
{% else %}
export DB_URL=localhost:5432
{% endif %}' env,_env_

# User-based paths
bkmr add '{% if env("USER") == "admin" %}
/var/log/system.log
{% else %}
/var/log/user.log
{% endif %}' logs
```

### Loops and Iteration

```bash
# Generate multiple exports
bkmr add '{% for port in [8000, 8001, 8002] %}
export SERVICE_{{ loop.index }}_PORT={{ port }}
{% endfor %}' env,_env_

# Multiple hosts
bkmr add '{% for host in ["web1", "web2", "web3"] %}
ping -c 1 {{ host }}.example.com
{% endfor %}' monitoring,_shell_
```

## When Templates Interpolate

**Automatic (No Flags):**
- ✅ FZF mode: `bkmr search --fzf`
- ✅ Open action: `bkmr open <id>`
- ✅ Yank action: `bkmr yank <id>`

**Manual (Flag Required):**
- Search results: `bkmr search --interpolate "backup"`

**Example:**
```bash
# Without interpolation (shows template)
bkmr search "backup"
# Output: URL: Backup {{ current_date | strftime('%Y-%m-%d') }}

# With interpolation (shows rendered)
bkmr search --interpolate "backup"
# Output: URL: Backup 2025-10-31
```

## Shell Function Stubs

Generate shell functions for quick access to bookmarked scripts:

```bash
# Generate and source stubs
source <(bkmr search --shell-stubs)

# Now use scripts as functions
backup-database production --incremental
deploy-app staging --rollback
monitoring-status --verbose
```

**How it works:**
1. Generates shell function for each `_shell_` bookmark
2. Function name from title (sanitized)
3. Function calls `bkmr open <id> -- "$@"`
4. Arguments passed through to script

**Example generated function:**
```bash
backup-database() {
    bkmr open 123 --no-edit -- "$@"
}
```

### Selective Stub Generation

```bash
# Generate stubs for specific tags
source <(bkmr search --shell-stubs -t production,deploy)

# Generate for project-specific scripts
source <(bkmr search --shell-stubs -t project-myapp)
```

## Advanced Search Patterns

### Column-Specific Search

```bash
# Search only URLs
bkmr search "url:github"

# Search only titles
bkmr search "metadata:docker"

# Search descriptions
bkmr search "desc:authentication"

# Search tags
bkmr search "tags:python"

# Combined
bkmr search "tags:docker desc:compose"
```

### Complex Tag Queries

```bash
# Must have ALL these tags (AND)
bkmr search -t python,async,web

# Must have ANY of these tags (OR) - use separate searches
bkmr search -t python -o rust -o go  # Not supported - use:
bkmr search -t python
bkmr search -t rust
bkmr search -t go

# Must NOT have these tags
bkmr search -t python -n deprecated,old

# Combine include and exclude
bkmr search -t python,web -n django,flask "authentication"
```

### Sort and Limit

```bash
# Most recent
bkmr search --descending --limit 10

# Oldest
bkmr search --ascending --limit 10

# Recent Python bookmarks
bkmr search -t python --descending --limit 5

# Find duplicates (same title)
bkmr search --json | jq -r '.[] | .title' | sort | uniq -d
```

## JSON Output and Scripting

### JSON Export

```bash
# JSON output
bkmr search --json "python"

# Pretty print
bkmr search --json "python" | jq .

# Extract fields
bkmr search --json "python" | jq '.[] | {title, url, tags}'

# Filter in jq
bkmr search --json | jq '.[] | select(.tags | contains("_snip_"))'
```

### Scripting Workflows

```bash
# Get IDs only (no-print mode)
bkmr search -t needs-update --np

# Process IDs in loop
for id in $(bkmr search -t needs-update --np); do
    echo "Processing bookmark $id"
    bkmr update -t updated -n needs-update "$id"
done

# Pipeline processing
bkmr search -t github --np | \
  head -50 | \
  xargs -I {} bkmr set-embeddable {} --enable
```

### Data Analysis

```bash
# Count by tag
bkmr tags | awk '{print $2}' | sort -rn | head -10

# Most popular tags
bkmr search --json | jq -r '.[] | .tags' | \
  tr ',' '\n' | sort | uniq -c | sort -rn | head -20

# Language distribution
bkmr search --json | jq -r '.[] | .tags' | \
  grep -o 'language-[^,]*' | sort | uniq -c
```

## Bulk Operations

### Tag Management

```bash
# Add tag to multiple bookmarks
bkmr update -t production $(bkmr search -t deploy,app --np)

# Remove deprecated tags
for id in $(bkmr search -t deprecated --np); do
    bkmr update -n deprecated -t archived "$id"
done

# Replace tags across collection
for id in $(bkmr search -t javascript --np); do
    bkmr update -n javascript -t language-javascript "$id"
done
```

### Content Updates

```bash
# Bulk description update (requires scripting)
bkmr search -t github --json | jq -r '.[] |
  "bkmr update --description \"Updated: \(.description)\" \(.id)"
' | bash

# Add prefix to titles
bkmr search -t project-myapp --json | jq -r '.[] |
  "bkmr update --title \"[MyApp] \(.title)\" \(.id)"
' | bash
```

### Migration Workflows

```bash
# Export from old system
old-bookmark-tool export --json > old-bookmarks.json

# Transform to bkmr format
cat old-bookmarks.json | jq -r '.[] |
  "bkmr add \"\(.url)\" \(.tags | join(",")) --title \"\(.title)\""
' > import-commands.sh

# Review and import
chmod +x import-commands.sh
./import-commands.sh

# Verify import
bkmr search --json | jq length
```

## Integration with External Tools

### Browser Integration

```bash
# Quick add from browser (bookmark script)
javascript:location.href='bkmr://add?url='+encodeURIComponent(location.href)+'&title='+encodeURIComponent(document.title)

# Or use curl for browser extension
curl -X POST http://localhost:8080/api/add \
  -d "url=$(xclip -o)" \
  -d "tags=web,reference"
```

### Editor Integration

```bash
# LSP server for snippet completion
bkmr lsp

# Configure in editor (VSCode settings.json)
{
  "bkmr.lsp.enabled": true,
  "bkmr.lsp.port": 7777
}
```

### Terminal Multiplexer

```bash
# Tmux integration
bind-key b run-shell "bkmr search --fzf"

# Zellij integration
bind "Ctrl b" { run "bkmr search --fzf"; }
```

## Performance Optimization

### Indexing

```bash
# Full-text search uses SQLite FTS5
# Automatically indexed on:
# - URL/content
# - Title
# - Description
# - Tags

# Rebuild index if needed (automatic via migrations)
```

### Query Optimization

```bash
# Tag filtering is faster than FTS
bkmr search -t python "async"  # Fast
bkmr search "python async"     # Slower

# Limit results early
bkmr search --limit 10 "common-term"

# Use fuzzy finder with pre-filter
bkmr search --fzf -t python  # Pre-filtered
```

### Embedding Performance

```bash
# Batch embeddings
bkmr --openai backfill  # Processes all at once

# Avoid individual embedding calls
# Bad: Loop with individual bkmr --openai add calls
# Good: Mark as embeddable, then backfill
```

## Multi-Machine Sync

### Export/Import Strategy

```bash
# Machine 1: Export
bkmr search --json > bkmr-export-$(date +%Y%m%d).json

# Transfer to Machine 2
scp bkmr-export-*.json machine2:~/

# Machine 2: Import
cat bkmr-export-*.json | jq -r '.[] |
  "bkmr add \"\(.url)\" \(.tags | join(",")) --title \"\(.title)\" --description \"\(.description)\""
' | bash
```

### Database Sync

```bash
# Copy database directly
rsync -avz ~/.config/bkmr/bkmr.db machine2:~/.config/bkmr/

# Or use cloud sync (Dropbox, Syncthing)
export BKMR_DB_URL=~/Dropbox/bkmr/bkmr.db
```

### Base Path Portability

```bash
# Machine-specific config.toml
# Machine 1:
[base_paths]
SCRIPTS_HOME = "/home/user1/scripts"

# Machine 2:
[base_paths]
SCRIPTS_HOME = "/home/user2/scripts"

# Bookmarks use $SCRIPTS_HOME - work on both machines
```

## Custom Actions

### Define Custom Handlers

```bash
# Create handler script
cat > ~/.config/bkmr/handlers/open-in-vscode.sh << 'EOF'
#!/bin/bash
# Custom handler for code files
url="$1"
code "$url"
EOF

chmod +x ~/.config/bkmr/handlers/open-in-vscode.sh

# Use with custom tag
bkmr add "/path/to/project" code,_vscode_ --title "Project Workspace"
```

### URL Schemes

```bash
# Custom protocols
bkmr add "ssh://server.example.com" servers,_ssh_
bkmr add "rdp://desktop.example.com" remote,_rdp_

# Handler scripts process custom schemes
```

## Backup and Recovery

### Regular Backups

```bash
# Daily backup script
#!/bin/bash
BACKUP_DIR="$HOME/backups/bkmr"
DATE=$(date +%Y%m%d)

mkdir -p "$BACKUP_DIR"
cp ~/.config/bkmr/bkmr.db "$BACKUP_DIR/bkmr-$DATE.db"

# Keep last 30 days
find "$BACKUP_DIR" -name "bkmr-*.db" -mtime +30 -delete
```

### Restore from Backup

```bash
# List backups
ls -lh ~/backups/bkmr/

# Restore specific backup
cp ~/backups/bkmr/bkmr-20251015.db ~/.config/bkmr/bkmr.db

# Verify
bkmr search --json | jq length
```

### Export for Version Control

```bash
# Export as text (git-friendly)
bkmr search --json | jq -r '.[] |
  "[\(.id)] \(.title)\n  URL: \(.url)\n  Tags: \(.tags)\n"
' > bookmarks.txt

# Commit to git
git add bookmarks.txt
git commit -m "Bookmark snapshot $(date +%Y-%m-%d)"
```

## Analytics and Reporting

### Usage Statistics

```bash
# Most used tags
bkmr tags | sort -k2 -rn | head -20

# Content type distribution
echo "Snippets: $(bkmr search -t _snip_ --np | wc -l)"
echo "Scripts: $(bkmr search -t _shell_ --np | wc -l)"
echo "Docs: $(bkmr search -t _md_ --np | wc -l)"
echo "URLs: $(bkmr search -n _snip_,_shell_,_md_ --np | wc -l)"
```

### Quality Reports

```bash
# Bookmarks without descriptions
bkmr search --json | jq -r '.[] |
  select(.description == "") |
  "\(.id) - \(.title)"
' > needs-description.txt

# Bookmarks with single tags (need more categorization)
bkmr search --json | jq -r '.[] |
  select((.tags | split(",") | length) == 1) |
  "\(.id) - \(.title) - Tags: \(.tags)"
' > needs-tags.txt
```

### Trend Analysis

```bash
# Bookmarks added this month
current_month=$(date +%Y-%m)
bkmr search --json | jq -r ".[] |
  select(.created_at | startswith(\"$current_month\")) |
  .title
" | wc -l

# Most active tags this year
bkmr search --json | jq -r '.[] |
  select(.created_at | startswith("2025")) |
  .tags
' | tr ',' '\n' | sort | uniq -c | sort -rn | head -20
```

## Power User Aliases

### Comprehensive Alias Set

```bash
# Add to ~/.bashrc or ~/.zshrc

# Quick fuzzy search
alias b='bkmr search --fzf --fzf-style enhanced'

# Search by type
alias bs='bkmr search --fzf -t _snip_'       # Snippets
alias bsh='bkmr search --fzf -t _shell_'     # Scripts
alias bd='bkmr search --fzf -t _md_'         # Docs
alias bgh='bkmr search --fzf -t github'      # GitHub stars

# Quick add
alias ba='bkmr add'
alias bas='bkmr add --type snip'
alias bash='bkmr add --type shell'
alias bamd='bkmr add --type md'

# Recent items
alias br='bkmr search --descending --limit 10'
alias brt='bkmr search --descending --limit 10 -t'

# Semantic search
alias bsem='bkmr --openai sem-search'
alias bseml='bkmr --openai sem-search --limit'

# Tag management
alias bt='bkmr tags'
alias bts='bkmr search --fzf -t'

# Quick edit
alias be='bkmr edit'

# Statistics
alias bstats='echo "Total: $(bkmr search --np | wc -l)"; echo "Snippets: $(bkmr search -t _snip_ --np | wc -l)"; echo "Scripts: $(bkmr search -t _shell_ --np | wc -l)"; echo "Docs: $(bkmr search -t _md_ --np | wc -l)"'
```

### Advanced Functions

```bash
# Quick open first result
bko() {
    local id=$(bkmr search "$@" --np | head -1)
    [[ -n "$id" ]] && bkmr open "$id"
}

# Add snippet from clipboard
bsc() {
    pbpaste | bkmr add --stdin "$@" --type snip
    # Or for Linux: wl-paste | bkmr add --stdin "$@" --type snip
}

# Quick semantic search with limit
bsem() {
    bkmr --openai sem-search "$1" --limit "${2:-10}"
}

# Bulk tag operation
btag() {
    local tag="$1"
    shift
    for id in $(bkmr search "$@" --np); do
        bkmr update -t "$tag" "$id"
    done
}

# Find and edit
bfe() {
    local id=$(bkmr search --fzf "$@" --np)
    [[ -n "$id" ]] && bkmr edit "$id"
}
```

## Integration Workflows

### Git Repository Bookmarking

```bash
# Add current repo
bkmr add "$(git remote get-url origin)" \
  git,project,$(basename $(pwd)) \
  --title "$(basename $(pwd)) Repository"

# Add with README
readme_path="$(git rev-parse --show-toplevel)/README.md"
if [[ -f "$readme_path" ]]; then
    bkmr --openai add "$readme_path" \
      git,docs,$(basename $(pwd)),_md_ \
      --title "$(basename $(pwd)) README"
fi
```

### Documentation Aggregation

```bash
# Import all project documentation
find ~/repos -name "README.md" -o -name "CONTRIBUTING.md" | while read file; do
    project=$(basename $(dirname "$file"))
    bkmr import-files "$file" \
      --base-path GITHUB_REPOS \
      --type md
done

# Enable embeddings for docs
for id in $(bkmr search -t _md_,docs --np); do
    bkmr set-embeddable "$id" --enable
done

bkmr --openai backfill
```

### Code Snippet Library

```bash
# Extract code from bookmarked repos
gh repo list --starred --limit 1000 --json nameWithOwner | \
  jq -r '.[].nameWithOwner' | while read repo; do
    # Clone or update
    gh repo clone "$repo" ~/repos-cache/"$repo" -- --depth 1 || \
      git -C ~/repos-cache/"$repo" pull

    # Import interesting files
    fd -e rs -e py -e ts ~/repos-cache/"$repo" | \
      head -10 | \
      xargs bkmr import-files --base-path GITHUB_REPOS
done
```

### API Collection

```bash
# Store API endpoints
bkmr add "https://api.github.com/users/danielbodnar" \
  api,github,_snip_ \
  --title "GitHub User API"

# Store curl commands
bkmr add 'curl -H "Authorization: Bearer {{ env(\"API_TOKEN\") }}" \
  https://api.example.com/v1/users' \
  api,_shell_ \
  --title "Fetch Users"

# Quick API reference
alias api='bkmr search --fzf -t api'
```

## Database Maintenance

### Vacuum and Optimize

```bash
# Optimize database
sqlite3 ~/.config/bkmr/bkmr.db "VACUUM;"

# Rebuild FTS index
sqlite3 ~/.config/bkmr/bkmr.db "INSERT INTO bookmarks_fts(bookmarks_fts) VALUES('rebuild');"

# Analyze for query optimization
sqlite3 ~/.config/bkmr/bkmr.db "ANALYZE;"
```

### Clean Up

```bash
# Find orphaned entries
bkmr search --json | jq -r '.[] | select(.tags == "") | .id'

# Remove test entries
bkmr delete $(bkmr search -t test,temporary --np)

# Find and remove duplicates
bkmr search --json | jq -r 'group_by(.url) | .[] |
  select(length > 1) | .[1:] | .[].id
' | xargs -I {} bkmr delete {}
```

## Advanced Semantic Search

### Multi-Step Search Refinement

```bash
# 1. Broad semantic search
bkmr --openai sem-search "web development" --limit 50 --np > results.txt

# 2. Filter by language
for id in $(cat results.txt); do
    bkmr search --json | jq -r ".[] |
      select(.id == $id and (.tags | contains(\"language-typescript\"))) |
      .id"
done > typescript-web.txt

# 3. Final selection
cat typescript-web.txt | xargs -I {} bkmr open {}
```

### Concept Clustering

```bash
# Find related concepts
query="microservice patterns"
bkmr --openai sem-search "$query" --limit 20 --json | \
  jq -r '.[].tags' | tr ',' '\n' | sort | uniq -c | sort -rn

# Discover related topics through embeddings
```

### Cross-Lingual Searches

```bash
# Find similar implementations across languages
bkmr --openai sem-search "dependency injection container"

# Results might include:
# - Python: dependency-injector
# - Rust: shaku, waiter
# - TypeScript: tsyringe, inversify
# - Go: wire, dig
```

## Troubleshooting

### Performance Issues

```bash
# Large database (>10k bookmarks)
# - Optimize FTS: VACUUM and ANALYZE
# - Use specific tags before FTS
# - Limit results aggressively

# Slow semantic search
# - Reduce cardinality_limit
# - Use tag filters
# - Process in batches
```

### Memory Usage

```bash
# Large README embeddings
# - Process in batches (--limit)
# - Skip large files (>100KB)
# - Use selective embedding

# Monitor memory
bkmr --openai backfill &
watch -n 1 'ps aux | grep bkmr'
```

### Consistency Issues

```bash
# Re-import files if out of sync
bkmr import-files ~/scripts/ --base-path SCRIPTS_HOME --update --force

# Rebuild embeddings if corrupted
for id in $(bkmr search --json | jq -r '.[] | select(.embeddable == true) | .id'); do
    bkmr set-embeddable "$id" --disable
    bkmr set-embeddable "$id" --enable
done

bkmr --openai backfill
```

## Best Practices Summary

1. **Use tags hierarchically** - `language-rust,type-cli,topic-bookmarks`
2. **Template sparingly** - Only when content truly needs to be dynamic
3. **Batch embeddings** - Use `backfill` instead of individual calls
4. **Base paths for portability** - Always use base paths for file imports
5. **Regular backups** - Daily database backups
6. **Curate embeddable content** - Not everything needs embeddings
7. **Test with dry-run** - Always preview bulk operations
8. **Document workflows** - Save complex commands as shell bookmarks
9. **Monitor costs** - Track OpenAI API usage
10. **Clean regularly** - Remove deprecated and duplicate entries
