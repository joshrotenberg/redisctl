# Create Your First Redis Enterprise Database

⏱️ **Time:** 5 minutes  
📋 **Prerequisites:**
- Redis Enterprise cluster running (see cluster setup)
- redisctl installed ([installation guide](../../getting-started/installation.md))
- Profile configured with Enterprise credentials ([authentication guide](../../getting-started/authentication.md))

## Quick Command

Create a basic database with one command:

```bash
redisctl enterprise database create \
  --data '{"name": "my-first-db", "memory_size": 1073741824}' \
  --wait
```

## Step-by-Step Guide

### 1. Verify Cluster Connection

Check that redisctl can connect to your cluster:

```bash
redisctl enterprise cluster get -o json -q 'name'
```

**What you should see:**
```json
"cluster1.local"
```

**Troubleshooting:**
- ❌ "Connection refused" → Check `REDIS_ENTERPRISE_URL` or profile settings
- ❌ "401 Unauthorized" → Verify credentials with `redisctl profile get`
- ❌ "SSL error" → Add `--insecure` flag or set `REDIS_ENTERPRISE_INSECURE=true`

### 2. Check Available Resources

See what resources are available:

```bash
redisctl enterprise cluster get -o json -q '{
  shards_limit: shards_limit,
  shards_used: shards_used,
  memory_size: memory_size
}'
```

**Example output:**
```json
{
  "shards_limit": 100,
  "shards_used": 5,
  "memory_size": 107374182400
}
```

### 3. Create the Database

Minimum configuration (1GB database):

```bash
redisctl enterprise database create \
  --data '{
    "name": "my-first-db",
    "memory_size": 1073741824,
    "type": "redis",
    "port": 12000
  }' \
  --wait
```

**Common options:**
- `memory_size`: Bytes (1073741824 = 1GB, 10737418240 = 10GB)
- `type`: `redis` or `memcached`
- `port`: Must be unique on cluster (12000-19999 typical range)
- `replication`: `true` for high availability
- `sharding`: `true` for clustering across shards

**What you should see:**

```json
{
  "uid": 1,
  "name": "my-first-db",
  "status": "active",
  "port": 12000,
  "memory_size": 1073741824,
  "endpoint": "redis-12000.cluster1.local"
}
```

### 4. Get Connection Details

Retrieve your database endpoint and authentication:

```bash
redisctl enterprise database get --database-id 1 -o json -q '{
  endpoint: dns_address_master,
  port: port,
  password: authentication_redis_pass
}'
```

**Output:**
```json
{
  "endpoint": "redis-12000.cluster1.local",
  "port": 12000,
  "password": "your-password-here"
}
```

### 5. Test Connection

Using redis-cli:

```bash
redis-cli -h redis-12000.cluster1.local \
  -p 12000 \
  -a your-password-here \
  PING
```

Expected response: `PONG`

## Advanced Configuration

### High Availability Database

Create a replicated database with automatic failover:

```bash
redisctl enterprise database create \
  --data '{
    "name": "ha-database",
    "memory_size": 10737418240,
    "type": "redis",
    "port": 12001,
    "replication": true,
    "data_persistence": "aof",
    "aof_policy": "appendfsync-every-sec"
  }' \
  --wait
```

### Clustered Database

Create a sharded database for scaling:

```bash
redisctl enterprise database create \
  --data '{
    "name": "clustered-db",
    "memory_size": 53687091200,
    "type": "redis",
    "port": 12002,
    "sharding": true,
    "shards_count": 5,
    "oss_cluster": true
  }' \
  --wait
```

### Using a Configuration File

For complex setups:

```bash
# Create database-config.json
cat > database-config.json << 'EOF'
{
  "name": "production-db",
  "memory_size": 21474836480,
  "type": "redis",
  "port": 12003,
  "replication": true,
  "sharding": true,
  "shards_count": 3,
  "data_persistence": "aof",
  "aof_policy": "appendfsync-every-sec",
  "eviction_policy": "volatile-lru",
  "oss_cluster": true,
  "authentication_redis_pass": "my-secure-password"
}
EOF

redisctl enterprise database create \
  --data @database-config.json \
  --wait
```

## Common Issues

### Port Already in Use

```
Error: Port 12000 is already allocated
```

**Solution:** Use a different port or check existing databases:
```bash
redisctl enterprise database list -o json -q '[].port'
```

### Insufficient Cluster Resources

```
Error: Not enough memory available
```

**Solution:** Check cluster capacity:
```bash
redisctl enterprise cluster get -q '{available_memory: (memory_size - memory_used)}'
```

### Database Stuck in "pending"

```
Status: pending
```

**Solution:** Check cluster node status:
```bash
redisctl enterprise node list -o table
```

All nodes should show `online` status. If not, investigate node issues first.

## Memory Size Reference

Quick conversion table:

| Description | Bytes | Human |
|-------------|-------|-------|
| 100 MB | 104857600 | 0.1 GB |
| 500 MB | 524288000 | 0.5 GB |
| 1 GB | 1073741824 | 1 GB |
| 5 GB | 5368709120 | 5 GB |
| 10 GB | 10737418240 | 10 GB |
| 50 GB | 53687091200 | 50 GB |
| 100 GB | 107374182400 | 100 GB |

Or use shell arithmetic expansion for 1GB in bash/zsh

## Next Steps

Now that you have a database:

- 🔒 Configure Redis ACLs - Secure your database with access controls
- 💾 [Generate Support Package](support-package.md) - Troubleshooting and diagnostics
- 🔄 [Configure Replication](configure-replication.md) - Set up replica databases
- 📊 Monitor Database Health - Track performance metrics

## See Also

- [Enterprise Database Command Reference](../../enterprise/core-resources/databases.md) - Complete command documentation
- Database Configuration Options - All configuration parameters
- [Redis Enterprise Documentation](https://redis.io/docs/latest/operate/rs/) - Official Redis Enterprise docs
