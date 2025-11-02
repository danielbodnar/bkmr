---
description: Quick prototype of simple feature
argument-hint: [feature description]
---

Quickly prototype and test a simple feature: $ARGUMENTS

## Rapid Prototyping Workflow

This is for small, experimental features. For major features, use /new-feature instead.

### 1. Identify minimal implementation

For "$ARGUMENTS", determine:
- Single file change or multi-file?
- Which layer (domain/application/cli)?
- Existing tests to verify?

### 2. Implement quickly

Create minimal viable implementation:
- No extensive refactoring
- Reuse existing patterns
- Add TODO comments for improvements

### 3. Test manually

```bash
cd bkmr && cargo build
cd bkmr && BKMR_DB_URL=/tmp/test.db cargo run -- $ARGUMENTS
```

### 4. Evaluate

- Does it work?
- Performance acceptable?
- Worth keeping or needs redesign?

## Decision Point

If prototype successful:
- Clean up code
- Add proper tests
- Update documentation
- Commit changes

If prototype needs work:
- Document learnings
- Create feature plan with /new-feature
- Discard prototype

This is meant for quick experimentation, not production features.
