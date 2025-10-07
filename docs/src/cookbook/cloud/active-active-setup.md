# Active-Active (CRDB) Setup

Time: 30-45 minutes  
Prerequisites:
- Redis Cloud account with Active-Active subscription
- redisctl configured with Cloud credentials
- Understanding of multi-region deployments

## What is Active-Active?

Active-Active (Conflict-free Replicated Database, CRDB) provides:
- Multiple writable regions simultaneously
- Automatic conflict resolution
- Local read/write latency in each region
- Geographic redundancy and disaster recovery

## Quick Setup

```bash
# Create Active-Active subscription
redisctl cloud subscription create \
  --data '{
    "name": "global-aa",
    "deployment_type": "active-active",
    "regions": [
      {"region": "us-east-1", "networking": {"cidr": "10.0.1.0/24"}},
      {"region": "eu-west-1", "networking": {"cidr": "10.0.2.0/24"}},
      {"region": "ap-southeast-1", "networking": {"cidr": "10.0.3.0/24"}}
    ]
  }' \
  --wait

# Create Active-Active database
redisctl cloud database create \
  --subscription-id 42 \
  --data '{
    "name": "global-cache",
    "memory_limit_in_gb": 2,
    "support_oss_cluster_api": true,
    "data_persistence": "aof-every-1-second",
    "replication": true
  }' \
  --wait
```

## Step-by-Step Setup

### 1. Plan Your Regions

Choose regions close to your users:

```bash
# List available regions
redisctl cloud region list -o json -q '[].{
  region: region,
  provider: provider,
  availability_zones: availability_zones
}'
```

**Common patterns:**
- **US + EU**: us-east-1, eu-west-1
- **Global**: us-east-1, eu-west-1, ap-southeast-1
- **US Multi-Region**: us-east-1, us-west-2

### 2. Create Active-Active Subscription

```bash
redisctl cloud subscription create \
  --data '{
    "name": "production-aa",
    "deployment_type": "active-active",
    "payment_method_id": 12345,
    "cloud_provider": "AWS",
    "regions": [
      {
        "region": "us-east-1",
        "networking": {
          "cidr": "10.1.0.0/24"
        },
        "preferred_availability_zones": ["use1-az1", "use1-az2"]
      },
      {
        "region": "eu-west-1",
        "networking": {
          "cidr": "10.2.0.0/24"
        },
        "preferred_availability_zones": ["euw1-az1", "euw1-az2"]
      }
    ]
  }' \
  --wait \
  --wait-timeout 900
```

**Important:** Each region needs a unique CIDR block.

### 3. Create Active-Active Database

```bash
redisctl cloud database create \
  --subscription-id 42 \
  --data '{
    "name": "global-sessions",
    "memory_limit_in_gb": 5,
    "protocol": "redis",
    "support_oss_cluster_api": true,
    "data_persistence": "aof-every-1-second",
    "replication": true,
    "throughput_measurement": {
      "by": "operations-per-second",
      "value": 50000
    },
    "data_eviction_policy": "volatile-lru",
    "modules": [
      {"name": "RedisJSON"}
    ]
  }' \
  --wait
```

### 4. Get Regional Endpoints

```bash
# Get all regional endpoints
redisctl cloud database get \
  --subscription-id 42 \
  --database-id 12345 \
  -o json \
  -q '{
    name: name,
    endpoints: regions[].{
      region: region,
      public_endpoint: public_endpoint,
      private_endpoint: private_endpoint
    }
  }'
```

**Example output:**
```json
{
  "name": "global-sessions",
  "endpoints": [
    {
      "region": "us-east-1",
      "public_endpoint": "redis-12345-us-east-1.cloud.redislabs.com:12345",
      "private_endpoint": "redis-12345-us-east-1.internal.cloud.redislabs.com:12345"
    },
    {
      "region": "eu-west-1",
      "public_endpoint": "redis-12345-eu-west-1.cloud.redislabs.com:12346",
      "private_endpoint": "redis-12345-eu-west-1.internal.cloud.redislabs.com:12346"
    }
  ]
}
```

### 5. Configure Applications

Connect each application to its nearest region:

**US Application:**
```python
import redis

r = redis.Redis(
    host='redis-12345-us-east-1.cloud.redislabs.com',
    port=12345,
    password='your-password',
    decode_responses=True
)
```

**EU Application:**
```python
r = redis.Redis(
    host='redis-12345-eu-west-1.cloud.redislabs.com',
    port=12346,
    password='your-password',
    decode_responses=True
)
```

## Network Connectivity

### Setup VPC Peering for Each Region

```bash
# US East peering
redisctl cloud connectivity vpc-peering create-aa \
  --subscription-id 42 \
  --region-id 1 \
  --data '{
    "provider_name": "AWS",
    "aws_account_id": "123456789012",
    "vpc_id": "vpc-us-east-abc",
    "vpc_cidr": "172.31.0.0/16",
    "region": "us-east-1"
  }' \
  --wait

# EU West peering
redisctl cloud connectivity vpc-peering create-aa \
  --subscription-id 42 \
  --region-id 2 \
  --data '{
    "provider_name": "AWS",
    "aws_account_id": "123456789012",
    "vpc_id": "vpc-eu-west-xyz",
    "vpc_cidr": "172.32.0.0/16",
    "region": "eu-west-1"
  }' \
  --wait
```

## Conflict Resolution

Active-Active uses automatic conflict resolution with LWW (Last-Write-Wins):

### Understanding Conflicts

