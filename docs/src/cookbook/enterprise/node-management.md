# Node Management

Time: 10-15 minutes  
Prerequisites:
- Redis Enterprise cluster with multiple nodes
- redisctl configured with Enterprise credentials
- Admin access to cluster

## Quick Commands

```bash
# List all nodes
redisctl enterprise node list -o table

# Get specific node details
redisctl enterprise node get --node-id 1 -o json

# Check node status
redisctl enterprise node list -q '[].{id: uid, status: status, role: role}'
```

## Node Operations

### 1. View Node Details

```bash
# Get comprehensive node info
redisctl enterprise node get --node-id 1 -o json -q '{
  uid: uid,
  addr: addr,
  status: status,
  role: role,
  cores: cores,
  memory: {
    total: total_memory,
    used: used_memory,
    available: (total_memory - used_memory)
  },
  storage: {
    total: ephemeral_storage_size,
    used: ephemeral_storage_used,
    available: ephemeral_storage_avail
  },
  uptime: uptime,
  version: software_version
}'
```

### 2. Add Node to Cluster

```bash
# Prepare new node (run on new node)
curl -k https://localhost:9443/v1/bootstrap/create_cluster \
  -H "Content-Type: application/json" \
  -d '{
    "action": "join_cluster",
    "cluster": {
      "nodes": ["10.0.1.10:9443"],
      "username": "admin@cluster.local",
      "password": "admin-password"
    }
  }'

# Verify node joined
redisctl enterprise node list -o table
```

### 3. Remove Node from Cluster

```bash
# First, ensure no databases are on this node
redisctl enterprise database list -o json -q '[?node_uid==`3`]'

# Remove node
redisctl enterprise node delete --node-id 3 --wait
```

### 4. Update Node Configuration

```bash
redisctl enterprise node update \
  --node-id 1 \
  --data '{
    "max_listeners": 100,
    "max_redis_servers": 50
  }'
```

## Node Maintenance

### Enable Maintenance Mode

```bash
# Put node in maintenance mode (no new shards)
redisctl enterprise node update \
  --node-id 2 \
  --data '{"accept_servers": false}'

# Verify
redisctl enterprise node get --node-id 2 -q 'accept_servers'
```

### Drain Node

Move all shards off a node before maintenance:

```bash
#!/bin/bash
NODE_ID=2

# Get all shards on this node
SHARDS=$(redisctl enterprise shard list \
  --node $NODE_ID \
  -o json \
  -q '[].uid')

# Migrate each shard to another node
for shard in $SHARDS; do
    echo "Migrating shard $shard..."
    redisctl enterprise shard migrate \
        --uid $shard \
        --target-node 1 \
        --force
done

echo "Node $NODE_ID drained"
```

### Check Node Resources

```bash
redisctl enterprise node get --node-id 1 -o json -q '{
  cpu_idle: cpu_idle,
  memory_free_pct: ((total_memory - used_memory) / total_memory * 100 | floor),
  disk_free_pct: (ephemeral_storage_avail / ephemeral_storage_size * 100 | floor),
  connections: conns,
  shards: shard_count
}'
```

## Monitoring Nodes

### Node Health Script

```bash
#!/bin/bash
# node-health.sh

echo "Node Health Report"
echo "==================" 

redisctl enterprise node list -o json | jq -r '
  .[] | 
  "Node \(.uid): \(.status) - CPU: \(.cpu_idle)% idle, " +
  "Memory: \((.used_memory / .total_memory * 100 | floor))% used, " +
  "Shards: \(.shard_count)"
'
```

### Resource Alerts

```bash
# Check for nodes with high resource usage
redisctl enterprise node list -o json -q '
  [?
    (used_memory / total_memory * 100) > 80 ||
    (ephemeral_storage_used / ephemeral_storage_size * 100) > 85 ||
    cpu_idle < 20
  ].{
    node: uid,
    memory_pct: (used_memory / total_memory * 100 | floor),
    disk_pct: (ephemeral_storage_used / ephemeral_storage_size * 100 | floor),
    cpu_idle: cpu_idle
  }
'
```

## Node Failover

### Check Quorum

```bash
# Ensure cluster has quorum before operations
redisctl enterprise cluster get -q '{
  quorum: quorum_only,
  nodes: nodes_count,
  required: ((nodes_count / 2 | floor) + 1)
}'
```

### Handle Failed Node

```bash
# Identify failed node
redisctl enterprise node list -q '[?status!=`active`].{id: uid, status: status}'

# Check affected databases
redisctl enterprise database list -o json -q '[?node_uid==`3`].{db: uid, name: name}'

# Trigger failover for affected databases
redisctl enterprise database update \
  --database-id 1 \
  --data '{"action": "failover"}'
```

## Common Issues

### Node Not Responding

```bash
# Check node connectivity
curl -k https://node-ip:9443/v1/cluster

# Check from another node
redisctl enterprise node get --node-id 2 -q 'status'
```

### High Memory Usage

```bash
# Find databases using most memory on node
redisctl enterprise database list -o json -q '
  [?node_uid==`1`] | 
  sort_by(@, &memory_size) | 
  reverse(@) |
  [].{db: uid, name: name, memory_gb: (memory_size / 1073741824)}
'
```

## Best Practices

1. **Always maintain quorum** - Keep odd number of nodes
2. **Monitor resources** - Set up alerts for CPU, memory, disk
3. **Regular health checks** - Automated monitoring
4. **Graceful operations** - Drain nodes before maintenance
5. **Plan capacity** - Add nodes before reaching limits

## Next Steps

- [Cluster Health Check](cluster-health.md) - Monitor overall cluster health
- [Generate Support Package](support-package.md) - Troubleshooting tools
- [Database Management](create-database.md) - Manage databases

## See Also

- [Node Command Reference](../../enterprise/cluster-management.md#node-operations)
- [Shard Management](../../enterprise/operations/shard-management.md)
