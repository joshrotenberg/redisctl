# redisctl Features

## Async Operations with --wait Support

The `redisctl` CLI provides comprehensive support for asynchronous operations across both Redis Cloud and Redis Enterprise APIs. All create, update, and delete operations support the `--wait` flag family for tracking long-running operations.

### Wait Flag Options

| Flag | Description | Default |
|------|-------------|---------|
| `--wait` | Wait for operation to complete | Timeout: 600s |
| `--wait-timeout <seconds>` | Custom timeout duration | 600 |
| `--wait-interval <seconds>` | Polling interval | 10 |

### Supported Operations by Category

## Database Operations

### Regular Databases
```bash
# Create database and wait for completion
redisctl cloud database create --subscription-id 12345 --data @database.json --wait

# Update database with custom timeout
redisctl cloud database update --subscription-id 12345 --database-id 67890 \
  --data @updates.json --wait --wait-timeout 300

# Delete database with force
redisctl cloud database delete --subscription-id 12345 --database-id 67890 \
  --force --wait

# Import data into database
redisctl cloud database import --subscription-id 12345 --database-id 67890 \
  --data @import.json --wait

# Create backup
redisctl cloud database backup --subscription-id 12345 --database-id 67890 --wait

# Migrate database
redisctl cloud database migrate --subscription-id 12345 --database-id 67890 \
  --data @migration.json --wait
```

### Fixed Databases
```bash
# Create fixed database
redisctl cloud fixed-database create --subscription-id 12345 \
  --data @fixed-db.json --wait

# Update fixed database
redisctl cloud fixed-database update --subscription-id 12345 --database-id 67890 \
  --data @updates.json --wait

# Delete fixed database
redisctl cloud fixed-database delete --subscription-id 12345 --database-id 67890 \
  --force --wait
```

### Active-Active Databases (CRDB)
```bash
# Create Active-Active database
redisctl cloud crdb create --subscription-id 12345 --data @crdb.json --wait

# Update Active-Active database
redisctl cloud crdb update --subscription-id 12345 --database-id 67890 \
  --data @updates.json --wait

# Delete Active-Active database
redisctl cloud crdb delete --subscription-id 12345 --database-id 67890 \
  --force --wait
```

## Subscription Management

### Regular Subscriptions
```bash
# Create subscription
redisctl cloud subscription create --data @subscription.json --wait

# Update subscription
redisctl cloud subscription update 12345 --data @updates.json \
  --wait --wait-interval 5

# Delete subscription
redisctl cloud subscription delete 12345 --force --wait
```

### Fixed Subscriptions
```bash
# Create fixed subscription
redisctl cloud fixed-subscription create --data @fixed-sub.json --wait

# Update fixed subscription
redisctl cloud fixed-subscription update 12345 --data @updates.json --wait

# Delete fixed subscription
redisctl cloud fixed-subscription delete 12345 --force --wait
```

## Network Connectivity

### VPC Peering
```bash
# Create VPC peering
redisctl cloud connectivity vpc-peering create 12345 --data @peering.json --wait

# Update VPC peering (regular)
redisctl cloud connectivity vpc-peering update 12345 67890 \
  --data @updates.json --wait

# Delete VPC peering
redisctl cloud connectivity vpc-peering delete 12345 67890 --force --wait

# Active-Active VPC operations
redisctl cloud connectivity vpc-peering create-aa 12345 --data @aa-peering.json --wait
redisctl cloud connectivity vpc-peering update-aa 12345 67890 \
  --data @updates.json --wait
redisctl cloud connectivity vpc-peering delete-aa 12345 67890 --force --wait
```

### Private Service Connect (GCP)
```bash
# Create PSC
redisctl cloud connectivity psc create 12345 --data @psc.json --wait

# Update PSC
redisctl cloud connectivity psc update 12345 67890 --data @updates.json --wait

# Delete PSC
redisctl cloud connectivity psc delete 12345 67890 --force --wait

# Active-Active PSC operations
redisctl cloud connectivity psc create-aa 12345 --data @aa-psc.json --wait
redisctl cloud connectivity psc update-aa 12345 67890 --data @updates.json --wait
redisctl cloud connectivity psc delete-aa 12345 67890 --force --wait
```

### Transit Gateway (AWS)
```bash
# Create Transit Gateway
redisctl cloud connectivity tgw create 12345 --data @tgw.json --wait

# Attach Transit Gateway
redisctl cloud connectivity tgw attach 12345 67890 --data @attach.json --wait

# Detach Transit Gateway
redisctl cloud connectivity tgw detach 12345 67890 --force --wait

# Delete Transit Gateway
redisctl cloud connectivity tgw delete 12345 67890 --force --wait

# Active-Active TGW operations
redisctl cloud connectivity tgw create-aa 12345 --data @aa-tgw.json --wait
redisctl cloud connectivity tgw attach-aa 12345 67890 --data @attach.json --wait
redisctl cloud connectivity tgw detach-aa 12345 67890 --force --wait
```

## ACL Management

