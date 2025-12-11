# Databases

Manage Redis Cloud databases within subscriptions.

## Commands

### List Databases

List all databases in a subscription.

```bash
redisctl cloud database list --subscription-id <ID> [OPTIONS]
```

**Required Options:**
- `--subscription-id <ID>` - The subscription ID

**Options:**
- `-o, --output <FORMAT>` - Output format: json, yaml, or table
- `-q, --query <JMESPATH>` - JMESPath query to filter output

**Examples:**

```bash
# List all databases in subscription
redisctl cloud database list --subscription-id 123456

# Show specific fields in table format
redisctl cloud database list --subscription-id 123456 -o table

# Filter active databases only
redisctl cloud database list --subscription-id 123456 -q "[?status=='active']"

# Get database names and endpoints
redisctl cloud database list --subscription-id 123456 \
  -q "[].{name: name, endpoint: publicEndpoint}"
```

### Get Database

Get details of a specific database.

```bash
redisctl cloud database get --subscription-id <SUB_ID> --database-id <DB_ID> [OPTIONS]
```

**Required Options:**
- `--subscription-id <SUB_ID>` - The subscription ID
- `--database-id <DB_ID>` - The database ID

**Examples:**

```bash
# Get database details
redisctl cloud database get --subscription-id 123456 --database-id 789

# Get connection details
redisctl cloud database get --subscription-id 123456 --database-id 789 \
  -q "{endpoint: publicEndpoint, port: port, password: password}"
```

### Create Database

Create a new database in a subscription.

```bash
redisctl cloud database create --subscription-id <ID> --data <JSON> [OPTIONS]
```

**Required Options:**
- `--subscription-id <ID>` - The subscription ID
- `--data <JSON>` - Database configuration (inline or @file.json)

**Async Options:**
- `--wait` - Wait for database creation to complete
- `--wait-timeout <SECONDS>` - Maximum time to wait (default: 600)
- `--wait-interval <SECONDS>` - Polling interval (default: 10)

**Example Payload:**

```json
{
  "name": "production-cache",
  "memoryLimitInGb": 4,
  "protocol": "redis",
  "port": 10000,
  "throughputMeasurement": {
    "by": "operations-per-second",
    "value": 25000
  },
  "replication": true,
  "dataPersistence": "aof-every-write",
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
    }
  ]
}
```

**Examples:**

```bash
# Create database from file
redisctl cloud database create --subscription-id 123456 --data @database.json

# Create and wait for completion
redisctl cloud database create --subscription-id 123456 --data @database.json --wait

# Create minimal database
redisctl cloud database create --subscription-id 123456 --data '{
  "name": "test-db",
  "memoryLimitInGb": 1
}'
```

### Update Database

Update database configuration.

```bash
redisctl cloud database update --subscription-id <SUB_ID> --database-id <DB_ID> --data <JSON> [OPTIONS]
```

**Required Options:**
- `--subscription-id <SUB_ID>` - The subscription ID
- `--database-id <DB_ID>` - The database ID
- `--data <JSON>` - Updates to apply

**Async Options:**
- `--wait` - Wait for update to complete
- `--wait-timeout <SECONDS>` - Maximum time to wait
- `--wait-interval <SECONDS>` - Polling interval

**Examples:**

```bash
# Increase memory limit
redisctl cloud database update \
  --subscription-id 123456 \
  --database-id 789 \
  --data '{"memoryLimitInGb": 8}' \
  --wait

# Update eviction policy
redisctl cloud database update \
  --subscription-id 123456 \
  --database-id 789 \
  --data '{"dataEvictionPolicy": "volatile-lru"}'

# Add modules
redisctl cloud database update \
  --subscription-id 123456 \
  --database-id 789 \
  --data '{"modules": [{"name": "RedisTimeSeries"}]}'
```

### Delete Database

Delete a database.

```bash
redisctl cloud database delete --subscription-id <SUB_ID> --database-id <DB_ID> [OPTIONS]
```

**Required Options:**
- `--subscription-id <SUB_ID>` - The subscription ID
- `--database-id <DB_ID>` - The database ID

**Async Options:**
- `--wait` - Wait for deletion to complete

**Examples:**

