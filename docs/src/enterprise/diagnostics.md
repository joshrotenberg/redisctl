# Diagnostics

The diagnostics commands provide tools for monitoring and troubleshooting Redis Enterprise cluster health, running diagnostic checks, and generating diagnostic reports.

## Overview

Redis Enterprise includes a built-in diagnostics system that performs various health checks on the cluster, nodes, and databases. These checks help identify potential issues before they become critical problems.

## Available Commands

### Get Diagnostics Configuration

Retrieve the current diagnostics configuration:

```bash
# Get full diagnostics config
redisctl enterprise diagnostics get

# Get specific configuration fields
redisctl enterprise diagnostics get -q "enabled_checks"
```

### Update Diagnostics Configuration

Modify diagnostics settings:

```bash
# Update from JSON file
redisctl enterprise diagnostics update --data @diagnostics-config.json

# Update from stdin
echo '{"check_interval": 300}' | redisctl enterprise diagnostics update --data -

# Disable specific checks
redisctl enterprise diagnostics update --data '{"disabled_checks": ["memory_check", "disk_check"]}'
```

### Run Diagnostics Checks

Trigger diagnostic checks manually:

```bash
# Run all diagnostics
redisctl enterprise diagnostics run

# Run with specific parameters
redisctl enterprise diagnostics run --data '{"checks": ["connectivity", "resources"]}'
```

### List Available Checks

View all available diagnostic checks:

```bash
# List all checks
redisctl enterprise diagnostics list-checks

# Output as table
redisctl enterprise diagnostics list-checks -o table
```

### Get Latest Report

Retrieve the most recent diagnostics report:

```bash
# Get latest report
redisctl enterprise diagnostics last-report

# Get specific sections
redisctl enterprise diagnostics last-report -q "cluster_health"
```

### Get Specific Report

Retrieve a diagnostics report by ID:

```bash
# Get report by ID
redisctl enterprise diagnostics get-report <report_id>

# Get report summary only
redisctl enterprise diagnostics get-report <report_id> -q "summary"
```

### List All Reports

View all available diagnostics reports:

```bash
# List all reports
redisctl enterprise diagnostics list-reports

# List recent reports only
redisctl enterprise diagnostics list-reports --data '{"limit": 10}'

# Filter by date range
redisctl enterprise diagnostics list-reports --data '{"start_date": "2024-01-01", "end_date": "2024-01-31"}'
```

## Diagnostic Check Types

Common diagnostic checks include:

- **Resource Checks**
  - Memory utilization
  - CPU usage
  - Disk space
  - Network bandwidth

- **Cluster Health**
  - Node connectivity
  - Replication status
  - Shard distribution
  - Quorum status

- **Database Health**
  - Endpoint availability
  - Persistence status
  - Backup status
  - Module functionality

- **Security Checks**
  - Certificate expiration
  - Authentication status
  - Encryption settings
  - ACL configuration

## Configuration Examples

### Enable Automatic Diagnostics

```json
{
  "enabled": true,
  "auto_run": true,
  "check_interval": 3600,
  "retention_days": 30,
  "email_alerts": true,
  "alert_recipients": ["ops@example.com"]
}
```

### Configure Check Thresholds

```json
{
  "thresholds": {
    "memory_usage_percent": 80,
    "disk_usage_percent": 85,
    "cpu_usage_percent": 75,
    "certificate_expiry_days": 30
  }
}
```

### Disable Specific Checks

```json
{
  "disabled_checks": [
    "backup_validation",
    "module_check"
  ],
  "check_timeout": 30
}
```

## Practical Examples

### Daily Health Check Script

```bash
#!/bin/bash
# Run daily diagnostics and email report

# Run diagnostics
redisctl enterprise diagnostics run

# Get latest report
REPORT=$(redisctl enterprise diagnostics last-report)

# Check for critical issues
CRITICAL=$(echo "$REPORT" | jq '.issues | map(select(.severity == "critical")) | length')

if [ "$CRITICAL" -gt 0 ]; then
  # Send alert for critical issues
  echo "$REPORT" | mail -s "Redis Enterprise: Critical Issues Found" ops@example.com
fi
```

### Monitor Cluster Health

```bash
# Continuous health monitoring
watch -n 60 'redisctl enterprise diagnostics last-report -q "summary" -o table'
```

### Generate Monthly Report

```bash
# Get all reports for the month
redisctl enterprise diagnostics list-reports \
  --data '{"start_date": "2024-01-01", "end_date": "2024-01-31"}' \
  -o json > monthly-diagnostics.json

# Extract key metrics
jq '.[] | {date: .timestamp, health_score: .summary.health_score}' monthly-diagnostics.json
```

### Pre-Maintenance Check

```bash
# Run comprehensive diagnostics before maintenance
redisctl enterprise diagnostics run --data '{
  "comprehensive": true,
  "include_logs": true,
  "validate_backups": true
}'

# Wait for completion and check results
sleep 30
redisctl enterprise diagnostics last-report -q "ready_for_maintenance"
```

## Report Structure

Diagnostics reports typically include:

```json
{
  "report_id": "diag-12345",
  "timestamp": "2024-01-15T10:30:00Z",
  "cluster_id": "cluster-1",
  "summary": {
    "health_score": 95,
    "total_checks": 50,
    "passed": 48,
    "warnings": 1,
    "failures": 1
  },
  "cluster_health": {
    "nodes": [...],
    "databases": [...],
    "replication": {...}
  },
  "resource_usage": {
    "memory": {...},
    "cpu": {...},
    "disk": {...}
  },
  "issues": [
    {
      "severity": "warning",
      "component": "node-2",
      "message": "Disk usage at 82%",
      "recommendation": "Consider adding storage"
    }
  ],
  "recommendations": [...]
}
```

## Best Practices

1. **Schedule Regular Checks** - Run diagnostics daily or weekly
2. **Monitor Trends** - Track health scores over time
3. **Set Up Alerts** - Configure email alerts for critical issues
4. **Archive Reports** - Keep historical reports for trend analysis
5. **Pre-Maintenance Checks** - Always run diagnostics before maintenance
6. **Custom Thresholds** - Adjust thresholds based on your environment

## Integration with Monitoring

The diagnostics system can be integrated with external monitoring tools:

```bash
# Export to Prometheus format
redisctl enterprise diagnostics last-report -q "metrics" | \
  prometheus-push-gateway

# Send to logging system
redisctl enterprise diagnostics last-report | \
  logger -t redis-diagnostics

# Create JIRA ticket for issues
ISSUES=$(redisctl enterprise diagnostics last-report -q "issues")
if [ -n "$ISSUES" ]; then
  create-jira-ticket --project OPS --summary "Redis Diagnostics Issues" --description "$ISSUES"
fi
```

## Troubleshooting

### Diagnostics Not Running

```bash
# Check if diagnostics are enabled
redisctl enterprise diagnostics get -q "enabled"

# Enable diagnostics
redisctl enterprise diagnostics update --data '{"enabled": true}'
```

### Reports Not Generated

```bash
# Check last run time
redisctl enterprise diagnostics get -q "last_run"

# Trigger manual run
redisctl enterprise diagnostics run
```

### Missing Checks

```bash
# List disabled checks
redisctl enterprise diagnostics get -q "disabled_checks"

# Re-enable all checks
redisctl enterprise diagnostics update --data '{"disabled_checks": []}'
```

## Related Commands

- [`enterprise cluster`](./cluster.md) - Cluster management and health
- [`enterprise stats`](./stats.md) - Performance statistics
- [`enterprise logs`](./logs.md) - System logs and events
- [`enterprise action`](./actions.md) - Monitor diagnostic task progress