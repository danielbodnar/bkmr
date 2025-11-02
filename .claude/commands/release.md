---
description: Prepare bkmr for release
allowed-tools: Bash(git:*), Bash(cargo:*), Bash(make:*), Read
argument-hint: [major|minor|patch]
---

Prepare bkmr for release with version bump: $ARGUMENTS

## Pre-Release Checklist

### 1. Run all tests
```bash
make test
```

### 2. Check for uncommitted changes
```bash
git status
```

### 3. Verify version in VERSION file
Current version: !`cat VERSION`

### 4. Build release
```bash
make build
```

### 5. Test release binary
```bash
./bkmr/target/release/bkmr --version
./bkmr/target/release/bkmr search "test"
```

## Release Process

If all checks pass, proceed with version bump:

```bash
# Ensure GITHUB_TOKEN is set
export GITHUB_TOKEN=your_token

# Bump version (creates tag and GitHub release)
make bump-$ARGUMENTS
```

This will:
1. Update VERSION file
2. Commit version change
3. Create git tag
4. Push to GitHub
5. Create GitHub release

## Post-Release

1. Verify GitHub release created
2. Update crates.io if needed: `make upload`
3. Announce release
4. Update documentation

Confirm each step before proceeding.
