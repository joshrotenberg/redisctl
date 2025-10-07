# Backup and Restore Workflow

Time: 10-15 minutes  
Prerequisites:
- Redis Cloud database with data persistence enabled
- redisctl configured with Cloud credentials
- Storage location configured (done automatically for Cloud)

## What are Backups?

Redis Cloud provides automated backups and on-demand manual backups to protect your data. Backups can be:
- **Automated** - Scheduled periodic backups (hourly, daily, weekly)
- **Manual** - On-demand backups triggered when needed
- **Stored** - In Redis Cloud storage or your own cloud storage (AWS S3, GCP GCS, Azure Blob)

## Quick Commands

```bash
# Trigger manual backup
redisctl cloud database backup \
  --database-id 42:12345 \
  --wait

# Check backup status
redisctl cloud database backup-status \
  --database-id 42:12345 \
  -o json
```

## Step-by-Step Guide

### 1. Check Current Backup Configuration

View your database's backup settings:

```bash
redisctl cloud database get \
  --subscription-id 42 \
  --database-id 12345 \
  -o json \
  -q '{
    data_persistence: data_persistence,
    backup_interval: backup_interval,
    backup_path: backup_path
  }'
```

**Example output:**
```json
{
  "data_persistence": "aof-every-1-second",
  "backup_interval": "every-24-hours",
  "backup_path": "redis-cloud-storage"
}
```

### 2. Configure Backup Settings

If backups aren't configured, enable them:

```bash
redisctl cloud database update \
  --subscription-id 42 \
  --database-id 12345 \
  --data '{
    "data_persistence": "aof-every-1-second",
    "backup_interval": "every-24-hours"
  }' \
  --wait
```

**Backup interval options:**
- `every-12-hours` - Twice daily
- `every-24-hours` - Daily (recommended for most)
- `every-week` - Weekly

### 3. Trigger Manual Backup

Create an on-demand backup before major changes:

```bash
redisctl cloud database backup \
  --database-id 42:12345 \
  --wait \
  --wait-timeout 600
```

**What you should see:**
```json
{
  "taskId": "backup-abc123",
  "status": "processing"
}
...
Backup completed successfully!
{
  "backup_id": "bkp-20251007-143022",
  "status": "completed",
  "size_bytes": 10485760,
  "timestamp": "2025-10-07T14:30:22Z"
}
```

### 4. Monitor Backup Status

Check backup progress and history:

```bash
redisctl cloud database backup-status \
  --database-id 42:12345 \
  -o json
```

**Example output:**
```json
{
  "last_backup": {
    "backup_id": "bkp-20251007-143022",
    "status": "completed",
    "timestamp": "2025-10-07T14:30:22Z",
    "size_bytes": 10485760,
    "type": "manual"
  },
  "next_scheduled": "2025-10-08T14:00:00Z",
  "backup_progress": null
}
```

### 5. List Available Backups

View all backups for a database:

```bash
# Get subscription backup info
redisctl cloud subscription get \
  --subscription-id 42 \
  -o json \
  -q 'databases[?database_id==`12345`].backup_status'
```

## Restore Scenarios

### Scenario 1: Restore from Recent Backup

If you need to restore to a previous state:

```bash
# Create new database from backup
redisctl cloud database create \
  --subscription-id 42 \
  --data '{
    "name": "restored-db",
    "memory_limit_in_gb": 1,
    "restore_from_backup": {
      "backup_id": "bkp-20251007-143022"
    }
  }' \
  --wait
```

**Note:** Redis Cloud doesn't support in-place restore. You create a new database from a backup, verify it, then switch your application.

### Scenario 2: Point-in-Time Recovery

For databases with AOF persistence:

```bash
# Create database with specific backup
redisctl cloud database create \
  --subscription-id 42 \
  --data '{
    "name": "pit-restore",
    "memory_limit_in_gb": 2,
    "restore_from_backup": {
      "backup_id": "bkp-20251007-120000",
      "timestamp": "2025-10-07T14:00:00Z"
    }
  }' \
  --wait
```

### Scenario 3: Clone Production to Staging

Use backups to create staging environments:

