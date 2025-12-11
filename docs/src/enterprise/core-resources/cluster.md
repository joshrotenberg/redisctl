# Cluster

Manage Redis Enterprise cluster configuration and operations.

## Commands

### Get Cluster Info

Get current cluster configuration and status.

```bash
redisctl enterprise cluster info [OPTIONS]
```

**Options:**
- `-o, --output <FORMAT>` - Output format: json, yaml, or table
- `-q, --query <JMESPATH>` - JMESPath query to filter output

**Examples:**

```bash
# Get full cluster information
redisctl enterprise cluster info

# Get specific fields in table format
redisctl enterprise cluster info -o table

# Get cluster name and version
redisctl enterprise cluster info -q "{name: name, version: version}"

# Check cluster health
redisctl enterprise cluster info -q "alert_settings"
```

### Update Cluster

Update cluster configuration.

```bash
redisctl enterprise cluster update --data <JSON> [OPTIONS]
```

**Options:**
- `--data <JSON>` - Configuration updates (inline or @file.json)

**Examples:**

```bash
# Update cluster name
redisctl enterprise cluster update --data '{"name": "production-cluster"}'

# Update alert settings
redisctl enterprise cluster update --data '{
  "alert_settings": {
    "cluster_certs_about_to_expire": {"enabled": true, "threshold": 30}
  }
}'

# Update from file
redisctl enterprise cluster update --data @cluster-config.json
```

### Get Cluster Policy

Get cluster-wide policies.

```bash
redisctl enterprise cluster get-policy [OPTIONS]
```

**Examples:**

```bash
# Get all policies
redisctl enterprise cluster get-policy

# Get specific policy in YAML
redisctl enterprise cluster get-policy -o yaml -q "rack_aware"
```

### Update Cluster Policy

Update cluster policies.

```bash
redisctl enterprise cluster update-policy --data <JSON> [OPTIONS]
```

**Examples:**

```bash
# Enable rack awareness
redisctl enterprise cluster update-policy --data '{"rack_aware": true}'

# Update multiple policies
redisctl enterprise cluster update-policy --data '{
  "rack_aware": true,
  "default_non_sharded_proxy_policy": "all-master-shards"
}'
```

## Certificate Management

### List Certificates

List cluster certificates.

```bash
redisctl enterprise cluster list-certificates [OPTIONS]
```

**Examples:**

```bash
# List all certificates
redisctl enterprise cluster list-certificates

# Check certificate expiration
redisctl enterprise cluster list-certificates -q "[].{name: name, expires: expiry_date}"
```

### Update Certificate

Update cluster certificate.

```bash
redisctl enterprise cluster update-certificate --data <JSON> [OPTIONS]
```

**Example Payload:**

```json
{
  "name": "api-cert",
  "key": "-----BEGIN RSA PRIVATE KEY-----\n...",
  "certificate": "-----BEGIN CERTIFICATE-----\n..."
}
```

**Examples:**

```bash
# Update API certificate
redisctl enterprise cluster update-certificate --data @new-cert.json

# Update proxy certificate
redisctl enterprise cluster update-certificate --data '{
  "name": "proxy-cert",
  "key": "...",
  "certificate": "..."
}'
```

### Rotate Certificates

Rotate cluster certificates.

```bash
redisctl enterprise cluster rotate-certificates [OPTIONS]
```

**Examples:**

```bash
# Rotate all certificates
redisctl enterprise cluster rotate-certificates

# Rotate with custom validity period
redisctl enterprise cluster rotate-certificates --days 365
```

## Cluster Operations

### Check Cluster Status

Get detailed cluster status.

```bash
redisctl enterprise cluster status [OPTIONS]
```

**Examples:**

```bash
# Full status check
redisctl enterprise cluster status

# Check specific components
redisctl enterprise cluster status -q "services"
```

### Get Cluster Stats

Get cluster statistics.

```bash
redisctl enterprise cluster stats [OPTIONS]
```

**Options:**
- `--interval <SECONDS>` - Stats interval (1sec, 1min, 5min, 15min, 1hour, 1day)

**Examples:**

```bash
# Get current stats
redisctl enterprise cluster stats

# Get hourly stats
redisctl enterprise cluster stats --interval 1hour

# Get memory usage
redisctl enterprise cluster stats -q "{used: used_memory, total: total_memory}"
```

### License Management

#### Get License

```bash
redisctl enterprise cluster get-license
```

#### Update License

```bash
redisctl enterprise cluster update-license --data <JSON>
```

**Example:**

```bash
# Update license
redisctl enterprise cluster update-license --data '{
  "license": "-----BEGIN LICENSE-----\n...\n-----END LICENSE-----"
}'
```

## Module Management

### List Modules

List available Redis modules.

```bash
redisctl enterprise module list
```

### Upload Module

Upload a new module.

```bash
redisctl enterprise module upload --file <PATH>
```

**Examples:**

```bash
# Upload module
redisctl enterprise module upload --file redisgraph.zip

# Upload and get module ID
MODULE_ID=$(redisctl enterprise module upload --file module.zip -q "uid")
```

## Common Patterns

### Health Check Script

```bash
#!/bin/bash
# Check cluster health

STATUS=$(redisctl enterprise cluster info -q "status")
if [ "$STATUS" != "active" ]; then
  echo "Cluster not healthy: $STATUS"
  exit 1
fi

# Check certificate expiration
DAYS_LEFT=$(redisctl enterprise cluster list-certificates \
  -q "[0].days_until_expiry")
if [ "$DAYS_LEFT" -lt 30 ]; then
  echo "Certificate expiring soon: $DAYS_LEFT days"
fi
```

### Monitor Cluster Resources

```bash
# Get resource utilization
redisctl enterprise cluster stats -q "{
  cpu: cpu_usage_percent,
  memory: memory_usage_percent,
  disk: persistent_storage_usage_percent
}" | jq
```

### Backup Cluster Configuration

```bash
# Export cluster config
redisctl enterprise cluster info > cluster-backup-$(date +%Y%m%d).json

# Export policies
redisctl enterprise cluster get-policy > policies-backup-$(date +%Y%m%d).json
```

## Troubleshooting

### Common Issues

**"Cluster not responding"**
- Check network connectivity to cluster endpoint
- Verify credentials are correct
- Check if API is enabled on cluster

**"Certificate expired"**
- Rotate certificates: `redisctl enterprise cluster rotate-certificates`
- Or update manually with new certificate

**"License expired"**
- Update license: `redisctl enterprise cluster update-license --data @license.json`
- Contact Redis support for new license

**"Policy update failed"**
- Some policies require cluster restart
- Check policy compatibility with cluster version

## Related Commands

- [Nodes](./nodes.md) - Manage cluster nodes
- Databases - Manage databases in cluster
- Users - Manage cluster users

## API Reference

These commands use the following REST endpoints:
- `GET /v1/cluster` - Get cluster info
- `PUT /v1/cluster` - Update cluster
- `GET /v1/cluster/policy` - Get policies
- `PUT /v1/cluster/policy` - Update policies
- `GET /v1/cluster/certificates` - List certificates
- `PUT /v1/cluster/update_cert` - Update certificate
- `POST /v1/cluster/certificates/rotate` - Rotate certificates

For direct API access: `redisctl api enterprise get /v1/cluster`