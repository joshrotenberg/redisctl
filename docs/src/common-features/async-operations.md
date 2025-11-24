# Async Operations

Many Redis Cloud and Enterprise operations are asynchronous - they return immediately with a task ID while the work happens in the background. redisctl handles this automatically.

## The --wait Flag

Use `--wait` to block until the operation completes:

```bash
# Returns immediately with task ID
redisctl cloud database create --subscription 123 --data '{...}'

# Waits for completion, returns final result
redisctl cloud database create --subscription 123 --data '{...}' --wait
```

## Polling Options

Control how redisctl polls for completion:

```bash
redisctl cloud subscription create \
  --data '{...}' \
  --wait \
  --poll-interval 10 \    # Check every 10 seconds (default: 5)
  --max-wait 600          # Timeout after 10 minutes (default: 300)
```

## Task Management

### Check Task Status

```bash
# Cloud
redisctl cloud task get <task-id>

# Enterprise
redisctl enterprise action get <action-id>
```

### List Recent Tasks

```bash
# Cloud - list all tasks
redisctl cloud task list

# Enterprise - list actions
redisctl enterprise action list
```

## Common Async Operations

### Redis Cloud

- `subscription create/delete`
- `database create/update/delete`
- `vpc-peering create/delete`
- `cloud-account create/delete`

### Redis Enterprise

- `database create/update/delete`
- `cluster join/remove-node`
- `module upload`

## Error Handling

When `--wait` is used and an operation fails:

```bash
$ redisctl cloud database create --data '{...}' --wait
Error: Task failed: Invalid memory configuration

# Check task details
$ redisctl cloud task get abc-123
{
  "taskId": "abc-123",
  "status": "failed",
  "error": "Invalid memory configuration"
}
```

## Scripting Patterns

### Wait and Extract Result

```bash
# Create and get the new database ID
DB_ID=$(redisctl cloud database create \
  --subscription 123 \
  --data '{"name": "mydb"}' \
  --wait \
  -q 'databaseId')

echo "Created database: $DB_ID"
```

### Fire and Forget

```bash
# Start multiple operations in parallel
redisctl cloud database delete 123 456 &
redisctl cloud database delete 123 789 &
wait
```

### Custom Polling

```bash
# Start operation
TASK_ID=$(redisctl cloud database create --data '{...}' -q 'taskId')

# Custom polling loop
while true; do
  STATUS=$(redisctl cloud task get $TASK_ID -q 'status')
  echo "Status: $STATUS"
  
  if [ "$STATUS" = "completed" ] || [ "$STATUS" = "failed" ]; then
    break
  fi
  
  sleep 10
done
```

## Timeouts

If an operation exceeds `--max-wait`:

```bash
$ redisctl cloud subscription create --data '{...}' --wait --max-wait 60
Error: Operation timed out after 60 seconds. Task ID: abc-123

# Check manually
$ redisctl cloud task get abc-123
```

The operation continues in the background - only the CLI stops waiting.
