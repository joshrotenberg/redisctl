# Examples

This section showcases common tasks and powerful features to get you started with redisctl.

## Quick Setup

First, set up your authentication profile:

```bash
# For Redis Cloud
redisctl profile set cloud --api-key YOUR_KEY --api-secret YOUR_SECRET

# For Redis Enterprise (local Docker)
redisctl profile set enterprise --url https://localhost:9443 \
  --username admin@redis.local --password Redis123! --insecure
```

## Redis Cloud Examples

### List All Resources

```bash
# See all your subscriptions
redisctl cloud subscription list -o table

# Get detailed info about databases
redisctl cloud database list -q '[].{name: name, port: port, status: status}'

# Output (example):
# [
#   {"name": "cache-prod", "port": 12000, "status": "active"}
# ]
```

### Create and Manage Databases

```bash
# Create a new database with wait for completion
redisctl cloud database create --subscription-id 123456 \
  --data '{"name": "my-cache", "memoryLimitInGb": 1}' --wait

# Monitor async operation progress
redisctl cloud task get 456789

# Update database configuration
redisctl cloud database update 123456:789 \
  --data '{"memoryLimitInGb": 2}' --wait
```

### Network Security

```bash
# Set up VPC peering
redisctl cloud connectivity vpc-peering create --subscription-id 123456 \
  --data @vpc-config.json --wait

# Configure ACL rules
redisctl cloud acl rule create --subscription-id 123456 \
  --data '{"name": "allow-app", "sourceIps": ["10.0.0.0/24"]}'
```

## Redis Enterprise Examples

### Cluster Management

```bash
# Get cluster health overview
redisctl enterprise cluster get -q '{name: name, license_expired: license_expired}'

# Output:
# {"name": "prod-cluster", "license_expired": false}

# View all nodes status
redisctl enterprise node list -o table
```

### Database Operations

```bash
# Create a database with replication
redisctl enterprise database create \
  --data '{"name": "session-store", "memory_size": 1073741824, "replication": true}' \
  --wait

# Get database metrics
redisctl enterprise database stats 1 \
  -q 'intervals[0].{ops_sec: ops_sec, used_memory: used_memory, connected_clients: connected_clients}'

# Trigger backup
redisctl enterprise database backup 1
```

### Support and Diagnostics

```bash
# Generate support package for troubleshooting
redisctl enterprise support-package create

# Check license status
redisctl enterprise license get -q '{license_expired: license_expired, expired_date: expired_date}'

# View recent cluster logs
redisctl enterprise logs list --limit 50
```

## Power User Features

### Raw API Access

```bash
# Direct API calls when you need something not yet wrapped
redisctl api cloud get /subscriptions/123456/databases \
  -q "[?status=='active'].{name:name, port:port}"

# POST with custom payload
redisctl api enterprise post /v1/bdbs --data @database-config.json
```

### Async Operations with Custom Timeouts

```bash
# Long-running operations with progress updates
redisctl cloud database create --subscription-id 123456 \
  --data @large-db.json \
  --wait --wait-timeout 1200 --wait-interval 30

# The command will:
# - Poll every 30 seconds
# - Show progress spinner
# - Timeout after 20 minutes
# - Return full operation result
```

### JMESPath Filtering

```bash
# Complex queries on JSON output
redisctl cloud subscription list -o json \
  -q "[?paymentMethodId=='12345'] | [0:3].{id:id, name:name, databases:databases[].name}"

# Find databases by port range
redisctl enterprise database list -o json \
  -q "[?port >= `12000` && port <= `13000`].{name:name, port:port}"
```

### Secure Credential Storage

```bash
# Store credentials in OS keyring (macOS Keychain, Windows Credential Store, etc.)
redisctl profile set cloud-prod \
  --api-key YOUR_KEY \
  --api-secret YOUR_SECRET \
  --use-keyring

# Credentials are now encrypted in your OS keyring
# No plaintext secrets in config files!
```

## Scripting and Automation

### CI/CD Pipeline Example

```bash
#!/bin/bash
# deploy-database.sh

# Exit on error
set -e

# Create database and extract ID using JMESPath
DB_ID=$(redisctl cloud database create \
  --subscription-id $SUBSCRIPTION_ID \
  --data @config.json \
  --wait \
  -q 'databaseId')

# Get endpoint
ENDPOINT=$(redisctl cloud database get $DB_ID -q 'endpoint')

# Update application configuration
echo "REDIS_URL=redis://$ENDPOINT" >> .env

# Verify connectivity
redisctl cloud database get $DB_ID -q 'status'
```

### Batch Operations

```bash
# Update multiple databases
for db_id in $(redisctl enterprise database list -q '[].uid' --raw); do
  echo "Updating database $db_id"
  redisctl enterprise database update $db_id \
    --data '{"backup_interval": 3600}' \
    --wait
done
```

## Next Steps

- Check out deployment-specific commands in [Cloud](../cloud/overview.md) or [Enterprise](../enterprise/overview.md) sections
- Learn about [Output Formats](../common-features/output-formats.md) for better data manipulation
- Set up [Secure Storage](../common-features/secure-storage.md) for your credentials
- Explore [Async Operations](../common-features/async-operations.md) for long-running tasks