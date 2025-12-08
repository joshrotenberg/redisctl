# Best Practices

Recommended patterns and practices for using redisctl effectively.

## Profile Management

### Use Separate Profiles for Each Environment

```bash
# Development
redisctl profile set dev \
  --deployment cloud \
  --api-key "$DEV_API_KEY" \
  --api-secret "$DEV_SECRET"

# Staging
redisctl profile set staging \
  --deployment cloud \
  --api-key "$STAGING_API_KEY" \
  --api-secret "$STAGING_SECRET"

# Production
redisctl profile set prod \
  --deployment cloud \
  --api-key "$PROD_API_KEY" \
  --api-secret "$PROD_SECRET"
```

### Naming Conventions

Use consistent, descriptive profile names:

```bash
# Good
cloud-prod
cloud-staging
enterprise-prod
enterprise-dr

# Avoid
prod1
test
my-profile
```

### Secure Credential Storage

```bash
# Use environment variables in config
# ~/.config/redisctl/config.toml
[profiles.prod]
deployment_type = "cloud"
api_key = "${REDIS_PROD_API_KEY}"
api_secret = "${REDIS_PROD_SECRET}"

# Set restrictive permissions
chmod 600 ~/.config/redisctl/config.toml

# Use secret management tools
export REDIS_PROD_API_KEY=$(vault kv get -field=api_key secret/redis/prod)
```

## Command Usage

### Always Specify Profile for Production

```bash
# Explicit is better than implicit
redisctl --profile prod cloud database list --subscription-id 123

# Avoid relying on default profile for production
redisctl cloud database delete --subscription-id 123 --database-id 456  # Dangerous!
```

### Use Output Formats Appropriately

```bash
# Human reading: table
redisctl cloud subscription list -o table

# Scripting: json with jq
redisctl cloud subscription list -o json | jq -r '.[].id'

# Quick checks: query
redisctl cloud database get --subscription-id 123 --database-id 456 -q "status"
```

### Implement Idempotent Operations

```bash
# Check before create
check_database_exists() {
  local name=$1
  redisctl cloud database list --subscription-id 123 \
    -q "[?name=='$name'].databaseId" | jq -r '.[]'
}

# Only create if doesn't exist
DB_ID=$(check_database_exists "my-database")
if [ -z "$DB_ID" ]; then
  redisctl cloud database create --subscription-id 123 --data @db.json --wait
fi
```

## Error Handling

### Always Check Exit Codes

```bash
#!/bin/bash
set -euo pipefail  # Exit on error, undefined variables, pipe failures

# Check individual commands
if ! redisctl cloud subscription list > /dev/null 2>&1; then
  echo "Failed to list subscriptions"
  exit 1
fi

# Or use && and ||
redisctl cloud database create --subscription-id 123 --data @db.json --wait \
  && echo "Database created successfully" \
  || { echo "Database creation failed"; exit 1; }
```

### Implement Retry Logic

```bash
retry_command() {
  local max_attempts=${MAX_ATTEMPTS:-3}
  local delay=${RETRY_DELAY:-5}
  local attempt=1
  
  while [ $attempt -le $max_attempts ]; do
    if "$@"; then
      return 0
    fi
    
    echo "Attempt $attempt failed. Retrying in ${delay}s..." >&2
    sleep $delay
    attempt=$((attempt + 1))
    delay=$((delay * 2))  # Exponential backoff
  done
  
  echo "Command failed after $max_attempts attempts" >&2
  return 1
}

# Usage
retry_command redisctl cloud database list --subscription-id 123
```

### Log Operations

```bash
# Create audit log
log_operation() {
  local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
  local user=$(whoami)
  local command="$*"
  
  echo "[$timestamp] User: $user, Command: $command" >> ~/.redisctl/audit.log
  
  # Execute and log result
  if "$@"; then
    echo "[$timestamp] Result: SUCCESS" >> ~/.redisctl/audit.log
    return 0
  else
    local exit_code=$?
    echo "[$timestamp] Result: FAILED (exit: $exit_code)" >> ~/.redisctl/audit.log
    return $exit_code
  fi
}

# Usage
log_operation redisctl --profile prod cloud database delete \
  --subscription-id 123 --database-id 456 --wait
```

## Async Operations

### Always Use --wait for Critical Operations

```bash
# Good: Wait for completion
redisctl cloud database create \
  --subscription-id 123 \
  --data @database.json \
  --wait \
  --wait-timeout 900

# Risky: Fire and forget
redisctl cloud database create \
  --subscription-id 123 \
  --data @database.json  # Might fail silently
```

