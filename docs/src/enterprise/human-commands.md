# Human-Friendly Commands

These commands provide a typed, user-friendly interface to common Redis Enterprise operations.

## Cluster Management

```bash
# Get cluster information
redisctl enterprise cluster info

# Update cluster settings
redisctl enterprise cluster update \
  --name "Production Cluster" \
  --rack-aware true

# Get cluster license
redisctl enterprise license get

# Update license
redisctl enterprise license set --license-file license.key
```

## Database Management

```bash
# List all databases
redisctl enterprise database list

# Get database details
redisctl enterprise database get <db-id>

# Create database
redisctl enterprise database create \
  --name "cache-db" \
  --memory-size 10gb \
  --port 12000 \
  --replication true \
  --shards-count 2

# Update database
redisctl enterprise database update <db-id> \
  --memory-size 20gb \
  --eviction-policy allkeys-lru

# Delete database
redisctl enterprise database delete <db-id>
```

## Node Management

```bash
# List all nodes
redisctl enterprise node list

# Get node details
redisctl enterprise node get <node-id>

# Add node to cluster
redisctl enterprise node join \
  --address 192.168.1.100 \
  --username admin@cluster.local \
  --password node-password

# Remove node
redisctl enterprise node remove <node-id>

# Get node stats
redisctl enterprise node stats <node-id>
```

## User Management

```bash
# List users
redisctl enterprise user list

# Create user
redisctl enterprise user create \
  --email "user@example.com" \
  --password "secure-password" \
  --role "db-member"

# Update user
redisctl enterprise user update <user-id> \
  --role "cluster-admin"

# Delete user
redisctl enterprise user delete <user-id>
```

## Role Management

```bash
# List roles
redisctl enterprise role list

# Get role details
redisctl enterprise role get <role-id>

# Create custom role
redisctl enterprise role create \
  --name "db-viewer" \
  --permissions "view-db,view-stats"
```

## Module Management

```bash
# List available modules
redisctl enterprise module list

# Upload module
redisctl enterprise module upload \
  --file redisgraph.so \
  --name "RedisGraph" \
  --version "2.8.0"

# Delete module
redisctl enterprise module delete <module-id>
```

## Alert Configuration

```bash
# List configured alerts
redisctl enterprise alert list

# Get alert details
redisctl enterprise alert get <alert-id>

# Create alert
redisctl enterprise alert create \
  --name "high-memory" \
  --threshold 80 \
  --email "ops@example.com"

# Update alert
redisctl enterprise alert update <alert-id> \
  --threshold 90

# Delete alert
redisctl enterprise alert delete <alert-id>
```

## Bootstrap Operations

```bash
# Bootstrap single node cluster
redisctl enterprise bootstrap create \
  --cluster-name "Dev Cluster" \
  --username "admin@cluster.local" \
  --password "admin-password" \
  --license-file license.key

# Get bootstrap status
redisctl enterprise bootstrap status

# Join existing cluster
redisctl enterprise bootstrap join \
  --cluster-address 192.168.1.100 \
  --username admin@cluster.local \
  --password cluster-password
```

## CRDB (Active-Active) Operations

```bash
# List CRDB databases
redisctl enterprise crdb list

# Get CRDB details
redisctl enterprise crdb get <crdb-id>

# Create CRDB
redisctl enterprise crdb create \
  --name "global-cache" \
  --memory-size 10gb \
  --participating-clusters "1,2,3"

# Add participating cluster
redisctl enterprise crdb add-instance <crdb-id> \
  --cluster-id 4 \
  --memory-size 10gb
```

## Statistics and Monitoring

```bash
# Get database statistics
redisctl enterprise database stats <db-id>

# Get node statistics  
redisctl enterprise node stats <node-id>

# Get cluster statistics
redisctl enterprise cluster stats

# Export metrics
redisctl enterprise stats export \
  --format prometheus \
  --output metrics.txt
```

## Output Formatting

All commands support output formatting:

```bash
# Table format
redisctl enterprise database list -o table

# JSON (default)
redisctl enterprise database list -o json

# YAML
redisctl enterprise database list -o yaml

# Filtered with JMESPath
redisctl enterprise database list \
  -q "[?status=='active'].{name:name,memory:memory_size,port:port}"
```