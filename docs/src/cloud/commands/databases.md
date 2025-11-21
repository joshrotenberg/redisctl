# Databases

Manage Redis Cloud databases within subscriptions.

## Commands

### List Databases

List all databases across subscriptions or in a specific subscription.

```bash
redisctl cloud database list [OPTIONS]
```

**Options:**
- `--subscription <ID>` - Filter by subscription ID
- `-o, --output <FORMAT>` - Output format: json, yaml, or table
- `-q, --query <JMESPATH>` - JMESPath query to filter output

**Examples:**

```bash
# List all databases
redisctl cloud database list

# List databases in specific subscription
redisctl cloud database list --subscription 123456

# Table format
redisctl cloud database list -o table

# Filter active databases
redisctl cloud database list -q "[?status=='active']"

# Get names and endpoints
redisctl cloud database list -q "[].{name: name, endpoint: publicEndpoint}"
```

### Get Database

Get details of a specific database.

```bash
redisctl cloud database get <ID> [OPTIONS]
```

**Arguments:**
- `<ID>` - Database ID (format: `subscription_id:database_id`)

**Examples:**

```bash
# Get database details
redisctl cloud database get 123456:789

# Get connection details
redisctl cloud database get 123456:789 \
  -q "{endpoint: publicEndpoint, port: port, password: password}"
```

### Create Database

Create a new database in a subscription.

```bash
redisctl cloud database create --subscription <ID> [OPTIONS]
```

**Required:**
- `--subscription <ID>` - Subscription ID

**Options:**
- `--name <NAME>` - Database name
- `--memory <GB>` - Memory limit in GB
- `--protocol <PROTOCOL>` - redis or memcached
- `--data <JSON>` - Full configuration as JSON
- `--wait` - Wait for completion

**Examples:**

```bash
# Create with flags
redisctl cloud database create \
  --subscription 123456 \
  --name mydb \
  --memory 1

# Create with JSON data
redisctl cloud database create \
  --subscription 123456 \
  --data @database.json \
  --wait

# Create with inline JSON
redisctl cloud database create \
  --subscription 123456 \
  --data '{"name": "test-db", "memoryLimitInGb": 1}'
```

**Example JSON Payload:**

```json
{
  "name": "production-cache",
  "memoryLimitInGb": 4,
  "protocol": "redis",
  "replication": true,
  "dataPersistence": "aof-every-write",
  "modules": [
    {"name": "RedisJSON"},
    {"name": "RediSearch"}
  ]
}
```

### Update Database

Update database configuration.

```bash
redisctl cloud database update <ID> [OPTIONS]
```

**Arguments:**
- `<ID>` - Database ID (format: `subscription_id:database_id`)

**Options:**
- `--data <JSON>` - Updates to apply
- `--wait` - Wait for completion

**Examples:**

```bash
# Increase memory
redisctl cloud database update 123456:789 \
  --data '{"memoryLimitInGb": 8}' \
  --wait

# Update eviction policy
redisctl cloud database update 123456:789 \
  --data '{"dataEvictionPolicy": "volatile-lru"}'
```

### Delete Database

Delete a database.

```bash
redisctl cloud database delete <ID> [OPTIONS]
```

**Arguments:**
- `<ID>` - Database ID (format: `subscription_id:database_id`)

**Options:**
- `--wait` - Wait for deletion to complete

**Examples:**

```bash
# Delete database
redisctl cloud database delete 123456:789

# Delete and wait
redisctl cloud database delete 123456:789 --wait
```

## Database Operations

### Backup Database

Trigger a manual backup.

```bash
redisctl cloud database backup <ID> [OPTIONS]
```

**Examples:**

```bash
redisctl cloud database backup 123456:789
redisctl cloud database backup 123456:789 --wait
```

### Import Data

Import data into a database.

```bash
redisctl cloud database import <ID> --data <JSON> [OPTIONS]
```

**Example:**

```bash
redisctl cloud database import 123456:789 --data '{
  "sourceType": "s3",
  "importFromUri": ["s3://bucket/backup.rdb"]
}'
```

## Common Patterns

### Get Connection String

```bash
DB=$(redisctl cloud database get 123456:789)
ENDPOINT=$(echo $DB | jq -r '.publicEndpoint')
PASSWORD=$(echo $DB | jq -r '.password')
echo "redis://:$PASSWORD@$ENDPOINT"
```

### Monitor Databases

```bash
# Check memory usage across all databases
redisctl cloud database list \
  -q "[].{name: name, used: usedMemoryInMB, limit: memoryLimitInGb}" \
  -o table
```

### Bulk Operations

```bash
# List all database IDs
for db in $(redisctl cloud database list -q "[].databaseId" | jq -r '.[]'); do
  echo "Processing $db"
done
```

## Troubleshooting

### "Database creation failed"
- Check subscription has available resources
- Verify region supports requested features
- Check module compatibility

### "Cannot connect"
- Verify database is active: check `status` field
- Check firewall/security group rules
- Ensure correct endpoint and port

## API Reference

REST endpoints:
- `GET /v1/subscriptions/{subId}/databases` - List
- `POST /v1/subscriptions/{subId}/databases` - Create
- `GET /v1/subscriptions/{subId}/databases/{dbId}` - Get
- `PUT /v1/subscriptions/{subId}/databases/{dbId}` - Update
- `DELETE /v1/subscriptions/{subId}/databases/{dbId}` - Delete

For direct API access: `redisctl api cloud get /subscriptions/123456/databases`
