# Managing Production Databases

This tutorial covers best practices for managing Redis databases in production using redisctl.

## Prerequisites

- redisctl installed and configured
- Appropriate API credentials with production access
- Understanding of Redis concepts (memory, persistence, replication)

## Setting Up Production Profiles

First, create separate profiles for different environments:

```bash
# Development environment
redisctl profile set dev-cloud \
  --deployment cloud \
  --api-key "$DEV_API_KEY" \
  --api-secret "$DEV_API_SECRET"

# Staging environment  
redisctl profile set staging-cloud \
  --deployment cloud \
  --api-key "$STAGING_API_KEY" \
  --api-secret "$STAGING_API_SECRET"

# Production environment
redisctl profile set prod-cloud \
  --deployment cloud \
  --api-key "$PROD_API_KEY" \
  --api-secret "$PROD_API_SECRET"

# Set production as default
redisctl profile default prod-cloud
```

## Creating a Production Database

### Step 1: Prepare Database Configuration

Create a production database configuration file:

```json
{
  "name": "prod-cache-01",
  "memoryLimitInGb": 16,
  "protocol": "redis",
  "port": 10000,
  "throughputMeasurement": {
    "by": "operations-per-second",
    "value": 100000
  },
  "replication": true,
  "dataPersistence": "aof-every-1-second",
  "dataEvictionPolicy": "allkeys-lru",
  "modules": [
    {
      "name": "RedisJSON"
    },
    {
      "name": "RediSearch"
    }
  ],
  "alerts": [
    {
      "name": "dataset-size",
      "value": 80
    },
    {
      "name": "throughput-higher-than",
      "value": 90000
    },
    {
      "name": "throughput-lower-than",
      "value": 1000
    },
    {
      "name": "latency",
      "value": 5
    }
  ],
  "backup": {
    "interval": 6,
    "enabled": true
  },
  "clustering": {
    "enabled": true,
    "shardCount": 3
  }
}
```

### Step 2: Create the Database

```bash
# Create database and wait for completion
redisctl cloud database create \
  --subscription-id 123456 \
  --data @prod-database.json \
  --wait \
  --wait-timeout 900

# Verify creation
redisctl cloud database list --subscription-id 123456 -o table
```

### Step 3: Configure Network Access

Set up VPC peering for secure access:

```bash
# Create VPC peering
redisctl cloud connectivity create-vpc \
  --subscription-id 123456 \
  --data @vpc-peering.json \
  --wait

# Verify connection
redisctl cloud connectivity list-vpc --subscription-id 123456
```

## Monitoring Production Databases

### Health Checks

Create a monitoring script:

```bash
#!/bin/bash
# monitor-redis.sh

PROFILE="prod-cloud"
SUBSCRIPTION_ID="123456"

# Check all databases
DATABASES=$(redisctl --profile $PROFILE cloud database list \
  --subscription-id $SUBSCRIPTION_ID \
  -q "[].{id: databaseId, name: name, status: status}")

# Iterate over databases using JMESPath to extract fields
for db_id in $(redisctl --profile $PROFILE cloud database list \
  --subscription-id $SUBSCRIPTION_ID \
  -q "[?status!='active'].databaseId" --raw); do
  
  DB_INFO=$(redisctl --profile $PROFILE cloud database get \
    --subscription-id $SUBSCRIPTION_ID \
    --database-id $db_id \
    -q "{name: name, status: status}")
  
  NAME=$(echo $DB_INFO | jq -r .name)
  STATUS=$(echo $DB_INFO | jq -r .status)
  
  echo "ALERT: Database $NAME ($db_id) is not active: $STATUS"
  # Send alert (PagerDuty, Slack, etc.)
done

# Check memory usage - get all database IDs with JMESPath
for db_id in $(redisctl --profile $PROFILE cloud database list \
  --subscription-id $SUBSCRIPTION_ID \
  -q "[].databaseId" --raw); do
  
  # Get memory stats using JMESPath
  MEMORY_USED=$(redisctl --profile $PROFILE cloud database get \
    --subscription-id $SUBSCRIPTION_ID \
    --database-id $db_id \
    -q "memoryUsageInMB" --raw)
  
  MEMORY_LIMIT=$(redisctl --profile $PROFILE cloud database get \
    --subscription-id $SUBSCRIPTION_ID \
    --database-id $db_id \
    -q "memoryLimitInGB" --raw)
  
  MEMORY_LIMIT_MB=$((MEMORY_LIMIT * 1024))
  USAGE_PERCENT=$((MEMORY_USED * 100 / MEMORY_LIMIT_MB))
  
  if [ $USAGE_PERCENT -gt 80 ]; then
    echo "WARNING: Database $db_id memory usage at ${USAGE_PERCENT}%"
  fi
done
```

