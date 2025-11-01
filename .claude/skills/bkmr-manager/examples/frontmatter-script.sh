#!/bin/bash
# name: Database Backup Script
# tags: database,backup,production,postgresql
# type: _shell_
# description: Daily PostgreSQL backup with rotation and compression

# Configuration
BACKUP_DIR="${BACKUP_DIR:-/var/backups/postgresql}"
DB_NAME="${DB_NAME:-myapp_production}"
RETENTION_DAYS="${RETENTION_DAYS:-7}"
DATE=$(date +%Y%m%d_%H%M%S)

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Perform backup
echo "Starting backup of $DB_NAME..."
pg_dump "$DB_NAME" | gzip > "$BACKUP_DIR/backup_${DATE}.sql.gz"

if [[ $? -eq 0 ]]; then
    echo "✓ Backup completed: backup_${DATE}.sql.gz"

    # Verify backup integrity
    if gzip -t "$BACKUP_DIR/backup_${DATE}.sql.gz"; then
        echo "✓ Backup integrity verified"
    else
        echo "✗ Backup file is corrupted!"
        exit 1
    fi

    # Rotate old backups
    echo "Removing backups older than $RETENTION_DAYS days..."
    find "$BACKUP_DIR" -name "backup_*.sql.gz" -mtime +$RETENTION_DAYS -delete
    echo "✓ Rotation completed"
else
    echo "✗ Backup failed!"
    exit 1
fi

# List current backups
echo ""
echo "Current backups:"
ls -lh "$BACKUP_DIR"/backup_*.sql.gz | tail -n 5
