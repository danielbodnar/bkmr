---
description: Analyze and suggest fixes for errors
---

Analyze the most recent error or failure.

## Error Analysis Strategy

### 1. Check recent test failures
```bash
cd bkmr && cargo test -- --test-threads=1 2>&1 | tail -50
```

### 2. Check compilation errors
```bash
cd bkmr && cargo check 2>&1
```

### 3. Check clippy issues
```bash
cd bkmr && cargo clippy 2>&1
```

## Analysis

For each error found:

1. **Identify error type:**
   - Compilation error
   - Test failure
   - Runtime error
   - Clippy warning

2. **Locate source:**
   - File and line number
   - Function/module
   - Layer (domain/application/infrastructure/cli)

3. **Suggest fix:**
   - Code changes needed
   - Related files to check
   - Test updates required

4. **Check error handling:**
   - Is error properly typed?
   - Is context added?
   - Is conversion correct?

Reference ERROR_HANDLING.md for proper error patterns.

Present analysis and suggested fixes for review.
