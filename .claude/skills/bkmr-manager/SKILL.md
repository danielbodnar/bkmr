---
name: bkmr-manager
description: Manage bookmarks, code snippets, shell scripts, and GitHub stars using bkmr CLI tool. Import GitHub starred repositories with metadata mapping (topics, language, description as tags), enable semantic search with OpenAI embeddings, fetch and embed README files, import files with frontmatter metadata, and organize knowledge bases. Use when user mentions bkmr, importing GitHub stars, bookmark management, snippet organization, semantic search, or building searchable knowledge bases.
---

# BKMR Manager Skill

**Version**: 1.0
**Type**: CLI Tool Integration
**Repository**: https://github.com/sysid/bkmr

## Overview

This skill enables advanced bookmark, snippet, and knowledge management workflows using `bkmr` - a fast, feature-rich CLI tool written in Rust. Specializes in importing GitHub stars with rich metadata, semantic search with embeddings, and intelligent file import with frontmatter.

## When to Use This Skill

Invoke this skill when the user wants to:
- Import GitHub stars or repositories into bkmr with full metadata
- Set up semantic search with OpenAI embeddings for their knowledge base
- Import and track markdown files, scripts, or code snippets
- Organize large collections of bookmarks with tags and full-text search
- Create a searchable personal knowledge base from various sources
- Automate bookmark management workflows

## Core Concepts (Essential)

### Bookmarks
In bkmr, a bookmark is any content: URLs, code snippets, shell scripts, markdown docs, or file references.

### Tags
- **User Tags**: Free-form labels (`python,web,tutorial`) for organization
- **System Tags**: Determine behavior:
  - `_snip_` - Code snippet → Copies to clipboard
  - `_shell_` - Shell script → Interactive execution
  - `_md_` - Markdown → Renders in browser
  - `_env_` - Environment vars → Prints for sourcing

### Content Actions
bkmr automatically selects the right action based on system tags:
```bash
bkmr open <id>  # Opens URL, copies snippet, executes script, or renders markdown
```

## Prerequisites

### Installation
```bash
# Via cargo
cargo install bkmr

# Via brew
brew install bkmr

# Via pip/pipx
pip install bkmr
```

### Initial Setup
```bash
# Create database
bkmr create-db ~/.config/bkmr/bkmr.db

# Set database location
export BKMR_DB_URL=~/.config/bkmr/bkmr.db

# For semantic search (optional)
export OPENAI_API_KEY=your_api_key_here
```

## Primary Use Case: GitHub Stars Import

### Overview
Import GitHub stars with comprehensive metadata mapping:
- **Topics** → bkmr tags
- **Language** → tag
- **Name/Full Name** → additional tags
- **Homepage** → metadata
- **Description** → bkmr description
- **README.md** → Embedded for semantic search

### Workflow

**Step 1: Fetch GitHub Stars**
```bash
# Use the provided import-github-stars.sh script
./scripts/import-github-stars.sh <github-username>
```

This script:
1. Fetches starred repos via GitHub API
2. Filters by: updated this year, not archived
3. Extracts: topics, language, name, description, homepage, URL
4. Creates bkmr-compatible import format

**Step 2: Import to bkmr**
```bash
# Import from generated JSON
bkmr add "$(cat github-stars-import.json)" --stdin
```

**Step 3: Enable Semantic Search (Optional)**
```bash
# Mark stars as embeddable
for id in $(bkmr search -t github --np); do
    bkmr set-embeddable "$id" --enable
done

# Backfill embeddings
bkmr --openai backfill
```

**Step 4: Embed README Files**
```bash
# Use the fetch-readme-embeddings.sh script
./scripts/fetch-readme-embeddings.sh
```

This script:
1. Fetches README.md from each starred repo
2. Converts to markdown content
3. Stores in bkmr with `_md_` tag
4. Generates embeddings for semantic search

### Tag Mapping Strategy

The import maps GitHub metadata to bkmr tags:

```
Topics:       rust,cli,bookmark     → rust,cli,bookmark
Language:     Rust                   → language-rust
Repo Name:    sysid/bkmr            → repo-bkmr
Owner:        sysid                  → author-sysid
Status:       active,maintained      → status-active,status-maintained
```

Combined example:
```bash
bkmr add https://github.com/sysid/bkmr \
  rust,cli,bookmark,language-rust,repo-bkmr,author-sysid,status-active \
  --title "bkmr - Fast bookmark manager"
```

## Common Workflows

### Searching GitHub Stars

**By language:**
```bash
bkmr search -t language-rust
bkmr search --fzf -t language-python
```

**By topic:**
```bash
bkmr search -t cli,terminal
bkmr search -t "web framework"
```

**Semantic search (finds related concepts):**
```bash
bkmr --openai sem-search "containerized application security"
bkmr --openai sem-search "async rust patterns"
```

### Managing Imported Content

**Update metadata:**
```bash
bkmr update -t reviewed,production <id>
```

**Bulk operations:**
```bash
# Add "reviewed" tag to all Rust repos
for id in $(bkmr search -t language-rust --np); do
    bkmr update -t reviewed "$id"
done
```

**Re-import updated stars:**
```bash
./scripts/import-github-stars.sh <username> --update
```

### File Import Workflows

