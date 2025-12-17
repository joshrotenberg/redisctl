# Alert Management Commands

Manage alerts for Redis Enterprise clusters, nodes, and databases.

## Overview

The alerts commands provide comprehensive monitoring and management of alerts across your Redis Enterprise deployment. You can:
- List and filter alerts by type and severity
- Get details on specific alerts
- Manage alert settings
- View alerts at cluster, node, and database levels

## Commands

### List All Alerts

List all alerts across the cluster with optional filtering:

```bash
# List all alerts
redisctl enterprise alerts list

# Filter by alert type (cluster, node, bdb)
redisctl enterprise alerts list --filter-type cluster

# Filter by severity (info, warning, error, critical)
redisctl enterprise alerts list --severity error

# Combine filters
redisctl enterprise alerts list --filter-type node --severity warning
```

### Get Specific Alert

Get details for a specific alert by UID:

```bash
redisctl enterprise alerts get 123
```

### Cluster Alerts

View alerts at the cluster level:

```bash
# Get all cluster alerts
redisctl enterprise alerts cluster

# Get specific cluster alert by name
redisctl enterprise alerts cluster --alert cluster_license_about_to_expire
```

### Node Alerts

View alerts for nodes:

```bash
# Get all node alerts
redisctl enterprise alerts node

# Get alerts for specific node
redisctl enterprise alerts node 1

# Get specific alert for a node
redisctl enterprise alerts node 1 --alert node_ephemeral_storage
```

### Database Alerts

View alerts for databases:

```bash
# Get all database alerts  
redisctl enterprise alerts database

# Get alerts for specific database
redisctl enterprise alerts database 1

# Get specific alert for a database
redisctl enterprise alerts database 1 --alert bdb_backup_failed
```

### Alert Settings

Manage alert configuration settings:

```bash
# Get current alert settings
redisctl enterprise alerts settings-get

# Update alert settings
redisctl enterprise alerts settings-update --data '{
  "cluster_license_about_to_expire": {
    "enabled": true,
    "threshold": "30"
  }
}'

# Update from file
redisctl enterprise alerts settings-update --data @alert-settings.json

# Update from stdin
echo '{"node_ephemeral_storage": {"enabled": true, "threshold": "80"}}' | \
  redisctl enterprise alerts settings-update --data -
```

## Output Formats

All commands support multiple output formats:

```bash
# JSON output (default)
redisctl enterprise alerts list -o json

# YAML output
redisctl enterprise alerts list -o yaml

# Table output
redisctl enterprise alerts list -o table
```

## JMESPath Filtering

Use JMESPath queries to filter and transform output:

```bash
# Get only alert names
redisctl enterprise alerts list -q '[].name'

# Get alerts with severity error or critical
redisctl enterprise alerts list -q "[?severity=='error' || severity=='critical']"

# Get alert count by type
redisctl enterprise alerts list -q 'length(@)'

# Get specific fields
redisctl enterprise alerts settings-get -q 'node_ephemeral_storage'
```

## Common Use Cases

### Monitor Critical Alerts

```bash
# List all critical alerts
redisctl enterprise alerts list --severity critical -o table

# Check for license expiration
redisctl enterprise alerts cluster --alert cluster_license_about_to_expire
```

### Alert Monitoring Script

```bash
#!/bin/bash
# Monitor for critical alerts

ALERT_COUNT=$(redisctl enterprise alerts list --severity critical -q 'length(@)')

if [ "$ALERT_COUNT" -gt 0 ]; then
    echo "Critical alerts found:"
    redisctl enterprise alerts list --severity critical \
      -q "[].{type: type, name: name, description: description}" -o table
    exit 1
fi
```

### Adjust Alert Thresholds

```bash
# Set more aggressive storage thresholds
redisctl enterprise alerts settings-update --data '{
  "node_ephemeral_storage": {
    "enabled": true,
    "threshold": "60"
  },
  "node_persistent_storage": {
    "enabled": true,
    "threshold": "60"
  }
}'
```

### Check Database Health

```bash
# Get all database alerts for monitoring
for db_id in $(redisctl enterprise database list -q '[].uid'); do
    echo "Checking database $db_id..."
    redisctl enterprise alerts database $db_id
done
```

## Alert Types

### Cluster Alert Types
- `cluster_ca_cert_about_to_expire` - CA certificate expiration warning
- `cluster_certs_about_to_expire` - SSL certificate expiration warning
- `cluster_license_about_to_expire` - License expiration warning
- `cluster_node_operation_failed` - Node operation failure
- `cluster_ocsp_query_failed` - OCSP query failure
- `cluster_ocsp_status_revoked` - Certificate revoked via OCSP

### Node Alert Types
- `node_checks_error` - Node health check errors
- `node_ephemeral_storage` - Ephemeral storage threshold exceeded
- `node_free_flash` - Flash storage threshold exceeded
- `node_internal_certs_about_to_expire` - Internal certificate expiration
- `node_persistent_storage` - Persistent storage threshold exceeded

### Database Alert Types
- `bdb_backup_failed` - Database backup failure
- `bdb_crdt_sync_error` - Active-Active synchronization error
- `bdb_high_latency` - High latency detected
- `bdb_high_memory` - Memory usage threshold exceeded
- `bdb_replica_sync_error` - Replica synchronization error

## Notes

- Alert thresholds are configured in the cluster settings
- Some alerts have configurable thresholds (e.g., storage, certificate expiration)
- Critical alerts should be addressed immediately
- Use profiles to manage multiple Redis Enterprise deployments:
  ```bash
  redisctl -p production enterprise alerts list --severity critical
  ```