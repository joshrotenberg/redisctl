# Async Operations

The `redisctl` CLI provides comprehensive support for asynchronous operations across both Redis Cloud and Redis Enterprise APIs. All create, update, and delete operations support the `--wait` flag family for tracking long-running operations.

## Overview

Many Redis Cloud API operations are asynchronous, returning immediately with a task ID while the operation continues in the background. The `--wait` flags allow you to:

- Wait for operations to complete before returning
- Track progress with visual indicators
- Set custom timeouts for long operations
- Configure polling intervals

## Wait Flag Options

| Flag | Description | Default |
|------|-------------|---------|
| `--wait` | Wait for operation to complete | Timeout: 600s |
| `--wait-timeout <seconds>` | Custom timeout duration | 600 |
| `--wait-interval <seconds>` | Polling interval | 10 |

## Basic Usage

```bash
# Create database and wait for completion
redisctl cloud database create --subscription-id 12345 \
  --data @database.json --wait

# With custom timeout for large operations
redisctl cloud database create --subscription-id 12345 \
  --data @large-db.json --wait --wait-timeout 1800

# With faster polling for quick operations
redisctl cloud database update --subscription-id 12345 \
  --database-id 67890 --data @updates.json \
  --wait --wait-interval 2
```

## Progress Tracking

When using the `--wait` flag, redisctl provides real-time progress tracking:

```
Creating database...
⠋ Waiting for task 12345 to complete... (10s)
⠙ Status: processing (20s)
⠹ Status: processing (30s)
✓ Database creation completed successfully
```

## Supported Operations

Async operations are supported across all major command categories:

- [Database Operations](./database-operations.md) - Create, update, delete, import, backup, migrate
- [Subscription Management](./subscription-management.md) - Regular and fixed subscriptions
- [Network Connectivity](./network-connectivity.md) - VPC Peering, PSC, Transit Gateway
- [ACL Management](./acl-management.md) - Rules, roles, and users
- [User & Account Management](./user-management.md) - Users and provider accounts

## Error Handling

### Timeout Behavior

If an operation exceeds the timeout:
- The CLI exits with an error
- The task continues running in the background
- You can check status using the task ID

```bash
# Operation times out
Error: Operation timed out after 600 seconds. Task 12345 is still running.

# Check task status manually
redisctl cloud task get 12345
```

### Recovery Options

```bash
# Retry with longer timeout
redisctl cloud database create --data @database.json \
  --wait --wait-timeout 1800

# Check task status without waiting
redisctl cloud task list --status pending
```

## Best Practices

### Choosing Timeouts

- **Small operations**: Default 600s is usually sufficient
- **Large databases**: Increase to 1800s (30 min) or more
- **Bulk operations**: Consider 3600s (1 hour) for very large datasets
- **Network operations**: May need longer timeouts in some regions

### Polling Intervals

- **Default (10s)**: Good balance for most operations
- **Fast operations (2-5s)**: For operations you expect to complete quickly
- **Long operations (30-60s)**: Reduce API calls for very long operations

### Automation

The `--wait` flags are designed for automation:

```bash
#!/bin/bash
# CI/CD pipeline example
set -e  # Exit on error

# Create infrastructure
redisctl cloud subscription create --data @prod-sub.json \
  --wait --wait-timeout 1800

SUB_ID=$(redisctl cloud subscription list -q "[0].id" -o json)

redisctl cloud database create --subscription-id $SUB_ID \
  --data @prod-db.json --wait --wait-timeout 900

echo "Infrastructure ready!"
```

## Parallel Operations

You can run multiple async operations in parallel:

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

## Implementation Details

All async operations use the centralized `handle_async_response` function which:
- Extracts task IDs from API responses
- Polls for task completion
- Provides consistent progress indicators
- Handles timeouts and errors uniformly

The system automatically detects task IDs from various response formats:
- `taskId` field in response
- `links` array with task references
- Nested task objects