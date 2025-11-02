---
description: Run bkmr tests single-threaded
allowed-tools: Bash(cargo test:*)
---

Run the bkmr test suite with required single-threaded execution.

## Current Status

Branch: !`git branch --show-current`
Uncommitted changes: !`git status --short`

## Task

Run the test suite with single-threaded execution (required for SQLite):

```bash
cd bkmr && cargo test -- --test-threads=1
```

**CRITICAL**: Tests MUST run single-threaded due to shared SQLite database and environment variables. Parallel execution causes race conditions.

If tests fail, analyze the output and suggest fixes. Check:
- Database connection issues
- Environment variable conflicts
- Test data setup/teardown
- Assertion failures
