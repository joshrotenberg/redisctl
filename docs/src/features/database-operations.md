# Database Operations

All database operations in redisctl support async tracking with `--wait` flags.

## Regular Databases

### Create Database
```bash
redisctl cloud database create --subscription-id 12345 \
  --data @database.json --wait

# With custom timeout for large databases
redisctl cloud database create --subscription-id 12345 \
  --data @large-db.json --wait --wait-timeout 1800
```

### Update Database
```bash
redisctl cloud database update --subscription-id 12345 \
  --database-id 67890 --data @updates.json --wait
```

### Delete Database
```bash
redisctl cloud database delete --subscription-id 12345 \
  --database-id 67890 --force --wait
```

### Import Data
```bash
redisctl cloud database import --subscription-id 12345 \
  --database-id 67890 --data @import.json --wait
```

### Backup Database
```bash
redisctl cloud database backup --subscription-id 12345 \
  --database-id 67890 --wait
```

### Migrate Database
```bash
redisctl cloud database migrate --subscription-id 12345 \
  --database-id 67890 --data @migration.json --wait
```

## Fixed Databases

Fixed databases use the same async patterns:

```bash
# Create fixed database
redisctl cloud fixed-database create --subscription-id 12345 \
  --data @fixed-db.json --wait

# Update fixed database
redisctl cloud fixed-database update --subscription-id 12345 \
  --database-id 67890 --data @updates.json --wait

# Delete fixed database
redisctl cloud fixed-database delete --subscription-id 12345 \
  --database-id 67890 --force --wait
```

## Active-Active Databases (CRDB)

Active-Active databases support full async operations:

```bash
# Create Active-Active database
redisctl cloud crdb create --subscription-id 12345 \
  --data @crdb.json --wait

# Update Active-Active database
redisctl cloud crdb update --subscription-id 12345 \
  --database-id 67890 --data @updates.json --wait

# Delete Active-Active database
redisctl cloud crdb delete --subscription-id 12345 \
  --database-id 67890 --force --wait
```

## Database Configuration Examples

### Basic Database Creation
```json
{
  "name": "my-database",
  "memoryLimitInGb": 1,
  "support": "redis",
  "dataPersistence": "none",
  "replication": false
}
```

### Production Database
```json
{
  "name": "prod-database",
  "memoryLimitInGb": 10,
  "support": "redis",
  "dataPersistence": "aof-every-1-second",
  "replication": true,
  "clustering": {
    "enabled": true,
    "shards": 3
  },
  "modules": [
    {
      "name": "RedisJSON"
    },
    {
      "name": "RediSearch"
    }
  ]
}
```

## Best Practices

### Timeout Recommendations
- Small databases (< 1GB): Default 600s
- Medium databases (1-10GB): 900-1200s
- Large databases (> 10GB): 1800s or more
- Migrations: Add 50% to creation time

### Error Recovery
```bash
#!/bin/bash
# Retry with exponential backoff
RETRY=0
MAX_RETRY=3

while [ $RETRY -lt $MAX_RETRY ]; do
  if redisctl cloud database create --subscription-id 12345 \
    --data @database.json --wait --wait-timeout $((600 * (RETRY + 1))); then
    echo "Success!"
    break
  fi
  RETRY=$((RETRY + 1))
  echo "Retry $RETRY of $MAX_RETRY..."
  sleep $((10 * RETRY))
done
```