### Performance Metrics

Track key performance indicators:

```bash
# Get database metrics
redisctl cloud database get \
  --subscription-id 123456 \
  --database-id 789 \
  -q "{
    name: name,
    ops: throughputMeasurement.value,
    connections: connectionsUsed,
    memory: memoryUsageInMB,
    evicted: evictedObjects
  }"

# Monitor over time
while true; do
  redisctl cloud database get \
    --subscription-id 123456 \
    --database-id 789 \
    -q "throughputMeasurement.value" >> ops.log
  sleep 60
done
```

## Scaling Operations

### Vertical Scaling (Resize)

```bash
# Increase memory limit
redisctl cloud database update \
  --subscription-id 123456 \
  --database-id 789 \
  --data '{"memoryLimitInGb": 32}' \
  --wait

# Increase throughput
redisctl cloud database update \
  --subscription-id 123456 \
  --database-id 789 \
  --data '{
    "throughputMeasurement": {
      "by": "operations-per-second",
      "value": 200000
    }
  }' \
  --wait
```

### Horizontal Scaling (Sharding)

For Redis Enterprise:

```bash
# Add shards
redisctl enterprise database update \
  --database-id 1 \
  --data '{"shardCount": 5}' \
  --wait
```

## Backup and Recovery

### Automated Backups

Configure backup schedule:

```bash
# Enable backups every 4 hours
redisctl cloud database update \
  --subscription-id 123456 \
  --database-id 789 \
  --data '{
    "backup": {
      "enabled": true,
      "interval": 4
    }
  }'
```

### Manual Backups

```bash
# Create manual backup before maintenance
redisctl cloud database backup \
  --subscription-id 123456 \
  --database-id 789 \
  --wait

# List available backups
redisctl cloud database list-backups \
  --subscription-id 123456 \
  --database-id 789
```

### Restore from Backup

```bash
# Prepare import configuration
cat > import.json <<EOF
{
  "sourceType": "s3",
  "importFromUri": ["s3://backup-bucket/backup-2024-01-15.rdb"],
  "s3Credentials": {
    "accessKey": "$AWS_ACCESS_KEY",
    "secretKey": "$AWS_SECRET_KEY"
  }
}
EOF

# Import data
redisctl cloud database import \
  --subscription-id 123456 \
  --database-id 789 \
  --data @import.json \
  --wait
```

## Maintenance Operations

### Rolling Updates

Update databases with zero downtime:

```bash
#!/bin/bash
# rolling-update.sh

DATABASES=(789 790 791)
UPDATE='{"dataEvictionPolicy": "volatile-lru"}'

for db_id in "${DATABASES[@]}"; do
  echo "Updating database $db_id..."
  
  # Remove from load balancer
  remove_from_lb $db_id
  
  # Update database
  redisctl cloud database update \
    --subscription-id 123456 \
    --database-id $db_id \
    --data "$UPDATE" \
    --wait
  
  # Health check
  while true; do
    STATUS=$(redisctl cloud database get \
      --subscription-id 123456 \
      --database-id $db_id \
      -q "status")
    
    if [ "$STATUS" = "active" ]; then
      break
    fi
    sleep 10
  done
  
  # Add back to load balancer
  add_to_lb $db_id
  
  echo "Database $db_id updated successfully"
  sleep 30  # Wait before next update
done
```