### Redis ACL Rules
```bash
# Create ACL rule
redisctl cloud acl create-redis-rule --name "read-only" --rule "+@read" --wait

# Update ACL rule
redisctl cloud acl update-redis-rule --id 123 --rule "+@write" --wait

# Delete ACL rule
redisctl cloud acl delete-redis-rule --id 123 --force --wait
```

### ACL Roles
```bash
# Create role
redisctl cloud acl create-role --name "app-role" \
  --redis-rules '[{"rule_id": 123}]' --wait

# Update role
redisctl cloud acl update-role --id 456 --name "updated-role" --wait

# Delete role
redisctl cloud acl delete-role --id 456 --force --wait
```

### ACL Users
```bash
# Create ACL user
redisctl cloud acl create-acl-user --name "app-user" --role "app-role" \
  --password "secure123" --wait

# Update ACL user
redisctl cloud acl update-acl-user --id 789 --role "admin" --wait

# Delete ACL user
redisctl cloud acl delete-acl-user --id 789 --force --wait
```

## User and Account Management

### User Operations
```bash
# Delete user (with async support)
redisctl cloud user delete 123 --force --wait
```

### Provider Account Operations
```bash
# Create provider account
redisctl cloud provider-account create --file @aws-account.json --wait

# Update provider account
redisctl cloud provider-account update 456 --file @updates.json --wait

# Delete provider account
redisctl cloud provider-account delete 456 --force --wait
```

## Progress Tracking

When using the `--wait` flag, redisctl provides real-time progress tracking:

1. **Initial Response**: Shows the task ID and initial status
2. **Progress Updates**: Animated spinner with status updates every interval
3. **Completion**: Final status with operation result
4. **Error Handling**: Clear error messages if operation fails

### Example Output
```
Creating database...
⠋ Waiting for task 12345 to complete... (10s)
⠙ Status: processing (20s)
⠹ Status: processing (30s)
✓ Database creation completed successfully
```

## Error Handling

### Timeout Behavior
If an operation exceeds the timeout:
- The CLI exits with an error
- The task continues running in the background
- You can check status using the task ID

### Recovery Options
```bash
# Check task status manually
redisctl cloud task get 12345

# Increase timeout for long operations
redisctl cloud database create --data @large-db.json \
  --wait --wait-timeout 1800  # 30 minutes

# Reduce polling interval for faster updates
redisctl cloud database create --data @database.json \
  --wait --wait-interval 2  # Check every 2 seconds
```

## Implementation Details

### Centralized Async Handling
All async operations use the `handle_async_response` function which:
- Extracts task IDs from API responses
- Polls for task completion
- Provides consistent progress indicators
- Handles timeouts and errors uniformly

### Parameter Grouping
To maintain clean code and avoid "too many arguments" warnings, operations use parameter structs:
- `AsyncOperationArgs` - Wait flag options
- `ConnectivityOperationParams` - Network operations
- `CloudAccountOperationParams` - Account operations
- `AclOperationParams` - ACL operations

### Task ID Detection
The system automatically detects task IDs from various response formats:
- `taskId` field in response
- `links` array with task references
- Nested task objects

## Best Practices

### Choosing Timeouts
- **Small operations**: Default 600s is usually sufficient
- **Large databases**: Increase to 1800s or more
- **Bulk operations**: Consider 3600s for very large datasets
- **Network operations**: May need longer timeouts in some regions

### Polling Intervals
- **Default (10s)**: Good balance for most operations
- **Fast operations (2-5s)**: For operations you expect to complete quickly
- **Long operations (30-60s)**: Reduce API calls for very long operations

### Force Flags
Always use `--force` carefully:
- Skips confirmation prompts
- Combines well with `--wait` for automation
- Required for scripted operations

## Automation Examples

### CI/CD Pipeline
```bash
#!/bin/bash
# Create infrastructure with full async tracking
redisctl cloud subscription create --data @prod-sub.json \
  --wait --wait-timeout 1800 || exit 1

SUB_ID=$(redisctl cloud subscription list -q "[0].id" -o json)

redisctl cloud database create --subscription-id $SUB_ID \
  --data @prod-db.json --wait --wait-timeout 900 || exit 1

echo "Infrastructure ready!"
```

### Bulk Operations
```bash
#!/bin/bash
# Create multiple databases in parallel
for i in {1..5}; do
  redisctl cloud database create --subscription-id 12345 \
    --data @db-$i.json --wait &
done

# Wait for all background jobs
wait
echo "All databases created!"
```

### Error Recovery
```bash
#!/bin/bash
# Retry with exponential backoff
RETRY=0
MAX_RETRY=3

while [ $RETRY -lt $MAX_RETRY ]; do
  if redisctl cloud database create --subscription-id 12345 \
    --data @database.json --wait --wait-timeout $((600 * (RETRY + 1))); then
    echo "Success!"
    break
  fi
  RETRY=$((RETRY + 1))
  echo "Retry $RETRY of $MAX_RETRY..."
  sleep $((10 * RETRY))
done
```