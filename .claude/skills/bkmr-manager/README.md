# BKMR Manager Skill

**Version**: 1.0
**Created**: 2025-10-31
**Type**: CLI Tool Integration
**Repository**: https://github.com/sysid/bkmr

## Description

This Claude Code skill enables advanced bookmark, snippet, and knowledge management workflows using `bkmr` - a fast, feature-rich CLI tool written in Rust. Specializes in importing GitHub stars with rich metadata, semantic search with OpenAI embeddings, and intelligent file import with frontmatter metadata.

## Key Features

- **GitHub Stars Import**: Fetch and import starred repositories with comprehensive metadata mapping
- **Semantic Search**: AI-powered search using OpenAI embeddings
- **README Embedding**: Automatically fetch and embed README files from starred repos
- **File Import**: Import scripts and documents with frontmatter metadata
- **Smart Editing**: Automatically edit source files vs database content
- **Template Interpolation**: Dynamic content with Jinja2 templates

## Quick Start

### 1. Install bkmr

```bash
cargo install bkmr
# Or: brew install bkmr
```

### 2. Setup Database

```bash
bkmr create-db ~/.config/bkmr/bkmr.db
export BKMR_DB_URL=~/.config/bkmr/bkmr.db
export OPENAI_API_KEY=sk-...
```

### 3. Import GitHub Stars

```bash
# Navigate to skill directory
cd ~/.claude/skills/bkmr-manager

# Import your stars
./scripts/import-github-stars.sh <your-github-username>

# Enable embeddings
for id in $(bkmr search -t github --np); do
    bkmr set-embeddable $id --enable
done

# Generate embeddings
bkmr --openai backfill

# Fetch READMEs
./scripts/fetch-readme-embeddings.sh --limit 100
```

### 4. Search and Discover

```bash
# Semantic search
bkmr --openai sem-search "CLI tools for developers"

# By language
bkmr search --fzf -t language-rust

# By topic
bkmr search -t kubernetes,docker

# Interactive fuzzy finder
bkmr search --fzf -t github
```

## Skill Structure

```
bkmr-manager/
├── SKILL.md                          # Main skill documentation
├── README.md                         # This file
├── references/                       # Detailed documentation
│   ├── 01-core-concepts.md          # Tags, system tags, content types
│   ├── 02-semantic-search.md        # AI-powered search with embeddings
│   ├── 03-file-import.md            # Frontmatter, base paths, smart editing
│   ├── 04-github-integration.md     # GitHub API, stars import workflows
│   └── 05-advanced-workflows.md     # Template interpolation, shell stubs
├── scripts/                          # Automation scripts
│   ├── import-github-stars.sh       # Fetch and import GitHub stars
│   └── fetch-readme-embeddings.sh   # Fetch and embed README files
└── examples/                         # Usage examples
    ├── frontmatter-script.sh        # Script with frontmatter
    ├── frontmatter-markdown.md      # Markdown with frontmatter
    └── github-stars-example.sh      # Complete workflow example
```

## Primary Use Case: GitHub Stars Knowledge Base

This skill was designed for the workflow:

1. **Import GitHub stars** with topics, language, name, description as bkmr tags
2. **Enable semantic search** by marking stars as embeddable
3. **Fetch README files** from starred repositories
4. **Embed README content** for AI-powered semantic search
5. **Query across documentation** using natural language queries

## Usage Examples

### Import Workflow

```bash
# Import stars from 2025 only
./scripts/import-github-stars.sh danielbodnar

# Filter by language
bkmr search --fzf -t language-rust,github

# Enable embeddings selectively
for id in $(bkmr search -t language-rust,stars-100 --np); do
    bkmr set-embeddable $id --enable
done

bkmr --openai backfill
```

### Semantic Search

