# Configure Database Replication

Time: 10-15 minutes  
Prerequisites:
- Redis Enterprise cluster with multiple nodes
- redisctl configured with Enterprise credentials
- Database already created

## What is Replication?

Replication provides:
- High availability - automatic failover if master fails
- Read scalability - distribute reads across replicas
- Data durability - multiple copies of data

## Quick Setup

```bash
# Enable replication on existing database
redisctl enterprise database update \
  --database-id 1 \
  --data '{
    "replication": true,
    "shards_count": 2
  }' \
  --wait
```

## Step-by-Step Setup

### 1. Create Database with Replication

```bash
redisctl enterprise database create \
  --data '{
    "name": "replicated-db",
    "memory_size": 1073741824,
    "type": "redis",
    "port": 12000,
    "replication": true,
    "shards_count": 1,
    "sharding": false
  }' \
  --wait
```

### 2. Verify Replication Status

```bash
redisctl enterprise database get \
  --database-id 1 \
  -o json \
  -q '{
    name: name,
    replication: replication,
    shards_count: shards_count,
    endpoints: endpoints
  }'
```

### 3. Check Shard Distribution

```bash
redisctl enterprise shard list-by-database \
  --bdb-uid 1 \
  -o json \
  -q '[].{
    uid: uid,
    role: role,
    node: node_uid,
    status: status
  }'
```

**Expected**: One master and one replica shard on different nodes.

## Replication Topology

### Single Master with Replica

```bash
# Default configuration
{
  "replication": true,
  "shards_count": 1,
  "sharding": false
}
# Result: 1 master + 1 replica = 2 total shards
```

### Sharded with Replication

```bash
# Clustered database with replication
{
  "replication": true,
  "shards_count": 3,
  "sharding": true
}
# Result: 3 master + 3 replica = 6 total shards
```

## Advanced Configuration

### Set Replica Count

```bash
# Multiple replicas per master
redisctl enterprise database update \
  --database-id 1 \
  --data '{
    "replication": true,
    "replica_sources": [
      {"replica_source_name": "replica1", "replica_source_type": "replica"},
      {"replica_source_name": "replica2", "replica_source_type": "replica"}
    ]
  }' \
  --wait
```

### Rack Awareness

Ensure master and replicas are on different racks/zones:

```bash
redisctl enterprise database update \
  --database-id 1 \
  --data '{
    "rack_aware": true
  }' \
  --wait
```

## Monitoring Replication

### Check Replication Lag

```bash
# Get replication lag for database
redis-cli -h localhost -p 12000 INFO replication

# Or via REST API
redisctl enterprise database get \
  --database-id 1 \
  -q 'replica_sync[].{
    replica: replica_uid,
    lag: lag,
    status: status
  }'
```

### Monitor Sync Status

```bash
# Check if replicas are in sync
redisctl enterprise shard list-by-database \
  --bdb-uid 1 \
  -o json \
  -q '[?role==`replica`].{
    shard: uid,
    status: status,
    sync_status: sync_status
  }'
```

## Failover Operations

### Manual Failover

```bash
# Failover specific shard
redisctl enterprise shard failover \
  --uid 1:1 \
  --force

# Verify new master
redisctl enterprise shard get --uid 1:1 -q 'role'
```

### Automatic Failover

Enabled by default with replication:

```bash
# Check failover settings
redisctl enterprise database get \
  --database-id 1 \
  -q '{
    replication: replication,
    watchdog_profile: watchdog_profile
  }'
```

## Replica Configuration

### Read-Only Replicas

```bash
# Configure replica as read-only (default)
redisctl enterprise database update \
  --database-id 1 \
  --data '{
    "replica_of": {
      "endpoints": ["master-db:12000"],
      "readonly": true
    }
  }' \
  --wait
```

### External Replication Source

Replicate from external Redis:

```bash
redisctl enterprise database create \
  --data '{
    "name": "replica-db",
    "memory_size": 1073741824,
    "type": "redis",
    "port": 12001,
    "replica_of": {
      "endpoints": ["external-redis.example.com:6379"],
      "authentication_redis_pass": "source-password"
    }
  }' \
  --wait
```

## Replication Performance

### Optimize Replication Speed

```bash
# Increase replication buffer
redisctl enterprise database update \
  --database-id 1 \
  --data '{
    "repl_backlog_size": 104857600  # 100MB
  }' \
  --wait
```

### Monitor Replication Traffic

```bash
redisctl enterprise database get \
  --database-id 1 \
  -o json \
  -q '{
    replication_traffic: repl_traffic,
    backlog_size: repl_backlog_size
  }'
```

## Common Patterns

### High Availability Setup

```bash
# Production-ready HA configuration
redisctl enterprise database create \
  --data '{
    "name": "ha-database",
    "memory_size": 10737418240,
    "type": "redis",
    "port": 12000,
    "replication": true,
    "shards_count": 3,
    "sharding": true,
    "rack_aware": true,
    "data_persistence": "aof",
    "aof_policy": "appendfsync-every-sec"
  }' \
  --wait
```

### Read Scaling with Replicas

```python
# Application pattern: writes to master, reads from replicas
from redis import Redis, Sentinel

# Connect to master for writes
master = Redis(host='master-endpoint', port=12000)
master.set('key', 'value')

# Connect to replica for reads
replica = Redis(host='replica-endpoint', port=12001)
value = replica.get('key')
```

## Disaster Recovery

### Backup Replication Status

```bash
# Save replication configuration
redisctl enterprise database get \
  --database-id 1 \
  -o json > db-replication-config.json
```

### Restore After Failure

```bash
# Recreate database with same configuration
redisctl enterprise database create \
  --data @db-replication-config.json \
  --wait
```

## Common Issues

### Replication Lag Increasing

```bash
# Check network between nodes
redisctl enterprise node list -o table

# Check shard placement
redisctl enterprise shard list-by-database --bdb-uid 1 -o table

# Consider adding more replicas or increasing bandwidth
```

### Replica Out of Sync

```bash
# Force resync
redisctl enterprise shard failover --uid 1:2 --force

# Check sync status
redisctl enterprise shard get --uid 1:2 -q 'sync_status'
```

### Split Brain Scenario

**Prevention:**
- Always use odd number of cluster nodes
- Enable rack awareness
- Monitor node connectivity

**Recovery:**
```bash
# Identify correct master
redisctl enterprise shard list-by-database --bdb-uid 1 \
  -q '[?role==`master`]'

# Force failover if needed
redisctl enterprise database update \
  --database-id 1 \
  --data '{"action": "recover"}' \
  --wait
```

## Best Practices

1. **Always Enable for Production** - Replication is critical for HA
2. **Use Rack Awareness** - Distribute across failure domains
3. **Monitor Replication Lag** - Alert on high lag
4. **Test Failover** - Regularly test automatic failover
5. **Plan Capacity** - Replicas consume same resources as master
6. **Persist Configuration** - Backup replication settings

## Next Steps

- [Cluster Health Check](cluster-health.md) - Monitor replication health
- [Node Management](node-management.md) - Manage replica placement
- [Generate Support Package](support-package.md) - Troubleshooting tools
- [Create Database](create-database.md) - Database configuration basics

## See Also

- [Database Replication Reference](../../enterprise/core-resources/databases.md#replication)
- [Shard Management](../../enterprise/operations/shard-management.md)
- [Redis Replication](https://redis.io/docs/latest/operate/oss_and_stack/management/replication/)
