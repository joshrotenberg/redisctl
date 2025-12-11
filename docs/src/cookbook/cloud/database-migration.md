# Database Migration

Time: 20-30 minutes  
Prerequisites:
- Source and destination databases (Redis Cloud or external Redis)
- redisctl configured with Cloud credentials
- Network connectivity between source and destination

## Migration Strategies

Three common migration approaches:

1. **Import from backup** - Best for one-time migrations
2. **Online replication** - For minimal downtime
3. **RIOT (Redis Input/Output Tool)** - For complex transformations

## Quick Migration (from backup)

```bash
# Create backup from source
redisctl cloud database backup \
  --database-id 42:12345 \
  --wait

# Create new database from backup
redisctl cloud database create \
  --subscription 42 \
  --data '{
    "name": "migrated-db",
    "memory_limit_in_gb": 2,
    "restore_from_backup": {
      "backup_id": "bkp-20251007-143022"
    }
  }' \
  --wait
```

## Method 1: Import from RDB File

### 1. Export from Source Database

```bash
# If source is Redis Cloud, create backup and download
redisctl cloud database backup \
  --database-id 42:12345 \
  --wait

# Get backup URL
redisctl cloud database backup-status \
  --database-id 42:12345 \
  -q 'last_backup.download_url'

# Download backup
curl -o source-backup.rdb "https://backup-url..."
```

### 2. Upload to Cloud Storage

```bash
# Upload to S3
aws s3 cp source-backup.rdb s3://my-bucket/redis-migration/

# Get presigned URL (valid for import)
aws s3 presign s3://my-bucket/redis-migration/source-backup.rdb --expires-in 3600
```

### 3. Import to Destination Database

```bash
# Import data
redisctl cloud database import \
  --database-id 43:67890 \
  --data '{
    "source_type": "http-url",
    "import_from_uri": "https://presigned-url..."
  }' \
  --wait \
  --wait-timeout 1800
```

### 4. Monitor Import Progress

```bash
# Check import status
redisctl cloud database import-status \
  --database-id 43:67890 \
  -o json -q '{
    status: status,
    progress: progress_percentage,
    imported_keys: keys_imported
  }'
```

## Method 2: Online Replication

For minimal downtime, use Redis replication:

### 1. Setup Destination as Replica

```bash
# Create destination database with replication source
redisctl cloud database create \
  --subscription 43 \
  --data '{
    "name": "replica-db",
    "memory_limit_in_gb": 2,
    "replication": true,
    "replica_of": ["redis-12345.cloud.redislabs.com:12345"]
  }' \
  --wait
```

### 2. Monitor Replication Lag

```bash
# Check replication status
redis-cli -h replica-endpoint -p 67890 INFO replication
```

### 3. Cutover to New Database

```bash
# Stop writes to source
# Wait for replication to catch up (lag = 0)

# Promote replica to master
redisctl cloud database update \
  --subscription 43 \
  --database-id 67890 \
  --data '{"replica_of": []}' \
  --wait

# Update application to use new endpoint
```

## Method 3: Cross-Region Migration

Migrate between different Redis Cloud regions:

```bash
# 1. Create backup in source region
redisctl cloud database backup \
  --database-id 42:12345 \
  --wait

# 2. Export backup to S3 in target region
# (This happens automatically with cross-region backup storage)

# 3. Create database in target region from backup
redisctl cloud database create \
  --subscription 55 \
  --data '{
    "name": "us-west-db",
    "memory_limit_in_gb": 2,
    "region": "us-west-2",
    "restore_from_backup": {
      "backup_id": "bkp-20251007-143022",
      "source_subscription_id": 42
    }
  }' \
  --wait
```

## Migration from External Redis

### From Self-Hosted Redis

```bash
# 1. Create RDB backup on source
redis-cli --rdb /tmp/redis-backup.rdb

# 2. Upload to cloud storage
aws s3 cp /tmp/redis-backup.rdb s3://my-bucket/migration/
aws s3 presign s3://my-bucket/migration/redis-backup.rdb --expires-in 3600

# 3. Import to Redis Cloud
redisctl cloud database import \
  --database-id 42:12345 \
  --data '{
    "source_type": "http-url",
    "import_from_uri": "https://presigned-url..."
  }' \
  --wait
```

### From AWS ElastiCache

```bash
# 1. Create ElastiCache backup
aws elasticache create-snapshot \
  --replication-group-id my-redis \
  --snapshot-name migration-snapshot

# 2. Export to S3
aws elasticache copy-snapshot \
  --source-snapshot-name migration-snapshot \
  --target-snapshot-name migration-export \
  --target-bucket my-bucket

# 3. Import to Redis Cloud (same as above)
```

## Data Validation

### Verify Migration Success

