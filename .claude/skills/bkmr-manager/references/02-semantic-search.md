# Semantic Search with Embeddings

This document covers bkmr's AI-powered semantic search capabilities using OpenAI embeddings.

## Overview

Semantic search uses AI embeddings (vector representations of text) to find content based on meaning rather than just keywords. This allows finding conceptually related bookmarks even when they don't contain the exact search terms.

## Requirements

- OpenAI API key: `export OPENAI_API_KEY=sk-...`
- The `--openai` flag when running commands that use embeddings

## Basic Usage

```bash
# Enable OpenAI and search for conceptually similar content
bkmr --openai sem-search "containerized application security"

# Limit results to top 5 matches
bkmr --openai sem-search "event-driven architecture" --limit 5

# Non-interactive mode (returns IDs only)
bkmr --openai sem-search "microservice patterns" --np
```

## Managing Embeddable Content

Not all content benefits from semantic embeddings. By default, new bookmarks are not marked as embeddable to save API costs.

### Mark Content as Embeddable

```bash
# Enable embeddings for a single bookmark
bkmr set-embeddable 123 --enable

# Disable embeddings
bkmr set-embeddable 123 --disable

# Backfill embeddings for all embeddable bookmarks
bkmr --openai backfill

# Preview what would be backfilled
bkmr --openai backfill --dry-run
```

### Bulk Enable Embeddings

```bash
# Enable for all GitHub stars
for id in $(bkmr search -t github --np); do
    bkmr set-embeddable "$id" --enable
done

# Enable for all documentation
for id in $(bkmr search -t _md_,docs --np); do
    bkmr set-embeddable "$id" --enable
done

# Enable for specific language/topic
for id in $(bkmr search -t language-rust,async --np); do
    bkmr set-embeddable "$id" --enable
done

# Then backfill
bkmr --openai backfill
```

## Loading Text Documents

Import text documents to make them searchable via semantic search:

```bash
# Import from NDJSON file
bkmr --openai load-texts path/to/documents.jsonl

# Preview without making changes
bkmr --openai load-texts path/to/documents.jsonl --dry-run
```

**NDJSON format** (one JSON object per line):
```json
{"id": "doc1.md", "content": "This is the content of document 1."}
{"id": "doc2.md", "content": "This is the content of document 2."}
```

## Markdown File Content Embedding

When adding markdown file references with `--openai`, bkmr automatically embeds the file content:

```bash
# Add markdown file with embedding
bkmr --openai add "~/documents/research.md" research,notes --type md
```

**What happens:**
1. File content is read and embedded
2. Content hash is stored
3. When accessed later, if content changed (detected via hash):
   - New embedding is generated automatically
   - Markdown is rendered with updated content

This ensures semantic search always uses the latest version of your documents.

## GitHub README Embedding Workflow

For imported GitHub stars, embed their README files for semantic search:

**Step 1: Import stars with metadata**
```bash
./scripts/import-github-stars.sh <username>
```

**Step 2: Enable embeddings**
```bash
for id in $(bkmr search -t github --np); do
    bkmr set-embeddable "$id" --enable
done
```

**Step 3: Fetch and embed READMEs**
```bash
./scripts/fetch-readme-embeddings.sh
```

This script:
1. Fetches README.md from each starred repo
2. Converts to markdown content
3. Creates bkmr bookmark with `_md_` tag
4. Generates embeddings automatically

**Step 4: Search across documentation**
```bash
# Find related documentation
bkmr --openai sem-search "authentication patterns in web frameworks"

# Find implementation examples
bkmr --openai sem-search "async rust error handling"
```

## Optimal Content for Embeddings

### Enable Embeddings For:
- ✅ Technical documentation and notes
- ✅ README files from GitHub repos
- ✅ Complex code snippets with explanatory comments
- ✅ Project descriptions and requirements
- ✅ Reference materials and guides
- ✅ Markdown files that change frequently

### Skip Embeddings For:
- ❌ Very short snippets or one-liners
- ❌ URLs without descriptive content
- ❌ Binary files or executables
- ❌ Simple shell commands
- ❌ Environment variable definitions

## Integration with Smart Actions

Semantic search results work seamlessly with the action system:

```bash
# Find and render documentation
bkmr --openai sem-search "kubernetes pod configuration"
# Press Enter → Renders markdown in browser

# Find and execute shell scripts
bkmr --openai sem-search "deployment automation script"
# Press Enter → Opens interactive editor for execution

# Find and copy code snippets
bkmr --openai sem-search "error handling patterns"
# Press Enter → Copies to clipboard
```

