# Database Management

Manage Redis databases on your Redis Enterprise cluster.

## Commands Overview

```bash
redisctl enterprise database --help
```

## List Databases

```bash
# List all databases
redisctl enterprise database list

# Output as JSON
redisctl enterprise database list -o json

# Filter with JMESPath
redisctl enterprise database list -q '[].{name: name, uid: uid, status: status}'
```

## Get Database Details

```bash
# Get details for a specific database
redisctl enterprise database get <db_id>

# Get specific fields
redisctl enterprise database get <db_id> -q '{name: name, memory: memory_size, endpoints: endpoints}'
```

## Create Database

```bash
# Create a new database
redisctl enterprise database create --data '{
  "name": "my-database",
  "memory_size": 1073741824,
  "port": 12000
}'

# Create with modules
redisctl enterprise database create --data '{
  "name": "search-db",
  "memory_size": 2147483648,
  "module_list": [{"module_name": "search"}]
}'
```

## Update Database

```bash
# Update database configuration
redisctl enterprise database update <db_id> --data '{
  "memory_size": 2147483648
}'
```

## Delete Database

```bash
# Delete a database
redisctl enterprise database delete <db_id>
```

## Watch Database Status

Monitor database status changes in real-time:

```bash
# Watch for status changes
redisctl enterprise database watch <db_id>

# Watch with custom interval
redisctl enterprise database watch <db_id> --interval 5
```

## Import/Export

```bash
# Export database
redisctl enterprise database export <db_id> --data '{
  "export_location": "s3://bucket/path"
}'

# Import to database
redisctl enterprise database import <db_id> --data '{
  "import_location": "s3://bucket/path/dump.rdb"
}'
```

## Backup and Restore

```bash
# Trigger an immediate backup
redisctl enterprise database backup <db_id>

# Restore from backup
redisctl enterprise database restore <db_id> --data '{
  "backup_path": "/path/to/backup"
}'
```

## Data Operations

```bash
# Flush all data from database
redisctl enterprise database flush <db_id>
```

## Sharding

```bash
# Get shard information
redisctl enterprise database get-shards <db_id>

# Update sharding configuration
redisctl enterprise database update-shards <db_id> --data '{
  "shards_count": 4
}'
```

## Modules

```bash
# Get enabled modules
redisctl enterprise database get-modules <db_id>

# Update modules configuration
redisctl enterprise database update-modules <db_id> --data '{
  "module_list": [{"module_name": "search"}, {"module_name": "json"}]
}'
```

## Upgrade Redis Version

```bash
# Upgrade database to a new Redis version
redisctl enterprise database upgrade <db_id> --data '{
  "redis_version": "7.2"
}'
```

## ACL Configuration

```bash
# Get ACL configuration
redisctl enterprise database get-acl <db_id>

# Update ACL configuration
redisctl enterprise database update-acl <db_id> --data '{...}'
```

## Monitoring

```bash
# Get database statistics
redisctl enterprise database stats <db_id>

# Get detailed metrics
redisctl enterprise database metrics <db_id>

# Get slow query log
redisctl enterprise database slowlog <db_id>

# Get connected clients
redisctl enterprise database client-list <db_id>
```

## JMESPath Query Examples

```bash
# Get all database names and memory sizes
redisctl enterprise database list -q '[].{name: name, memory_size: memory_size}'

# Find databases using more than 1GB
redisctl enterprise database list -q "[?memory_size > \`1073741824\`]"

# Export database config for backup
redisctl enterprise database get <db_id> > db-config-backup.json
```
