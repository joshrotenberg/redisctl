# CRDB Tasks

CRDB tasks are background operations related to Active-Active (CRDB) databases in Redis Enterprise. These commands allow you to monitor and manage async tasks for CRDB operations like synchronization, migration, and backup.

## Overview

CRDB tasks include:
- Database synchronization between participating clusters
- Data migration operations
- Backup and restore operations for Active-Active databases
- Replication and conflict resolution tasks
- Schema changes across participating clusters

## Available Commands

### List All CRDB Tasks

List all CRDB tasks with optional filtering:

```bash
# List all CRDB tasks
redisctl enterprise crdb-task list

# Filter by task status
redisctl enterprise crdb-task list --status running
redisctl enterprise crdb-task list --status completed
redisctl enterprise crdb-task list --status failed

# Filter by task type
redisctl enterprise crdb-task list --type sync
redisctl enterprise crdb-task list --type migration
redisctl enterprise crdb-task list --type backup

# Filter by CRDB UID
redisctl enterprise crdb-task list --crdb-uid 1

# Combine filters
redisctl enterprise crdb-task list --status running --type sync --crdb-uid 1

# Output as table
redisctl enterprise crdb-task list -o table
```

### Get Task Details

Get detailed information about a specific CRDB task:

```bash
# Get task by ID
redisctl enterprise crdb-task get <task_id>

# Get specific fields using JMESPath
redisctl enterprise crdb-task get <task_id> -q "status"
redisctl enterprise crdb-task get <task_id> -q "{id: task_id, status: status, type: type}"
```

### Check Task Status

Quick status check for a CRDB task:

```bash
# Get just the status
redisctl enterprise crdb-task status <task_id>
```

### Get Task Progress

Monitor task progress information:

```bash
# Get progress details
redisctl enterprise crdb-task progress <task_id>

# Get progress percentage only
redisctl enterprise crdb-task progress <task_id> -q "progress_percent"
```

### Get Task Logs

Retrieve logs for a CRDB task:

```bash
# Get task logs
redisctl enterprise crdb-task logs <task_id>
```

### List Tasks by CRDB

List all tasks for a specific Active-Active database:

```bash
# List all tasks for a CRDB
redisctl enterprise crdb-task list-by-crdb <crdb_uid>

# Filter by status for specific CRDB
redisctl enterprise crdb-task list-by-crdb <crdb_uid> --status running

# Filter by type for specific CRDB
redisctl enterprise crdb-task list-by-crdb <crdb_uid> --type sync
```

### Task Control Operations

#### Cancel Task

Cancel a running CRDB task:

```bash
# Cancel with confirmation
redisctl enterprise crdb-task cancel <task_id>

# Cancel without confirmation
redisctl enterprise crdb-task cancel <task_id> --force
```

#### Retry Failed Task

Retry a failed CRDB task:

```bash
redisctl enterprise crdb-task retry <task_id>
```

Note: Retry functionality may not be available for all task types or Redis Enterprise versions.

#### Pause/Resume Tasks

Pause and resume CRDB tasks:

```bash
# Pause a running task
redisctl enterprise crdb-task pause <task_id>

# Resume a paused task
redisctl enterprise crdb-task resume <task_id>
```

Note: Pause/resume functionality may not be supported for all task types.

## Task Types

Common CRDB task types include:

- **sync** - Data synchronization between clusters
- **migration** - Data migration operations
- **backup** - CRDB backup operations
- **restore** - CRDB restore operations
- **rebalance** - Shard rebalancing across clusters
- **schema_change** - Schema modifications across participating clusters
- **conflict_resolution** - Resolving data conflicts between clusters

## Task Statuses

CRDB tasks can have the following statuses:

- **pending** - Task is queued for execution
- **running** - Task is currently executing
- **completed** - Task completed successfully
- **failed** - Task failed with errors
- **canceled** - Task was canceled by user
- **paused** - Task is paused (if supported)

## Examples

### Monitor CRDB Synchronization

```bash
# List all sync tasks
redisctl enterprise crdb-task list --type sync

# Check status of specific sync task
TASK_ID="task-12345"
redisctl enterprise crdb-task status $TASK_ID

# Monitor progress
watch -n 5 "redisctl enterprise crdb-task progress $TASK_ID"
```

### Handle Failed Migration

```bash
# Find failed migration tasks
redisctl enterprise crdb-task list --type migration --status failed

# Get error details
redisctl enterprise crdb-task get <failed_task_id> -q "error"

# Retry the migration
redisctl enterprise crdb-task retry <failed_task_id>
```

### Monitor CRDB Backup

```bash
# Start monitoring backup task
CRDB_UID=1
redisctl enterprise crdb-task list-by-crdb $CRDB_UID --type backup --status running

# Get progress updates
BACKUP_TASK="backup-task-123"
while [ "$(redisctl enterprise crdb-task status $BACKUP_TASK)" = "running" ]; do
  echo "Progress: $(redisctl enterprise crdb-task progress $BACKUP_TASK -q progress_percent)%"
  sleep 10
done
```

### Cancel Long-Running Task

