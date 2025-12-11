# Proxy Management

The proxy commands provide management and monitoring capabilities for Redis Enterprise proxy processes. Proxies handle connection management, load balancing, and request routing between clients and Redis database shards.

## Available Commands

### List Proxies

List all proxy processes in the cluster:

```bash
# List all proxies
redisctl enterprise proxy list

# List proxies as a table
redisctl enterprise proxy list -o table

# Filter to active proxies
redisctl enterprise proxy list -q "[?status == 'active']"

# List proxies by node
redisctl enterprise proxy list -q "[?node_uid == \`1\`]"

# Get proxy IDs and their nodes
redisctl enterprise proxy list -q "[].{id: uid, node: node_uid, status: status}"
```

### Get Proxy Details

Get detailed information about a specific proxy:

```bash
# Get proxy details
redisctl enterprise proxy get 1

# Get proxy in YAML format
redisctl enterprise proxy get 1 -o yaml

# Extract specific fields
redisctl enterprise proxy get 1 -q '{uid: uid, port: port, threads: threads}'

# Check proxy load
redisctl enterprise proxy get 1 -q '{connections: connections, cpu: cpu_usage}'
```

### Update Proxy Configuration

Update configuration for a specific proxy:

```bash
# Update proxy configuration from file
redisctl enterprise proxy update 1 --data @proxy-config.json

# Update proxy with inline JSON
redisctl enterprise proxy update 1 --data '{"threads": 8, "max_connections": 10000}'

# Update proxy from stdin
echo '{"threads": 4}' | redisctl enterprise proxy update 1 --data -

# Update with specific settings
cat <<EOF | redisctl enterprise proxy update 1 --data -
{
  "threads": 8,
  "max_connections": 10000,
  "tcp_keepalive": 60,
  "tcp_backlog": 512
}
EOF
```

### Update All Proxies

Update configuration for all proxies simultaneously:

```bash
# Update all proxies from file
redisctl enterprise proxy update-all --data @global-proxy-config.json

# Update all proxies with inline configuration
redisctl enterprise proxy update-all --data '{"threads": 8}'

# Apply global proxy policy
cat <<EOF | redisctl enterprise proxy update-all --data -
{
  "threads": 8,
  "max_connections": 10000,
  "tcp_keepalive": 60,
  "timeout": 300
}
EOF
```

## Output Examples

### Proxy List
```json
[
  {
    "uid": 1,
    "node_uid": 1,
    "port": 8080,
    "status": "active",
    "threads": 4,
    "connections": 245,
    "cpu_usage": 12.5,
    "memory_usage": 128,
    "databases": [1, 2, 3]
  },
  {
    "uid": 2,
    "node_uid": 2,
    "port": 8080,
    "status": "active",
    "threads": 4,
    "connections": 189,
    "cpu_usage": 10.2,
    "memory_usage": 115,
    "databases": [1, 2, 3]
  }
]
```

### Proxy Details
```json
{
  "uid": 1,
  "node_uid": 1,
  "port": 8080,
  "status": "active",
  "threads": 4,
  "max_connections": 10000,
  "current_connections": 245,
  "total_connections": 1234567,
  "cpu_usage": 12.5,
  "memory_usage": 128,
  "tcp_keepalive": 60,
  "tcp_backlog": 512,
  "timeout": 300,
  "databases": [
    {
      "bdb_uid": 1,
      "name": "cache-db",
      "connections": 89
    },
    {
      "bdb_uid": 2,
      "name": "session-db",
      "connections": 67
    }
  ],
  "stats": {
    "requests_per_sec": 5432,
    "operations_per_sec": 8901,
    "latency_avg": 0.8,
    "errors_per_sec": 0.1
  }
}
```

## Common Use Cases

### Monitoring Proxy Load

Monitor proxy load and performance:

```bash
# Check proxy connections across cluster
redisctl enterprise proxy list -q "[].{proxy: uid, node: node_uid, connections: connections}" -o table

# Find overloaded proxies
redisctl enterprise proxy list -q "[?connections > \`1000\`]"

# Monitor CPU usage
redisctl enterprise proxy list -q "[?cpu_usage > \`50\`].{proxy: uid, cpu: cpu_usage}"

# Check memory usage
redisctl enterprise proxy list -q "[].{proxy: uid, memory_mb: memory_usage}" -o table
```

### Performance Tuning

Optimize proxy performance:

```bash
# Increase threads for high-load proxies
for proxy in $(redisctl enterprise proxy list -q "[?cpu_usage > \`75\`].uid" -o json | jq -r '.[]'); do
  echo "Updating proxy $proxy"
  redisctl enterprise proxy update "$proxy" --data '{"threads": 8}'
done

# Update connection limits
redisctl enterprise proxy update-all --data '{"max_connections": 20000}'

# Apply optimized settings
cat <<EOF | redisctl enterprise proxy update-all --data -
{
  "threads": 8,
  "max_connections": 15000,
  "tcp_keepalive": 30,
  "tcp_backlog": 1024,
  "timeout": 600
}
EOF
```

