---
description: Inspect bkmr database schema and data
allowed-tools: Bash(sqlite3:*), Read
---

Inspect the bkmr database schema and contents.

## Database Location

Current database: !`echo ${BKMR_DB_URL:-~/.config/bkmr/bkmr.db}`

## Inspection

### 1. Show schema
```bash
sqlite3 $BKMR_DB_URL ".schema"
```

### 2. List tables
```bash
sqlite3 $BKMR_DB_URL ".tables"
```

### 3. Show table info
```bash
sqlite3 $BKMR_DB_URL "PRAGMA table_info(bookmarks);"
```

### 4. Count records
```bash
sqlite3 $BKMR_DB_URL "SELECT COUNT(*) FROM bookmarks;"
sqlite3 $BKMR_DB_URL "SELECT COUNT(*) FROM tags;"
```

### 5. Sample data
```bash
sqlite3 $BKMR_DB_URL "SELECT id, title, tags FROM bookmarks LIMIT 5;"
```

### 6. Check indexes
```bash
sqlite3 $BKMR_DB_URL "SELECT name, sql FROM sqlite_master WHERE type='index';"
```

### 7. Database size
```bash
du -h $BKMR_DB_URL
```

### 8. FTS index status
```bash
sqlite3 $BKMR_DB_URL "SELECT * FROM sqlite_master WHERE type='table' AND name LIKE '%fts%';"
```

## Health Check

- Verify all expected tables exist
- Check for orphaned records
- Validate foreign key constraints
- Report any schema issues

Present database overview and any issues found.