**Import scripts with frontmatter:**
```bash
# Create script with metadata
cat > ~/scripts/backup.sh << 'EOF'
#!/bin/bash
# name: Database Backup
# tags: database,backup,production
# type: _shell_

pg_dump mydb | gzip > backup_$(date +%Y%m%d).sql.gz
EOF

# Import with base path
bkmr import-files ~/scripts/ --base-path SCRIPTS_HOME
```

**Smart editing (edits source file):**
```bash
bkmr edit <imported-id>  # Opens ~/scripts/backup.sh in $EDITOR
```

**Track changes:**
```bash
# Update when files change
bkmr import-files ~/scripts/ --base-path SCRIPTS_HOME --update
```

## Advanced Features

### Template Interpolation

Make content dynamic with Jinja2 templates:

```bash
# Dynamic date in URL
bkmr add "https://reports.com/{{ current_date | strftime('%Y-%m-%d') }}" reports

# Environment variables
bkmr add "export PATH={{ env('HOME') }}/bin:\$PATH" env,_env_

# Shell commands
bkmr add 'Branch: {{ "git branch --show-current" | shell }}' git,_snip_
```

### Fuzzy Finding

Interactive selection with keyboard shortcuts:

```bash
bkmr search --fzf --fzf-style enhanced

# Keyboard shortcuts:
# Enter   - Execute action (open/copy/run)
# Ctrl-O  - Copy to clipboard
# Ctrl-E  - Edit bookmark
# Ctrl-D  - Delete bookmark
```

### Shell Function Stubs

Use bookmarked scripts as shell functions:

```bash
# Generate and source stubs
source <(bkmr search --shell-stubs)

# Now use directly
backup-database production --incremental
deploy-app staging
```

## Configuration

### Basic Configuration (`~/.config/bkmr/config.toml`)

```toml
# Database location
db_url = "~/.config/bkmr/bkmr.db"

# FZF options
[fzf_opts]
height = "70%"
reverse = true
show_tags = true

# Base paths for file import
[base_paths]
SCRIPTS_HOME = "$HOME/scripts"
DOCS_HOME = "$HOME/documents"
GITHUB_REPOS = "$HOME/repos"
```

### Environment Variables

```bash
export BKMR_DB_URL="$HOME/.config/bkmr/bkmr.db"
export OPENAI_API_KEY="sk-..."
export EDITOR="vim"
export BKMR_FZF_OPTS="--height 80% --reverse"
```

## Scripts Reference

### `import-github-stars.sh`
Fetches and formats GitHub stars for bkmr import.

**Usage:**
```bash
./scripts/import-github-stars.sh <github-username> [--year 2025] [--update]
```

**Options:**
- `--year` - Filter by update year (default: current year)
- `--update` - Update existing bookmarks
- `--include-archived` - Include archived repos

### `fetch-readme-embeddings.sh`
Fetches README files and creates semantic embeddings.

**Usage:**
```bash
./scripts/fetch-readme-embeddings.sh [--limit 50] [--force]
```

**Options:**
- `--limit` - Number of READMEs to process
- `--force` - Re-process existing embeddings

### `sync-file-imports.sh`
Batch update file-imported bookmarks.

**Usage:**
```bash
./scripts/sync-file-imports.sh ~/scripts/ SCRIPTS_HOME
```

## Reference Documentation

See the `references/` directory for detailed documentation:

- **`01-core-concepts.md`** - Tags, system tags, content types
- **`02-semantic-search.md`** - AI-powered search with embeddings
- **`03-file-import.md`** - Frontmatter, base paths, smart editing
- **`04-github-integration.md`** - GitHub API, star import workflows
- **`05-advanced-workflows.md`** - Template interpolation, shell stubs

## Examples

See the `examples/` directory for:

- `github-stars-import.json` - Example import format
- `frontmatter-script.sh` - Script with metadata
- `bookmark-collection.md` - Markdown with frontmatter
- `semantic-search-queries.txt` - Example search queries

## Troubleshooting

### Common Issues

**Database not found:**
```bash
export BKMR_DB_URL=/path/to/bkmr.db
# Or create new database
bkmr create-db ~/.config/bkmr/bkmr.db
```

**OpenAI API errors:**
```bash
# Verify API key
echo $OPENAI_API_KEY

# Test connection
bkmr --openai sem-search "test query" --limit 1
```

**Import script failures:**
```bash
# Verify GitHub token (if using authenticated requests)
echo $GITHUB_TOKEN

# Check JSON format
cat github-stars-import.json | jq .
```

## Best Practices

### Tagging Strategy
- Use hierarchical tags: `language-rust,type-cli,topic-bookmarks`
- Consistent prefixes: `language-*`, `author-*`, `status-*`, `repo-*`
- System tags for behavior: `_snip_`, `_shell_`, `_md_`, `_env_`

### Semantic Search
- Enable embeddings for documentation and complex content
- Skip embeddings for short URLs and simple snippets
- Re-embed when content changes significantly

### File Import
- Use base paths for portability across machines
- Include metadata in frontmatter
- Run `--update` regularly to sync changes

### Performance
- Use tag filtering before full-text search
- Limit fuzzy finder results with pre-filtering
- Use `--np` (no-print) for scripting

## Related Resources

- **bkmr Wiki**: https://github.com/sysid/bkmr/wiki
- **GitHub Repository**: https://github.com/sysid/bkmr
- **Documentation**: See `references/` directory in this skill

## Version History

- **1.0** (2025-10-31) - Initial release with GitHub stars import workflow
