# Enterprise Databases

Manage databases (BDBs) in Redis Enterprise clusters.

## Commands

### List Databases

```bash
redisctl enterprise database list [OPTIONS]
```

**Examples:**

```bash
# List all databases
redisctl enterprise database list

# Table format
redisctl enterprise database list -o table

# Get names and memory
redisctl enterprise database list -q "[].{name:name,memory:memory_size,status:status}"
```

### Get Database

```bash
redisctl enterprise database get <ID> [OPTIONS]
```

**Examples:**

```bash
# Get database details
redisctl enterprise database get 1

# Get connection info
redisctl enterprise database get 1 -q "{endpoint:endpoints[0].addr,port:port}"
```

### Create Database

```bash
redisctl enterprise database create [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--name <NAME>` | Database name |
| `--memory-size <BYTES>` | Memory limit in bytes |
| `--port <PORT>` | Database port (optional, auto-assigned) |
| `--replication` | Enable replication |
| `--shards-count <N>` | Number of shards |
| `--data <JSON>` | Full configuration as JSON |

**Examples:**

```bash
# Create with flags
redisctl enterprise database create \
  --name mydb \
  --memory-size 1073741824 \
  --port 12000 \
  --replication

# Create with JSON
redisctl enterprise database create --data '{
  "name": "mydb",
  "memory_size": 1073741824,
  "port": 12000,
  "replication": true,
  "shards_count": 2
}'

# Create with modules
redisctl enterprise database create --data '{
  "name": "search-db",
  "memory_size": 2147483648,
  "module_list": ["search"]
}'
```

### Update Database

```bash
redisctl enterprise database update <ID> --data <JSON> [OPTIONS]
```

**Examples:**

```bash
# Increase memory
redisctl enterprise database update 1 --data '{"memory_size": 2147483648}'

# Add modules
redisctl enterprise database update 1 --data '{
  "module_list": ["json", "search"]
}'

# Change eviction policy
redisctl enterprise database update 1 --data '{"eviction_policy": "volatile-lru"}'
```

### Delete Database

```bash
redisctl enterprise database delete <ID> [OPTIONS]
```

**Examples:**

```bash
# Delete database
redisctl enterprise database delete 1

# Force delete (skip confirmation)
redisctl enterprise database delete 1 --force
```

## Database Operations

### Backup Database

```bash
redisctl enterprise database backup <ID> [OPTIONS]
```

**Examples:**

```bash
# Trigger backup
redisctl enterprise database backup 1
```

### Import Data

```bash
redisctl enterprise database import <ID> --data <JSON>
```

**Example:**

```bash
redisctl enterprise database import 1 --data '{
  "source_type": "rdb_url",
  "source_url": "http://backup-server/backup.rdb"
}'
```

### Export Data

```bash
redisctl enterprise database export <ID> --data <JSON>
```

### Flush Database

```bash
redisctl enterprise database flush <ID>
```

**Warning:** This deletes all data in the database.

## Shards

### List Shards

```bash
redisctl enterprise database shards <ID>
```

### Get Shard Stats

```bash
redisctl enterprise database shard-stats <ID>
```

## Common Patterns

### Get Connection String

```bash
ENDPOINT=$(redisctl enterprise database get 1 -q 'endpoints[0].addr[0]')
PORT=$(redisctl enterprise database get 1 -q 'port')
echo "redis://$ENDPOINT:$PORT"
```

### Monitor Memory Usage

```bash
redisctl enterprise database list \
  -q "[].{name:name,used:used_memory,limit:memory_size}" \
  -o table
```

### Bulk Update Databases

```bash
# Update all databases
for id in $(redisctl enterprise database list -q '[].uid' --raw); do
  echo "Updating database $id"
  redisctl enterprise database update $id --data '{"eviction_policy": "volatile-lru"}'
done
```

### Create Database with Persistence

```bash
redisctl enterprise database create --data '{
  "name": "persistent-db",
  "memory_size": 1073741824,
  "data_persistence": "aof",
  "aof_policy": "appendfsync-every-sec"
}'
```

## Troubleshooting

### "Not enough memory"
- Check cluster has available memory
- Reduce memory_size or add nodes

### "Port already in use"
- Omit port to auto-assign
- Or specify a different port

### "Module not found"
- Upload module first: `redisctl enterprise module upload`
- Check module name is correct

## API Reference

REST endpoints:
- `GET /v1/bdbs` - List databases
- `POST /v1/bdbs` - Create database
- `GET /v1/bdbs/{id}` - Get database
- `PUT /v1/bdbs/{id}` - Update database
- `DELETE /v1/bdbs/{id}` - Delete database

For direct API access: `redisctl api enterprise get /v1/bdbs`
