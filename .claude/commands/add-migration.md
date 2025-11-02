---
description: Create new database migration
allowed-tools: Bash(diesel:*), Write, Read
argument-hint: [migration-name]
---

Create a new Diesel migration for: $ARGUMENTS

## Steps

1. **Generate migration:**
```bash
cd bkmr && diesel migration generate $ARGUMENTS
```

2. **Edit migration files:**
   - up.sql: Schema changes to apply
   - down.sql: How to revert changes

3. **Template for up.sql:**
```sql
-- Add your schema changes here
-- Example: ALTER TABLE bookmarks ADD COLUMN new_field TEXT;
```

4. **Template for down.sql:**
```sql
-- Revert the changes
-- Example: ALTER TABLE bookmarks DROP COLUMN new_field;
```

5. **Test migration:**
```bash
make run-migrate-db
```

6. **Update domain models** if schema changed:
   - Update `src/domain/models.rs`
   - Update Diesel schema if needed

Present the migration files for review before proceeding.
