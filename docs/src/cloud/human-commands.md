# Human-Friendly Commands

These commands provide a typed, user-friendly interface to common Redis Cloud operations.

## Subscription Management

```bash
# List all subscriptions
redisctl cloud subscription list

# Get subscription details
redisctl cloud subscription get <subscription-id>

# Create new subscription
redisctl cloud subscription create \
  --name "Production" \
  --payment-method-id <id> \
  --cloud-provider "AWS" \
  --region "us-east-1"
```

## Database Management

```bash
# List databases
redisctl cloud database list --subscription-id <id>

# Get database details
redisctl cloud database get \
  --subscription-id <id> \
  --database-id <id>

# Create database
redisctl cloud database create \
  --subscription-id <id> \
  --name "cache-db" \
  --memory-limit 1024 \
  --modules "search,json"

# Update database
redisctl cloud database update \
  --subscription-id <id> \
  --database-id <id> \
  --memory-limit 2048
```

## User Management

```bash
# List users
redisctl cloud user list

# Create user
redisctl cloud user create \
  --email "user@example.com" \
  --role "viewer"

# Update user role
redisctl cloud user update <user-id> \
  --role "admin"
```

## ACL Management

```bash
# List ACL rules
redisctl cloud acl list --subscription-id <id>

# Create ACL rule
redisctl cloud acl create \
  --subscription-id <id> \
  --name "readonly" \
  --rule "+get +mget -flushdb"
```

## Backup Operations

```bash
# List backups
redisctl cloud backup list \
  --subscription-id <id> \
  --database-id <id>

# Create backup
redisctl cloud backup create \
  --subscription-id <id> \
  --database-id <id>

# Restore from backup
redisctl cloud backup restore \
  --subscription-id <id> \
  --database-id <id> \
  --backup-id <id>
```

## Account Information

```bash
# Get account details
redisctl cloud account info

# Get payment methods
redisctl cloud account payment-methods

# Get cloud accounts
redisctl cloud cloud-account list
```

## Output Formatting

All commands support output formatting:

```bash
# Table format
redisctl cloud subscription list -o table

# JSON (default)
redisctl cloud subscription list -o json

# YAML
redisctl cloud subscription list -o yaml

# Filtered with JMESPath
redisctl cloud database list \
  --subscription-id <id> \
  -q "[?status=='active'].{name:name,memory:memoryLimitInGb}"
```