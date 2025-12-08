# Redis Enterprise Examples

Real-world examples of managing Redis Enterprise clusters.

## Cluster Setup

### Bootstrap Single-Node Cluster

```bash
# 1. Bootstrap the cluster
redisctl api enterprise post /v1/bootstrap \
  --data '{
    "action": "create_cluster",
    "cluster": {
      "name": "Development Cluster"
    },
    "node": {
      "paths": {
        "persistent_path": "/var/opt/redislabs/persist",
        "ephemeral_path": "/var/opt/redislabs/tmp"
      }
    },
    "credentials": {
      "username": "admin@cluster.local",
      "password": "SecurePassword123!"
    }
  }'

# 2. Check bootstrap status
redisctl api enterprise get /v1/bootstrap

# 3. Apply license
redisctl enterprise license set --license-file license.key

# 4. Verify cluster is ready
redisctl enterprise cluster info
```

### Add Nodes to Cluster

```bash
# On new node, join existing cluster
redisctl api enterprise post /v1/bootstrap \
  --data '{
    "action": "join_cluster",
    "cluster": {
      "nodes": ["192.168.1.100"]
    },
    "credentials": {
      "username": "admin@cluster.local",
      "password": "SecurePassword123!"
    }
  }'

# Check all nodes
redisctl enterprise node list -o table

# Verify node status
redisctl enterprise node get 2 -q '{status:status,role:role}'
```

## Database Management

### Create High-Performance Database

```bash
# Create database optimized for caching
redisctl api enterprise post /v1/bdbs \
  --data '{
    "name": "cache-db",
    "memory_size": 10737418240,
    "type": "redis",
    "port": 12000,
    "replication": true,
    "shards_count": 4,
    "shard_key_regex": ".*{(.*)}.*",
    "eviction_policy": "allkeys-lru",
    "persistence": "disabled",
    "redis_version": "7.2"
  }'

# Get connection string
redisctl enterprise database get 1 \
  -q '"redis://:" + password + "@" + endpoints[0].addr[0] + ":" + (endpoints[0].port|tostring)'
```

### Enable Modules

```bash
# Upload custom module
redisctl enterprise module upload \
  --file /path/to/module.so \
  --name "CustomModule" \
  --version "1.0.0"

# Create database with modules
redisctl api enterprise post /v1/bdbs \
  --data '{
    "name": "feature-db",
    "memory_size": 5368709120,
    "port": 12001,
    "module_list": [
      {"module_name": "search", "module_args": ""},
      {"module_name": "timeseries", "module_args": ""},
      {"module_name": "json", "module_args": ""}
    ]
  }'
```

## User Management

### Set Up RBAC

```bash
# Create custom role
redisctl api enterprise post /v1/roles \
  --data '{
    "name": "developer",
    "management": "db_member",
    "data_access": "read-write"
  }'

# Create users with different roles
redisctl enterprise user create \
  --email "admin@company.com" \
  --password "AdminPass123!" \
  --role "admin"

redisctl enterprise user create \
  --email "dev@company.com" \
  --password "DevPass123!" \
  --role "db_member"

redisctl enterprise user create \
  --email "viewer@company.com" \
  --password "ViewPass123!" \
  --role "db_viewer"

# List users and their roles
redisctl enterprise user list \
  -q "[].{email:email,role:role}" \
  -o table
```

## Monitoring and Maintenance

### Health Check Script

```bash
#!/bin/bash
# Comprehensive cluster health check

echo "=== Cluster Health Check ==="

# Cluster status
echo -e "\nCluster Status:"
redisctl enterprise cluster info -q '{name:name,status:status}'

# Node health
echo -e "\nNode Status:"
redisctl enterprise node list \
  -q "[].{id:uid,address:addr,status:status,role:role}" \
  -o table

# Database health
echo -e "\nDatabase Status:"
redisctl enterprise database list \
  -q "[].{name:name,status:status,memory_used:used_memory}" \
  -o table

# Check for alerts
echo -e "\nActive Alerts:"
redisctl api enterprise get /v1/cluster/alerts \
  -q "[?state=='active'].{severity:severity,alert:alert_name}"
```

### Performance Monitoring

