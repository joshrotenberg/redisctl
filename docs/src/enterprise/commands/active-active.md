# Enterprise Active-Active

Manage Active-Active (CRDB) databases for geo-distributed deployments.

## Overview

Active-Active databases replicate data across multiple clusters with conflict-free resolution. Each cluster can handle local reads and writes with automatic synchronization.

## CRDB Commands

### List CRDBs

```bash
redisctl enterprise crdb list [OPTIONS]
```

**Examples:**

```bash
# List all CRDBs
redisctl enterprise crdb list

# Table format
redisctl enterprise crdb list -o table

# Get names and status
redisctl enterprise crdb list -q "[].{name:name,guid:guid,status:status}"
```

### Get CRDB

```bash
redisctl enterprise crdb get <GUID> [OPTIONS]
```

**Examples:**

```bash
# Get CRDB details
redisctl enterprise crdb get abc-123-def

# Get instances
redisctl enterprise crdb get abc-123-def -q 'instances'
```

### Create CRDB

```bash
redisctl enterprise crdb create --data <JSON>
```

**Example Payload:**

```json
{
  "name": "global-cache",
  "memory_size": 1073741824,
  "port": 12000,
  "instances": [
    {
      "cluster": {
        "url": "https://cluster1.example.com:9443",
        "credentials": {
          "username": "admin@cluster.local",
          "password": "password1"
        }
      }
    },
    {
      "cluster": {
        "url": "https://cluster2.example.com:9443",
        "credentials": {
          "username": "admin@cluster.local",
          "password": "password2"
        }
      }
    }
  ]
}
```

**Examples:**

```bash
# Create from file
redisctl enterprise crdb create --data @crdb.json

# Simple two-cluster setup
redisctl enterprise crdb create --data '{
  "name": "geo-cache",
  "memory_size": 1073741824,
  "instances": [...]
}'
```

### Update CRDB

```bash
redisctl enterprise crdb update <GUID> --data <JSON>
```

**Examples:**

```bash
# Increase memory
redisctl enterprise crdb update abc-123 --data '{"memory_size": 2147483648}'

# Add instance
redisctl enterprise crdb update abc-123 --data '{
  "instances": [
    {"cluster": {"url": "https://cluster3.example.com:9443", ...}}
  ]
}'
```

### Delete CRDB

```bash
redisctl enterprise crdb delete <GUID>
```

## CRDB Tasks

Active-Active operations are asynchronous and managed through tasks.

### List CRDB Tasks

```bash
redisctl enterprise crdb-task list [OPTIONS]
```

**Examples:**

```bash
# All tasks
redisctl enterprise crdb-task list

# Tasks for specific CRDB
redisctl enterprise crdb-task list --crdb-guid abc-123

# Filter by status
redisctl enterprise crdb-task list -q "[?status=='completed']"
```

### Get CRDB Task

```bash
redisctl enterprise crdb-task get <TASK-ID>
```

**Example:**

```bash
# Check task status
redisctl enterprise crdb-task get task-123 -q '{status:status,progress:progress}'
```

### Cancel CRDB Task

```bash
redisctl enterprise crdb-task cancel <TASK-ID>
```

## Instance Management

### Get Instance Status

```bash
redisctl enterprise crdb get <GUID> -q 'instances[].{cluster:cluster.name,status:status}'
```

### Add Instance to CRDB

```bash
redisctl enterprise crdb update <GUID> --data '{
  "add_instances": [{
    "cluster": {
      "url": "https://new-cluster:9443",
      "credentials": {...}
    }
  }]
}'
```

### Remove Instance from CRDB

```bash
redisctl enterprise crdb update <GUID> --data '{
  "remove_instances": ["instance-id"]
}'
```

## Common Patterns

### Create Two-Region Active-Active

```bash
#!/bin/bash
# Create CRDB across two regions

cat > crdb-config.json << 'EOF'
{
  "name": "global-sessions",
  "memory_size": 2147483648,
  "port": 12000,
  "causal_consistency": false,
  "encryption": true,
  "instances": [
    {
      "cluster": {
        "url": "https://us-east.example.com:9443",
        "credentials": {
          "username": "admin@cluster.local",
          "password": "$US_EAST_PASSWORD"
        }
      }
    },
    {
      "cluster": {
        "url": "https://eu-west.example.com:9443",
        "credentials": {
          "username": "admin@cluster.local",
          "password": "$EU_WEST_PASSWORD"
        }
      }
    }
  ]
}
EOF

envsubst < crdb-config.json | redisctl enterprise crdb create --data @-
```

### Monitor CRDB Sync Status

```bash
# Check all instances are synced
redisctl enterprise crdb get abc-123 \
  -q 'instances[].{cluster:cluster.name,lag:sync_lag}' \
  -o table
```

### Wait for CRDB Task

```bash
TASK_ID=$(redisctl enterprise crdb create --data @crdb.json -q 'task_id')

while true; do
  STATUS=$(redisctl enterprise crdb-task get $TASK_ID -q 'status')
  echo "Status: $STATUS"
  
  case $STATUS in
    "completed") echo "Success!"; break ;;
    "failed") echo "Failed!"; exit 1 ;;
    *) sleep 10 ;;
  esac
done
```

## Conflict Resolution

Active-Active uses CRDTs (Conflict-free Replicated Data Types) for automatic conflict resolution:

| Data Type | Resolution |
|-----------|------------|
| Strings | Last-write-wins |
| Counters | Add/remove operations merge |
| Sets | Union of all operations |
| Sorted Sets | Union with max score |

## Troubleshooting

### "Instance not reachable"
- Check network connectivity between clusters
- Verify firewall allows CRDB ports
- Check cluster credentials

### "Sync lag increasing"
- Check network latency between clusters
- Verify cluster resources (CPU, memory)
- Check for large write volumes

### "Task stuck"
- Check all instances are healthy
- Cancel and retry: `redisctl enterprise crdb-task cancel`
- Check cluster logs for errors

### "Conflict resolution issues"
- Review data types being used
- Consider causal consistency if needed
- Check application logic for proper CRDT usage

## API Reference

REST endpoints:
- `GET /v1/crdbs` - List CRDBs
- `POST /v1/crdbs` - Create CRDB
- `GET /v1/crdbs/{guid}` - Get CRDB
- `PUT /v1/crdbs/{guid}` - Update CRDB
- `DELETE /v1/crdbs/{guid}` - Delete CRDB
- `GET /v1/crdb_tasks` - List tasks
- `GET /v1/crdb_tasks/{id}` - Get task

For direct API access: `redisctl api enterprise get /v1/crdbs`
