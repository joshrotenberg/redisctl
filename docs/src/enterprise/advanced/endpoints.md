# Endpoint Management

The endpoint commands provide access to Redis Enterprise database endpoint statistics and availability monitoring.

> **Note**: Redis Enterprise manages most endpoint configurations through database commands. These commands provide monitoring and statistics capabilities.

## Available Commands

### Get Endpoint Statistics

Get aggregate statistics for all database endpoints in the cluster:

```bash
# Get all endpoint statistics
redisctl enterprise endpoint stats

# Get statistics as YAML
redisctl enterprise endpoint stats -o yaml

# Filter to specific metrics
redisctl enterprise endpoint stats -q '[].{name: endpoint_name, connections: current_connections}'

# Get statistics for endpoints with high connection counts
redisctl enterprise endpoint stats -q "[?current_connections > `100`]"
```

The statistics include:
- Connection metrics (current, total, failed)
- Request/response rates
- Latency information
- Error counts
- Bandwidth usage

### Check Endpoint Availability

Check the availability status of a specific database endpoint:

```bash
# Check endpoint availability for database 1
redisctl enterprise endpoint availability 1

# Get availability as table
redisctl enterprise endpoint availability 1 -o table

# Extract specific availability information
redisctl enterprise endpoint availability 1 -q 'available'
```

Availability information includes:
- Current availability status
- Node availability
- Shard distribution
- Failover status
- Connection health

## Output Examples

### Endpoint Statistics
```json
[
  {
    "endpoint_name": "redis-12345.cluster.local:16379",
    "bdb_uid": 1,
    "current_connections": 45,
    "total_connections": 12543,
    "failed_connections": 2,
    "requests_per_sec": 5432,
    "responses_per_sec": 5430,
    "avg_latency_ms": 0.8,
    "bandwidth_in_mbps": 12.5,
    "bandwidth_out_mbps": 8.3,
    "errors_per_sec": 0.1
  }
]
```

### Endpoint Availability
```json
{
  "bdb_uid": 1,
  "available": true,
  "endpoints": [
    {
      "addr": "redis-12345.cluster.local:16379",
      "node": 1,
      "role": "master",
      "status": "active"
    }
  ],
  "shards_placement": "optimal",
  "last_failover": null
}
```

## Common Use Cases

### Monitoring Endpoint Health

Monitor endpoint statistics and set up alerts:

```bash
# Check endpoints with high error rates
redisctl enterprise endpoint stats -q "[?errors_per_sec > `10`]"

# Monitor endpoints with connection issues
redisctl enterprise endpoint stats -q "[?failed_connections > `0`].{name: endpoint_name, failed: failed_connections}"

# Check latency across all endpoints
redisctl enterprise endpoint stats -q "[].{endpoint: endpoint_name, latency: avg_latency_ms}" -o table
```

### Availability Monitoring

Check database endpoint availability during maintenance:

```bash
# Check availability for critical databases
for db in 1 2 3; do
  echo "Database $db:"
  redisctl enterprise endpoint availability $db -q 'available'
done

# Get detailed availability for troubleshooting
redisctl enterprise endpoint availability 1 -o yaml
```

### Performance Analysis

Analyze endpoint performance metrics:

```bash
# Get top endpoints by connection count
redisctl enterprise endpoint stats -q "reverse(sort_by([],&current_connections))[:5]" -o table

# Find endpoints with bandwidth issues
redisctl enterprise endpoint stats -q "[?bandwidth_in_mbps > `100` || bandwidth_out_mbps > `100`]"

# Compare request/response rates
redisctl enterprise endpoint stats -q "[].{endpoint: endpoint_name, req_rate: requests_per_sec, resp_rate: responses_per_sec, diff: requests_per_sec - responses_per_sec}"
```

## Integration with Monitoring

Export endpoint metrics for monitoring systems:

```bash
# Export to monitoring format
redisctl enterprise endpoint stats -o json > endpoint_metrics.json

# View stats in table format
redisctl enterprise endpoint stats -q "[].{endpoint: endpoint_name, connections: current_connections, latency: avg_latency_ms, errors: errors_per_sec}" -o table

# Stream to monitoring pipeline
while true; do
  redisctl enterprise endpoint stats -q '[].{timestamp: now(), metrics: @}' | \
    curl -X POST http://metrics-collector/ingest -d @-
  sleep 60
done
```

## Troubleshooting

### High Connection Counts

If endpoints show high connection counts:

```bash
# Identify affected endpoints
redisctl enterprise endpoint stats -q "[?current_connections > `1000`]"

# Check database configuration
redisctl enterprise database get <bdb_uid> -q '{max_connections: max_connections, current: @ | current_connections}'

# Monitor connection trends
for i in {1..10}; do
  redisctl enterprise endpoint stats -q "[].{endpoint: endpoint_name, connections: current_connections}" -o table
  sleep 30
done
```

### Availability Issues

When endpoints report availability problems:

```bash
# Check specific database endpoint
redisctl enterprise endpoint availability <bdb_uid>

# Verify node status
redisctl enterprise node list -q "[?status != 'active']"

# Check shard distribution
redisctl enterprise database get <bdb_uid> -q 'shards_placement'
```

## Best Practices

1. **Regular Monitoring**: Set up regular checks of endpoint statistics to catch issues early
2. **Baseline Metrics**: Establish baseline performance metrics for comparison
3. **Alert Thresholds**: Configure alerts based on your specific workload patterns
4. **Correlation**: Correlate endpoint metrics with database and node statistics
5. **Capacity Planning**: Use connection and bandwidth metrics for capacity planning

## Related Commands

- `redisctl enterprise database` - Manage databases and their endpoints
- `redisctl enterprise stats` - View detailed statistics
- `redisctl enterprise node` - Check node status affecting endpoints
- `redisctl enterprise cluster` - View cluster-wide endpoint configuration