```bash
# Get database metrics
DB_ID=1
redisctl api enterprise get /v1/bdbs/$DB_ID/stats \
  -q '{
    ops_per_sec: avg_ops_per_sec,
    memory_used: used_memory,
    connections: conns,
    cpu_percent: cpu_user
  }'

# Monitor in real-time
watch -n 5 'redisctl api enterprise get /v1/bdbs/1/stats \
  -q "{ops: avg_ops_per_sec, memory: used_memory, cpu: cpu_user}"'

# Export metrics for Prometheus
redisctl api enterprise get /v1/bdbs/metrics?format=prometheus > metrics.txt
```

## Backup and Recovery

### Automated Backup

```bash
#!/bin/bash
# Backup all databases

BACKUP_DIR="/backups/$(date +%Y%m%d)"
mkdir -p $BACKUP_DIR

# List all databases and backup
redisctl enterprise database list -q "[].uid" | while read db_id; do
  echo "Backing up database $db_id..."
  
  # Trigger backup
  redisctl api enterprise post /v1/bdbs/$db_id/actions/export \
    --data '{
      "location": "'"$BACKUP_DIR"'/db-'"$db_id"'.rdb"
    }'
done

# Create backup manifest
redisctl enterprise database list > $BACKUP_DIR/manifest.json
```

### Database Migration

```bash
# Export from source database
redisctl api enterprise post /v1/bdbs/1/actions/export \
  --data '{
    "location": "/tmp/export.rdb"
  }'

# Wait for export to complete
sleep 10

# Import to new database
redisctl api enterprise post /v1/bdbs/2/actions/import \
  --data '{
    "location": "/tmp/export.rdb",
    "sync": "merge"
  }'
```

## High Availability Setup

### Configure Database HA

```bash
# Create HA database with rack awareness
redisctl api enterprise post /v1/bdbs \
  --data '{
    "name": "ha-database",
    "memory_size": 10737418240,
    "replication": true,
    "replica_ha": true,
    "rack_aware": true,
    "shards_count": 3,
    "shard_key_regex": ".*{(.*)}.*",
    "proxy_policy": "all-master-shards"
  }'

# Configure automatic failover
redisctl api enterprise put /v1/bdbs/1 \
  --data '{
    "replica_ha": true,
    "replica_ha_grace": 60
  }'
```

## CRDB (Active-Active) Setup

```bash
# Create CRDB on first cluster
redisctl api enterprise post /v1/crdbs \
  --data '{
    "name": "global-cache",
    "memory_size": 5368709120,
    "port": 12100,
    "replication": false,
    "sharding": true,
    "shard_count": 2
  }'

# Get CRDB configuration for other clusters
CRDB_GUID=$(redisctl api enterprise get /v1/crdbs/1 -q crdb_guid)

# On second cluster, create participating instance
redisctl api enterprise post /v1/crdbs \
  --data '{
    "name": "global-cache",
    "memory_size": 5368709120,
    "port": 12100,
    "crdb_guid": "'"$CRDB_GUID"'",
    "instance": {
      "cluster": {
        "url": "https://cluster1.example.com:9443",
        "credentials": {
          "username": "admin@cluster.local",
          "password": "password"
        }
      }
    }
  }'
```

## Maintenance Operations

### Rolling Restart

```bash
#!/bin/bash
# Perform rolling restart of all databases

redisctl enterprise database list -q "[].uid" | while read db_id; do
  echo "Restarting database $db_id..."
  
  # Restart database
  redisctl api enterprise post /v1/bdbs/$db_id/actions/restart
  
  # Wait for database to be active
  while [ "$(redisctl enterprise database get $db_id -q status)" != "active" ]; do
    sleep 5
  done
  
  echo "Database $db_id restarted successfully"
done
```

### Cluster Upgrade Preparation

```bash
# Check upgrade readiness
echo "=== Pre-Upgrade Check ==="

# Check cluster version
redisctl enterprise cluster info -q '{version:version}'

# Check node versions
redisctl enterprise node list -q "[].{node:uid,version:software_version}"

# Check for active alerts
ALERTS=$(redisctl api enterprise get /v1/cluster/alerts -q "[?state=='active'] | length(@)")
if [ "$ALERTS" -gt 0 ]; then
  echo "WARNING: Active alerts found. Resolve before upgrading."
  redisctl api enterprise get /v1/cluster/alerts -q "[?state=='active']"
fi

# Backup critical databases
redisctl enterprise database list -q "[?contains(name, 'prod')].uid" | while read db_id; do
  redisctl api enterprise post /v1/bdbs/$db_id/actions/export \
    --data '{"location": "/backup/pre-upgrade-db-'"$db_id"'.rdb"}'
done
```