### Handle Timeouts Gracefully

```bash
# Create with timeout handling
create_database_with_retry() {
  local attempt=1
  local max_attempts=3
  
  while [ $attempt -le $max_attempts ]; do
    echo "Creating database (attempt $attempt)..."
    
    if redisctl cloud database create \
      --subscription-id 123 \
      --data @database.json \
      --wait \
      --wait-timeout 600; then
      echo "Database created successfully"
      return 0
    fi
    
    echo "Creation failed or timed out"
    attempt=$((attempt + 1))
    
    # Check if partially created
    DB_ID=$(redisctl cloud database list --subscription-id 123 \
      -q "[?name=='my-database'].databaseId" | jq -r '.[]')
    
    if [ -n "$DB_ID" ]; then
      echo "Database partially created with ID: $DB_ID"
      # Clean up or continue based on state
      return 1
    fi
  done
  
  return 1
}
```

## Security

### Never Hardcode Credentials

```bash
# Bad
redisctl profile set prod \
  --api-key "abc123def456" \
  --api-secret "secret789xyz"

# Good
redisctl profile set prod \
  --api-key "$REDIS_API_KEY" \
  --api-secret "$REDIS_API_SECRET"

# Better
redisctl profile set prod \
  --api-key "$(vault kv get -field=api_key secret/redis)" \
  --api-secret "$(vault kv get -field=api_secret secret/redis)"
```

### Rotate Credentials Regularly

```bash
#!/bin/bash
# rotate-credentials.sh

# Generate new API key (via Redis Cloud UI or API)
NEW_API_KEY=$(generate_new_api_key)
NEW_API_SECRET=$(generate_new_api_secret)

# Update profile
redisctl profile set prod \
  --api-key "$NEW_API_KEY" \
  --api-secret "$NEW_API_SECRET"

# Test new credentials
if redisctl --profile prod cloud subscription list > /dev/null 2>&1; then
  echo "New credentials working"
  # Revoke old credentials
  revoke_old_credentials
else
  echo "New credentials failed, keeping old ones"
  exit 1
fi
```

### Audit Access

```bash
# Track who uses production credentials
alias redisctl-prod='log_operation redisctl --profile prod'

# Review audit logs regularly
grep "profile prod" ~/.redisctl/audit.log | tail -20
```

## Performance

### Cache Frequently Used Data

```bash
# Cache subscription list for 5 minutes
get_subscriptions() {
  local cache_file="/tmp/redisctl-subs-cache.json"
  local cache_age=$((5 * 60))  # 5 minutes
  
  # Check cache age
  if [ -f "$cache_file" ]; then
    local file_age=$(($(date +%s) - $(stat -f %m "$cache_file" 2>/dev/null || stat -c %Y "$cache_file")))
    if [ $file_age -lt $cache_age ]; then
      cat "$cache_file"
      return 0
    fi
  fi
  
  # Refresh cache
  redisctl cloud subscription list -o json | tee "$cache_file"
}
```

### Batch Operations

```bash
# Good: Single command with multiple operations
redisctl cloud database update \
  --subscription-id 123 \
  --database-id 456 \
  --data '{
    "memoryLimitInGb": 16,
    "throughputMeasurement": {"by": "operations-per-second", "value": 50000},
    "alerts": [{"name": "dataset-size", "value": 90}]
  }'

# Avoid: Multiple separate updates
redisctl cloud database update --subscription-id 123 --database-id 456 \
  --data '{"memoryLimitInGb": 16}'
redisctl cloud database update --subscription-id 123 --database-id 456 \
  --data '{"throughputMeasurement": {"by": "operations-per-second", "value": 50000}}'
```

### Use Appropriate Query Filters

```bash
# Efficient: Filter at API level
redisctl api cloud get /subscriptions --query-params "status=active"

# Less efficient: Filter after fetching
redisctl cloud subscription list -o json | jq '.[] | select(.status == "active")'
```

## Automation

### Create Reusable Scripts

```bash
#!/bin/bash
# provision-database.sh

set -euo pipefail

# Required parameters
ENVIRONMENT=${1:?Environment required (dev/staging/prod)}
DATABASE_NAME=${2:?Database name required}
MEMORY_GB=${3:-4}

# Load environment config
source "config/${ENVIRONMENT}.env"

# Create database config
cat > /tmp/database.json <<EOF
{
  "name": "${DATABASE_NAME}-${ENVIRONMENT}",
  "memoryLimitInGb": ${MEMORY_GB},
  "replication": $([ "$ENVIRONMENT" = "prod" ] && echo "true" || echo "false"),
  "dataPersistence": "$([ "$ENVIRONMENT" = "prod" ] && echo "aof-every-1-second" || echo "none")"
}
EOF

# Create database
redisctl --profile "${ENVIRONMENT}-cloud" cloud database create \
  --subscription-id "${SUBSCRIPTION_ID}" \
  --data @/tmp/database.json \
  --wait

# Clean up
rm /tmp/database.json
```

