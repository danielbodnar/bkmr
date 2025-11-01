# File Import and Smart Editing

This document covers bkmr's file import system with frontmatter metadata and intelligent editing.

## Overview

The file import system bridges file-based workflows and bkmr's database:
- Import files with structured metadata (frontmatter)
- Track changes automatically (SHA-256 hashing)
- Portable paths across machines (base path configuration)
- Smart editing (automatically edits source files)

## Supported File Types

bkmr recognizes file types and assigns appropriate system tags:

| Extension | Type | System Tag | Action |
|-----------|------|------------|--------|
| `.sh` | Shell script | `_shell_` | Interactive execution |
| `.py` | Python script | `_shell_` | Interactive execution |
| `.md` | Markdown | `_md_` | Render in browser |
| Others | Text | `_imported_` | Copy to clipboard |

## Frontmatter Formats

### YAML Frontmatter (Preferred for Markdown)

```yaml
---
name: "Database Backup Script"
tags: ["database", "backup", "automation"]
type: "_shell_"
description: "Daily backup with compression"
---
#!/bin/bash
# Script content below
pg_dump mydb | gzip > backup_$(date +%Y%m%d).sql.gz
```

**Benefits:**
- Clean, standard format
- Lists for tags
- Works in any file type

### Hash-Style Frontmatter (Better for Scripts)

```bash
#!/bin/bash
# name: Database Backup Script
# tags: database, backup, automation
# type: _shell_
# description: Daily backup with compression

# Script starts here
pg_dump mydb | gzip > backup_$(date +%Y%m%d).sql.gz
```

**Benefits:**
- Native comments (scripts stay executable)
- No special delimiters
- Natural to read in file

### Metadata Fields

| Field | Required | Description | Example |
|-------|----------|-------------|---------|
| `name` | ✅ Yes | Title/name | `"Backup Script"` |
| `tags` | ❌ No | Tags | `database,backup` or `["database", "backup"]` |
| `type` | ❌ No | System tag override | `_shell_`, `_md_`, `_snip_` |
| `description` | ❌ No | Additional context | `"Daily backup with rotation"` |

## Base Path Configuration

### Why Use Base Paths?

**Without base paths:**
```bash
# Breaks on different machines
/Users/john/scripts/backup.sh  # ❌
```

**With base paths:**
```bash
# Portable across machines
$SCRIPTS_HOME/backup.sh  # ✅
```

### Configuration

Add to `~/.config/bkmr/config.toml`:

```toml
[base_paths]
SCRIPTS_HOME = "$HOME/scripts"
DOCS_HOME = "$HOME/documents"
WORK_SCRIPTS = "/work/automation/scripts"
PROJECT_NOTES = "$HOME/projects/documentation"
DOTFILES = "$HOME/.config"
GITHUB_REPOS = "$HOME/repos"
```

**Environment variables supported:**
- `$HOME` - User home directory
- `$USER` - Current username
- `$PWD` - Current directory
- Any custom environment variables

### Validation

```bash
# Base path not configured error:
$ bkmr import-files backup.sh --base-path SCRIPTS_HOME
Error: Base path 'SCRIPTS_HOME' not found in configuration
Add it to ~/.config/bkmr/config.toml under [base_paths]
```

## Importing Files

### Basic Import

```bash
# Single file
bkmr import-files ~/scripts/backup.sh

# Multiple files
bkmr import-files ~/scripts/backup.sh ~/scripts/deploy.sh

# Directory (recursive)
bkmr import-files ~/scripts/

# Respects .gitignore automatically
bkmr import-files ~/scripts/  # Skips ignored files
```

### Import with Base Paths

```bash
# Import with base path (stores as $SCRIPTS_HOME/backup.sh)
bkmr import-files scripts/backup.sh --base-path SCRIPTS_HOME

# Directory with base path
bkmr import-files scripts/ --base-path SCRIPTS_HOME

# Multiple directories with different base paths
bkmr import-files ~/scripts/ --base-path SCRIPTS_HOME
bkmr import-files ~/docs/ --base-path DOCS_HOME
bkmr import-files ~/repos/ --base-path GITHUB_REPOS
```

