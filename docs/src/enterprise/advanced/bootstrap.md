# Bootstrap Commands

Initialize and manage Redis Enterprise cluster setup.

## Overview

Bootstrap commands handle the initial setup and configuration of Redis Enterprise clusters, including node initialization, cluster creation, and joining existing clusters.

## Available Commands

### Get Bootstrap Status

```bash
redisctl enterprise bootstrap status
```

Returns the current bootstrap status and node information:
- Bootstrap state (not_started, in_progress, completed)
- Local node details (architecture, memory, storage paths)
- Available network addresses
- Supported database versions

### Create New Cluster

```bash
redisctl enterprise bootstrap create-cluster --data '{
  "cluster_name": "my-cluster",
  "rack_aware": false,
  "license": "...",
  "nodes": [...]
}'
```

Initialize a new Redis Enterprise cluster with the specified configuration.

### Join Existing Cluster

```bash
redisctl enterprise bootstrap join-cluster --data '{
  "cluster_address": "192.168.1.100",
  "username": "admin@redis.local",
  "password": "password",
  "replace_node": false
}'
```

Join this node to an existing Redis Enterprise cluster.

### Validate Configuration

```bash
# Validate cluster creation config
redisctl enterprise bootstrap validate create_cluster --data '{...}'

# Validate join cluster config
redisctl enterprise bootstrap validate join_cluster --data '{...}'
```

Pre-flight validation of bootstrap configurations before execution.

## Common Use Cases

### Initial Cluster Setup

```bash
# 1. Check bootstrap status
redisctl enterprise bootstrap status

# 2. Validate configuration
redisctl enterprise bootstrap validate create_cluster --data @cluster-config.json

# 3. Create the cluster
redisctl enterprise bootstrap create-cluster --data @cluster-config.json
```

### Adding Nodes to Cluster

```bash
# 1. On new node, check status
redisctl enterprise bootstrap status

# 2. Join the cluster
redisctl enterprise bootstrap join-cluster --data '{
  "cluster_address": "node1.redis.local",
  "username": "admin@redis.local",
  "password": "${REDIS_PASSWORD}"
}'
```

## Output Examples

### Bootstrap Status

```json
{
  "bootstrap_status": {
    "state": "completed",
    "start_time": "2025-09-15T00:18:27Z",
    "end_time": "2025-09-15T00:18:49Z"
  },
  "local_node_info": {
    "uid": "1",
    "architecture": "x86_64",
    "total_memory": 8217473024,
    "cores": 14,
    "persistent_storage_path": "/var/opt/redislabs/persist",
    "ephemeral_storage_path": "/var/opt/redislabs/tmp",
    "os_version": "Red Hat Enterprise Linux 9.6"
  }
}
```

## Important Notes

- Bootstrap operations are typically one-time actions during initial cluster setup
- Most bootstrap operations require root or sudo privileges
- Always validate configurations before applying them
- Bootstrap operations cannot be undone - ensure backups exist

## Related Commands

- Cluster Commands - Manage cluster after bootstrap
- Node Commands - Manage individual nodes
- Auth Commands - Configure authentication after bootstrap