# Service Management Commands

Configure and manage internal Redis Enterprise services.

## Overview

Service commands provide control over Redis Enterprise's internal services including the cluster manager, proxy, statistics collector, and other system components.

## Available Commands

### List Services

```bash
redisctl enterprise services list
```

Shows all available services and their current status.

### Get Service Configuration

```bash
redisctl enterprise services get <service_name>
```

Retrieves configuration for a specific service.

### Update Service Configuration

```bash
redisctl enterprise services update <service_name> --data '{
  "enabled": true,
  "port": 8080,
  "log_level": "info"
}'
```

### Restart Service

```bash
redisctl enterprise services restart <service_name>
```

Restarts a specific service across the cluster.

### Get Service Status

```bash
redisctl enterprise services status <service_name>
```

Shows detailed status information for a service.

### Enable Service

```bash
redisctl enterprise services enable <service_name>
```

Enables a previously disabled service.

### Disable Service

```bash
redisctl enterprise services disable <service_name>
```

Disables a service (use with caution).

## Common Services

| Service | Description | Critical |
|---------|-------------|----------|
| `cm_server` | Cluster Manager Server | Yes |
| `crdb_coordinator` | Active-Active Coordinator | For CRDB |
| `crdb_worker` | Active-Active Worker | For CRDB |
| `mdns_server` | Multicast DNS Server | No |
| `pdns_server` | PowerDNS Server | Yes |
| `saslauthd` | SASL Authentication | For LDAP |
| `stats_archiver` | Statistics Archiver | No |
| `cnm_http` | Cluster Node Manager | Yes |
| `cnm_https` | Secure CNM | Yes |

## Common Use Cases

### Checking Service Health

```bash
# List all services with status
redisctl enterprise services list -o table

# Check specific critical service
redisctl enterprise services status cm_server

# Get services in JSON for monitoring
redisctl enterprise services list -o json | jq '.[] | select(.status != "running")'
```

### Troubleshooting Service Issues

```bash
# 1. Check service status
redisctl enterprise services status pdns_server

# 2. Review service configuration
redisctl enterprise services get pdns_server

# 3. Restart if needed
redisctl enterprise services restart pdns_server

# 4. Verify after restart
sleep 10
redisctl enterprise services status pdns_server
```

### Managing Statistics Collection

```bash
# Check stats archiver
redisctl enterprise services get stats_archiver

# Adjust retention settings
redisctl enterprise services update stats_archiver --data '{
  "retention_days": 30,
  "collection_interval": 60
}'

# Restart to apply changes
redisctl enterprise services restart stats_archiver
```

### LDAP Service Management

```bash
# Enable SASL for LDAP authentication
redisctl enterprise services enable saslauthd

# Configure SASL service
redisctl enterprise services update saslauthd --data '{
  "mechanisms": ["ldap"],
  "ldap_servers": "ldap://ldap.company.com",
  "ldap_search_base": "dc=company,dc=com"
}'

# Restart SASL service
redisctl enterprise services restart saslauthd
```

## Service Configuration Examples

### Cluster Manager Configuration

```json
{
  "enabled": true,
  "port": 9443,
  "bind_address": "0.0.0.0",
  "log_level": "info",
  "max_connections": 1000,
  "timeout": 30
}
```

### DNS Service Configuration

```json
{
  "enabled": true,
  "port": 53,
  "cache_size": 10000,
  "negative_ttl": 60,
  "query_timeout": 2,
  "recursion": false
}
```

## Monitoring Scripts

### Service Health Check

```bash
#!/bin/bash
# Monitor critical services

CRITICAL_SERVICES="cm_server pdns_server cnm_https"

for service in $CRITICAL_SERVICES; do
  STATUS=$(redisctl enterprise services status $service -q 'status')
  if [[ "$STATUS" != "running" ]]; then
    echo "ALERT: Service $service is $STATUS"
    # Send notification
  fi
done
```

### Service Performance Monitoring

```bash
# Track service resource usage
redisctl enterprise services list -o json | jq -r '.[] | 
  "\(.name): CPU=\(.cpu_usage)% MEM=\(.memory_mb)MB"'
```

## Safety Considerations

### Critical Services

**Never disable these services:**
- `cm_server` - Cluster manager
- `cnm_http/https` - Node management
- `pdns_server` - DNS resolution

### Pre-Restart Checks

```bash
# Before restarting a service
# 1. Check cluster health
redisctl enterprise cluster status

# 2. Verify no ongoing operations
redisctl enterprise action list

# 3. Consider maintenance window
echo "Current load:"
redisctl enterprise stats cluster -q 'operations_per_second'
```

### Service Dependencies

Some services depend on others:
- `saslauthd` requires LDAP configuration
- `crdb_*` services require Active-Active setup
- `stats_archiver` requires sufficient disk space

## Troubleshooting

### Service Won't Start

```bash
# Check logs
redisctl enterprise logs list --filter "service_name=$SERVICE"

# Verify configuration
redisctl enterprise services get $SERVICE

# Check system resources
df -h  # Disk space
free -m  # Memory
```

### Service Consuming High Resources

```bash
# Get detailed status
redisctl enterprise services status $SERVICE -o json

# Check configuration limits
redisctl enterprise services get $SERVICE -q 'resource_limits'

# Adjust if needed
redisctl enterprise services update $SERVICE --data '{
  "max_memory": "2G",
  "max_cpu": 2
}'
```

## Output Examples

### Service List Output

```json
[
  {
    "name": "cm_server",
    "status": "running",
    "enabled": true,
    "pid": 1234,
    "uptime": "7d 2h 15m",
    "cpu_usage": 2.5,
    "memory_mb": 512
  },
  {
    "name": "pdns_server",
    "status": "running",
    "enabled": true,
    "pid": 1235,
    "uptime": "7d 2h 15m",
    "cpu_usage": 0.5,
    "memory_mb": 128
  }
]
```

### Service Status Output

```json
{
  "name": "cm_server",
  "status": "running",
  "enabled": true,
  "configuration": {
    "port": 9443,
    "log_level": "info"
  },
  "statistics": {
    "requests_processed": 1000000,
    "errors": 0,
    "average_response_ms": 50
  },
  "health": {
    "status": "healthy",
    "last_check": "2025-09-15T10:30:00Z"
  }
}
```

## Related Commands

- Cluster Commands - Cluster-wide operations
- Node Commands - Node-specific management
- Logs Commands - Service log viewing