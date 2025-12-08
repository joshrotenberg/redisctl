# Active-Active Databases (CRDB)

Active-Active databases (also known as CRDBs - Conflict-free Replicated Databases) enable geo-distributed deployments with local read/write latency and automatic conflict resolution.

## Commands Overview

```bash
redisctl enterprise crdb --help
```

## List CRDBs

```bash
# List all Active-Active databases
redisctl enterprise crdb list

# Output as JSON
redisctl enterprise crdb list -o json
```

## Get CRDB Details

```bash
# Get details for a specific CRDB
redisctl enterprise crdb get <crdb_guid>

# Get specific fields
redisctl enterprise crdb get <crdb_guid> -q '{name: name, instances: instances}'
```

## Create CRDB

```bash
# Create a new Active-Active database
redisctl enterprise crdb create --data '{
  "name": "geo-cache",
  "memory_size": 1073741824,
  "port": 12000,
  "instances": [
    {"cluster": {"url": "https://cluster1:9443"}},
    {"cluster": {"url": "https://cluster2:9443"}}
  ]
}'
```

## Update CRDB

```bash
# Update CRDB configuration
redisctl enterprise crdb update <crdb_guid> --data '{
  "memory_size": 2147483648
}'
```

## Delete CRDB

```bash
# Delete an Active-Active database
redisctl enterprise crdb delete <crdb_guid>
```

## Cluster Management

### View Participating Clusters

```bash
# Get all clusters participating in the CRDB
redisctl enterprise crdb get-clusters <crdb_guid>
```

### Add a Cluster

```bash
# Add a new cluster to the CRDB
redisctl enterprise crdb add-cluster <crdb_guid> --data '{
  "cluster": {"url": "https://cluster3:9443"}
}'
```

### Remove a Cluster

```bash
# Remove a cluster from the CRDB
redisctl enterprise crdb remove-cluster <crdb_guid> <cluster_id>
```

### Update Cluster Configuration

```bash
# Update a cluster's configuration within the CRDB
redisctl enterprise crdb update-cluster <crdb_guid> <cluster_id> --data '{...}'
```

## Instance Management

```bash
# Get all CRDB instances
redisctl enterprise crdb get-instances <crdb_guid>

# Get specific instance
redisctl enterprise crdb get-instance <crdb_guid> <instance_id>

# Update instance
redisctl enterprise crdb update-instance <crdb_guid> <instance_id> --data '{...}'

# Flush instance data
redisctl enterprise crdb flush-instance <crdb_guid> <instance_id>
```

## Replication

### Monitor Replication

```bash
# Get replication status
redisctl enterprise crdb get-replication-status <crdb_guid>

# Get replication lag metrics
redisctl enterprise crdb get-lag <crdb_guid>
```

### Control Replication

```bash
# Force synchronization
redisctl enterprise crdb force-sync <crdb_guid>

# Pause replication
redisctl enterprise crdb pause-replication <crdb_guid>

# Resume replication
redisctl enterprise crdb resume-replication <crdb_guid>
```

## Conflict Resolution

```bash
# Get conflict history
redisctl enterprise crdb get-conflicts <crdb_guid>

# Get conflict resolution policy
redisctl enterprise crdb get-conflict-policy <crdb_guid>

# Update conflict resolution policy
redisctl enterprise crdb update-conflict-policy <crdb_guid> --data '{...}'

# Manually resolve a conflict
redisctl enterprise crdb resolve-conflict <crdb_guid> <conflict_id> --data '{...}'
```

## Statistics and Metrics

```bash
# Get CRDB statistics
redisctl enterprise crdb stats <crdb_guid>

# Get detailed metrics
redisctl enterprise crdb metrics <crdb_guid>

# Get connection details per instance
redisctl enterprise crdb get-connections <crdb_guid>

# Get throughput metrics
redisctl enterprise crdb get-throughput <crdb_guid>
```

## Health and Operations

```bash
# Run health check
redisctl enterprise crdb health-check <crdb_guid>

# Create backup
redisctl enterprise crdb backup <crdb_guid>

# List available backups
redisctl enterprise crdb get-backups <crdb_guid>

# Restore from backup
redisctl enterprise crdb restore <crdb_guid> --data '{...}'

# Export CRDB data
redisctl enterprise crdb export <crdb_guid>
```

## Task Management

CRDB operations are often asynchronous:

```bash
# Get all CRDB tasks
redisctl enterprise crdb get-tasks <crdb_guid>

# Get specific task details
redisctl enterprise crdb get-task <crdb_guid> <task_id>

# Retry a failed task
redisctl enterprise crdb retry-task <crdb_guid> <task_id>

# Cancel a running task
redisctl enterprise crdb cancel-task <crdb_guid> <task_id>
```

## See Also

- [CRDB Tasks](crdb-tasks.md) - Detailed task management for Active-Active operations