```bash
# Find authentication libraries
bkmr --openai sem-search "authentication and authorization libraries"

# Find CLI frameworks
bkmr --openai sem-search "command line interface frameworks"

# Cross-language pattern search
bkmr --openai sem-search "dependency injection patterns"
```

### File Import

```bash
# Import scripts with metadata
bkmr import-files ~/scripts/ --base-path SCRIPTS_HOME

# Track changes
bkmr import-files ~/scripts/ --update

# Smart editing (edits source file)
bkmr edit <id>
```

## Requirements

### Software
- **bkmr** (v6.0+): `cargo install bkmr`
- **gh** (GitHub CLI): `brew install gh` or `apt install gh`
- **jq**: `brew install jq` or `apt install jq`

### Configuration
- **BKMR_DB_URL**: Path to bkmr database
- **OPENAI_API_KEY**: For semantic search (optional)
- **EDITOR**: For editing bookmarks (e.g., vim, code, nano)

### Authentication
```bash
# GitHub CLI
gh auth login

# Verify
gh auth status
```

## Documentation

See `references/` directory for comprehensive documentation:

- **01-core-concepts.md** - Tags, system tags, content types
- **02-semantic-search.md** - AI-powered search with embeddings
- **03-file-import.md** - Frontmatter, base paths, smart editing
- **04-github-integration.md** - GitHub API, stars import workflows
- **05-advanced-workflows.md** - Template interpolation, shell stubs, automation

## Scripts

### import-github-stars.sh

Fetches starred repositories and generates bkmr import commands.

**Basic usage:**
```bash
./scripts/import-github-stars.sh <github-username>
```

**Advanced:**
```bash
./scripts/import-github-stars.sh danielbodnar --year 2025 --output stars-2025.sh
```

### fetch-readme-embeddings.sh

Fetches README files from starred repos and creates embedded bookmarks.

**Basic usage:**
```bash
./scripts/fetch-readme-embeddings.sh
```

**Filtered:**
```bash
./scripts/fetch-readme-embeddings.sh --filter language-rust --limit 50
```

## Cost Considerations

### GitHub API
- Free with authentication (5000 requests/hour)

### OpenAI Embeddings
- `text-embedding-ada-002`: $0.0001 per 1K tokens
- Average README: ~1000 tokens = $0.0001
- 1000 starred repos: ~$0.10 total

## Troubleshooting

### Common Issues

**GitHub authentication:**
```bash
gh auth status
gh auth login  # If not authenticated
```

**Database not found:**
```bash
export BKMR_DB_URL=~/.config/bkmr/bkmr.db
bkmr create-db ~/.config/bkmr/bkmr.db
```

**OpenAI errors:**
```bash
echo $OPENAI_API_KEY
bkmr --openai sem-search "test" --limit 1
```

## Best Practices

1. **Tag hierarchically**: `language-rust,type-cli,topic-bookmarks`
2. **Use base paths**: Configure in `~/.config/bkmr/config.toml`
3. **Enable embeddings selectively**: Not everything needs AI search
4. **Regular backups**: `cp ~/.config/bkmr/bkmr.db bkmr_backup.db`
5. **Update regularly**: Re-run import scripts to sync new stars

## Resources

- **bkmr Repository**: https://github.com/sysid/bkmr
- **bkmr Wiki**: https://github.com/sysid/bkmr/wiki
- **Issue Tracker**: https://github.com/sysid/bkmr/issues

## Version History

- **1.0** (2025-10-31) - Initial release
  - GitHub stars import with metadata mapping
  - README embedding workflow
  - Semantic search integration
  - File import with frontmatter

## License

This skill follows the same BSD-3-Clause license as bkmr.

## Contributing

To improve this skill:
1. Test workflows and report issues
2. Suggest additional automation scripts
3. Share advanced usage patterns
4. Contribute to bkmr upstream: https://github.com/sysid/bkmr

## Author

Created for Daniel Bodnar (@danielbodnar) to manage GitHub stars and build a searchable knowledge base.
