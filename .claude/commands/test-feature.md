---
description: Test specific bkmr feature
allowed-tools: Bash(cargo test:*), Read, Grep
argument-hint: [feature-name or test-pattern]
---

Run tests for a specific feature or matching pattern: $ARGUMENTS

## Steps

1. Find matching test files:
```bash
cd bkmr && rg "fn.*$ARGUMENTS" --type rust -l tests/
```

2. Run matching tests (single-threaded):
```bash
cd bkmr && cargo test $ARGUMENTS -- --test-threads=1 --nocapture
```

If no pattern provided, show available test modules:
```bash
cd bkmr && cargo test --lib -- --list
```

Show test results and suggest fixes for any failures.
