# Shard Management

Shards are the fundamental units of data storage and processing in Redis Enterprise. Each database is composed of one or more shards distributed across cluster nodes. The shard commands provide tools for monitoring, managing, and optimizing shard placement and performance.

## Overview

Shards in Redis Enterprise:
- **Primary building blocks** of databases
- **Distributed across nodes** for high availability
- **Replicated** for data redundancy
- **Can be migrated** between nodes for load balancing
- **Support failover** for high availability

## Available Commands

### List Shards

List all shards in the cluster with optional filtering:

```bash
# List all shards
redisctl enterprise shard list

# Filter by node
redisctl enterprise shard list --node 1

# Filter by database
redisctl enterprise shard list --database 1

# Filter by role (master/slave)
redisctl enterprise shard list --role master

# Combine filters
redisctl enterprise shard list --node 1 --role slave

# Output as table
redisctl enterprise shard list -o table
```

### Get Shard Details

Get detailed information about a specific shard:

```bash
# Get shard details
redisctl enterprise shard get <shard_uid>

# Get specific fields
redisctl enterprise shard get <shard_uid> -q "role"
redisctl enterprise shard get <shard_uid> -q "{uid: uid, node: node, role: role, status: status}"
```

### List Database Shards

List all shards for a specific database:

```bash
# List shards for database
redisctl enterprise shard list-by-database <bdb_uid>

# Get shard distribution
redisctl enterprise shard list-by-database <bdb_uid> -q "[].{shard: uid, node: node, role: role}"
```

### Shard Failover

Perform manual failover of a shard to its replica:

```bash
# Failover with confirmation
redisctl enterprise shard failover <shard_uid>

# Failover without confirmation
redisctl enterprise shard failover <shard_uid> --force
```

### Shard Migration

Migrate a shard to a different node:

```bash
# Migrate shard to target node
redisctl enterprise shard migrate <shard_uid> --target-node <node_uid>

# Migrate without confirmation
redisctl enterprise shard migrate <shard_uid> --target-node <node_uid> --force
```

### Bulk Operations

Perform failover or migration on multiple shards:

```bash
# Bulk failover from JSON file
redisctl enterprise shard bulk-failover --data @failover-list.json

# Bulk migration from stdin
echo '{"shards": [{"uid": 1, "target_node": 2}]}' | \
  redisctl enterprise shard bulk-migrate --data -

# Force bulk operations
redisctl enterprise shard bulk-failover --data @failover.json --force
```

### Shard Statistics

Get performance statistics for shards:

```bash
# Get stats for specific shard
redisctl enterprise shard stats <shard_uid>

# Get stats for all shards
redisctl enterprise shard stats

# Specify time interval
redisctl enterprise shard stats --interval 1hour
redisctl enterprise shard stats --interval 1day

# Specify time range
redisctl enterprise shard stats \
  --stime "2024-01-01T00:00:00Z" \
  --etime "2024-01-02T00:00:00Z"

# Get latest stats
redisctl enterprise shard stats-last

# Get latest stats for specific shard
redisctl enterprise shard stats-last <shard_uid> --interval 1sec
```

### Shard Health & Configuration

Check shard health and configuration:

```bash
# Check shard health
redisctl enterprise shard health <shard_uid>

# Get shard configuration
redisctl enterprise shard config <shard_uid>
```

## Shard Structure

A typical shard object contains:

```json
{
  "uid": 1,
  "bdb_uid": 1,
  "node": 1,
  "role": "master",
  "status": "active",
  "loading": false,
  "memory_usage": 1048576,
  "cpu_usage": 0.5,
  "connections": 10,
  "shard_key_regex": ".*",
  "backup": true,
  "replication": {
    "status": "in-sync",
    "lag": 0
  },
  "persistence": {
    "type": "aof",
    "status": "active"
  }
}
```

## Use Cases

### Load Balancing

Redistribute shards across nodes for better resource utilization:

```bash
#!/bin/bash
# Balance shards across nodes

# Get shard distribution
for node in 1 2 3; do
  COUNT=$(redisctl enterprise shard list --node $node -q "[] | length")
  echo "Node $node: $COUNT shards"
done

# Migrate shards from overloaded node
redisctl enterprise shard list --node 1 --role master -q "[].uid" | \
  head -2 | while read shard; do
    echo "Migrating shard $shard to node 2"
    redisctl enterprise shard migrate $shard --target-node 2
  done
```

### Failover Management

Handle node maintenance with controlled failovers:

```bash
#!/bin/bash
# Failover all master shards on a node before maintenance

NODE_ID=1

# Get all master shards on the node
SHARDS=$(redisctl enterprise shard list --node $NODE_ID --role master -q "[].uid")

# Failover each shard
for shard in $SHARDS; do
  echo "Failing over shard $shard"
  redisctl enterprise shard failover $shard --force
  sleep 5
done

echo "All master shards failed over from node $NODE_ID"
```

