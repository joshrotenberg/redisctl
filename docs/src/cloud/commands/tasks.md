# Cloud Tasks

Monitor async operations in Redis Cloud.

## Overview

Most Redis Cloud operations (create, update, delete) are asynchronous. They return a task ID immediately while the work happens in the background. Use these commands to monitor task progress.

## Commands

### List Tasks

```bash
redisctl cloud task list
```

### Get Task

```bash
redisctl cloud task get <task-id>
```

**Example Output:**

```json
{
  "taskId": "abc-123-def",
  "commandType": "createDatabase",
  "status": "processing-completed",
  "description": "Create database",
  "timestamp": "2024-01-15T10:30:00Z",
  "response": {
    "resourceId": 789
  }
}
```

## Task States

| Status | Description |
|--------|-------------|
| `received` | Task received, queued for processing |
| `processing-in-progress` | Task is currently executing |
| `processing-completed` | Task completed successfully |
| `processing-error` | Task failed with error |

## Examples

### Check Task Status

```bash
# Get task details
redisctl cloud task get abc-123-def

# Get just the status
redisctl cloud task get abc-123-def -q 'status'

# Get error message if failed
redisctl cloud task get abc-123-def -q 'response.error.description'
```

### Wait for Task Completion

```bash
# Using --wait flag (recommended)
redisctl cloud database create --subscription-id 123 --data '{...}' --wait

# Manual polling
TASK_ID=$(redisctl cloud database create --subscription-id 123 --data '{...}' -q 'taskId')

while true; do
  STATUS=$(redisctl cloud task get $TASK_ID -q 'status')
  echo "Status: $STATUS"
  
  case $STATUS in
    "processing-completed") echo "Success!"; break ;;
    "processing-error") echo "Failed!"; exit 1 ;;
    *) sleep 10 ;;
  esac
done
```

### List Recent Tasks

```bash
# All recent tasks
redisctl cloud task list -o table

# Filter by type
redisctl cloud task list -q "[?commandType=='createDatabase']"

# Failed tasks only
redisctl cloud task list -q "[?status=='processing-error']"
```

### Get Resource ID from Completed Task

```bash
# After task completes, get the created resource ID
redisctl cloud task get abc-123-def -q 'response.resourceId'
```

## Common Task Types

| Command Type | Description |
|-------------|-------------|
| `createSubscription` | Create subscription |
| `deleteSubscription` | Delete subscription |
| `createDatabase` | Create database |
| `updateDatabase` | Update database |
| `deleteDatabase` | Delete database |
| `createVpcPeering` | Create VPC peering |

## Troubleshooting

### Task Not Found

Tasks are retained for a limited time. If you get a 404:
- The task may have expired
- Check the task ID is correct

### Task Stuck in Processing

If a task stays in `processing-in-progress` for too long:
- Check Redis Cloud status page for outages
- Contact support if it exceeds expected time

### Understanding Errors

```bash
# Get full error details
redisctl cloud task get abc-123-def -q 'response.error'
```

Common errors:
- `INVALID_REQUEST` - Bad input data
- `RESOURCE_LIMIT_EXCEEDED` - Quota exceeded
- `RESOURCE_NOT_FOUND` - Referenced resource doesn't exist

## API Reference

These commands use the following REST endpoints:
- `GET /v1/tasks` - List all tasks
- `GET /v1/tasks/{taskId}` - Get specific task

For direct API access: `redisctl api cloud get /tasks`