### Module Management

Add or update modules:

```bash
# Add RedisTimeSeries module
redisctl cloud database update \
  --subscription-id 123456 \
  --database-id 789 \
  --data '{
    "modules": [
      {"name": "RedisJSON"},
      {"name": "RediSearch"},
      {"name": "RedisTimeSeries"}
    ]
  }' \
  --wait
```

## Security Best Practices

### Access Control

Configure ACL rules:

```bash
# Create ACL rule
redisctl cloud acl create-rule \
  --subscription-id 123456 \
  --database-id 789 \
  --data '{
    "name": "read-only-user",
    "rule": "+@read ~* -@dangerous"
  }'

# Create user with ACL
redisctl cloud acl create-user \
  --subscription-id 123456 \
  --database-id 789 \
  --data '{
    "username": "app-reader",
    "password": "$(openssl rand -base64 32)",
    "aclRule": "read-only-user"
  }'
```

### Password Rotation

```bash
#!/bin/bash
# rotate-passwords.sh

# Generate new password
NEW_PASSWORD=$(openssl rand -base64 32)

# Update database password
redisctl cloud database update \
  --subscription-id 123456 \
  --database-id 789 \
  --data "{\"password\": \"$NEW_PASSWORD\"}" \
  --wait

# Store in secret manager
aws secretsmanager update-secret \
  --secret-id redis-prod-password \
  --secret-string "$NEW_PASSWORD"

# Update application configuration
kubectl set secret redis-secret password="$NEW_PASSWORD"
```

## Troubleshooting Common Issues

### High Memory Usage

```bash
# Check memory stats
redisctl cloud database get \
  --subscription-id 123456 \
  --database-id 789 \
  -q "{
    used: memoryUsageInMB,
    limit: memoryLimitInGB,
    evicted: evictedObjects
  }"

# If evictions are happening, increase memory or adjust policy
redisctl cloud database update \
  --subscription-id 123456 \
  --database-id 789 \
  --data '{"memoryLimitInGb": 24}'
```

### Connection Issues

```bash
# Check connection limit using JMESPath
CONNECTIONS_USED=$(redisctl cloud database get \
  --subscription-id 123456 \
  --database-id 789 \
  -q "connectionsUsed" --raw)

CONNECTIONS_LIMIT=$(redisctl cloud database get \
  --subscription-id 123456 \
  --database-id 789 \
  -q "connectionsLimit" --raw)

if [ $CONNECTIONS_USED -gt $((CONNECTIONS_LIMIT * 80 / 100)) ]; then
  echo "Warning: Using $CONNECTIONS_USED of $CONNECTIONS_LIMIT connections"
  # Increase connection limit or investigate connection leaks
fi
```

### Performance Degradation

```bash
# Check slow log equivalent (through metrics)
redisctl cloud database get \
  --subscription-id 123456 \
  --database-id 789 \
  -q "{
    latency: latency,
    ops: throughputMeasurement.value,
    cpu: cpuUsagePercentage
  }"

# If CPU is high, consider sharding or upgrading
```

## Best Practices Summary

1. **Always use profiles** for different environments
2. **Enable replication** for production databases
3. **Configure appropriate persistence** (AOF or RDB)
4. **Set up monitoring and alerts** before issues occur
5. **Automate backups** and test restore procedures
6. **Use VPC peering** for secure network access
7. **Implement proper ACLs** for security
8. **Plan for scaling** before you need it
9. **Document your database configurations**
10. **Test changes in staging** before production

## Next Steps

- [Setting Up Monitoring](./monitoring.md)
- [Disaster Recovery](./disaster-recovery.md)
- [Network Security](./network-security.md)