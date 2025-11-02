---
description: Refactor code for better architecture
argument-hint: [file-path or module-name]
---

Refactor code in: $ARGUMENTS

## Refactoring Checklist

### 1. Analyze current code

Read the target file or module:
@$ARGUMENTS

### 2. Identify issues

Check for:
- [ ] Layer violations (domain importing infrastructure)
- [ ] Concrete dependencies (should use traits)
- [ ] God objects (too many responsibilities)
- [ ] Duplicate code
- [ ] Poor error handling
- [ ] Missing tests
- [ ] Unclear naming

### 3. Propose refactoring

Present refactoring plan:
- Files to modify
- New abstractions needed
- Tests to update
- Risks and migration strategy

### 4. Execute refactoring

With approval:
1. Create backup branch
2. Apply refactoring
3. Run tests (single-threaded)
4. Verify functionality
5. Update documentation

Delegate to rust-architect agent for complex architectural refactoring.

Proceed with analysis?
