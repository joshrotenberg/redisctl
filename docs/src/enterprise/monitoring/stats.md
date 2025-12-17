# Statistics and Metrics

Monitor your Redis Enterprise cluster with comprehensive statistics and metrics.

## Commands Overview

```bash
redisctl enterprise stats --help
```

## Database Statistics

```bash
# Get statistics for a specific database
redisctl enterprise stats database <db_id>

# Get shard-level statistics
redisctl enterprise stats database-shards <db_id>

# Get metrics over time
redisctl enterprise stats database-metrics <db_id>
```

## Node Statistics

```bash
# Get statistics for a specific node
redisctl enterprise stats node <node_id>

# Get node metrics over time
redisctl enterprise stats node-metrics <node_id>
```

## Cluster Statistics

```bash
# Get cluster-wide statistics
redisctl enterprise stats cluster

# Get cluster metrics over time
redisctl enterprise stats cluster-metrics
```

## Listener Statistics

```bash
# Get listener (proxy endpoint) statistics
redisctl enterprise stats listener <listener_id>
```

## Export Statistics

Export statistics in various formats for analysis or integration:

```bash
# Export statistics
redisctl enterprise stats export --format json

# Export to file
redisctl enterprise stats export -o json > stats-export.json
```

## Streaming Statistics

For real-time monitoring, use the `--follow` flag:

```bash
# Stream database stats continuously
redisctl enterprise stats database <db_id> --follow

# Stream with custom interval (seconds)
redisctl enterprise stats database <db_id> --follow --interval 5
```

Press `Ctrl+C` to stop streaming.

## Common Metrics

### Database Metrics

| Metric | Description |
|--------|-------------|
| `used_memory` | Memory used by the database |
| `total_keys` | Total number of keys |
| `ops_per_sec` | Operations per second |
| `read_hits` | Successful read operations |
| `read_misses` | Read operations with no data |
| `write_hits` | Successful write operations |
| `connected_clients` | Number of connected clients |

### Node Metrics

| Metric | Description |
|--------|-------------|
| `cpu_usage` | CPU utilization percentage |
| `memory_usage` | Memory utilization |
| `network_in` | Incoming network traffic |
| `network_out` | Outgoing network traffic |
| `disk_usage` | Disk utilization |

## JMESPath Query Examples

```bash
# Get database ops/sec
redisctl enterprise stats database <db_id> -q 'instantaneous_ops_per_sec'

# Monitor memory usage
redisctl enterprise stats database <db_id> -q '{used_memory: used_memory, peak_memory: peak_memory}'

# Get all node CPU usage
for node in $(redisctl enterprise node list -q '[].uid' --raw); do
  echo "Node $node:"
  redisctl enterprise stats node $node -q 'cpu_usage'
done
```

## Alerting Integration

Use stats output for custom alerting:

```bash
#!/bin/bash
# Alert if ops/sec exceeds threshold
OPS=$(redisctl enterprise stats database 1 -q 'instantaneous_ops_per_sec')
if (( $(echo "$OPS > 10000" | bc -l) )); then
  echo "High traffic alert: $OPS ops/sec"
fi
```

## Related Commands

- [Alerts](alerts.md) - View and manage cluster alerts
- [Usage Reports](usage-report.md) - Generate usage reports
- [Database Monitoring](../core-resources/databases.md#monitoring) - Database-specific monitoring commands