### Troubleshooting

Diagnose proxy issues:

```bash
# Find proxies with errors
redisctl enterprise proxy list -q "[?status != 'active']"

# Check proxy distribution
redisctl enterprise proxy list -q "[].node_uid" | jq -s 'group_by(.) | map({node: .[0], count: length})'

# Monitor connection distribution
for proxy in 1 2 3; do
  echo "Proxy $proxy:"
  redisctl enterprise proxy get "$proxy" -q 'databases[].{db: name, connections: connections}' -o table
done

# Check proxy resource usage
redisctl enterprise proxy list -q "[].{proxy: uid, cpu: cpu_usage, memory: memory_usage, connections: connections}" -o table
```

### Capacity Planning

Plan proxy capacity:

```bash
# Calculate total connections
redisctl enterprise proxy list -q "[].connections" | jq -s 'add'

# Get average connections per proxy
redisctl enterprise proxy list -q "[].connections" | jq -s 'add/length'

# Find proxies near connection limit
redisctl enterprise proxy list -q "[?connections > max_connections * \`0.8\`].{proxy: uid, usage_pct: (connections / max_connections * \`100\`)}"

# Resource utilization summary
redisctl enterprise proxy list -q "{total_proxies: length(@), avg_cpu: avg([].cpu_usage), avg_memory: avg([].memory_usage), total_connections: sum([].connections)}"
```

## Configuration Examples

### Basic Proxy Configuration
```json
{
  "threads": 4,
  "max_connections": 10000,
  "timeout": 300
}
```

### High-Performance Configuration
```json
{
  "threads": 16,
  "max_connections": 50000,
  "tcp_keepalive": 30,
  "tcp_backlog": 2048,
  "timeout": 600,
  "tcp_nodelay": true
}
```

### Resource-Constrained Configuration
```json
{
  "threads": 2,
  "max_connections": 5000,
  "tcp_keepalive": 120,
  "tcp_backlog": 256,
  "timeout": 120
}
```

## Best Practices

1. **Load Distribution**: Ensure proxies are evenly distributed across nodes
2. **Thread Tuning**: Set threads based on CPU cores and expected load
3. **Connection Limits**: Set appropriate connection limits based on available resources
4. **Monitoring**: Regularly monitor proxy metrics for performance issues
5. **Gradual Changes**: Test configuration changes on individual proxies before applying globally
6. **Resource Planning**: Plan proxy resources based on expected client connections

## Integration with Monitoring

Export proxy metrics for monitoring systems:

```bash
# Export metrics to monitoring system
redisctl enterprise proxy list -o json | \
  jq '.[] | {
    timestamp: now,
    proxy_id: .uid,
    node_id: .node_uid,
    connections: .connections,
    cpu_usage: .cpu_usage,
    memory_usage: .memory_usage
  }' | \
  curl -X POST http://metrics-collector/ingest -d @-

# Create Prometheus-compatible metrics
redisctl enterprise proxy list -q "[].{proxy: uid, metric: @}" | \
  jq -r '.[] | "
redis_proxy_connections{proxy=\"\(.proxy)\"} \(.metric.connections)
redis_proxy_cpu_usage{proxy=\"\(.proxy)\"} \(.metric.cpu_usage)
redis_proxy_memory_mb{proxy=\"\(.proxy)\"} \(.metric.memory_usage)
"'
```

## Proxy Troubleshooting

### High CPU Usage

When proxies show high CPU usage:

```bash
# Identify high-CPU proxies
redisctl enterprise proxy list -q "[?cpu_usage > \`80\`]"

# Check thread configuration
redisctl enterprise proxy get <uid> -q 'threads'

# Increase threads
redisctl enterprise proxy update <uid> --data '{"threads": 8}'

# Monitor after change
watch -n 5 "redisctl enterprise proxy get <uid> -q 'cpu_usage'"
```

### Connection Issues

When experiencing connection problems:

```bash
# Check connection limits
redisctl enterprise proxy list -q "[].{proxy: uid, current: connections, max: max_connections, pct: (connections / max_connections * \`100\`)}"

# Find proxies at capacity
redisctl enterprise proxy list -q "[?connections >= max_connections * \`0.95\`]"

# Increase connection limits
redisctl enterprise proxy update <uid> --data '{"max_connections": 20000}'
```

## Related Commands

- `redisctl enterprise node` - View nodes hosting proxies
- `redisctl enterprise database` - Manage databases served by proxies
- `redisctl enterprise stats` - View detailed statistics including proxy metrics
- `redisctl enterprise cluster` - View cluster-wide proxy configuration