```bash
# Example: Counter increment in both regions simultaneously
# US: INCR counter (value becomes 1)
# EU: INCR counter (value becomes 1)
# After sync: counter = 2 (both increments applied)
```

### Conflict-Free Data Types

Use Redis data types that resolve conflicts automatically:
- **Counters** - INCR/DECR (additive)
- **Sets** - SADD/SREM (union)
- **Sorted Sets** - ZADD (merge by score)
- **Hashes** - HSET (field-level LWW)

### Best Practices

```python
# Good: Using counters
redis.incr('page:views')

# Good: Using sets
redis.sadd('user:tags', 'premium')

# Caution: Simple strings (LWW conflicts)
redis.set('user:status', 'active')  # May conflict with other region
```

## Monitoring Active-Active

### Check Replication Lag

```bash
# Get replication status for all regions
redisctl cloud database get \
  --subscription-id 42 \
  --database-id 12345 \
  -o json \
  -q 'regions[].{
    region: region,
    replication_lag: replication_lag_ms,
    status: status
  }'
```

### Monitor Sync Traffic

```bash
# Check inter-region bandwidth usage
redisctl cloud subscription get \
  --subscription-id 42 \
  -q 'deployment.regions[].{
    region: region,
    sync_traffic_gb: sync_traffic_gb_per_month
  }'
```

## Scaling Active-Active

### Add Region to Existing Database

```bash
# Add new region to subscription
redisctl cloud subscription update \
  --subscription-id 42 \
  --data '{
    "add_regions": [
      {
        "region": "ap-southeast-1",
        "networking": {
          "cidr": "10.3.0.0/24"
        }
      }
    ]
  }' \
  --wait

# Database automatically extends to new region
```

### Remove Region

```bash
redisctl cloud subscription update \
  --subscription-id 42 \
  --data '{
    "remove_regions": ["ap-southeast-1"]
  }' \
  --wait
```

## Disaster Recovery

### Regional Failover

If a region becomes unavailable:
1. Applications automatically retry to local endpoint
2. Update application config to use different region
3. Data remains consistent across remaining regions

```bash
# Check region health
redisctl cloud database get \
  --subscription-id 42 \
  --database-id 12345 \
  -q 'regions[].{region: region, status: status}'

# Update application to use healthy region
# No data loss - all writes in healthy regions preserved
```

## Cost Optimization

### Monitor Inter-Region Traffic

```bash
# Check sync costs
redisctl cloud subscription get \
  --subscription-id 42 \
  -o json \
  -q '{
    monthly_sync_gb: (deployment.regions | map(&sync_traffic_gb_per_month, @) | sum(@)),
    monthly_cost_estimate: monthly_cost
  }'
```

### Optimize for Read-Heavy Workloads

```bash
# Use read replicas in regions with heavy reads
redisctl cloud database update \
  --subscription-id 42 \
  --database-id 12345 \
  --data '{
    "replication": true,
    "replica_count": 2
  }' \
  --wait
```

## Common Patterns

### Session Store

```python
# Store sessions in nearest region
def store_session(session_id, data):
    redis.hset(f'session:{session_id}', mapping=data)
    redis.expire(f'session:{session_id}', 86400)  # 24 hours

# Read from any region
def get_session(session_id):
    return redis.hgetall(f'session:{session_id}')
```

### Global Rate Limiting

```python
# Distributed rate limit across regions
def check_rate_limit(user_id, limit=100):
    key = f'rate:limit:{user_id}:{int(time.time() / 60)}'
    count = redis.incr(key)
    redis.expire(key, 120)
    return count <= limit
```

### Leaderboards

```python
# Global leaderboard
def update_score(user_id, score):
    redis.zadd('leaderboard:global', {user_id: score})

def get_top_players(n=10):
    return redis.zrevrange('leaderboard:global', 0, n-1, withscores=True)
```

## Common Issues

### High Replication Lag

```bash
# Check network connectivity between regions
# Increase bandwidth allocation
redisctl cloud subscription update \
  --subscription-id 42 \
  --data '{"bandwidth_gb_per_month": 500}' \
  --wait
```

### Conflict Resolution Issues

**Solution:** Design data model for conflict-free types:
- Use INCR instead of SET for counters
- Use SADD instead of SET for collections
- Use HSET for field-level updates instead of full object replacement

### Region Addition Takes Long Time

**Solution:** Adding regions requires data sync. For large databases:
- Expect 1-2 hours for initial sync
- Monitor with `--wait-timeout 7200`

## Best Practices

1. **Design for Conflicts** - Use conflict-free data types
2. **Local Writes** - Always write to nearest region
3. **Monitor Lag** - Alert on high replication lag
4. **Test Failover** - Regularly test regional failures
5. **Plan CIDRs** - Use non-overlapping CIDR blocks
6. **Optimize Bandwidth** - Monitor inter-region traffic costs

## Next Steps

- [Setup VPC Peering](setup-vpc-peering.md) - Private connectivity per region
- [Configure ACLs](configure-acls.md) - Secure all regional endpoints
- [Monitor Performance](../common/monitor-performance.md) - Track per-region metrics
- [Backup and Restore](backup-restore.md) - Multi-region backup strategy

## See Also

- [Active-Active Documentation](https://redis.io/docs/latest/operate/rc/databases/configuration/active-active/)
- [CRDB Architecture](https://redis.io/docs/latest/operate/rs/databases/active-active/)
- [Conflict Resolution](https://redis.io/docs/latest/operate/rs/databases/active-active/develop/)