```bash
# Get latest production backup
BACKUP_ID=$(redisctl cloud database backup-status \
  --database-id 42:12345 \
  -o json \
  -q 'last_backup.backup_id' \
  | jq -r '.')

# Create staging database from production backup
redisctl cloud database create \
  --subscription-id 42 \
  --data '{
    "name": "staging-db",
    "memory_limit_in_gb": 1,
    "restore_from_backup": {
      "backup_id": "'$BACKUP_ID'"
    }
  }' \
  --wait
```

## Advanced: Custom Backup Storage

### Configure S3 Backup Storage

Store backups in your own AWS S3 bucket:

```bash
redisctl cloud database update \
  --subscription-id 42 \
  --database-id 12345 \
  --data '{
    "backup_path": "s3://my-backup-bucket/redis-backups",
    "backup_s3_access_key_id": "AKIAIOSFODNN7EXAMPLE",
    "backup_s3_secret_access_key": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
  }' \
  --wait
```

### Configure GCS Backup Storage

Store backups in Google Cloud Storage:

```bash
redisctl cloud database update \
  --subscription-id 42 \
  --database-id 12345 \
  --data '{
    "backup_path": "gs://my-backup-bucket/redis-backups",
    "backup_gcs_credentials": "$(cat gcs-key.json | jq -c .)"
  }' \
  --wait
```

### Configure Azure Blob Storage

Store backups in Azure:

```bash
redisctl cloud database update \
  --subscription-id 42 \
  --database-id 12345 \
  --data '{
    "backup_path": "abs://my-storage-account/redis-backups",
    "backup_abs_account_name": "mystorageaccount",
    "backup_abs_account_key": "your-account-key"
  }' \
  --wait
```

## Backup Automation Strategy

### Daily Backups with Retention

```bash
#!/bin/bash
# backup-daily.sh - Daily backup script

SUBSCRIPTION_ID=42
DATABASE_ID=12345
DATE=$(date +%Y%m%d)

echo "Starting daily backup for database $DATABASE_ID..."

# Trigger backup
redisctl cloud database backup \
  --database-id ${SUBSCRIPTION_ID}:${DATABASE_ID} \
  --wait \
  --wait-timeout 900 \
  -o json | tee backup-${DATE}.log

# Check backup status
redisctl cloud database backup-status \
  --database-id ${SUBSCRIPTION_ID}:${DATABASE_ID} \
  -o json \
  -q 'last_backup.{id: backup_id, status: status, size_mb: (size_bytes / 1048576)}'

echo "Backup completed: $DATE"
```

**Schedule with cron:**
```bash
# Daily at 2 AM
0 2 * * * /path/to/backup-daily.sh >> /var/log/redis-backup.log 2>&1
```

### Pre-Deployment Backup

```bash
#!/bin/bash
# pre-deploy-backup.sh - Backup before deployments

SUBSCRIPTION_ID=42
DATABASE_ID=12345
DEPLOYMENT_ID=$(git rev-parse --short HEAD)

echo "Creating pre-deployment backup for $DEPLOYMENT_ID..."

# Trigger backup
BACKUP_RESULT=$(redisctl cloud database backup \
  --database-id ${SUBSCRIPTION_ID}:${DATABASE_ID} \
  --wait \
  -o json)

BACKUP_ID=$(echo "$BACKUP_RESULT" | jq -r '.backup_id')

echo "Backup created: $BACKUP_ID"
echo "Safe to proceed with deployment $DEPLOYMENT_ID"

# Save backup ID for potential rollback
echo "$BACKUP_ID" > .last-backup-id
```

## Backup Verification

### Verify Backup Integrity

```bash
# Create test database from backup
redisctl cloud database create \
  --subscription-id 42 \
  --data '{
    "name": "backup-verify",
    "memory_limit_in_gb": 1,
    "restore_from_backup": {
      "backup_id": "bkp-20251007-143022"
    }
  }' \
  --wait

# Test data integrity (example with known key)
redis-cli -h backup-verify-endpoint -p 12346 GET test-key

# Clean up test database
redisctl cloud database delete \
  --subscription-id 42 \
  --database-id 67890 \
  --wait
```

## Monitoring Backup Health

### Check Backup Metrics

```bash
# Get backup statistics
redisctl cloud database backup-status \
  --database-id 42:12345 \
  -o json \
  -q '{
    last_backup_age: ((now - last_backup.timestamp) / 86400 | floor),
    backup_size_mb: (last_backup.size_bytes / 1048576 | floor),
    next_backup: next_scheduled,
    status: last_backup.status
  }'
```

