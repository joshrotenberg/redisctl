# Node Management

Manage nodes in your Redis Enterprise cluster.

## Commands Overview

```bash
redisctl enterprise node --help
```

## List Nodes

```bash
# List all nodes in the cluster
redisctl enterprise node list

# Output as JSON
redisctl enterprise node list -o json

# Get specific fields
redisctl enterprise node list -q '[].{id: uid, addr: addr, status: status}'
```

## Get Node Details

```bash
# Get details for a specific node
redisctl enterprise node get <node_id>

# Get specific fields
redisctl enterprise node get <node_id> -o json | jq '{addr, status, total_memory, available_memory}'
```

## Add Node

```bash
# Add a new node to the cluster
redisctl enterprise node add --data '{
  "addr": "10.0.0.5",
  "username": "admin@redis.local",
  "password": "password"
}'
```

## Remove Node

```bash
# Remove a node from the cluster
redisctl enterprise node remove <node_id>
```

## Update Node Configuration

```bash
# Update node settings
redisctl enterprise node update <node_id> --data '{...}'

# Get current configuration
redisctl enterprise node get-config <node_id>

# Update configuration
redisctl enterprise node update-config <node_id> --data '{...}'
```

## Node Status and Health

```bash
# Get node status
redisctl enterprise node status <node_id>

# Run health check on node
redisctl enterprise node check <node_id>

# Get node-specific alerts
redisctl enterprise node alerts <node_id>
```

## Statistics and Metrics

```bash
# Get node statistics
redisctl enterprise node stats <node_id>

# Get detailed metrics
redisctl enterprise node metrics <node_id>
```

## Resource Utilization

```bash
# Get overall resource utilization
redisctl enterprise node resources <node_id>

# Get memory usage details
redisctl enterprise node memory <node_id>

# Get CPU usage details
redisctl enterprise node cpu <node_id>

# Get storage usage details
redisctl enterprise node storage <node_id>

# Get network statistics
redisctl enterprise node network <node_id>
```

## Maintenance Mode

```bash
# Put node in maintenance mode
redisctl enterprise node maintenance-enable <node_id>

# Remove node from maintenance mode
redisctl enterprise node maintenance-disable <node_id>
```

## Shard Operations

```bash
# Rebalance shards on node
redisctl enterprise node rebalance <node_id>

# Drain node before removal (moves all shards)
redisctl enterprise node drain <node_id>
```

## Service Control

```bash
# Restart node services
redisctl enterprise node restart <node_id>
```

## Rack Awareness

```bash
# Get rack awareness configuration
redisctl enterprise node get-rack <node_id>

# Set rack ID for a node
redisctl enterprise node set-rack <node_id> --data '{
  "rack_id": "rack-1"
}'
```

## Node Roles

```bash
# Get the node's role
redisctl enterprise node get-role <node_id>
```

## JSON Output Examples

```bash
# List all nodes with their addresses and status
redisctl enterprise node list -o json | jq '.[] | {uid, addr, status}'

# Find nodes with low memory
redisctl enterprise node list -o json | jq '.[] | select(.available_memory < 1073741824)'

# Get cluster node count
redisctl enterprise node list -o json | jq 'length'
```

## Common Operations

### Add a Node to the Cluster

```bash
# 1. Add the node
redisctl enterprise node add --data '{
  "addr": "10.0.0.5",
  "username": "admin@redis.local", 
  "password": "password"
}'

# 2. Check status
redisctl enterprise node status <new_node_id>

# 3. Optionally set rack awareness
redisctl enterprise node set-rack <new_node_id> --data '{"rack_id": "rack-2"}'
```

### Safely Remove a Node

```bash
# 1. Put in maintenance mode
redisctl enterprise node maintenance-enable <node_id>

# 2. Drain all shards
redisctl enterprise node drain <node_id>

# 3. Verify shards moved
redisctl enterprise node get <node_id> -o json | jq '.shards'

# 4. Remove the node
redisctl enterprise node remove <node_id>
```
