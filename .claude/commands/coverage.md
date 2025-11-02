---
description: Generate test coverage report
allowed-tools: Bash(cargo:*)
---

Generate test coverage report for bkmr.

## Coverage Tools

### Option 1: cargo-tarpaulin (Linux)

```bash
cargo install cargo-tarpaulin
cd bkmr && cargo tarpaulin --out Html --output-dir coverage -- --test-threads=1
```

Opens HTML coverage report in browser.

### Option 2: cargo-llvm-cov (all platforms)

```bash
cargo install cargo-llvm-cov
cd bkmr && cargo llvm-cov --html -- --test-threads=1
```

## Coverage Analysis

Report:
1. **Overall coverage percentage**
2. **Uncovered modules:**
   - Domain layer coverage
   - Application layer coverage
   - Infrastructure layer coverage
   - CLI layer coverage

3. **Critical untested code:**
   - Error handling paths
   - Edge cases
   - Repository implementations

4. **Recommendations:**
   - Tests to add
   - Code to refactor for testability
   - Integration tests needed

Target: >80% coverage for all layers.