### Alert on Backup Failures

```bash
#!/bin/bash
# check-backup-health.sh

SUBSCRIPTION_ID=42
DATABASE_ID=12345
MAX_AGE_HOURS=36

BACKUP_STATUS=$(redisctl cloud database backup-status \
  --database-id ${SUBSCRIPTION_ID}:${DATABASE_ID} \
  -o json)

LAST_BACKUP_TIME=$(echo "$BACKUP_STATUS" | jq -r '.last_backup.timestamp')
BACKUP_STATUS=$(echo "$BACKUP_STATUS" | jq -r '.last_backup.status')

# Calculate age in hours
CURRENT_TIME=$(date +%s)
BACKUP_TIME=$(date -d "$LAST_BACKUP_TIME" +%s)
AGE_HOURS=$(( ($CURRENT_TIME - $BACKUP_TIME) / 3600 ))

if [ "$BACKUP_STATUS" != "completed" ] || [ $AGE_HOURS -gt $MAX_AGE_HOURS ]; then
    echo "ALERT: Backup health check failed!"
    echo "Status: $BACKUP_STATUS"
    echo "Age: $AGE_HOURS hours"
    # Send alert (email, Slack, PagerDuty, etc.)
    exit 1
fi

echo "Backup health OK - Last backup: $AGE_HOURS hours ago"
```

## Disaster Recovery Plan

### 1. Document Current State

```bash
# Save current database configuration
redisctl cloud database get \
  --subscription-id 42 \
  --database-id 12345 \
  -o json > database-config-$(date +%Y%m%d).json

# Record backup details
redisctl cloud database backup-status \
  --database-id 42:12345 \
  -o json > backup-status-$(date +%Y%m%d).json
```

### 2. Test Recovery Procedure

Regularly test your restore process:

```bash
# Quarterly DR test
./scripts/dr-test.sh production-db test-restore-db
```

### 3. Recovery Time Objective (RTO)

Estimate restore time based on database size:
- Small (< 1GB): 5-10 minutes
- Medium (1-10GB): 15-30 minutes
- Large (> 10GB): 30-60+ minutes

## Common Issues

### Backup Takes Too Long

```
Error: Backup timed out after 300 seconds
```

**Solution:** Increase timeout for large databases:
```bash
redisctl cloud database backup \
  --database-id 42:12345 \
  --wait \
  --wait-timeout 1800  # 30 minutes
```

### Restore Fails with "Backup Not Found"

```
Error: Backup ID not found
```

**Solution:** List available backups and verify ID:
```bash
redisctl cloud database backup-status \
  --database-id 42:12345 \
  -q 'last_backup.backup_id'
```

### Insufficient Storage for Backup

```
Error: Insufficient storage space
```

**Solution:** 
1. Review backup retention policy
2. Clean up old backups
3. Upgrade storage capacity
4. Use custom storage (S3/GCS/Azure)

### Restored Database Has Missing Data

**Troubleshooting:**
1. Check backup timestamp vs. expected data
2. Verify AOF persistence was enabled
3. Check if backup completed successfully
4. Consider point-in-time recovery if available

## Best Practices

1. **Enable Persistence:** Always use AOF or snapshot persistence
2. **Multiple Backup Windows:** Daily automated + manual before changes
3. **Test Restores:** Regularly verify backups can be restored
4. **Off-Site Backups:** Use custom storage in different region
5. **Monitor Backup Age:** Alert if backups are too old
6. **Document Procedures:** Maintain runbooks for recovery
7. **Verify Backup Size:** Sudden size changes may indicate issues

## Next Steps

- [Database Migration](database-migration.md) - Migrate data between databases
- [Monitor Performance](../common/monitor-performance.md) - Track database health
- [Configure ACLs](configure-acls.md) - Secure your restored database
- [Setup High Availability](setup-replication.md) - Add redundancy

## See Also

- [Database Backup Reference](../../cloud/core-resources/databases.md#backup-operations) - Complete command documentation
- [Redis Cloud Backup Guide](https://redis.io/docs/latest/operate/rc/databases/back-up-data/) - Official backup documentation
- [Data Persistence Options](https://redis.io/docs/latest/operate/oss_and_stack/management/persistence/) - Understanding AOF vs. RDB