```bash
# Find long-running tasks
redisctl enterprise crdb-task list --status running -o table

# Cancel specific task
redisctl enterprise crdb-task cancel <task_id> --force
```

## Practical Scripts

### Task Monitoring Script

```bash
#!/bin/bash
# Monitor all CRDB tasks for a specific database

CRDB_UID=$1
if [ -z "$CRDB_UID" ]; then
  echo "Usage: $0 <crdb_uid>"
  exit 1
fi

echo "Monitoring tasks for CRDB $CRDB_UID..."

while true; do
  clear
  echo "=== CRDB $CRDB_UID Tasks ==="
  echo ""
  
  # Get running tasks
  echo "Running Tasks:"
  redisctl enterprise crdb-task list-by-crdb $CRDB_UID --status running -o table
  
  # Get failed tasks
  echo -e "\nFailed Tasks:"
  redisctl enterprise crdb-task list-by-crdb $CRDB_UID --status failed -o table
  
  # Get completed tasks (last 5)
  echo -e "\nRecent Completed Tasks:"
  redisctl enterprise crdb-task list-by-crdb $CRDB_UID --status completed -q "tasks[:5]" -o table
  
  sleep 30
done
```

### Task Health Check

```bash
#!/bin/bash
# Check health of all CRDB tasks

echo "CRDB Task Health Report"
echo "======================="

# Check for failed tasks
FAILED_COUNT=$(redisctl enterprise crdb-task list --status failed -q "tasks | length")
echo "Failed tasks: $FAILED_COUNT"

if [ "$FAILED_COUNT" -gt 0 ]; then
  echo "Failed task details:"
  redisctl enterprise crdb-task list --status failed -q "tasks[].{id: task_id, type: type, error: error_message}"
fi

# Check for stuck tasks (running > 1 hour)
echo -e "\nLong-running tasks (>1 hour):"
redisctl enterprise crdb-task list --status running -q "tasks[?duration_seconds > \`3600\`]"

# Check task distribution by type
echo -e "\nTask distribution by type:"
for type in sync migration backup restore; do
  COUNT=$(redisctl enterprise crdb-task list --type $type -q "tasks | length")
  echo "  $type: $COUNT"
done
```

### Automated Task Retry

```bash
#!/bin/bash
# Automatically retry failed tasks

# Get all failed tasks
for task_id in $(redisctl enterprise crdb-task list --status failed -q "tasks[].task_id" --raw); do
  echo "Retrying task $task_id..."
  
  # Get task type for logging
  TASK_TYPE=$(redisctl enterprise crdb-task get $task_id -q "type")
  
  # Attempt retry
  if redisctl enterprise crdb-task retry $task_id; then
    echo "Successfully initiated retry for $TASK_TYPE task $task_id"
  else
    echo "Failed to retry $TASK_TYPE task $task_id - manual intervention required"
  fi
  
  sleep 5
done
```

## Integration with CRDB Commands

CRDB task commands work alongside regular CRDB commands:

```bash
# Create a CRDB (returns task_id)
TASK_ID=$(redisctl enterprise crdb create --data @crdb.json -q "task_id")

# Monitor the creation task
redisctl enterprise crdb-task progress $TASK_ID

# Wait for completion
while [ "$(redisctl enterprise crdb-task status $TASK_ID)" = "running" ]; do
  sleep 10
done

# Check if successful
if [ "$(redisctl enterprise crdb-task status $TASK_ID)" = "completed" ]; then
  echo "CRDB created successfully"
else
  echo "CRDB creation failed"
  redisctl enterprise crdb-task get $TASK_ID -q "error"
fi
```

## Best Practices

1. **Monitor Critical Tasks** - Set up monitoring for backup and migration tasks
2. **Handle Failures Promptly** - Check failed tasks regularly and retry or escalate
3. **Track Long-Running Tasks** - Monitor tasks that run longer than expected
4. **Use Filtering** - Filter by status and type to focus on relevant tasks
5. **Automate Monitoring** - Create scripts to track task health
6. **Log Task History** - Keep records of completed and failed tasks for auditing

## Troubleshooting

### Tasks Not Listed

```bash
# Verify CRDB exists
redisctl enterprise crdb list

# Check if tasks endpoint is available
redisctl enterprise api get /crdb_tasks
```

### Cannot Cancel Task

```bash
# Check task status first
redisctl enterprise crdb-task get <task_id> -q "status"

# Only running tasks can be canceled
# Completed or failed tasks cannot be canceled
```

### Retry Not Available

Some task types or Redis Enterprise versions may not support retry:
- Check Redis Enterprise version compatibility
- Consider creating a new task instead of retrying
- Review task configuration for issues

### Progress Not Updating

```bash
# Check if task supports progress reporting
redisctl enterprise crdb-task get <task_id> -q "supports_progress"

# Some quick tasks may complete before progress is reported
```

## Related Commands

- `enterprise crdb` - CRDB management operations
- `enterprise action` - General action/task monitoring
- `enterprise database` - Regular database operations
- `api enterprise` - Direct API access for advanced operations