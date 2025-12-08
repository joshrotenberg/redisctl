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
redisctl enterprise logs list -o json | jq '.[] | select(.severity == "ERROR")'

# Get events from the last hour
redisctl enterprise logs list -o json | jq '.[] | select(.time > "2024-01-01T00:00:00")'

# Get events for a specific node
redisctl enterprise logs list -o json | jq '.[] | select(.node_uid == "1")'
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

## JSON Output Examples

```bash
# Get recent errors
redisctl enterprise logs list -o json | jq '[.[] | select(.severity == "ERROR")] | .[0:10]'

# Count events by severity
redisctl enterprise logs list -o json | jq 'group_by(.severity) | map({severity: .[0].severity, count: length})'

# Export logs for analysis
redisctl enterprise logs list -o json > cluster-logs.json
```

## Integration with External Systems

Export logs for integration with log aggregation systems:

```bash
# Export as newline-delimited JSON for ingestion
redisctl enterprise logs list -o json | jq -c '.[]'

# Pipe to a log shipper
redisctl enterprise logs list -o json | jq -c '.[]' | your-log-shipper
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