```bash
# Delete database
redisctl cloud database delete --subscription-id 123456 --database-id 789

# Delete and wait for completion
redisctl cloud database delete --subscription-id 123456 --database-id 789 --wait
```

## Database Operations

### Backup Database

Create a manual backup.

```bash
redisctl cloud database backup --subscription-id <SUB_ID> --database-id <DB_ID> [OPTIONS]
```

**Examples:**

```bash
# Create backup
redisctl cloud database backup --subscription-id 123456 --database-id 789

# Create and wait
redisctl cloud database backup --subscription-id 123456 --database-id 789 --wait
```

### Import Data

Import data from a backup.

```bash
redisctl cloud database import --subscription-id <SUB_ID> --database-id <DB_ID> --data <JSON> [OPTIONS]
```

**Example Payload:**

```json
{
  "sourceType": "s3",
  "importFromUri": ["s3://bucket/backup.rdb"],
  "s3Credentials": {
    "accessKey": "AWS_ACCESS_KEY",
    "secretKey": "AWS_SECRET_KEY"
  }
}
```

### Export Data

Export database data.

```bash
redisctl cloud database export --subscription-id <SUB_ID> --database-id <DB_ID> --data <JSON> [OPTIONS]
```

## Fixed Databases

Fixed databases run on reserved infrastructure.

### List Fixed Databases

```bash
redisctl cloud fixed-database list --subscription-id <ID>
```

### Create Fixed Database

```bash
redisctl cloud fixed-database create --subscription-id <ID> --data @fixed-db.json --wait
```

## Active-Active Databases

Multi-region Active-Active (CRDB) databases.

### Create Active-Active Database

```bash
redisctl cloud database create-active-active --subscription-id <ID> --data @crdb.json --wait
```

**Example Payload:**

```json
{
  "name": "global-cache",
  "memoryLimitInGb": 10,
  "regions": [
    {
      "region": "us-east-1",
      "localThroughputMeasurement": {
        "by": "operations-per-second",
        "value": 10000
      }
    },
    {
      "region": "eu-west-1",
      "localThroughputMeasurement": {
        "by": "operations-per-second",
        "value": 10000
      }
    }
  ]
}
```

## Common Patterns

### Get Database Connection String

```bash
# Get Redis URI
DB=$(redisctl cloud database get --subscription-id 123456 --database-id 789)
echo "redis://:$(echo $DB | jq -r .password)@$(echo $DB | jq -r .publicEndpoint)"
```

### Monitor Database Metrics

```bash
# Check memory usage
redisctl cloud database get --subscription-id 123456 --database-id 789 \
  -q "{used: usedMemoryInMB, limit: memoryLimitInGB}" | \
  jq -r '"Memory: \(.used)MB / \(.limit)GB"'
```

### Bulk Operations

```bash
# Update all databases in subscription
for db in $(redisctl cloud database list --subscription-id 123456 -q "[].databaseId" | jq -r '.[]'); do
  echo "Updating database $db"
  redisctl cloud database update \
    --subscription-id 123456 \
    --database-id $db \
    --data '{"alerts": [{"name": "dataset-size", "value": 90}]}'
done
```

## Troubleshooting

### Common Issues

**"Database creation failed"**
- Check subscription has available resources
- Verify region supports requested features
- Check module compatibility

**"Cannot connect to database"**
- Verify security group/firewall rules
- Check if database is active: `status == 'active'`
- Ensure correct endpoint and port

**"Module not available"**
- Some modules require specific Redis versions
- Check module compatibility in subscription settings

## Related Commands

- [Subscriptions](./subscriptions.md) - Manage parent subscriptions
- ACL - Configure access control
- Connectivity - Set up VPC peering

## API Reference

These commands use the following REST endpoints:
- `GET /v1/subscriptions/{subId}/databases` - List databases
- `GET /v1/subscriptions/{subId}/databases/{dbId}` - Get database
- `POST /v1/subscriptions/{subId}/databases` - Create database
- `PUT /v1/subscriptions/{subId}/databases/{dbId}` - Update database
- `DELETE /v1/subscriptions/{subId}/databases/{dbId}` - Delete database

For direct API access: `redisctl api cloud get /subscriptions/123456/databases`