### Update Existing Bookmarks

```bash
# Update when content changes
bkmr import-files scripts/ --base-path SCRIPTS_HOME --update

# Detects:
# ✅ Content changes (SHA-256 hash)
# ✅ Metadata changes (frontmatter)
# ✅ Path changes (file moved)
```

**Example output:**
```
Content changed: backup-database.sh
Metadata changed: deploy-app.sh (tags updated)
Path changed: monitoring.sh (moved from scripts/old/)
Updated 3 bookmarks
```

### Delete Missing Files

```bash
# Remove bookmarks for deleted/moved files
bkmr import-files scripts/ --delete-missing

# Safe with dry-run first
bkmr import-files scripts/ --delete-missing --dry-run
```

### Dry Run (Preview)

```bash
# Preview changes
bkmr import-files scripts/ --base-path SCRIPTS_HOME --dry-run --update

# Example output:
# Would add: new-script.sh
# Would update: backup.sh (content changed)
# Would skip: deploy.sh (unchanged)
```

## Smart Editing System

### How It Works

When you run `bkmr edit <id>`:

```
1. Check bookmark metadata
   ├─ Has file_path? → File-imported bookmark
   │  ├─ Source file exists? → Open in $EDITOR
   │  └─ Source missing? → Fall back to database editor
   └─ No file_path? → Regular bookmark → Database editor
```

### Editing File-Imported Bookmarks

```bash
# Smart editing (opens source file automatically)
bkmr edit 123

# What happens:
# 1. Opens /path/to/source/file.sh in $EDITOR
# 2. You edit and save
# 3. On next access, bkmr re-reads the file
# 4. Metadata from frontmatter syncs to database
```

**Benefits:**
- Edit files in natural environment
- Keep files version-controlled (git)
- Metadata stays in sync automatically
- Use editor's full features

### Editing Regular Bookmarks

```bash
# For non-imported bookmarks
bkmr edit 456

# Opens database editor:
=== ID ===
456
=== URL ===
SELECT * FROM users WHERE active = true
=== TITLE ===
Active Users Query
=== TAGS ===
sql,_snip_
=== COMMENTS ===
Production database query
```

### Force Database Editing

```bash
# Force database editor (ignores source file)
bkmr edit 123 --force-db

# Use cases:
# - Source file on unmounted drive
# - Temporary change without affecting source
# - Testing different content
```

## Change Detection

bkmr tracks three types of changes:

### 1. Content Changes (SHA-256)
```bash
# File content modified
echo "new line" >> backup.sh
bkmr import-files scripts/ --update
# Output: Content changed: backup.sh
```

### 2. Metadata Changes (Frontmatter)
```bash
# Changed tags in frontmatter
# name: Database Backup
# tags: backup,production  # <-- changed from "backup"

bkmr import-files scripts/ --update
# Output: Metadata changed: backup.sh
```

### 3. Path Changes (File Moved)
```bash
# File moved to different directory
mv scripts/backup.sh scripts/database/backup.sh

bkmr import-files scripts/ --update
# Output: Path changed: backup.sh
```

## Path Resolution

bkmr resolves paths intelligently:

```bash
# Base path variable expansion
$SCRIPTS_HOME/backup.sh → /home/user/scripts/backup.sh

# Environment variable expansion
$HOME/docs/notes.md → /home/user/docs/notes.md

# Tilde expansion
~/scripts/deploy.sh → /home/user/scripts/deploy.sh

# Relative paths (from import directory)
scripts/backup.sh → /current/working/dir/scripts/backup.sh
```

## Incremental Updates

Only changed files are processed:

```bash
# First import
bkmr import-files scripts/  # Imports 10 files

# Change one file
echo "update" >> scripts/backup.sh

# Re-import (only processes changed file)
bkmr import-files scripts/ --update
# Processing: 1/10 files (9 unchanged, 1 updated)
```

## Integration with bkmr Features

### Search and Discovery

File-imported bookmarks are fully searchable:

