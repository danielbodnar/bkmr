---
description: Compare bkmr with similar tools
allowed-tools: WebSearch, WebFetch, Bash(bkmr:*), Bash(hyperfine:*)
---

Compare bkmr with similar bookmark/snippet management tools.

## Comparison Tools

- **buku** - Python-based bookmark manager
- **kb** - Minimalist knowledge base manager
- **pet** - Simple command-line snippet manager
- **eureka** - Store and retrieve CLI commands
- **marker** - Terminal command palette

## Comparison Matrix

Research and compare:

| Feature | bkmr | buku | pet | kb | eureka |
|---------|------|------|-----|----|----|
| Language | Rust | Python | Go | Python | Rust |
| Speed | | | | | |
| Snippets | ✅ | | | | |
| Templates | ✅ | | | | |
| LSP | ✅ | | | | |
| Semantic Search | ✅ | | | | |
| File Import | ✅ | | | | |

## Performance Comparison

```bash
# Benchmark search speed
hyperfine 'bkmr search "test"' 'buku -s "test"'

# Benchmark add speed
hyperfine 'bkmr add "https://example.com" test' 'buku -a "https://example.com"'
```

## Feature Gaps

Identify features that competitors have that bkmr lacks:
- Integration possibilities
- UI features
- Export formats
- Sync capabilities

## Competitive Advantages

Highlight bkmr's unique features:
- LSP server for editor integration
- Template interpolation
- Smart file import
- Semantic search with embeddings
- Multi-content type support

Present comparison and suggest areas for improvement.
