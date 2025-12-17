# Database Migration

The migration commands provide tools for database import/export operations and migration status tracking in Redis Enterprise.

## Available Commands

### Get Migration Status

Check the status of a specific migration operation:

```bash
# Get migration status
redisctl enterprise migration get 12345

# Get migration status as YAML
redisctl enterprise migration get 12345 -o yaml

# Extract specific fields
redisctl enterprise migration get 12345 -q '{status: status, progress: progress_percentage}'

# Check if migration is complete
redisctl enterprise migration get 12345 -q 'status == "completed"'
```

### Export Database

Export database data for backup or migration:

```bash
# Export database
redisctl enterprise migration export 1

# Export and save task ID
TASK_ID=$(redisctl enterprise migration export 1 -q 'task_id')

# Monitor export progress
redisctl enterprise action get $TASK_ID

# Export with specific options (via database commands)
redisctl enterprise database export 1 --data '{
  "export_type": "rdb",
  "compression": "gzip"
}'
```

### Import Database

Import data into a database:

```bash
# Import from RDB file URL
cat <<EOF | redisctl enterprise migration import 1 --data -
{
  "source_type": "url",
  "source_url": "https://storage.example.com/backup.rdb.gz",
  "import_type": "rdb"
}
EOF

# Import from another database
redisctl enterprise migration import 2 --data '{
  "source_type": "database",
  "source_database_uid": 1
}'

# Import from file
redisctl enterprise migration import 3 --data @import-config.json
```

## Output Examples

### Migration Status
```json
{
  "uid": 12345,
  "status": "in_progress",
  "type": "import",
  "database_uid": 1,
  "started": "2024-03-15T10:00:00Z",
  "progress_percentage": 65,
  "estimated_completion": "2024-03-15T10:30:00Z",
  "bytes_transferred": 1073741824,
  "total_bytes": 1649267441
}
```

### Export Response
```json
{
  "task_id": "task-export-67890",
  "status": "queued",
  "database_uid": 1,
  "export_location": "s3://backups/db1-20240315.rdb.gz"
}
```

### Import Response
```json
{
  "task_id": "task-import-11111",
  "status": "started",
  "database_uid": 2,
  "source": "https://storage.example.com/backup.rdb.gz"
}
```

## Common Use Cases

### Database Backup

Create and manage database backups:

```bash
# Export database for backup
redisctl enterprise migration export 1

# Check export status
redisctl enterprise action list -q "[?contains(name, 'export')]"

# Download exported file (if accessible)
EXPORT_URL=$(redisctl enterprise action get <task_id> -q 'result.export_url')
curl -o backup.rdb.gz "$EXPORT_URL"
```

### Database Cloning

Clone a database within the cluster:

```bash
# Export source database
EXPORT_TASK=$(redisctl enterprise migration export 1 -q 'task_id')

# Wait for export to complete
redisctl enterprise action wait $EXPORT_TASK

# Get export location
EXPORT_LOC=$(redisctl enterprise action get $EXPORT_TASK -q 'result.location')

# Import to new database
cat <<EOF | redisctl enterprise migration import 2 --data -
{
  "source_type": "internal",
  "source_location": "$EXPORT_LOC"
}
EOF
```

### Cross-Cluster Migration

Migrate databases between clusters:

```bash
# On source cluster: Export database
redisctl enterprise migration export 1
# Note the export location

# Transfer file to destination cluster storage
# (Use appropriate method: S3, FTP, SCP, etc.)

# On destination cluster: Import database
cat <<EOF | redisctl enterprise migration import 1 --data -
{
  "source_type": "url",
  "source_url": "https://storage.example.com/export.rdb.gz",
  "skip_verify_ssl": false
}
EOF
```

### Scheduled Backups

Automate regular database exports:

```bash
#!/bin/bash
# backup.sh - Daily backup script

DBS=$(redisctl enterprise database list -q '[].uid' --raw)

for DB in $DBS; do
  echo "Backing up database $DB"
  TASK=$(redisctl enterprise migration export $DB -q 'task_id')
  
  # Store task IDs for monitoring
  echo "$TASK:$DB:$(date +%Y%m%d)" >> backup-tasks.log
done

# Monitor all backup tasks
while read line; do
  TASK=$(echo $line | cut -d: -f1)
  DB=$(echo $line | cut -d: -f2)
  STATUS=$(redisctl enterprise action get $TASK -q 'status')
  echo "Database $DB backup: $STATUS"
done < backup-tasks.log
```

## Migration Monitoring

Track migration progress and handle issues:

```bash
# List all migration-related tasks
redisctl enterprise action list -q "[?contains(name, 'migration') || contains(name, 'import') || contains(name, 'export')]"

# Monitor specific migration
MIGRATION_ID=12345
while true; do
  STATUS=$(redisctl enterprise migration get $MIGRATION_ID -q 'status')
  PROGRESS=$(redisctl enterprise migration get $MIGRATION_ID -q 'progress_percentage')
  echo "Status: $STATUS, Progress: $PROGRESS%"
  [ "$STATUS" = "completed" ] && break
  sleep 10
done

# Check for errors
redisctl enterprise migration get $MIGRATION_ID -q 'error'
```

## Error Handling

Handle migration failures:

```bash
# Check migration error details
redisctl enterprise migration get <uid> -q '{status: status, error: error_message, failed_at: failed_timestamp}'

# List failed migrations
redisctl enterprise action list -q "[?status == 'failed' && contains(name, 'migration')]"

# Retry failed import
FAILED_CONFIG=$(redisctl enterprise migration get <uid> -q 'configuration')
echo "$FAILED_CONFIG" | redisctl enterprise migration import <bdb_uid> --data -
```

## Best Practices

1. **Pre-Migration Checks**: Verify source and target compatibility
2. **Test Migrations**: Always test with non-production data first
3. **Monitor Progress**: Track migration status throughout the process
4. **Verify Data**: Confirm data integrity after migration
5. **Schedule Wisely**: Run large migrations during maintenance windows
6. **Keep Backups**: Maintain backups before starting migrations

## Troubleshooting

### Import Failures

When imports fail:

```bash
# Check database status
redisctl enterprise database get <bdb_uid> -q 'status'

# Verify available memory
redisctl enterprise database get <bdb_uid> -q '{memory_size: memory_size, used_memory: used_memory}'

# Check cluster resources
redisctl enterprise cluster get -q 'resources'

# Review error logs
redisctl enterprise logs get --filter "database=$BDB_UID"
```

### Export Issues

When exports fail:

```bash
# Check disk space on nodes
redisctl enterprise node list -q '[].{node: uid, disk_free: disk_free_size}'

# Verify database is accessible
redisctl enterprise database get <bdb_uid> -q 'status'

# Check export permissions
redisctl enterprise database get <bdb_uid> -q 'backup_configuration'
```

## Related Commands

- `redisctl enterprise database` - Database management including import/export
- `redisctl enterprise action` - Track migration tasks
- `redisctl enterprise cluster` - Check cluster resources
- `redisctl enterprise logs` - View migration-related logs