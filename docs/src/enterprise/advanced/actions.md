# Actions (Async Tasks)

Actions in Redis Enterprise represent asynchronous operations or tasks that are running or have completed. The action commands allow you to monitor and manage these background operations.

## Overview

Many Redis Enterprise operations are asynchronous, returning an action ID that can be used to track progress. Actions include database creation/deletion, backup operations, imports/exports, and cluster maintenance tasks.

## Available Commands

### List All Actions

List all actions in the cluster with optional filtering:

```bash
# List all actions
redisctl enterprise action list

# Filter by status
redisctl enterprise action list --status completed
redisctl enterprise action list --status running

# Filter by type
redisctl enterprise action list --type bdb_backup

# Combine filters
redisctl enterprise action list --status running --type bdb_import

# Output as table
redisctl enterprise action list -o table
```

### Get Action Details

Get detailed information about a specific action:

```bash
# Get action by UID
redisctl enterprise action get <action_uid>

# Get action with specific fields using JMESPath
redisctl enterprise action get <action_uid> -q "status"
```

### Check Action Status

Quick status check for an action (returns just the status field):

```bash
redisctl enterprise action status <action_uid>
```

### Cancel Running Action

Cancel a running action:

```bash
redisctl enterprise action cancel <action_uid>
```

### List Actions for Database

List all actions for a specific database:

```bash
redisctl enterprise action list-for-bdb <bdb_uid>

# Filter by status for specific database
redisctl enterprise action list-for-bdb <bdb_uid> --status running
```

## Action Types

Common action types you'll encounter:

- `bdb_create` - Database creation
- `bdb_delete` - Database deletion
- `bdb_update` - Database configuration update
- `bdb_backup` - Database backup operation
- `bdb_import` - Database import operation
- `bdb_export` - Database export operation
- `crdb_create` - Active-Active database creation
- `node_join` - Node joining cluster
- `cluster_recovery` - Cluster recovery operation

## Action Statuses

Actions can have the following statuses:

- `queued` - Action is queued for execution
- `running` - Action is currently executing
- `completed` - Action completed successfully
- `failed` - Action failed with errors
- `canceled` - Action was canceled

## Examples

### Monitor Database Creation

```bash
# Create a database (returns action_uid)
ACTION_UID=$(redisctl enterprise database create --data @db.json -q "action_uid")

# Check status
redisctl enterprise action status $ACTION_UID

# Get full details when complete
redisctl enterprise action get $ACTION_UID
```

### List Recent Failed Actions

```bash
# List failed actions in table format
redisctl enterprise action list --status failed -o table

# Get details of a failed action
redisctl enterprise action get <failed_action_uid> -q "{error: error_message, started: start_time}"
```

### Cancel Long-Running Import

```bash
# List running imports
redisctl enterprise action list --status running --type bdb_import

# Cancel specific import
redisctl enterprise action cancel <import_action_uid>
```

### Monitor All Database Actions

```bash
# Watch all actions for a database
watch -n 5 "redisctl enterprise action list-for-bdb 1 -o table"
```

## Integration with Async Operations

The action commands work seamlessly with the `--wait` flag available on create/update/delete operations:

```bash
# This uses action monitoring internally
redisctl enterprise database create --data @db.json --wait

# Equivalent to manually monitoring:
ACTION_UID=$(redisctl enterprise database create --data @db.json -q "action_uid")
while [ "$(redisctl enterprise action status $ACTION_UID)" = "running" ]; do
  sleep 5
done
```

## API Versions

The action commands support both v1 and v2 API endpoints:
- v2 endpoints (`/v2/actions`) are preferred when available
- v1 endpoints (`/v1/actions`) are used as fallback
- Both return the same data structure

## Best Practices

1. **Always check action status** for async operations before proceeding
2. **Use filtering** to reduce output when listing many actions
3. **Save action UIDs** from create/update operations for tracking
4. **Set up monitoring** for critical long-running actions
5. **Check failed actions** for error details to diagnose issues

## Related Commands

- `enterprise database` - Database operations that create actions
- `enterprise cluster` - Cluster operations that create actions
- `enterprise crdb` - Active-Active operations that create actions