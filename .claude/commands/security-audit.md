---
description: Run security audit on dependencies
allowed-tools: Bash(cargo:*)
---

Run security audit on bkmr dependencies.

## Security Checks

### 1. Install cargo-audit
```bash
cargo install cargo-audit
```

### 2. Run audit
```bash
cd bkmr && cargo audit
```

### 3. Check for advisories
```bash
cd bkmr && cargo audit --deny warnings
```

### 4. Generate audit report
```bash
cd bkmr && cargo audit --json > security-audit.json
```

## Vulnerability Analysis

For each vulnerability found:

1. **Severity**: Critical/High/Medium/Low
2. **Affected crate**: Name and version
3. **Impact**: How it affects bkmr
4. **Fix**: Update version or find alternative

## Dependency Tree Analysis

```bash
# Show dependency tree
cd bkmr && cargo tree

# Find why a crate is included
cd bkmr && cargo tree -i suspicious-crate
```

## Recommendations

1. **Update immediately**: Critical/High severity
2. **Plan update**: Medium severity
3. **Monitor**: Low severity
4. **Consider alternatives**: If no fix available

Report findings and create update plan.