### Use Configuration Files

```bash
# config/environments.yaml
environments:
  development:
    profile: dev-cloud
    subscription_id: 12345
    defaults:
      memory_gb: 2
      replication: false
      persistence: none
  
  production:
    profile: prod-cloud
    subscription_id: 67890
    defaults:
      memory_gb: 16
      replication: true
      persistence: aof-every-1-second
```

### Implement GitOps

```bash
# .github/workflows/redis-sync.yml
name: Sync Redis Configuration

on:
  push:
    paths:
      - 'redis-config/*.json'

jobs:
  sync:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Apply configurations
        run: |
          for config in redis-config/*.json; do
            db_name=$(basename "$config" .json)
            redisctl cloud database update \
              --subscription-id ${{ secrets.SUBSCRIPTION_ID }} \
              --database-id $(cat "redis-config/${db_name}.id") \
              --data @"$config" \
              --wait
          done
```

## Monitoring and Alerting

### Regular Health Checks

```bash
#!/bin/bash
# health-check.sh

check_database_health() {
  local sub_id=$1
  local db_id=$2
  
  local status=$(redisctl cloud database get \
    --subscription-id "$sub_id" \
    --database-id "$db_id" \
    -q "status")
  
  if [ "$status" != "active" ]; then
    alert "Database $db_id is $status"
    return 1
  fi
  
  return 0
}

# Run checks
while read -r sub_id db_id; do
  check_database_health "$sub_id" "$db_id"
done < databases.txt
```

### Track Changes

```bash
# Before making changes
backup_configuration() {
  local timestamp=$(date +%Y%m%d_%H%M%S)
  local backup_dir="backups/${timestamp}"
  
  mkdir -p "$backup_dir"
  
  # Backup all database configs
  while read -r sub_id; do
    redisctl cloud database list --subscription-id "$sub_id" \
      -o json > "${backup_dir}/sub_${sub_id}_databases.json"
  done < subscriptions.txt
  
  echo "Configuration backed up to $backup_dir"
}
```

## Documentation

### Document Your Setup

```bash
# Create README for your Redis setup
cat > Redis-Setup.md <<'EOF'
# Redis Infrastructure

## Profiles
- `prod-cloud`: Production Cloud environment
- `prod-enterprise`: Production Enterprise cluster
- `dr-enterprise`: Disaster recovery cluster

## Key Databases
- `user-sessions`: Session storage (16GB, 100k ops/sec)
- `product-cache`: Product catalog cache (8GB, 50k ops/sec)
- `analytics-stream`: Analytics event stream (32GB, 200k ops/sec)

## Maintenance Windows
- Production: Sunday 2-4 AM UTC
- Staging: Any time

## Runbooks
- [Database Creation](./runbooks/create-database.md)
- [Scaling Operations](./runbooks/scaling.md)
- [Disaster Recovery](./runbooks/dr.md)
EOF
```

### Maintain Runbooks

```markdown
# Runbook: Database Scaling

## When to Scale
- Memory usage > 80% for 30 minutes
- Throughput > 90% of limit
- Latency > 5ms p99

## How to Scale

1. Check current metrics:
   ```bash
   ./scripts/check-metrics.sh prod-database
   ```

2. Calculate new size:
   - Memory: Current usage * 1.5
   - Throughput: Current peak * 2

3. Apply scaling:
   ```bash
   ./scripts/scale-database.sh prod-database --memory 32 --throughput 200000
   ```

4. Verify:
   ```bash
   ./scripts/verify-scaling.sh prod-database
   ```
```

## Summary Checklist

✅ **Profiles**: Use separate profiles for each environment  
✅ **Security**: Never hardcode credentials  
✅ **Error Handling**: Check exit codes and implement retries  
✅ **Async Ops**: Always use --wait for critical operations  
✅ **Logging**: Audit all production operations  
✅ **Automation**: Create reusable, parameterized scripts  
✅ **Monitoring**: Implement regular health checks  
✅ **Documentation**: Maintain runbooks and setup documentation  
✅ **Testing**: Test changes in non-production first  
✅ **Backups**: Backup configurations before changes