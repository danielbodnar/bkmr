---
description: Run performance benchmarks
allowed-tools: Bash(cargo bench:*), Bash(hyperfine:*)
---

Run performance benchmarks for bkmr.

## Benchmark Strategy

1. **Criterion benchmarks** (if they exist):
```bash
cd bkmr && cargo bench
```

2. **Command-line benchmarks** with hyperfine:
```bash
# Search performance
hyperfine --warmup 3 'bkmr search "test query"'

# Add performance
hyperfine --warmup 3 'bkmr add "https://example.com" test'

# FZF performance
hyperfine --warmup 3 --prepare 'echo "test" | bkmr search --fzf'
```

3. **Compare with alternatives:**
```bash
hyperfine 'bkmr search "query"' 'buku -s "query"'
```

Report results and suggest optimizations if performance is below targets:
- Search: < 50ms for 10k bookmarks
- Add: < 10ms
- LSP completion: < 100ms

Delegate to rust-performance agent for detailed optimization if needed.
