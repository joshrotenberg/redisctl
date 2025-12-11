# Job Scheduler

The job scheduler commands allow you to manage and configure scheduled background jobs in Redis Enterprise. These jobs handle critical maintenance tasks like backups, log rotation, certificate renewal, and health checks.

## Overview

Redis Enterprise runs several scheduled jobs automatically to maintain cluster health and perform routine maintenance. The job scheduler commands let you view and customize the schedule and configuration of these jobs.

## Available Commands

### Get Configuration

Retrieve the current job scheduler configuration:

```bash
# Get all job scheduler settings
redisctl enterprise job-scheduler get

# Get specific job configuration using JMESPath
redisctl enterprise job-scheduler get -q "backup_job_settings"

# Output as table
redisctl enterprise job-scheduler get -o table
```

### Update Configuration

Modify job scheduler settings:

```bash
# Update from JSON file
redisctl enterprise job-scheduler update --data @scheduler-config.json

# Update from stdin
echo '{"backup_job_settings": {"cron_expression": "*/10 * * * *"}}' | \
  redisctl enterprise job-scheduler update --data -

# Update inline
redisctl enterprise job-scheduler update --data '{
  "log_rotation_job_settings": {
    "cron_expression": "0 */6 * * *",
    "enabled": true
  }
}'
```

## Scheduled Job Types

### Backup Job
Manages automatic database backups:

```json
{
  "backup_job_settings": {
    "cron_expression": "*/5 * * * *",
    "enabled": true
  }
}
```

### Database Usage Report
Generates usage statistics for databases:

```json
{
  "bdb_usage_report_job_settings": {
    "cron_expression": "0 */1 * * *",
    "enabled": true,
    "file_retention_days": 365
  }
}
```

### Certificate Rotation
Handles automatic certificate renewal:

```json
{
  "cert_rotation_job_settings": {
    "cron_expression": "0 * * * *",
    "enabled": true,
    "expiry_days_before_rotation": 60
  }
}
```

### Log Rotation
Manages log file rotation and cleanup:

```json
{
  "log_rotation_job_settings": {
    "cron_expression": "*/5 * * * *",
    "enabled": true
  }
}
```

### Node Health Checks
Performs periodic node health validation:

```json
{
  "node_checks_job_settings": {
    "cron_expression": "0 * * * *",
    "enabled": true
  }
}
```

### Redis Cleanup
Cleans up temporary Redis data:

```json
{
  "redis_cleanup_job_settings": {
    "cron_expression": "0 * * * *"
  }
}
```

### CCS Log Rotation
Rotates cluster configuration service logs:

```json
{
  "rotate_ccs_job_settings": {
    "cron_expression": "*/5 * * * *",
    "enabled": true,
    "file_suffix": "5min",
    "rotate_max_num": 24
  }
}
```

## Cron Expression Format

Job schedules use standard cron expression format:

```
┌───────────── minute (0 - 59)
│ ┌───────────── hour (0 - 23)
│ │ ┌───────────── day of month (1 - 31)
│ │ │ ┌───────────── month (1 - 12)
│ │ │ │ ┌───────────── day of week (0 - 6) (Sunday to Saturday)
│ │ │ │ │
│ │ │ │ │
* * * * *
```

### Common Patterns

- `*/5 * * * *` - Every 5 minutes
- `0 * * * *` - Every hour
- `0 0 * * *` - Daily at midnight
- `0 2 * * 0` - Weekly on Sunday at 2 AM
- `0 0 1 * *` - Monthly on the 1st at midnight

## Examples

### Adjust Backup Frequency

Change backups from every 5 minutes to every 30 minutes:

```bash
redisctl enterprise job-scheduler update --data '{
  "backup_job_settings": {
    "cron_expression": "*/30 * * * *"
  }
}'
```

### Configure Aggressive Log Rotation

Rotate logs every hour and keep fewer files:

```bash
redisctl enterprise job-scheduler update --data '{
  "log_rotation_job_settings": {
    "cron_expression": "0 * * * *",
    "enabled": true
  },
  "rotate_ccs_job_settings": {
    "cron_expression": "0 * * * *",
    "file_suffix": "hourly",
    "rotate_max_num": 12
  }
}'
```

### Extend Certificate Renewal Window

Check certificates 90 days before expiry:

```bash
redisctl enterprise job-scheduler update --data '{
  "cert_rotation_job_settings": {
    "expiry_days_before_rotation": 90
  }
}'
```

### Reduce Database Report Retention

Keep usage reports for only 30 days:

```bash
redisctl enterprise job-scheduler update --data '{
  "bdb_usage_report_job_settings": {
    "file_retention_days": 30
  }
}'
```

## Configuration Templates

### Production Environment

High-frequency backups with extended retention:

```json
{
  "backup_job_settings": {
    "cron_expression": "*/15 * * * *",
    "enabled": true
  },
  "bdb_usage_report_job_settings": {
    "cron_expression": "0 0 * * *",
    "enabled": true,
    "file_retention_days": 730
  },
  "cert_rotation_job_settings": {
    "cron_expression": "0 0 * * *",
    "enabled": true,
    "expiry_days_before_rotation": 90
  },
  "log_rotation_job_settings": {
    "cron_expression": "0 */4 * * *",
    "enabled": true
  }
}
```

### Development Environment

Less frequent operations to reduce overhead:

```json
{
  "backup_job_settings": {
    "cron_expression": "0 */6 * * *",
    "enabled": true
  },
  "bdb_usage_report_job_settings": {
    "cron_expression": "0 0 * * 0",
    "enabled": true,
    "file_retention_days": 7
  },
  "node_checks_job_settings": {
    "cron_expression": "0 */12 * * *",
    "enabled": true
  }
}
```

## Monitoring Job Execution

Jobs create actions that can be monitored:

```bash
# Check recent backup jobs
redisctl enterprise action list --type backup_job

# Monitor job execution
watch -n 60 'redisctl enterprise action list --status running -o table'
```

## Best Practices

1. **Balance Frequency vs Load** - More frequent jobs provide better protection but increase system load
2. **Align with Maintenance Windows** - Schedule intensive jobs during low-traffic periods
3. **Monitor Job Success** - Regularly check that scheduled jobs complete successfully
4. **Test Configuration Changes** - Verify new schedules work as expected before production deployment
5. **Document Custom Schedules** - Keep notes on why default schedules were modified

## Limitations

- Some jobs cannot be disabled (marked as internal scheduled jobs)
- Cron expressions must be valid or the update will fail
- Changes take effect at the next scheduled run
- Job execution history is available through the actions API

## Troubleshooting

### Jobs Not Running

```bash
# Check if job is enabled
redisctl enterprise job-scheduler get -q "backup_job_settings.enabled"

# Verify cron expression
redisctl enterprise job-scheduler get -q "backup_job_settings.cron_expression"
```

### Failed Job Updates

```bash
# Check current configuration
redisctl enterprise job-scheduler get

# Validate JSON before updating
echo '{"backup_job_settings": {"enabled": true}}' | jq .

# Try update with valid configuration
redisctl enterprise job-scheduler update --data '{"backup_job_settings": {"enabled": true}}'
```

## Related Commands

- `enterprise action` - Monitor job execution status
- `enterprise cluster` - Cluster configuration that affects jobs
- `enterprise database` - Database backup operations
- `enterprise logs` - View logs generated by scheduled jobs