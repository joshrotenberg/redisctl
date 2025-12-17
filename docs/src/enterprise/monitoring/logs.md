# Log Operations

View and manage cluster event logs in Redis Enterprise.

## Commands Overview

```bash
redisctl enterprise logs --help
```

## List Event Logs

```bash
# List cluster event logs
redisctl enterprise logs list

# Or use the alias
redisctl enterprise logs ls

# Output as JSON
redisctl enterprise logs list -o json
```

## Filtering Logs

Use JMESPath queries to filter log output:

```bash
# Get only error-level events
redisctl enterprise logs list -q "[?severity=='ERROR']"

# Get events from the last hour
redisctl enterprise logs list -q "[?time > '2024-01-01T00:00:00']"

# Get events for a specific node
redisctl enterprise logs list -q "[?node_uid=='1']"
```

## Log Entry Fields

Each log entry typically includes:

| Field | Description |
|-------|-------------|
| `time` | Timestamp of the event |
| `severity` | Log level (INFO, WARNING, ERROR, etc.) |
| `type` | Event type |
| `node_uid` | Node where the event occurred |
| `message` | Human-readable description |

## JMESPath Query Examples

```bash
# Get recent errors (first 10)
redisctl enterprise logs list -q "[?severity=='ERROR'] | [0:10]"

# Get errors with specific fields
redisctl enterprise logs list -q "[?severity=='ERROR'].{time: time, message: message}" -o table

# Export logs for analysis
redisctl enterprise logs list > cluster-logs.json
```

## Integration with External Systems

Export logs for integration with log aggregation systems:

```bash
# Export as JSON for ingestion
redisctl enterprise logs list > logs.json

# Get logs and pipe to a log shipper
redisctl enterprise logs list | your-log-shipper
```

## Related Commands

For streaming logs with real-time updates, see the logs streaming feature:

```bash
# Stream logs with --follow flag (if available)
redisctl enterprise logs list --follow
```

For database-specific logs:

```bash
# Get slow query log for a database
redisctl enterprise database slowlog <db_id>
```