### Performance Monitoring

Monitor shard performance metrics:

```bash
#!/bin/bash
# Monitor shard performance

# Get top memory-consuming shards
redisctl enterprise shard list -q "[] | sort_by(@, &memory_usage) | reverse(@) | [:5]"

# Check for lagging replicas
redisctl enterprise shard list --role slave -q \
  "[?replication.lag > \`100\`].{shard: uid, lag: replication.lag, node: node}"

# Monitor shard connections
while true; do
  clear
  echo "=== Shard Connection Count ==="
  redisctl enterprise shard list -q \
    "[].{shard: uid, connections: connections}" -o table
  sleep 10
done
```

### Shard Health Check

Comprehensive health check script:

```bash
#!/bin/bash
# Check shard health across cluster

echo "=== Shard Health Report ==="

# Check for inactive shards
INACTIVE=$(redisctl enterprise shard list -q "[?status != 'active'].uid")
if [ -n "$INACTIVE" ]; then
  echo "WARNING: Inactive shards found: $INACTIVE"
fi

# Check for loading shards
LOADING=$(redisctl enterprise shard list -q "[?loading == \`true\`].uid")
if [ -n "$LOADING" ]; then
  echo "INFO: Shards currently loading: $LOADING"
fi

# Check replication lag
HIGH_LAG=$(redisctl enterprise shard list --role slave -q \
  "[?replication.lag > \`1000\`].uid")
if [ -n "$HIGH_LAG" ]; then
  echo "WARNING: High replication lag on shards: $HIGH_LAG"
fi

# Check memory usage
for shard in $(redisctl enterprise shard list -q "[].uid"); do
  MEMORY=$(redisctl enterprise shard get $shard -q "memory_usage")
  if [ "$MEMORY" -gt 1073741824 ]; then  # 1GB
    echo "INFO: Shard $shard using $(($MEMORY / 1048576))MB"
  fi
done
```

## Bulk Operation Examples

### Bulk Failover Configuration

```json
{
  "shards": [1, 2, 3, 4]
}
```

### Bulk Migration Configuration

```json
{
  "migrations": [
    {
      "shard_uid": 1,
      "target_node": 2
    },
    {
      "shard_uid": 3,
      "target_node": 3
    }
  ]
}
```

## Best Practices

1. **Monitor shard distribution** - Ensure even distribution across nodes
2. **Check replication lag** - High lag indicates performance issues
3. **Plan migrations carefully** - Migrations consume resources
4. **Use controlled failovers** - For planned maintenance
5. **Monitor memory usage** - Prevent out-of-memory situations
6. **Regular health checks** - Detect issues early

## Troubleshooting

### Shard Not Responding

```bash
# Check shard status
redisctl enterprise shard get <shard_uid> -q "status"

# Check node status
NODE=$(redisctl enterprise shard get <shard_uid> -q "node")
redisctl enterprise node get $NODE -q "status"

# Force failover if needed
redisctl enterprise shard failover <shard_uid> --force
```

### Migration Stuck

```bash
# Check migration status
redisctl enterprise action list --type shard_migration --status running

# Cancel if needed
redisctl enterprise action cancel <action_uid>

# Retry migration
redisctl enterprise shard migrate <shard_uid> --target-node <node_uid>
```

### High Memory Usage

```bash
# Identify high-memory shards
redisctl enterprise shard list -q \
  "[] | sort_by(@, &memory_usage) | reverse(@) | [:10]"

# Check database configuration
BDB=$(redisctl enterprise shard get <shard_uid> -q "bdb_uid")
redisctl enterprise database get $BDB -q "memory_size"

# Consider adding shards to database
redisctl enterprise database update $BDB --data '{"shards_count": 4}'
```

### Replication Issues

```bash
# Check replication status
redisctl enterprise shard list --role slave -q \
  "[].{shard: uid, status: replication.status, lag: replication.lag}"

# Force re-sync if needed
redisctl enterprise shard get <shard_uid> -q "replication"
```

## Integration with Other Commands

Shard commands work with:

```bash
# Get database shard count
redisctl enterprise database get 1 -q "shards_count"

# Check node shard capacity
redisctl enterprise node get 1 -q "max_shards"

# Monitor shard-related actions
redisctl enterprise action list --type shard_migration
```

## Performance Considerations

- **Migration impact**: Shard migrations consume network and CPU resources
- **Failover time**: Typically completes in seconds but depends on data size
- **Replication overhead**: More replicas mean more network traffic
- **Memory overhead**: Each shard has memory overhead for metadata

## Related Commands

- `enterprise database` - Database configuration affects shards
- [`enterprise node`](./nodes.md) - Node capacity and shard placement
- `enterprise action` - Monitor shard operations
- `enterprise stats` - Detailed performance metrics