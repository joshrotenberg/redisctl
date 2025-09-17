# License Management Commands

Manage Redis Enterprise licenses with comprehensive tools for compliance monitoring, multi-instance management, and automated workflows.

## Overview

The license commands provide powerful capabilities for managing Redis Enterprise licenses:
- View and update license information
- Monitor expiration across multiple instances
- Generate compliance reports
- Bulk license updates across deployments
- Automated monitoring and alerting

## Core License Commands

### Get License Information

```bash
# Get full license details
redisctl enterprise license get

# Get specific fields with JMESPath
redisctl enterprise license get -q 'expiration_date'
redisctl enterprise license get -q '{name: cluster_name, expires: expiration_date}'
```

### Update License

```bash
# Update with JSON data
redisctl enterprise license update --data '{
  "license": "YOUR_LICENSE_KEY_HERE"
}'

# Update from file
redisctl enterprise license update --data @new-license.json

# Update from stdin
echo '{"license": "..."}' | redisctl enterprise license update --data -
```

### Upload License File

```bash
# Upload a license file directly
redisctl enterprise license upload --file /path/to/license.txt

# Supports both raw license text and JSON format
redisctl enterprise license upload --file license.json
```

### Validate License

```bash
# Validate license before applying
redisctl enterprise license validate --data @license.json

# Validate from stdin
cat license.txt | redisctl enterprise license validate --data -
```

### Check License Expiration

```bash
# Get expiration information
redisctl enterprise license expiry

# Check if expiring soon
redisctl enterprise license expiry -q 'warning'

# Get days remaining
redisctl enterprise license expiry -q 'days_remaining'
```

### View Licensed Features

```bash
# List all licensed features
redisctl enterprise license features

# Check specific features
redisctl enterprise license features -q 'flash_enabled'
redisctl enterprise license features -q 'modules'
```

### License Usage Report

```bash
# Get current usage vs limits
redisctl enterprise license usage

# Get RAM usage
redisctl enterprise license usage -q 'ram'

# Check shard availability
redisctl enterprise license usage -q 'shards.available'
```

## Multi-Instance License Workflows

### License Audit Across All Profiles

```bash
# Audit all configured Redis Enterprise instances
redisctl enterprise workflow license audit

# Show only expiring licenses (within 30 days)
redisctl enterprise workflow license audit --expiring

# Show only expired licenses
redisctl enterprise workflow license audit --expired

# Export as JSON for processing
redisctl enterprise workflow license audit -o json > license-audit.json
```

### Bulk License Updates

```bash
# Update license across all enterprise profiles
redisctl enterprise workflow license bulk-update \
  --profiles all \
  --data @new-license.json

# Update specific profiles
redisctl enterprise workflow license bulk-update \
  --profiles "prod-east,prod-west,staging" \
  --data @new-license.json

# Dry run to see what would be updated
redisctl enterprise workflow license bulk-update \
  --profiles all \
  --data @new-license.json \
  --dry-run
```

### License Compliance Report

```bash
# Generate comprehensive compliance report
redisctl enterprise workflow license report

# Export as CSV for spreadsheets
redisctl enterprise workflow license report --format csv > compliance-report.csv

# Generate JSON report for automation
redisctl enterprise workflow license report -o json
```

### License Monitoring

```bash
# Monitor all profiles for expiring licenses
redisctl enterprise workflow license monitor

# Custom warning threshold (default 30 days)
redisctl enterprise workflow license monitor --warning-days 60

# Exit with error code if any licenses are expiring (for CI/CD)
redisctl enterprise workflow license monitor --fail-on-warning
```

## Automation Examples

### CI/CD License Check

```bash
#!/bin/bash
# Check license status in CI/CD pipeline

if ! redisctl enterprise workflow license monitor --warning-days 14 --fail-on-warning; then
    echo "ERROR: License issues detected!"
    exit 1
fi
```

### License Expiration Script

```bash
#!/bin/bash
# Email alert for expiring licenses

AUDIT=$(redisctl enterprise workflow license audit --expiring -o json)
COUNT=$(echo "$AUDIT" | jq 'length')

if [ "$COUNT" -gt 0 ]; then
    echo "Warning: $COUNT licenses expiring soon!" | \
        mail -s "Redis Enterprise License Alert" admin@company.com
    
    echo "$AUDIT" | jq -r '.[] | 
        "Profile: \(.profile) - Expires: \(.expiration_date) (\(.days_remaining) days)"'
fi
```

### Monthly Compliance Report

