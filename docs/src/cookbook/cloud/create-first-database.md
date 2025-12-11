# Create Your First Redis Cloud Database

⏱️ **Time:** 5-10 minutes  
📋 **Prerequisites:**
- Redis Cloud account ([sign up](https://redis.io/try-free/))
- redisctl installed ([installation guide](../../getting-started/installation.md))
- Profile configured with Cloud credentials ([authentication guide](../../getting-started/authentication.md))

## Quick Command

If you already have a subscription, create a database with one command:

```bash
redisctl cloud database create \
  --subscription YOUR_SUBSCRIPTION_ID \
  --data '{"name": "my-first-db", "memoryLimitInGb": 1}' \
  --wait
```

## Step-by-Step Guide

### 1. Verify Your Setup

First, check that redisctl can connect to Redis Cloud:

```bash
redisctl cloud subscription list -o table
```

**What you should see:**
```
┌────┬─────────────────┬────────┬────────────┐
│ ID │ Name            │ Status │ Provider   │
├────┼─────────────────┼────────┼────────────┤
│ 42 │ my-subscription │ active │ AWS        │
└────┴─────────────────┴────────┴────────────┘
```

**Troubleshooting:**
- ❌ "401 Unauthorized" → Check your API credentials with `redisctl profile get`
- ❌ Empty table → Create a subscription first (see subscription guide)

### 2. Choose Your Database Configuration

Decide on your database specifications. Here's a minimal configuration:

```json
{
  "name": "my-first-db",
  "memoryLimitInGb": 1,
  "protocol": "redis"
}
```

**Common options:**
- `memoryLimitInGb`: Memory size (1-100+ GB)
- `protocol`: `redis` or `memcached`
- `dataPersistence`: `none`, `aof-every-1-second`, `snapshot-every-1-hour`
- `replication`: `true` for high availability

### 3. Create the Database

Use the subscription ID from step 1:

```bash
redisctl cloud database create \
  --subscription 42 \
  --data '{
    "name": "my-first-db",
    "memoryLimitInGb": 1,
    "protocol": "redis",
    "dataPersistence": "aof-every-1-second",
    "replication": true
  }' \
  --wait \
  --wait-timeout 300
```

**What's happening:**
- `--wait`: Waits for database to become active
- `--wait-timeout 300`: Waits up to 5 minutes
- Without `--wait`: Returns immediately with task ID

**What you should see:**

```json
{
  "taskId": "abc123...",
  "status": "processing"
}
...
Database creation completed successfully!
{
  "database_id": 12345,
  "name": "my-first-db",
  "status": "active",
  "public_endpoint": "redis-12345.c123.us-east-1-1.ec2.cloud.redislabs.com:12345"
}
```

### 4. Get Your Connection Details

Retrieve your database credentials:

```bash
redisctl cloud database get \
  --subscription 42 \
  --database-id 12345 \
  -o json \
  -q '{endpoint: public_endpoint, password: password}'
```

**Output:**
```json
{
  "endpoint": "redis-12345.c123.us-east-1-1.ec2.cloud.redislabs.com:12345",
  "password": "your-password-here"
}
```

### 5. Test Your Connection

Using redis-cli:

```bash
redis-cli -h redis-12345.c123.us-east-1-1.ec2.cloud.redislabs.com \
  -p 12345 \
  -a your-password-here \
  PING
```

Expected response: `PONG`

## Advanced Options

### Using a JSON File

For complex configurations, use a file:

```bash
# Create database-config.json
cat > database-config.json << 'EOF'
{
  "name": "production-db",
  "memoryLimitInGb": 10,
  "protocol": "redis",
  "dataPersistence": "aof-every-1-second",
  "replication": true,
  "throughputMeasurement": {
    "by": "operations-per-second",
    "value": 25000
  },
  "dataEvictionPolicy": "volatile-lru",
  "modules": [
    {"name": "RedisJSON"}
  ]
}
EOF

# Create database
redisctl cloud database create \
  --subscription 42 \
  --data @database-config.json \
  --wait
```

### JSON Output for Automation

Use `-o json` for scripts:

```bash
DB_INFO=$(redisctl cloud database create \
  --subscription 42 \
  --data '{"name": "api-cache", "memoryLimitInGb": 2}' \
  --wait \
  -o json)

DB_ID=$(echo "$DB_INFO" | jq -r '.database_id')
echo "Created database: $DB_ID"
```

## Common Issues

### Database Creation Times Out

```
Error: Database creation timed out after 300 seconds
```

**Solution:** Some regions take longer. Increase timeout:
```bash
redisctl cloud database create ... --wait --wait-timeout 600
```

### Insufficient Subscription Capacity

```
Error: Subscription has insufficient capacity
```

**Solution:** Either:
1. Delete unused databases: `redisctl cloud database delete ...`
2. Upgrade subscription: Contact Redis support or use the web console

### Invalid Configuration

```
Error: 400 Bad Request - Invalid memory limit
```

**Solution:** Check subscription limits:
```bash
redisctl cloud subscription get --subscription 42 -q 'pricing'
```

## Next Steps

Now that you have a database:

- 🔒 Configure ACL Security - Secure your database with access controls
- 🌐 [Set Up VPC Peering](setup-vpc-peering.md) - Connect to your private network
- 💾 [Configure Backups](backup-restore.md) - Protect your data
- 📊 Monitor Performance - Track your database metrics

## See Also

- [Cloud Database Command Reference](../../cloud/core-resources/databases.md) - Complete command documentation
- Database Configuration Guide - All configuration options
- [Redis Cloud Pricing](https://redis.io/pricing/) - Understand costs
