# Cluster Manager Settings

Cluster Manager (CM) settings control various cluster-wide behaviors and policies in Redis Enterprise. These settings affect how the cluster operates, manages resources, and handles various operations.

## Overview

CM settings provide configuration for:
- Resource management policies
- Operational behaviors
- System defaults
- Performance tuning
- Security policies
- Maintenance settings

**Warning**: Modifying CM settings affects the entire cluster. Changes should be made carefully and tested in non-production environments first.

## Available Commands

### Get Settings

Retrieve current cluster manager settings:

```bash
# Get all settings
redisctl enterprise cm-settings get

# Get specific setting using JMESPath
redisctl enterprise cm-settings get --setting "timezone"

# Get nested settings
redisctl enterprise cm-settings get --setting "backup_job_settings.enabled"

# Output as YAML
redisctl enterprise cm-settings get -o yaml
```

### Update Settings

Update cluster manager settings:

```bash
# Update from JSON file
redisctl enterprise cm-settings set --data @settings.json

# Update from stdin
echo '{"timezone": "America/New_York"}' | redisctl enterprise cm-settings set --data -

# Update with force (skip confirmation)
redisctl enterprise cm-settings set --data @settings.json --force
```

### Update Specific Setting

Update a single setting value:

```bash
# Update timezone
redisctl enterprise cm-settings set-value timezone --value "Europe/London"

# Update nested setting
redisctl enterprise cm-settings set-value backup_job_settings.enabled --value true

# Update with force
redisctl enterprise cm-settings set-value timezone --value "UTC" --force
```

### Reset Settings

Reset settings to cluster defaults:

```bash
# Reset all settings (with confirmation)
redisctl enterprise cm-settings reset

# Reset without confirmation
redisctl enterprise cm-settings reset --force
```

### Export/Import Settings

Export and import settings for backup or migration:

```bash
# Export to file
redisctl enterprise cm-settings export --output settings-backup.json

# Export to stdout
redisctl enterprise cm-settings export --output -

# Import from file
redisctl enterprise cm-settings import --file @settings-backup.json

# Import from stdin
cat settings.json | redisctl enterprise cm-settings import --file -
```

### Validate Settings

Validate settings file before importing:

```bash
# Validate settings file
redisctl enterprise cm-settings validate --file @settings.json

# Validate from stdin
echo '{"timezone": "UTC"}' | redisctl enterprise cm-settings validate --file -
```

### List Categories

View available setting categories:

```bash
# List all categories
redisctl enterprise cm-settings list-categories

# Output as table
redisctl enterprise cm-settings list-categories -o table
```

### Get Category Settings

Get all settings within a specific category:

```bash
# Get all backup-related settings
redisctl enterprise cm-settings get-category backup_job_settings

# Get specific field from category
redisctl enterprise cm-settings get-category backup_job_settings -q "cron_expression"
```

## Common Settings

### Time Zone Configuration

```json
{
  "timezone": "UTC"
}
```

Common timezone values:
- `UTC` - Coordinated Universal Time
- `America/New_York` - Eastern Time
- `America/Los_Angeles` - Pacific Time
- `Europe/London` - British Time
- `Asia/Tokyo` - Japan Time

### Backup Job Settings

```json
{
  "backup_job_settings": {
    "enabled": true,
    "cron_expression": "0 2 * * *",
    "retention_days": 7
  }
}
```

### Resource Management

```json
{
  "resource_management": {
    "memory_reserve_percent": 15,
    "cpu_reserve_percent": 10,
    "max_databases_per_node": 100
  }
}
```

### Security Settings

```json
{
  "security": {
    "password_complexity": "high",
    "session_timeout_minutes": 30,
    "max_login_attempts": 5,
    "audit_logging": true
  }
}
```

## Examples

### Backup Current Settings

```bash
#!/bin/bash
# Backup current settings with timestamp

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="cm_settings_backup_${TIMESTAMP}.json"

redisctl enterprise cm-settings export --output "$BACKUP_FILE"
echo "Settings backed up to: $BACKUP_FILE"
```

### Configure for Production

