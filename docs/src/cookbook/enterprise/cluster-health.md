# Cluster Health Check

Time: 5 minutes  
Prerequisites:
- Redis Enterprise cluster running
- redisctl configured with Enterprise credentials

## Quick Health Check

```bash
# Get cluster overview
redisctl enterprise cluster get -o json -q '{
  name: name,
  nodes: nodes_count,
  shards: shards_count,
  databases: databases_count,
  status: cluster_state
}'

# Check all nodes
redisctl enterprise node list -o table -q '[].{
  id: uid,
  addr: addr,
  role: role,
  status: status,
  cores: cores,
  memory_available: (total_memory - provisional_memory - used_memory)
}'
```

## Detailed Health Checks

### 1. Cluster Status

```bash
redisctl enterprise cluster get -o json -q '{
  state: cluster_state,
  license_state: license_state,
  quorum: quorum_only,
  shards: {used: shards_count, limit: shards_limit},
  memory: {used: memory_size, available: ephemeral_storage_size}
}'
```

### 2. Node Health

```bash
# Check each node status
redisctl enterprise node list -o json -q '[].{
  node: uid,
  status: status,
  uptime: uptime,
  cpu: cpu_idle,
  memory_used: (used_memory / total_memory * 100),
  disk_used: (ephemeral_storage_used / ephemeral_storage_size * 100)
}'
```

### 3. Database Health

```bash
# List all databases with key metrics
redisctl enterprise database list -o json -q '[].{
  db: uid,
  name: name,
  status: status,
  memory: memory_size,
  shards: shards_count,
  ops_sec: total_req
}'
```

### 4. Alert Status

```bash
# Check cluster alerts
redisctl enterprise cluster alerts -o json -q '{
  enabled: alerts_settings.enabled,
  active_alerts: alerts[?state==`active`].name
}'

# Check node alerts
redisctl enterprise node alerts -o table
```

## Automated Health Monitoring

```bash
#!/bin/bash
# cluster-health-check.sh

echo "Redis Enterprise Cluster Health Check"
echo "======================================"

# Cluster state
echo "Cluster Status:"
redisctl enterprise cluster get -q 'cluster_state'

# Node count and status
NODES=$(redisctl enterprise node list -o json -q 'length([])')
HEALTHY_NODES=$(redisctl enterprise node list -o json -q '[?status==`active`] | length([])')
echo "Nodes: $HEALTHY_NODES/$NODES healthy"

# Database status
DBS=$(redisctl enterprise database list -o json -q 'length([])')
ACTIVE_DBS=$(redisctl enterprise database list -o json -q '[?status==`active`] | length([])')
echo "Databases: $ACTIVE_DBS/$DBS active"

# Resource usage
SHARD_USAGE=$(redisctl enterprise cluster get -o json -q '((shards_count / shards_limit * 100) | floor)')
MEMORY_USAGE=$(redisctl enterprise cluster get -o json -q '((memory_size / ephemeral_storage_size * 100) | floor)')
echo "Resource Usage: Shards $SHARD_USAGE%, Memory $MEMORY_USAGE%"

# Exit code based on health
if [ "$HEALTHY_NODES" -eq "$NODES" ] && [ "$ACTIVE_DBS" -eq "$DBS" ]; then
    echo "Status: HEALTHY"
    exit 0
else
    echo "Status: DEGRADED"
    exit 1
fi
```

## Next Steps

- [Node Management](node-management.md) - Manage cluster nodes
- Database Monitoring - Track database metrics
- [Generate Support Package](support-package.md) - Troubleshooting tools

## See Also

- Cluster Command Reference
- Node Command Reference
