# Redis Cloud Examples

Real-world examples of managing Redis Cloud resources.

## Database Lifecycle

### Create a Production Database

```bash
# 1. List available subscriptions
redisctl cloud subscription list -o table

# 2. Create the database
redisctl api cloud post /subscriptions/12345/databases \
  --data '{
    "name": "production-cache",
    "memoryLimitInGb": 5,
    "throughputMeasurement": {
      "by": "operations-per-second",
      "value": 10000
    },
    "modules": ["RedisJSON", "RediSearch"],
    "replication": true,
    "dataPersistence": "aof-every-1-second",
    "alerts": [
      {
        "name": "high-memory",
        "value": 80
      }
    ]
  }'

# 3. Check creation status
redisctl api cloud get /tasks/<task-id>

# 4. Get connection details
redisctl cloud database get \
  --subscription-id 12345 \
  --database-id 67890 \
  -q '{endpoint: publicEndpoint, password: password}'
```

### Backup and Restore

```bash
# Create manual backup
redisctl cloud backup create \
  --subscription-id 12345 \
  --database-id 67890

# List available backups
redisctl cloud backup list \
  --subscription-id 12345 \
  --database-id 67890 \
  -o table

# Restore from backup
redisctl cloud backup restore \
  --subscription-id 12345 \
  --database-id 67890 \
  --backup-id <backup-id>
```

## User Management

### Set Up Team Access

```bash
# Create team members
redisctl cloud user create \
  --email "dev@company.com" \
  --role "db-member"

redisctl cloud user create \
  --email "ops@company.com" \
  --role "db-viewer"

# List all users
redisctl cloud user list -q "[].{email:email,role:role}" -o table

# Update user role
redisctl cloud user update <user-id> --role "admin"
```

## Monitoring and Metrics

### Get Database Metrics

```bash
# Get current stats
redisctl api cloud get /subscriptions/12345/databases/67890/metrics \
  -q '{
    cpu: cpuUsagePercentage,
    memory: memoryUsagePercentage,
    connections: connectionsCount,
    ops: operationsPerSecond
  }'

# Monitor database status
watch -n 5 'redisctl cloud database get \
  --subscription-id 12345 \
  --database-id 67890 \
  -q "{status:status,memory:memoryUsagePercentage}"'
```

## Migration Scenarios

### Export Database List

```bash
# Export all databases to JSON
redisctl cloud subscription list -q "[].id" | \
while read sub_id; do
  redisctl cloud database list --subscription-id $sub_id
done > all-databases.json

# Create summary report
redisctl cloud subscription list | \
jq -r '.[] | 
  "\(.name): \(.numberOfDatabases) databases, \(.status)"'
```

### Bulk Operations

```bash
# Scale all databases in subscription
for db_id in $(redisctl cloud database list --subscription-id 12345 -q "[].id" -r); do
  redisctl cloud database update \
    --subscription-id 12345 \
    --database-id $db_id \
    --memory-limit 2048
done

# Add module to multiple databases
redisctl cloud database list --subscription-id 12345 -q "[].id" | \
while read db_id; do
  redisctl api cloud patch /subscriptions/12345/databases/$db_id \
    --data '{"modules": ["RedisJSON", "RediSearch", "RedisTimeSeries"]}'
done
```

## Cost Management

### Analyze Costs

```bash
# Get subscription costs
redisctl api cloud get /subscriptions/12345/pricing \
  -q '{
    total: totalPrice,
    databases: databases[].{
      name: name,
      cost: price
    }
  }' -o yaml

# Find most expensive databases
redisctl cloud database list --subscription-id 12345 \
  -q "reverse(sort_by([].{name:name,memory:memoryLimitInGb}, &memory))[:5]" \
  -o table
```

## Security

### Set Up ACLs

```bash
# Create read-only ACL
redisctl cloud acl create \
  --subscription-id 12345 \
  --name "readonly-access" \
  --rule "+get +mget +exists +scan +xread -flushdb -flushall -keys"

# Create write-limited ACL
redisctl cloud acl create \
  --subscription-id 12345 \
  --name "app-access" \
  --rule "+@all -@dangerous -flushdb -flushall -keys -config"

# Apply ACL to database
redisctl api cloud patch /subscriptions/12345/databases/67890 \
  --data '{"redisAclId": "<acl-id>"}'
```

## Automation Scripts

### Health Check Script

```bash
#!/bin/bash
# Check all database health

redisctl cloud subscription list -q "[].id" | while read sub_id; do
  echo "Checking subscription $sub_id..."
  
  redisctl cloud database list --subscription-id $sub_id \
    -q "[?status!='active'].{name:name,status:status}" | \
  jq -r '.[] | "  WARNING: \(.name) is \(.status)"'
done
```

### Daily Report

```bash
#!/bin/bash
# Generate daily report

echo "Redis Cloud Daily Report - $(date)"
echo "========================"

# Account summary
echo -e "\nAccount:"
redisctl cloud account info -q '{owner:owner,id:id}'

# Subscription summary
echo -e "\nSubscriptions:"
redisctl cloud subscription list \
  -q "[].{name:name,databases:numberOfDatabases,status:status}" \
  -o table

# Database summary
echo -e "\nActive Databases:"
for sub_id in $(redisctl cloud subscription list -q "[].id" -r); do
  redisctl cloud database list --subscription-id $sub_id \
    -q "[?status=='active'] | length(@)"
done | awk '{sum+=$1} END {print sum}'

# Alert summary
echo -e "\nRecent Alerts:"
redisctl api cloud get /logs?type=alert&limit=10 \
  -q "[:5].{time:timestamp,message:message}" \
  -o table
```