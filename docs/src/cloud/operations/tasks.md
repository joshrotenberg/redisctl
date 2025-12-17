# Task Operations

Many Redis Cloud operations (creating databases, subscriptions, etc.) are asynchronous. They return a task ID that you can use to track progress.

## Commands Overview

```bash
redisctl cloud task --help
```

## Get Task Status

```bash
# Get the current status of a task
redisctl cloud task get <task_id>

# Output as JSON
redisctl cloud task get <task_id> -o json
```

## Wait for Task Completion

Block until a task completes:

```bash
# Wait for task to finish (with default timeout)
redisctl cloud task wait <task_id>

# Wait with custom timeout (in seconds)
redisctl cloud task wait <task_id> --timeout 600
```

## Poll Task Status

Watch task progress with live updates:

```bash
# Poll with live status updates
redisctl cloud task poll <task_id>

# Poll with custom interval
redisctl cloud task poll <task_id> --interval 5
```

## Automatic Task Handling

Most create/update/delete commands support automatic task waiting with the `--wait` flag:

```bash
# Create database and wait for completion
redisctl cloud database create <subscription_id> --data '{...}' --wait

# Delete subscription and wait
redisctl cloud subscription delete <subscription_id> --wait
```

You can also poll with live updates using `--poll`:

```bash
# Create with live progress updates
redisctl cloud database create <subscription_id> --data '{...}' --poll
```

## Task States

Tasks progress through these states:

- **initialized**: Task created, not yet started
- **processing-in-progress**: Currently executing
- **processing-completed**: Finished successfully
- **processing-error**: Failed with an error

## Scripting with Tasks

```bash
# Create a database and capture the task ID
TASK_ID=$(redisctl cloud database create 12345 --data '{...}' -q 'taskId')

# Wait for it
redisctl cloud task wait $TASK_ID

# Check the result
redisctl cloud task get $TASK_ID -q 'response'
```

## Error Handling

When a task fails, you can inspect the error:

```bash
# Get task details including error info
redisctl cloud task get <task_id> -q '{status: status, description: description, response: response}'
```
