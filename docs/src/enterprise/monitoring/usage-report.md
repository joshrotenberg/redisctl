# Usage Report

The usage report commands provide access to Redis Enterprise cluster usage data, important for license compliance, capacity planning, and resource utilization analysis.

## Available Commands

### Get Usage Report

Retrieve the current usage report for the cluster:

```bash
# Get full usage report
redisctl enterprise usage-report get

# Get usage report as YAML
redisctl enterprise usage-report get -o yaml

# Extract specific metrics
redisctl enterprise usage-report get -q '{cluster: cluster_name, databases: total_databases, memory_gb: total_memory_gb}'

# Get database-specific usage
redisctl enterprise usage-report get -q 'databases[].{name: name, memory: memory_mb, shards: shard_count}'
```

### Export Usage Report

Export usage report to a file for reporting or analysis:

```bash
# Export to JSON file
redisctl enterprise usage-report export --output usage-report.json

# Export to CSV for spreadsheet analysis
redisctl enterprise usage-report export --output usage-report.csv --format csv

# Export with specific date in filename
redisctl enterprise usage-report export -o "usage-$(date +%Y%m%d).json"

# Export filtered data
redisctl enterprise usage-report export -o databases.json -q 'databases'
```

## Output Examples

### Usage Report Structure
```json
{
  "cluster_name": "production-cluster",
  "cluster_uid": "cluster-12345",
  "report_date": "2024-03-15T10:00:00Z",
  "license": {
    "type": "enterprise",
    "expiry": "2025-01-01T00:00:00Z",
    "shards_limit": 100,
    "memory_limit_gb": 1024
  },
  "usage": {
    "total_databases": 25,
    "total_shards": 75,
    "total_memory_gb": 512,
    "total_nodes": 5,
    "total_cpus": 40
  },
  "databases": [
    {
      "uid": 1,
      "name": "cache-db",
      "memory_mb": 8192,
      "shard_count": 4,
      "replication": true,
      "persistence": "aof",
      "modules": ["search", "json"]
    },
    {
      "uid": 2,
      "name": "session-store",
      "memory_mb": 4096,
      "shard_count": 2,
      "replication": false,
      "persistence": "none",
      "modules": []
    }
  ],
  "nodes": [
    {
      "uid": 1,
      "address": "node1.cluster.local",
      "cpus": 8,
      "memory_gb": 128,
      "databases": 5,
      "shards": 15
    }
  ]
}
```

## Common Use Cases

### License Compliance

Monitor usage against license limits:

```bash
# Check current usage vs limits
redisctl enterprise usage-report get -q '{
  shards_used: usage.total_shards,
  shards_limit: license.shards_limit,
  shards_available: license.shards_limit - usage.total_shards,
  memory_used_gb: usage.total_memory_gb,
  memory_limit_gb: license.memory_limit_gb,
  memory_available_gb: license.memory_limit_gb - usage.total_memory_gb
}'

# Check license expiry
redisctl enterprise usage-report get -q 'license.expiry'

# Alert if approaching limits
usage=$(redisctl enterprise usage-report get -q '{
  shard_pct: (usage.total_shards / license.shards_limit * 100),
  memory_pct: (usage.total_memory_gb / license.memory_limit_gb * 100)
}')
```

### Capacity Planning

Analyze resource utilization for capacity planning:

```bash
# Get growth metrics
redisctl enterprise usage-report export -o usage-$(date +%Y%m).json

# Database memory distribution
redisctl enterprise usage-report get -q 'databases | sort_by(@, &memory_mb) | reverse(@)[:10]' -o table

# Shards per database
redisctl enterprise usage-report get -q 'databases[].{name: name, shards: shard_count}' -o table

# Node utilization
redisctl enterprise usage-report get -q 'nodes[].{node: address, memory_gb: memory_gb, databases: databases, shards: shards}' -o table
```

### Module Usage Analysis

Track module adoption and usage:

```bash
# List databases with modules
redisctl enterprise usage-report get -q 'databases[?length(modules) > `0`].{name: name, modules: modules}'

# Count module usage
redisctl enterprise usage-report get -q 'databases[].modules[] | group_by(@) | [].{module: [0], count: length(@)}'

# Find databases with specific module
redisctl enterprise usage-report get -q 'databases[?contains(modules, `search`)].name'
```

### Regular Reporting

Create automated usage reports:

```bash
#!/bin/bash
# Monthly usage report script

REPORT_DIR="/var/reports/redis"
DATE=$(date +%Y%m%d)
MONTH=$(date +%B-%Y)

# Create report directory
mkdir -p "$REPORT_DIR"

# Export full report
redisctl enterprise usage-report export -o "$REPORT_DIR/usage-$DATE.json"

# Create summary CSV
redisctl enterprise usage-report get -q '{
  date: report_date,
  databases: usage.total_databases,
  shards: usage.total_shards,
  memory_gb: usage.total_memory_gb,
  nodes: usage.total_nodes
}' | jq -r '[.date, .databases, .shards, .memory_gb, .nodes] | @csv' >> "$REPORT_DIR/usage-summary.csv"

# Email report
echo "Redis Enterprise Usage Report for $MONTH" | \
  mail -s "Redis Usage Report - $MONTH" \
  -a "$REPORT_DIR/usage-$DATE.json" \
  ops-team@company.com
```

### Chargeback/Showback

Generate department or team usage reports:

```bash
# Assuming database names include team identifiers
# e.g., "team-a-cache", "team-b-sessions"

# Group databases by team
for team in team-a team-b team-c; do
  echo "Usage for $team:"
  redisctl enterprise usage-report get \
    -q "databases[?contains(name, '$team')].{name: name, memory_mb: memory_mb, shards: shard_count}" \
    -o table
done

# Calculate team memory usage using JMESPath extensions
# group_by and sum are available as extended functions
redisctl enterprise usage-report get -q '
  group_by(databases[].{team: split(name, `-`)[0], memory_mb: memory_mb}, `"team"`)
  | items(@)
  | [*].{
      team: [0],
      total_memory_mb: sum([1][].memory_mb),
      database_count: length([1])
    }'
```

## Export Formats

### JSON Export

Full structured data for programmatic processing:

```bash
# Export and process with jq
redisctl enterprise usage-report export -o report.json
cat report.json | jq '.databases | length'

# Export and upload to S3
redisctl enterprise usage-report export -o /tmp/usage.json
aws s3 cp /tmp/usage.json s3://bucket/redis-reports/$(date +%Y/%m)/usage.json
```

### CSV Export

Tabular format for spreadsheet analysis:

```bash
# Export to CSV
redisctl enterprise usage-report export -o report.csv -f csv

# Export specific data as CSV
redisctl enterprise usage-report get -q 'databases' | \
  jq -r '["name","memory_mb","shards"], (.[] | [.name, .memory_mb, .shard_count]) | @csv' > databases.csv

# Import to Google Sheets
redisctl enterprise usage-report export -o /tmp/usage.csv -f csv
gcloud auth login
gdrive upload /tmp/usage.csv
```

## Integration Examples

### Monitoring Systems

Send usage metrics to monitoring systems:

```bash
# Prometheus metrics format using JMESPath sprintf()
redisctl enterprise usage-report get -q '
  join(`"\n"`, [
    sprintf(`"redis_cluster_databases %d"`, usage.total_databases),
    sprintf(`"redis_cluster_shards %d"`, usage.total_shards),
    sprintf(`"redis_cluster_memory_gb %.2f"`, usage.total_memory_gb),
    sprintf(`"redis_cluster_nodes %d"`, usage.total_nodes),
    sprintf(`"redis_license_shards_limit %d"`, license.shards_limit),
    sprintf(`"redis_license_memory_limit_gb %.2f"`, license.memory_limit_gb)
  ])' --raw | curl -X POST http://pushgateway:9091/metrics/job/redis-usage --data-binary @-

# Datadog metrics
redisctl enterprise usage-report get -o json | \
  python -c "
import json, sys
from datadog import initialize, api
data = json.load(sys.stdin)
api.Metric.send([
    {'metric': 'redis.usage.databases', 'points': data['usage']['total_databases']},
    {'metric': 'redis.usage.shards', 'points': data['usage']['total_shards']},
    {'metric': 'redis.usage.memory_gb', 'points': data['usage']['total_memory_gb']}
])
"
```

### Ticketing Systems

Create tickets for capacity warnings:

```bash
#!/bin/bash
# Check usage and create tickets

# Use JMESPath divide() and multiply() for percentage calculations
SHARD_PCT=$(redisctl enterprise usage-report get \
  -q 'multiply(divide(usage.total_shards, license.shards_limit), `100`)' --raw)
MEMORY_PCT=$(redisctl enterprise usage-report get \
  -q 'multiply(divide(usage.total_memory_gb, license.memory_limit_gb), `100`)' --raw)

if (( $(echo "$SHARD_PCT > 80" | bc -l) )); then
  echo "High shard usage: ${SHARD_PCT}%" | \
    gh issue create --title "Redis Cluster: High Shard Usage Alert" \
    --body "Shard usage is at ${SHARD_PCT}% of licensed capacity"
fi

if (( $(echo "$MEMORY_PCT > 80" | bc -l) )); then
  echo "High memory usage: ${MEMORY_PCT}%" | \
    jira create --project OPS --type Alert \
    --summary "Redis Cluster: High Memory Usage" \
    --description "Memory usage is at ${MEMORY_PCT}% of licensed capacity"
fi
```

## Best Practices

1. **Regular Exports**: Schedule regular exports for historical tracking
2. **Automated Monitoring**: Set up automated checks for license limits
3. **Trend Analysis**: Compare reports over time to identify growth patterns
4. **Capacity Alerts**: Configure alerts when approaching license limits
5. **Cost Attribution**: Use naming conventions to enable chargeback/showback
6. **Archive Reports**: Keep historical reports for compliance and auditing

## Troubleshooting

### Report Generation Issues

If usage reports fail to generate:

```bash
# Check cluster status
redisctl enterprise cluster get -q 'name'

# Verify authentication
redisctl enterprise auth test

# Check with raw API
redisctl api enterprise get /v1/usage_report
```

### Export Failures

When exports fail:

```bash
# Check write permissions
touch test-file.json && rm test-file.json

# Verify disk space
df -h .

# Try different format
redisctl enterprise usage-report export -o report.json
redisctl enterprise usage-report export -o report.csv -f csv
```

## Related Commands

- `redisctl enterprise cluster` - View cluster information
- `redisctl enterprise database list` - List all databases
- `redisctl enterprise stats` - View detailed statistics
- `redisctl enterprise node list` - View node resources