```bash
# Production settings template
cat << EOF > production-settings.json
{
  "timezone": "UTC",
  "backup_job_settings": {
    "enabled": true,
    "cron_expression": "0 2 * * *",
    "retention_days": 30
  },
  "security": {
    "audit_logging": true,
    "password_complexity": "high"
  },
  "resource_management": {
    "memory_reserve_percent": 20
  }
}
EOF

# Apply production settings
redisctl enterprise cm-settings import --file @production-settings.json
```

### Compare Settings Between Clusters

```bash
#!/bin/bash
# Compare settings between two clusters

# Export from cluster 1
redisctl profile use cluster1
redisctl enterprise cm-settings export --output cluster1-settings.json

# Export from cluster 2
redisctl profile use cluster2
redisctl enterprise cm-settings export --output cluster2-settings.json

# Compare
diff -u cluster1-settings.json cluster2-settings.json
```

### Audit Settings Changes

```bash
#!/bin/bash
# Track settings changes over time

AUDIT_DIR="cm_settings_audit"
mkdir -p "$AUDIT_DIR"

# Get current settings
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
redisctl enterprise cm-settings get > "$AUDIT_DIR/settings_${TIMESTAMP}.json"

# Show changes from last audit
if [ -f "$AUDIT_DIR/settings_latest.json" ]; then
  diff -u "$AUDIT_DIR/settings_latest.json" "$AUDIT_DIR/settings_${TIMESTAMP}.json"
fi

# Update latest link
ln -sf "settings_${TIMESTAMP}.json" "$AUDIT_DIR/settings_latest.json"
```

### Safe Settings Update

```bash
#!/bin/bash
# Safely update settings with validation and backup

NEW_SETTINGS="$1"
if [ -z "$NEW_SETTINGS" ]; then
  echo "Usage: $0 <settings-file>"
  exit 1
fi

# Validate new settings
echo "Validating settings..."
if ! redisctl enterprise cm-settings validate --file "@$NEW_SETTINGS"; then
  echo "Settings validation failed!"
  exit 1
fi

# Backup current settings
echo "Backing up current settings..."
redisctl enterprise cm-settings export --output settings-backup-$(date +%s).json

# Apply new settings
echo "Applying new settings..."
redisctl enterprise cm-settings import --file "@$NEW_SETTINGS"

echo "Settings updated successfully"
```

## Settings Migration

### Export from Source Cluster

```bash
# Export all settings
redisctl enterprise cm-settings export --output source-settings.json

# Review exported settings
cat source-settings.json
```

### Import to Target Cluster

```bash
# Validate before import
redisctl enterprise cm-settings validate --file @source-settings.json

# Import settings
redisctl enterprise cm-settings import --file @source-settings.json --force
```

## Best Practices

1. **Always backup before changes** - Export current settings before modifications
2. **Test in non-production** - Validate changes in test environments first
3. **Document changes** - Keep records of what was changed and why
4. **Use version control** - Store settings files in Git for tracking
5. **Validate before import** - Always validate settings files before importing
6. **Monitor after changes** - Watch cluster behavior after settings updates

## Troubleshooting

### Settings Not Applied

```bash
# Check if settings were saved
redisctl enterprise cm-settings get

# Verify specific setting
redisctl enterprise cm-settings get --setting "your.setting.path"

# Check cluster logs for errors
redisctl enterprise logs list --type error
```

### Invalid Settings Format

```bash
# Validate JSON syntax
python3 -c "import json; json.load(open('settings.json'))" && echo "Valid JSON"

# Validate against schema
redisctl enterprise cm-settings validate --file @settings.json
```

### Reset to Defaults

If settings cause issues:

```bash
# Reset all settings to defaults
redisctl enterprise cm-settings reset --force

# Restart cluster services if needed
redisctl enterprise cluster restart-services
```

### Permission Denied

CM settings require admin privileges:

```bash
# Check user permissions
redisctl enterprise user whoami

# Ensure admin role
redisctl enterprise user get <user_id> -q "role"
```

## Related Commands

- `enterprise cluster` - Cluster configuration and management
- [`enterprise job-scheduler`](./job-scheduler.md) - Job scheduling configuration
- `enterprise diagnostics` - Cluster diagnostics
- `api enterprise` - Direct API access for advanced operations