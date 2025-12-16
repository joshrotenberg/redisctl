# Enterprise Quick Examples

Examples showing the three-tier model for Redis Enterprise.

## Setup

```bash
# Set credentials
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"
export REDIS_ENTERPRISE_INSECURE="true"  # for self-signed certs

# Or use Docker
alias redisctl='docker run --rm \
  -e REDIS_ENTERPRISE_URL \
  -e REDIS_ENTERPRISE_USER \
  -e REDIS_ENTERPRISE_PASSWORD \
  -e REDIS_ENTERPRISE_INSECURE \
  ghcr.io/redis-developer/redisctl'
```

## API Layer Examples

Direct REST access for scripting:

```bash
# Get cluster info
redisctl api enterprise get /v1/cluster

# List all nodes
redisctl api enterprise get /v1/nodes

# List all databases
redisctl api enterprise get /v1/bdbs

# Get specific database
redisctl api enterprise get /v1/bdbs/1

# Create database
redisctl api enterprise post /v1/bdbs -d '{
  "name": "cache",
  "memory_size": 1073741824
}'

# Get cluster stats
redisctl api enterprise get /v1/cluster/stats/last
```

## Human Command Examples

Type-safe operations for daily use:

```bash
# Get cluster info
redisctl enterprise cluster get

# List nodes in table format
redisctl enterprise node list -o table

# List databases with specific fields
redisctl enterprise database list \
  -q "[].{id:uid,name:name,memory:memory_size,status:status}" \
  -o table

# Create database with flags
redisctl enterprise database create \
  --name sessions \
  --memory-size 1073741824 \
  --port 12000 \
  --replication

# Stream cluster stats continuously
redisctl enterprise stats cluster --follow

# Stream database stats every 2 seconds
redisctl enterprise stats database 1 --follow --poll-interval 2

# Check license
redisctl enterprise license get

# Generate support package
redisctl enterprise support-package cluster --optimize --upload
```

## Workflow Examples

Multi-step operations:

```bash
# Initialize a new cluster
redisctl enterprise workflow init-cluster \
  --cluster-name production \
  --username admin@cluster.local \
  --password SecurePass123! \
  --license-file license.txt \
  --create-database \
  --database-name default-cache
```

This single command:
1. Bootstraps the cluster
2. Sets up authentication
3. Applies the license
4. Creates an initial database
5. Returns cluster details

## Common Patterns

### Health Check Script

```bash
#!/bin/bash
echo "=== Cluster Health ==="
redisctl enterprise cluster get -q '{name:name,status:status}'

echo "=== Nodes ==="
redisctl enterprise node list -q "[].{id:uid,status:status}" -o table

echo "=== Databases ==="
redisctl enterprise database list -q "[].{name:name,status:status}" -o table

echo "=== Active Alerts ==="
redisctl enterprise alerts list -q "[?state=='active']" -o table
```

### Monitor Resources

```bash
# Watch cluster stats
watch -n 5 "redisctl enterprise stats cluster \
  -q '{cpu:cpu_user,memory:free_memory,conns:total_connections}'"
```

### Bulk Database Operations

```bash
# Update eviction policy on all databases
for db in $(redisctl enterprise database list -q '[].uid' | jq -r '.[]'); do
  echo "Updating database $db"
  redisctl enterprise database update $db \
    --data '{"eviction_policy": "volatile-lru"}'
done
```

### Export Cluster Configuration

```bash
# Backup all configuration
DATE=$(date +%Y%m%d)
redisctl enterprise cluster get > cluster-$DATE.json
redisctl enterprise node list > nodes-$DATE.json
redisctl enterprise database list > databases-$DATE.json
redisctl enterprise user list > users-$DATE.json
```

### Rolling Node Maintenance

```bash
#!/bin/bash
for node in $(redisctl enterprise node list -q '[].uid' | jq -r '.[]'); do
  echo "Maintaining node $node..."
  
  # Enter maintenance mode
  redisctl enterprise node action $node maintenance-on
  sleep 60
  
  # Do maintenance work here
  
  # Exit maintenance mode
  redisctl enterprise node action $node maintenance-off
  sleep 30
  
  # Verify healthy
  STATUS=$(redisctl enterprise node get $node -q 'status')
  [ "$STATUS" != "active" ] && echo "Warning: node $node status is $STATUS"
done
```

### Support Package for Incident

```bash
# Generate optimized package and upload to Redis Support
redisctl enterprise support-package cluster \
  --optimize \
  --upload \
  --output support-$(date +%Y%m%d-%H%M%S).tar.gz
```

## Next Steps

- [Cloud Quick Examples](./cloud-examples.md) - Cloud-specific examples
- [Enterprise Overview](../enterprise/overview.md) - Full Enterprise documentation
- [Enterprise Cookbook](../cookbook/README.md) - Practical recipes
