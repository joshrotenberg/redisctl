# Enterprise Monitoring

Statistics, logs, and alerts for Redis Enterprise clusters.

## Statistics

### Cluster Stats

```bash
redisctl enterprise stats cluster [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--follow` / `-f` | Stream stats continuously |
| `--poll-interval <SECS>` | Polling interval (default: 5) |

**Examples:**

```bash
# Current cluster stats
redisctl enterprise stats cluster

# Stream continuously
redisctl enterprise stats cluster --follow

# Stream every 2 seconds
redisctl enterprise stats cluster --follow --poll-interval 2

# Get specific metrics
redisctl enterprise stats cluster -q "{cpu:cpu_user,memory:free_memory}"
```

### Database Stats

```bash
redisctl enterprise stats database <ID> [OPTIONS]
```

**Examples:**

```bash
# Database stats
redisctl enterprise stats database 1

# Stream database stats
redisctl enterprise stats database 1 --follow

# Get ops/sec and memory
redisctl enterprise stats database 1 -q "{ops:total_req,memory:used_memory}"
```

### Node Stats

```bash
redisctl enterprise stats node <ID> [OPTIONS]
```

**Examples:**

```bash
# Node stats
redisctl enterprise stats node 1

# Stream node stats
redisctl enterprise stats node 1 --follow
```

## Logs

### List Logs

```bash
redisctl enterprise logs list [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--limit <N>` | Number of entries |
| `--offset <N>` | Skip entries |
| `--order <ASC/DESC>` | Sort order |
| `--since <TIME>` | Start time |
| `--until <TIME>` | End time |

**Examples:**

```bash
# Recent logs
redisctl enterprise logs list --limit 100

# Logs from specific time
redisctl enterprise logs list --since "2024-01-01T00:00:00Z"

# Filter by severity
redisctl enterprise logs list -q "[?severity=='ERROR']"
```

## Alerts

### List Alerts

```bash
redisctl enterprise alerts list [OPTIONS]
```

**Examples:**

```bash
# All active alerts
redisctl enterprise alerts list

# Filter by state
redisctl enterprise alerts list -q "[?state=='active']"

# Get alert summary
redisctl enterprise alerts list -q "[].{type:type,state:state,severity:severity}" -o table
```

### Get Alert

```bash
redisctl enterprise alerts get <ID>
```

### Clear Alert

```bash
redisctl enterprise alerts clear <ID>
```

### Alert Settings

```bash
# Get alert settings
redisctl enterprise alerts get-settings

# Update alert settings
redisctl enterprise alerts update-settings --data '{
  "cluster_certs_about_to_expire": {
    "enabled": true,
    "threshold": 30
  }
}'
```

## Common Alert Types

| Alert | Description |
|-------|-------------|
| `node_failed` | Node unreachable |
| `node_memory` | Node memory threshold |
| `bdb_size` | Database size threshold |
| `cluster_certs_about_to_expire` | Certificate expiration |
| `license_about_to_expire` | License expiration |

## Usage Reports

### Generate Usage Report

```bash
redisctl enterprise usage-report generate [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--start <DATE>` | Report start date |
| `--end <DATE>` | Report end date |

**Examples:**

```bash
# Generate monthly report
redisctl enterprise usage-report generate \
  --start 2024-01-01 \
  --end 2024-01-31
```

## Common Patterns

### Health Check Dashboard

```bash
#!/bin/bash
echo "=== Cluster Health ==="

# Cluster stats
echo "Cluster:"
redisctl enterprise stats cluster -q "{cpu:cpu_user,memory:free_memory}" | jq

# Node status
echo "Nodes:"
redisctl enterprise node list -q "[].{id:uid,status:status}" -o table

# Active alerts
echo "Alerts:"
redisctl enterprise alerts list -q "[?state=='active'].{type:type,severity:severity}" -o table
```

### Monitor Database Performance

```bash
# Watch database ops/sec
watch -n 5 "redisctl enterprise stats database 1 -q '{ops:total_req,latency:avg_latency}'"
```

### Export Metrics for Grafana

```bash
# Export to JSON for external monitoring
while true; do
  redisctl enterprise stats cluster > /var/metrics/cluster-$(date +%s).json
  sleep 60
done
```

### Alert on Thresholds

```bash
#!/bin/bash
MEMORY=$(redisctl enterprise stats cluster -q 'used_memory')
THRESHOLD=80000000000  # 80GB

if [ "$MEMORY" -gt "$THRESHOLD" ]; then
  echo "ALERT: Memory usage high: $MEMORY bytes"
  # Send notification
fi
```

## Troubleshooting

### Stats Not Updating
- Check cluster connectivity
- Verify stats collection is enabled
- Check node health

### Missing Logs
- Adjust time range with `--since`/`--until`
- Increase `--limit`
- Check log retention settings

### Alert Not Clearing
- Resolve underlying issue first
- Use `redisctl enterprise alerts clear`
- Check alert isn't recurring

## API Reference

REST endpoints:
- `GET /v1/cluster/stats/last` - Cluster stats
- `GET /v1/bdbs/{id}/stats/last` - Database stats
- `GET /v1/nodes/{id}/stats/last` - Node stats
- `GET /v1/logs` - Logs
- `GET /v1/cluster/alerts` - Alerts

For direct API access: `redisctl api enterprise get /v1/cluster/stats/last`
