---
description: Test file import functionality
allowed-tools: Bash(make:*), Bash(cargo run:*)
---

Test the file import functionality with sample files.

## Test Import Workflow

### 1. Run import test
```bash
make import-files
```

This imports files from `bkmr/tests/resources/import_test/` directory.

### 2. Verify import
```bash
cd bkmr && BKMR_DB_URL=../db/bkmr.db cargo run -- search -t _imported_
```

### 3. Test smart editing
```bash
cd bkmr && BKMR_DB_URL=../db/bkmr.db cargo run -- edit <imported-id>
```

Should open the source file in $EDITOR.

### 4. Test update detection
```bash
# Modify a test file
echo "# Updated content" >> bkmr/tests/resources/import_test/docs/api-reference.md

# Re-import with update
cd bkmr && BKMR_DB_URL=../db/bkmr.db cargo run -- import-files bkmr/tests/resources/import_test/ --update
```

Should detect content change via SHA-256 hash.

### 5. Test base path
```bash
cd bkmr && BKMR_DB_URL=../db/bkmr.db cargo run -- import-files tests/resources/import_test/ --base-path TEST_FILES
```

Report results and any issues found.