```bash
#!/bin/bash
# Generate monthly compliance report

REPORT_DATE=$(date +%Y-%m)
REPORT_FILE="license-compliance-${REPORT_DATE}.csv"

# Generate CSV report
redisctl enterprise workflow license report --format csv > "$REPORT_FILE"

# Email the report
echo "Please find attached the monthly license compliance report." | \
    mail -s "Redis License Report - $REPORT_DATE" \
    -a "$REPORT_FILE" \
    compliance@company.com
```

### Automated License Renewal

```bash
#!/bin/bash
# Automatically apply new license when available

LICENSE_FILE="/secure/path/new-license.json"

if [ -f "$LICENSE_FILE" ]; then
    # Validate the license first
    if redisctl enterprise license validate --data @"$LICENSE_FILE"; then
        # Apply to all production instances
        redisctl enterprise workflow license bulk-update \
            --profiles "prod-east,prod-west" \
            --data @"$LICENSE_FILE"
        
        # Archive the applied license
        mv "$LICENSE_FILE" "/secure/path/applied/$(date +%Y%m%d)-license.json"
    else
        echo "ERROR: Invalid license file!"
        exit 1
    fi
fi
```

## Profile Management for Multi-Instance

### Setup Multiple Profiles

```bash
# Add production profiles
redisctl profile set prod-east \
    --deployment-type enterprise \
    --url https://redis-east.company.com:9443 \
    --username admin@redis.local \
    --password $REDIS_PASS_EAST

redisctl profile set prod-west \
    --deployment-type enterprise \
    --url https://redis-west.company.com:9443 \
    --username admin@redis.local \
    --password $REDIS_PASS_WEST

# Add staging profile
redisctl profile set staging \
    --deployment-type enterprise \
    --url https://redis-staging.company.com:9443 \
    --username admin@redis.local \
    --password $REDIS_PASS_STAGING
```

### Check License Per Profile

```bash
# Check specific profile
redisctl -p prod-east enterprise license expiry
redisctl -p prod-west enterprise license usage
redisctl -p staging enterprise license features
```

## Common Use Cases

### Pre-Renewal Planning

```bash
# Get usage across all instances for capacity planning
for profile in $(redisctl profile list -q '[].name'); do
    echo "=== Profile: $profile ==="
    redisctl -p "$profile" enterprise license usage -o yaml
done
```

### License Synchronization

```bash
# Ensure all instances have the same license
MASTER_LICENSE=$(redisctl -p prod-east enterprise license get -o json)
echo "$MASTER_LICENSE" | \
    redisctl enterprise workflow license bulk-update \
    --profiles "prod-west,staging,dev" \
    --data -
```

### Compliance Dashboard Data

```bash
# Generate JSON data for dashboard
{
    echo '{"timestamp": "'$(date -Iseconds)'",'
    echo '"instances": '
    redisctl enterprise workflow license audit -o json
    echo '}'
} > dashboard-data.json
```

## Output Formats

All commands support multiple output formats:

```bash
# JSON output (default)
redisctl enterprise license get -o json

# YAML output
redisctl enterprise license get -o yaml

# Table output
redisctl enterprise license get -o table
```

## JMESPath Filtering

Use JMESPath queries to extract specific information:

```bash
# Get expiration dates for all profiles
redisctl enterprise workflow license audit -q '[].{profile: profile, expires: expiration_date}'

# Filter only expiring licenses
redisctl enterprise workflow license audit -q "[?expiring_soon==`true`]"

# Get usage percentages
redisctl enterprise license usage -q '{
  ram_used_pct: (ram.used_gb / ram.limit_gb * `100`),
  shards_used_pct: (shards.used / shards.limit * `100`)
}'
```

## Troubleshooting

### Common Issues

1. **License validation fails**
   ```bash
   # Check license format
   redisctl enterprise license validate --data @license.json
   ```

2. **Bulk update fails for some profiles**
   ```bash
   # Use dry-run to identify issues
   redisctl enterprise workflow license bulk-update --profiles all --data @license.json --dry-run
   ```

3. **Monitoring shows unexpected results**
   ```bash
   # Verify profile configurations
   redisctl profile list
   # Test connection to each profile
   for p in $(redisctl profile list -q '[].name'); do
       echo "Testing $p..."
       redisctl -p "$p" enterprise cluster get -q 'name' || echo "Failed: $p"
   done
   ```

## Notes

- License files can be in JSON format or raw license text
- Workflow commands operate on all configured enterprise profiles
- Use `--dry-run` for bulk operations to preview changes
- Monitor commands can integrate with CI/CD pipelines using exit codes
- CSV export format is ideal for spreadsheet analysis and reporting
- All sensitive license data should be handled securely