```bash
#!/bin/bash
# validate-migration.sh

SOURCE_HOST="source-redis"
SOURCE_PORT=6379
DEST_HOST="dest-redis"
DEST_PORT=12345

echo "Validating migration..."

# Compare key counts
SOURCE_KEYS=$(redis-cli -h $SOURCE_HOST -p $SOURCE_PORT DBSIZE)
DEST_KEYS=$(redis-cli -h $DEST_HOST -p $DEST_PORT DBSIZE)

echo "Source keys: $SOURCE_KEYS"
echo "Destination keys: $DEST_KEYS"

if [ "$SOURCE_KEYS" -eq "$DEST_KEYS" ]; then
    echo "Key count matches!"
else
    echo "WARNING: Key count mismatch!"
    exit 1
fi

# Sample key validation
redis-cli -h $SOURCE_HOST -p $SOURCE_PORT --scan --pattern "*" | \
  head -100 | \
  while read key; do
    SOURCE_VAL=$(redis-cli -h $SOURCE_HOST -p $SOURCE_PORT GET "$key")
    DEST_VAL=$(redis-cli -h $DEST_HOST -p $DEST_PORT GET "$key")
    if [ "$SOURCE_VAL" != "$DEST_VAL" ]; then
        echo "Mismatch for key: $key"
        exit 1
    fi
done

echo "Validation successful!"
```

## Zero-Downtime Migration Pattern

```bash
#!/bin/bash
# zero-downtime-migration.sh

# 1. Setup replication
echo "Setting up replication..."
redisctl cloud database update \
  --subscription 43 \
  --database-id 67890 \
  --data '{"replica_of": ["source-redis:6379"]}' \
  --wait

# 2. Monitor lag until synced
echo "Waiting for initial sync..."
while true; do
    LAG=$(redis-cli -h new-redis -p 67890 INFO replication | \
          grep master_repl_offset | cut -d: -f2)
    if [ "$LAG" -lt 100 ]; then
        break
    fi
    sleep 5
done

echo "Replication synced. Ready for cutover."
echo "Press ENTER to proceed with cutover..."
read

# 3. Stop writes to source (application-specific)
echo "Stop writes to source now!"
echo "Press ENTER when source is read-only..."
read

# 4. Wait for final sync
sleep 10

# 5. Promote replica
echo "Promoting replica to master..."
redisctl cloud database update \
  --subscription 43 \
  --database-id 67890 \
  --data '{"replica_of": []}' \
  --wait

echo "Migration complete! Update application to new endpoint."
```

## Handling Large Databases

For databases > 10GB:

```bash
# 1. Use parallel import (if supported)
redisctl cloud database import \
  --database-id 42:12345 \
  --data '{
    "source_type": "http-url",
    "import_from_uri": "https://backup-url...",
    "parallel_streams": 4
  }' \
  --wait \
  --wait-timeout 7200  # 2 hours
```

## Common Issues

### Import Times Out

```bash
# Increase timeout for large databases
redisctl cloud database import \
  --database-id 42:12345 \
  --data '{"source_type": "http-url", "import_from_uri": "..."}' \
  --wait \
  --wait-timeout 3600  # 1 hour
```

### RDB Version Mismatch

```
Error: Unsupported RDB version
```

**Solution:** Ensure source Redis version is compatible. Redis Cloud supports RDB versions from Redis 2.6+

### Network Timeout During Import

```
Error: Failed to download from URI
```

**Solution:**
1. Verify URL is accessible
2. Check presigned URL hasn't expired
3. Ensure no firewall blocks
4. Use cloud storage in same region

### Partial Import

```
Warning: Import completed but key count mismatch
```

**Solution:**
1. Check for keys with TTL that expired
2. Verify no writes during migration
3. Check for maxmemory-policy evictions
4. Review logs for specific errors

## Best Practices

1. **Test First** - Always test migration on staging
2. **Backup Source** - Create backup before migration
3. **Plan Downtime** - Communicate maintenance window
4. **Validate Data** - Compare key counts and sample data
5. **Monitor Performance** - Watch latency during cutover
6. **Keep Source** - Don't delete source immediately
7. **Update DNS** - Use DNS for easy rollback

## Migration Checklist

- \[ \] Source database backed up
- \[ \] Destination database created and configured
- \[ \] Network connectivity verified
- \[ \] Import method selected
- \[ \] Dry run completed successfully
- \[ \] Monitoring in place
- \[ \] Rollback plan documented
- \[ \] Application updated with new endpoint
- \[ \] Data validation successful
- \[ \] Source database retained for N days

## Next Steps

- [Backup and Restore](backup-restore.md) - Protect migrated data
- Configure ACLs - Secure new database
- Monitor Performance - Track after migration
- [Setup High Availability](../cloud/active-active-setup.md) - Add redundancy

## See Also

- [Database Import Reference](../../cloud/core-resources/databases.md)
- [Redis Migration Guide](https://redis.io/docs/latest/operate/oss_and_stack/management/migration/)
- [RIOT Tool](https://github.com/redis-developer/riot) - Advanced migration tool
