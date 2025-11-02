---
description: Validate clean architecture compliance
allowed-tools: Read, Grep, Bash(cargo:*)
---

Validate that the codebase follows clean architecture principles.

## Architecture Checks

### 1. Layer Dependency Violations

Check that domain doesn't import infrastructure:
```bash
cd bkmr && rg "use crate::infrastructure" src/domain/ || echo "✓ No violations found"
```

Check that domain doesn't import application:
```bash
cd bkmr && rg "use crate::application" src/domain/ || echo "✓ No violations found"
```

Check that application doesn't import infrastructure:
```bash
cd bkmr && rg "use crate::infrastructure" src/application/ || echo "✓ No violations found"
```

### 2. Repository Pattern Compliance

Verify all data access goes through repository traits:
```bash
cd bkmr && rg "SqliteConnection::establish" src/application/ src/domain/ || echo "✓ No direct database access"
```

### 3. Error Handling Hierarchy

Check that layers use proper error types:
```bash
cd bkmr && rg "DomainError|ApplicationError|CliError|SqliteRepositoryError" src/ -t rust
```

### 4. Compilation Check

Ensure code compiles:
```bash
cd bkmr && cargo check
```

## Report

For each check, report:
- ✅ Passing checks
- ❌ Violations found with file locations
- Suggestions for fixes

If violations found, delegate to rust-architect agent for refactoring guidance.
