# 7. Advanced Features

## Support Package Automation

**The killer feature**

### Before redisctl: 10+ Minutes
1. SSH to cluster node (1 min)
2. Run `rladmin cluster debug_info` (2 min)
3. Find generated file (30 sec)
4. SCP to local machine (1 min)
5. Open Redis Support portal (1 min)
6. Click through upload form (3 min)
7. Wait for upload (2+ min)

### With redisctl: 30 Seconds

```bash
# Generate, optimize, and upload in one command
redisctl enterprise support-package cluster \
  --optimize \
  --upload
```

**Features:**
- Automatic generation
- 20-30% compression with `--optimize`
- Direct upload to Redis Support (Files.com)
- Works from anywhere (no SSH)

## Log Streaming

Real-time log following:

```bash
# Stream logs with auto-refresh
redisctl enterprise logs list --follow --interval 2

# With JMESPath filtering
redisctl enterprise logs list --follow \
  -q "[?level=='error']"
```

## License Management

```bash
# Check current license
redisctl enterprise license get

# Set new license
redisctl enterprise license set --key "license-string"

# View expiration
redisctl enterprise license get -q 'expiration_date'
```

## JMESPath Queries

Powerful filtering and transformation:

```bash
# Extract specific fields
redisctl enterprise database list \
  -q "[].{name:name,status:status,port:port}"

# Filter by condition
redisctl enterprise database list \
  -q "[?status=='active' && memory_size > `1000000000`]"

# Aggregate data
redisctl enterprise database list \
  -q "length([?status=='active'])"

# Sort results
redisctl enterprise database list \
  -q "sort_by(@, &name)[].name"
```

[Learn more about JMESPath →](https://jmespath.org)

## Multiple Output Formats

```bash
# JSON for scripts
redisctl enterprise database list | jq

# YAML for configs
redisctl enterprise database get 1 -o yaml > db-config.yaml

# Table for humans
redisctl enterprise database list -o table
```

## CI/CD Integration

Perfect for pipelines:

```bash
# Get status and fail if not active
DB_STATUS=$(redisctl enterprise database get 1 -o json -q 'status')
if [ "$DB_STATUS" != "active" ]; then
  echo "Database not ready"
  exit 1
fi

# Create database and wait
redisctl enterprise database create \
  --name "ci-db-$BUILD_NUMBER" \
  --memory-size "100MB" \
  --wait

# Get connection endpoint
ENDPOINT=$(redisctl enterprise database get 1 \
  -q 'endpoints[0].dns_name')
```

## Async Operation Handling

No more manual polling:

```bash
# Old way: manual polling loop
TASK_ID=$(curl ... | jq -r '.taskId')
while true; do
  STATUS=$(curl .../tasks/$TASK_ID | jq -r '.status')
  [ "$STATUS" = "completed" ] && break
  sleep 2
done

# New way: automatic with --wait
redisctl cloud database create ... --wait
```

---

**Previous:** [6. Workflows Layer](./06-workflows.md)  
**Next →** [8. Library Architecture](./08-libraries.md)