```bash
# Full-text search includes file content
bkmr search "database backup"

# Tag filtering with frontmatter tags
bkmr search -t backup,production

# System tags auto-assigned
bkmr search -t _shell_  # All imported shell scripts
```

### Fuzzy Finder

```bash
bkmr search --fzf

# Keyboard shortcuts:
# CTRL-E → Smart editing (opens source file)
# CTRL-D → Delete (removes from tracking)
```

### Content-Aware Actions

```bash
# Shell scripts execute with interactive editing
bkmr open <shell-script-id>

# Markdown files render in browser
bkmr open <markdown-id>

# With arguments
bkmr open <shell-script-id> --no-edit -- arg1 arg2
```

## Complete Workflow Example

### 1. Setup Configuration

```bash
# Generate config
bkmr --generate-config > ~/.config/bkmr/config.toml

# Edit to add base paths
[base_paths]
SCRIPTS_HOME = "$HOME/scripts"
GITHUB_REPOS = "$HOME/repos"
```

### 2. Create Script with Frontmatter

```bash
cat > ~/scripts/backup-db.sh << 'EOF'
#!/bin/bash
# name: Database Backup
# tags: database,backup,production
# type: _shell_
# description: Daily PostgreSQL backup with rotation

BACKUP_DIR="/backups"
DATE=$(date +%Y%m%d)

pg_dump mydb | gzip > "$BACKUP_DIR/backup_$DATE.sql.gz"

# Keep only last 7 days
find "$BACKUP_DIR" -name "backup_*.sql.gz" -mtime +7 -delete
EOF

chmod +x ~/scripts/backup-db.sh
```

### 3. Import

```bash
bkmr import-files ~/scripts/ --base-path SCRIPTS_HOME
# Output: Imported: backup-db.sh (shell script)
```

### 4. Search and Edit

```bash
# Find script
bkmr search --fzf -t backup

# Press CTRL-E → Opens ~/scripts/backup-db.sh in $EDITOR
# Make changes and save
# Changes automatically reflected in bkmr
```

### 5. Update After Changes

```bash
bkmr import-files ~/scripts/ --base-path SCRIPTS_HOME --update
# Only changed files processed
```

## Best Practices

### 1. Descriptive Names
```bash
# Good
# name: Backup Production Database

# Avoid
# name: backup.sh
```

### 2. Consistent Tags
```bash
# Good - hierarchical
# tags: production,database,backup,postgresql

# Avoid - vague
# tags: script,important,work
```

### 3. Minimal Frontmatter
```bash
# Only necessary metadata
# name: Deploy to Staging
# tags: deploy,staging
```

### 4. Regular Updates
```bash
# Daily or weekly
bkmr import-files ~/scripts/ --base-path SCRIPTS_HOME --update

# Or create alias
alias bkmr-sync='bkmr import-files ~/scripts/ --update'
```

### 5. Backup Before Bulk Operations
```bash
cp ~/.config/bkmr/bkmr.db ~/.config/bkmr/bkmr_backup.db

bkmr import-files large-directory/ --update --delete-missing
```

## Troubleshooting

### Source File Not Found
```bash
$ bkmr edit 123
Source file does not exist: $SCRIPTS_HOME/old-script.sh
Falling back to database content editing...
```

**Solutions:**
- Check if file moved/deleted
- Update base path in config.toml
- Re-import with correct paths
- Use `--force-db` to edit database content

### Base Path Not Configured
```bash
$ bkmr import-files file.sh --base-path SCRIPTS_HOME
Error: Base path 'SCRIPTS_HOME' not found
```

**Solution:**
```toml
[base_paths]
SCRIPTS_HOME = "/path/to/scripts"
```

### Editor Not Found
```bash
$ bkmr edit 123
Error: EDITOR environment variable not set
```

**Solution:**
```bash
export EDITOR=vim  # or nano, code, emacs
```

### Frontmatter Parse Errors
```bash
$ bkmr import-files script.sh
Warning: Invalid frontmatter - using filename as title
```

**Check:**
- YAML: proper `---` delimiters
- Hash-style: proper `# key: value` format
- Required field: `name`
