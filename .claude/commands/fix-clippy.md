---
description: Fix all clippy warnings
allowed-tools: Bash(cargo clippy:*), Bash(cargo fix:*)
---

Fix all clippy warnings and lints in the bkmr codebase.

## Linting Strategy

### 1. Run clippy with auto-fix
```bash
cd bkmr && cargo clippy --fix -- -A unused_imports
```

### 2. Run cargo fix for edition issues
```bash
cd bkmr && cargo fix --lib -p bkmr --tests
```

### 3. Format code
```bash
cd bkmr && cargo fmt
```

### 4. Check for remaining issues
```bash
cd bkmr && cargo clippy -- -D warnings
```

## Common Issues to Fix

- Unused imports (already allowed)
- Needless borrows
- Unnecessary clones
- Complex expressions that can be simplified
- Missing documentation on public items
- Inefficient string concatenation

Report all fixes made and any warnings that require manual intervention.
