# Debug Info Commands

Collect diagnostic information for troubleshooting Redis Enterprise clusters.

## Overview

Debug info commands gather comprehensive diagnostic data from Redis Enterprise clusters, nodes, and databases. This information is essential for troubleshooting issues and working with Redis support.

## Available Commands

### Collect All Debug Info

```bash
redisctl enterprise debug-info all
```

Collects complete diagnostic information from the entire cluster, including:
- Cluster configuration and state
- All node information
- Database configurations
- Log files
- Performance metrics

**Note**: This can generate large amounts of data and may take several minutes.

### Collect Node Debug Info

```bash
redisctl enterprise debug-info node
```

Collects diagnostic information from the current node only:
- Node configuration
- System resources
- Local log files
- Process information
- Network configuration

### Collect Database Debug Info

```bash
redisctl enterprise debug-info database <bdb_uid>
```

Collects diagnostic information for a specific database:
- Database configuration
- Shard distribution
- Replication state
- Performance metrics
- Recent operations

## Output Options

### Save to File

```bash
# Save debug info to file
redisctl enterprise debug-info all > debug-$(date +%Y%m%d-%H%M%S).json

# Compress large debug outputs
redisctl enterprise debug-info all | gzip > debug-$(date +%Y%m%d-%H%M%S).json.gz
```

### Filter Output

```bash
# Get specific sections with JMESPath
redisctl enterprise debug-info all -q 'cluster_info'
redisctl enterprise debug-info node -q 'system_info.memory'
```

## Common Use Cases

### Troubleshooting Cluster Issues

```bash
# Collect full cluster diagnostics
redisctl enterprise debug-info all > cluster-debug.json

# Check specific node
redisctl enterprise debug-info node -q 'errors'
```

### Database Performance Issues

```bash
# Collect database-specific diagnostics
redisctl enterprise debug-info database 1 > db1-debug.json

# Check shard distribution
redisctl enterprise debug-info database 1 -q 'shards'
```

### Preparing Support Tickets

```bash
# Collect and compress all diagnostics
redisctl enterprise debug-info all | gzip > support-$(date +%Y%m%d).json.gz

# Include cluster and node info
echo "=== Cluster Info ===" > support-info.txt
redisctl enterprise cluster info >> support-info.txt
echo "=== Debug Info ===" >> support-info.txt
redisctl enterprise debug-info node >> support-info.txt
```

## Important Notes

- Debug info may contain sensitive information (hostnames, IPs, configuration)
- Large clusters can generate gigabytes of debug data
- Collection may impact cluster performance during execution
- Always review debug info before sharing with support
- Some debug operations require admin privileges

## Output Example

```json
{
  "collection_time": "2025-09-15T10:30:00Z",
  "cluster_info": {
    "name": "prod-cluster",
    "nodes": 3,
    "databases": 5,
    "version": "7.2.4-92"
  },
  "system_info": {
    "total_memory": "64GB",
    "cpu_cores": 16,
    "storage": {
      "persistent": "/var/opt/redislabs/persist",
      "ephemeral": "/var/opt/redislabs/tmp"
    }
  },
  "diagnostics": {
    "warnings": [],
    "errors": [],
    "recommendations": []
  }
}
```

## Performance Considerations

- Use `node` or `database` specific commands when possible
- Run during maintenance windows for production clusters
- Consider network bandwidth when collecting from remote clusters
- Compress output for large datasets

## Related Commands

- [Logs Commands](logs.md) - View cluster logs
- [Stats Commands](stats.md) - Monitor performance metrics
- [Cluster Commands](cluster.md) - Check cluster health