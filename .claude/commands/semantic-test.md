---
description: Test semantic search functionality
allowed-tools: Bash(cargo run:*), Bash(bkmr:*)
---

Test semantic search with OpenAI embeddings.

## Prerequisites

Check OpenAI API key:
```bash
echo ${OPENAI_API_KEY:+API key is set}
```

If not set:
```bash
export OPENAI_API_KEY=sk-...
```

## Test Workflow

### 1. Create test database
```bash
cd bkmr && cargo run -- create-db /tmp/bkmr_semantic_test.db
```

### 2. Add test bookmarks
```bash
export BKMR_DB_URL=/tmp/bkmr_semantic_test.db

cd bkmr && cargo run -- add "https://github.com/rust-lang/rust" rust,programming
cd bkmr && cargo run -- add "https://docs.python.org" python,documentation
cd bkmr && cargo run -- add "Async programming patterns" async,tutorial,_snip_
```

### 3. Enable embeddings
```bash
cd bkmr && cargo run -- set-embeddable 1 --enable
cd bkmr && cargo run -- set-embeddable 2 --enable
cd bkmr && cargo run -- set-embeddable 3 --enable
```

### 4. Generate embeddings
```bash
cd bkmr && cargo run -- --openai backfill
```

### 5. Test semantic search
```bash
cd bkmr && cargo run -- --openai sem-search "async programming" --limit 5
```

Should return the async tutorial as top result.

### 6. Test with tag filtering
```bash
cd bkmr && cargo run -- --openai sem-search "documentation" -t python
```

Report results and similarity scores.
