# Core Concepts

This document provides foundational knowledge for working with bkmr.

## Bookmarks

In bkmr, a **bookmark** is any piece of content you want to store and retrieve later:
- Web URLs
- Code snippets
- Shell scripts
- Markdown documents
- Environment variables
- File references

Each bookmark has:
- **URL/Content**: The actual content (can be a URL or text)
- **Title**: A descriptive name
- **Description/Comments**: Optional additional context
- **Tags**: For organization and filtering
- **System Tags**: Automatic tags that determine behavior

## Tags

Tags are the primary organization mechanism in bkmr.

### User Tags
- Free-form labels you assign
- Comma-separated, no spaces: `python,security,auth`
- Used for filtering and organization
- Case-sensitive

**Examples:**
```bash
bkmr add <content> python,web,tutorial
bkmr search -t python,security
```

### System Tags

Special tags that determine how bkmr handles content:

| System Tag | Purpose | Default Action |
|------------|---------|----------------|
| `_snip_` | Code snippet | Copy to clipboard |
| `_shell_` | Shell script | Interactive edit + execute |
| `_md_` | Markdown document | Render in browser |
| `_env_` | Environment variables | Print for sourcing |
| `_imported_` | Imported file content | Copy to clipboard |

**Automatic assignment:**
- Added with `--type snip` → Gets `_snip_` tag
- Added with `--type shell` → Gets `_shell_` tag
- Imported `.md` files → Get `_md_` tag
- Imported `.sh` files → Get `_shell_` tag

## Content Types

### URLs
```bash
bkmr add https://example.com dev,reference
# Opens in browser when accessed via: bkmr open <id>
```

### Snippets
```bash
bkmr add "console.log('test')" javascript,_snip_ --title "Debug Log"
# Copies to clipboard when accessed
```

### Shell Scripts
```bash
bkmr add "#!/bin/bash\necho 'Hello'" utils,_shell_ --title "Greeting"
# Interactive editor before execution
```

### Markdown
```bash
bkmr add "# Title\n## Section" docs,_md_ --title "Documentation"
# Renders in browser with TOC
```

## Smart Actions

bkmr automatically selects the appropriate action based on content type:

```bash
# Same command, different actions based on system tag:
bkmr open <url-id>      # Opens in browser
bkmr open <snippet-id>  # Copies to clipboard
bkmr open <shell-id>    # Interactive editor + execute
bkmr open <md-id>       # Renders in browser
```

## Database

bkmr stores everything in a local SQLite database:
- Default location: `~/.config/bkmr/bkmr.db`
- Configurable via `BKMR_DB_URL` environment variable
- Includes full-text search (FTS5) index
- Optional: Semantic embeddings for AI-powered search

## Search Methods

### 1. Full-Text Search (FTS)
```bash
bkmr search "python security"
bkmr search "containerization"
```

### 2. Tag Filtering
```bash
bkmr search -t python,tutorial       # Must have ALL tags
bkmr search -T deprecated,old        # Must NOT have these tags
```

### 3. Fuzzy Finding (Interactive)
```bash
bkmr search --fzf                    # Interactive picker
bkmr search --fzf -t python          # Pre-filtered fuzzy search
```

### 4. Semantic Search (AI-powered)
```bash
bkmr --openai sem-search "deploy automation"
# Finds conceptually similar content using embeddings
```

## Template Interpolation

Make content dynamic with Jinja2 templates:

```bash
# Dynamic date in URL
bkmr add "https://reports.com/{{ current_date | strftime('%Y-%m-%d') }}" reports

# Environment variables
bkmr add "export PATH={{ env('HOME') }}/bin:\$PATH" env,_env_

# Shell commands
bkmr add 'Branch: {{ "git branch --show-current" | shell }}' git,_snip_
```

Templates interpolate automatically in:
- FZF mode: `bkmr search --fzf`
- Open action: `bkmr open <id>`
- Yank action: `bkmr yank <id>`

## File Integration

bkmr can import files with metadata and track changes:

```bash
# Import with frontmatter
bkmr import-files ~/scripts/backup.sh

# Smart editing (edits source file)
bkmr edit <imported-id>

# Update when files change
bkmr import-files ~/scripts/ --update
```

Files with frontmatter keep metadata in sync between the file and database.

## Best Practices

### Tagging Strategy
```bash
# Use hierarchical tags
python,flask,api,_snip_
rust,async,network,_snip_
k8s,deployment,tutorial

# Consistent prefixes for categories
language-rust
author-sysid
status-active
repo-bkmr
```

### Search Efficiency
```bash
# Tag first, then full-text (more efficient)
bkmr search -t python "async"

# Pre-filter for fuzzy finder
bkmr search --fzf -t python,async
```

### Content Organization
```bash
# Use descriptive titles
bkmr add "code" python,_snip_ --title "Async HTTP Client with Retry"

# Regular cleanup
bkmr search --ascending --limit 20  # Review old entries
bkmr delete $(bkmr search -t deprecated --np)
```

## Common Patterns

### Daily Snippet Access
```bash
alias bs='bkmr search --fzf --fzf-style enhanced -t _snip_'
bs  # Quick snippet picker
```

### Script Execution
```bash
# Generate shell function stubs
source <(bkmr search --shell-stubs)

# Use scripts directly
backup-database production
deploy-app staging
```

### Documentation Workflow
```bash
# Store docs
bkmr add "/path/to/docs/api.md" api,docs --type md

# Quick access
alias docs='bkmr search --fzf -t _md_'
```

### Environment Switching
```bash
# Store environments
bkmr add "export DB_URL=..." dev,_env_ --title "Dev Env"
bkmr add "export DB_URL=..." prod,_env_ --title "Prod Env"

# Quick switching
eval "$(bkmr search --fzf -t _env_)"
```