## Interactive Search Mode

When using semantic search without `--np`:

1. Results displayed with similarity scores
2. Select which result(s) to open
3. Appropriate action executed based on content type

**Example:**
```bash
$ bkmr --openai sem-search "async patterns"

Results:
1. [0.92] Async HTTP Client with Retry (python,async,_snip_)
2. [0.88] Tokio Runtime Configuration (rust,async,_snip_)
3. [0.85] Async/Await Best Practices (docs,async,_md_)

Select bookmark (1-3): 1
# Copies snippet to clipboard
```

## Combining Searches

Combine semantic search with tag filtering:

```bash
# Semantic search within Python content only
bkmr --openai sem-search "database connection pooling" -t python

# Semantic search in documentation
bkmr --openai sem-search "deployment strategies" -t _md_

# Find Rust examples
bkmr --openai sem-search "memory safety patterns" -t language-rust
```

## Technical Details

### Embedding Model
- Uses OpenAI's `text-embedding-ada-002` model
- 1536-dimensional vectors
- Supports up to 8191 tokens per input

### Storage
- Embeddings stored in SQLite database
- Content hashes tracked for change detection
- Similarity calculated using cosine similarity

### API Usage
- Only embeddable bookmarks sent to OpenAI
- Content cached with SHA-256 hash
- Re-embedding only when content changes
- Batch processing for efficiency

## Privacy Considerations

When using OpenAI integration:
- Content from bookmarks sent to OpenAI API for embedding generation
- No content stored by OpenAI (per their API policy)
- Content may be used to improve OpenAI services
- Mark only trusted content as embeddable if concerned about privacy

## Cost Optimization

### Minimize API Costs

**1. Be selective with embeddings:**
```bash
# Only enable for valuable content
bkmr set-embeddable <id> --enable  # Individual items
```

**2. Use content hashing:**
```bash
# Content unchanged = no re-embedding
# Hashing prevents unnecessary API calls
```

**3. Batch processing:**
```bash
# Process in bulk to minimize requests
bkmr --openai backfill  # One API call per item
```

**4. Test with dry-run:**
```bash
# Preview before processing
bkmr --openai backfill --dry-run
```

### Estimate Costs

OpenAI embeddings pricing (as of 2024):
- `text-embedding-ada-002`: $0.0001 per 1K tokens
- Average README: ~1000 tokens = $0.0001
- 1000 READMEs: ~$0.10

**Calculate your costs:**
```bash
# Count embeddable items
bkmr search --json | jq '[.[] | select(.embeddable == true)] | length'

# Estimate: count * $0.0001 per item
```

## Troubleshooting

### API Key Issues
```bash
# Verify API key set
echo $OPENAI_API_KEY

# Test connection
bkmr --openai sem-search "test" --limit 1
```

### Embedding Errors
```bash
# Check error logs
bkmr --openai backfill 2>&1 | tee embedding-errors.log

# Common issues:
# - Rate limiting: Wait and retry
# - Invalid API key: Check OPENAI_API_KEY
# - Content too long: Trim content before embedding
```

### Performance Issues
```bash
# Process in smaller batches
for id in $(bkmr search -t github --np | head -50); do
    bkmr set-embeddable "$id" --enable
done
bkmr --openai backfill

# Continue with next 50...
```

## Best Practices

### 1. Curate Embeddable Content
```bash
# Enable embeddings for documentation and complex content
# Skip simple URLs and short snippets
```

### 2. Regular Backfills
```bash
# Weekly or monthly
bkmr --openai backfill  # Only processes new/changed content
```

### 3. Monitor Costs
```bash
# Track API usage in OpenAI dashboard
# Set billing alerts
```

### 4. Combine with Tags
```bash
# Use tags to narrow before semantic search
bkmr --openai sem-search "auth patterns" -t python,web
# More focused results, less processing
```

### 5. Test Queries
```bash
# Start with specific queries
bkmr --openai sem-search "JWT authentication middleware" --limit 3

# Refine based on results
bkmr --openai sem-search "token-based auth middleware" --limit 5
```

## Examples

### Find Related Documentation
```bash
bkmr --openai sem-search "microservice architecture patterns" -t _md_
```

### Discover Similar Repos
```bash
bkmr --openai sem-search "CLI tools for developers" -t github
```

### Cross-Language Patterns
```bash
bkmr --openai sem-search "dependency injection patterns"
# Finds DI examples across Python, Rust, TypeScript, etc.
```

### Implementation Examples
```bash
bkmr --openai sem-search "rate limiting implementation"
# Finds code examples and documentation
```
