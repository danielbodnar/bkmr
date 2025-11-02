---
description: Update Rust dependencies
allowed-tools: Bash(cargo:*), Read, Edit
---

Update Rust dependencies in Cargo.toml.

## Update Strategy

### 1. Check for outdated dependencies
```bash
cargo install cargo-outdated
cd bkmr && cargo outdated
```

### 2. Update dependencies

For each outdated dependency, decide:
- **Patch updates** (x.y.Z) - Usually safe
- **Minor updates** (x.Y.0) - Check changelog
- **Major updates** (X.0.0) - Review breaking changes

### 3. Update Cargo.toml

Read current: @bkmr/Cargo.toml

Update version numbers for selected dependencies.

### 4. Update Cargo.lock
```bash
cd bkmr && cargo update
```

### 5. Test compatibility
```bash
cd bkmr && cargo build
cd bkmr && cargo test -- --test-threads=1
```

### 6. Check for deprecations
```bash
cd bkmr && cargo build 2>&1 | grep -i deprecat
```

## High-Priority Dependencies

- **diesel** - Database ORM (check for SQLite compatibility)
- **tower-lsp** - LSP framework (protocol changes)
- **tokio** - Async runtime (LSP dependency)
- **clap** - CLI parsing (API changes)
- **serde** - Serialization (stability)

Report outdated dependencies and suggest update plan.
