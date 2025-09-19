# Debug Info Commands

Collect diagnostic information and support packages for troubleshooting Redis Enterprise clusters.

## Overview

Debug info commands gather comprehensive diagnostic data from Redis Enterprise clusters, nodes, and databases. As of Phase 1 improvements, these commands now properly download binary tar.gz support packages that can be directly uploaded to Redis Support.

## Available Commands

### Collect Cluster Support Package

```bash
# Download cluster-wide support package (recommended)
redisctl enterprise debug-info all

# With custom output file
redisctl enterprise debug-info all --file /tmp/cluster-support.tar.gz

# Use new API endpoint (for Redis Enterprise 7.4+)
redisctl enterprise debug-info all --use-new-api
```

**Output**: Downloads a tar.gz file containing:
- Complete cluster configuration
- All node information and logs
- Database configurations
- System metrics and diagnostics
- Network configuration
- Performance data

**Default filename**: `support-package-cluster-{timestamp}.tar.gz`

### Collect Node Support Package

```bash
# Download support package for all nodes
redisctl enterprise debug-info node

# Download for specific node
redisctl enterprise debug-info node 1

# With custom output
redisctl enterprise debug-info node 1 --file /tmp/node1-support.tar.gz
```

**Output**: Downloads a tar.gz file containing:
- Node configuration and state
- System resources and metrics
- Local log files
- Process information
- Network configuration

**Default filename**: 
- All nodes: `support-package-nodes-{timestamp}.tar.gz`
- Specific node: `support-package-node-{uid}-{timestamp}.tar.gz`

### Collect Database Support Package

```bash
# Download support package for specific database
redisctl enterprise debug-info database 1

# With custom output
redisctl enterprise debug-info database 1 --file /tmp/db1-support.tar.gz

# Use new API endpoint
redisctl enterprise debug-info database 1 --use-new-api
```

**Output**: Downloads a tar.gz file containing:
- Database configuration
- Shard distribution and state
- Replication information
- Performance metrics
- Recent operations and logs

**Default filename**: `support-package-db-{uid}-{timestamp}.tar.gz`

## Binary Download Support (Phase 1)

Starting with v0.5.1, all debug-info commands properly handle binary responses:

```bash
# Downloads actual tar.gz file (not JSON)
redisctl enterprise debug-info all

# Verify the downloaded file
file support-package-cluster-*.tar.gz
# Output: gzip compressed data, from Unix

# Extract and view contents
tar -tzf support-package-cluster-*.tar.gz | head
```

### API Endpoint Compatibility

The tool supports both old (deprecated) and new API endpoints:

| Command | Old Endpoint (default) | New Endpoint (--use-new-api) |
|---------|------------------------|------------------------------|
| `all` | `/v1/debuginfo/all` | `/v1/cluster/debuginfo` |
| `node` | `/v1/debuginfo/node` | `/v1/nodes/{uid}/debuginfo` |
| `database` | `/v1/debuginfo/all/bdb/{uid}` | `/v1/bdbs/{uid}/debuginfo` |

**Note**: Old endpoints are deprecated as of Redis Enterprise 7.4. Use `--use-new-api` for newer clusters.

## Common Use Cases

### Quick Support Package for Troubleshooting

```bash
# Generate support package with automatic naming
redisctl enterprise debug-info all

# Output shows:
# ✓ Support package created successfully
#   File: support-package-cluster-20250916-110539.tar.gz
#   Size: 305.7 KB
```

### Preparing for Support Ticket

```bash
# 1. Generate cluster support package
redisctl enterprise debug-info all --file support-case-12345.tar.gz

# 2. Verify the file
ls -lh support-case-12345.tar.gz
file support-case-12345.tar.gz

# 3. Upload to Redis Support portal
# Reference your case number: 12345
```

### Database-Specific Issues

```bash
# Generate package for problematic database
redisctl enterprise debug-info database 1

# The package includes database-specific logs and metrics
# Upload directly to support ticket
```

### Automated Collection Script

```bash
#!/bin/bash
# Collect support packages for all components

TIMESTAMP=$(date +%Y%m%d-%H%M%S)
OUTPUT_DIR="./support-$TIMESTAMP"
mkdir -p "$OUTPUT_DIR"

echo "Collecting cluster support package..."
redisctl enterprise debug-info all \
  --file "$OUTPUT_DIR/cluster.tar.gz"

echo "Collecting node support packages..."
for node_id in 1 2 3; do
  redisctl enterprise debug-info node $node_id \
    --file "$OUTPUT_DIR/node-$node_id.tar.gz"
done

echo "Support packages saved to $OUTPUT_DIR"
```

## Important Notes

### Security Considerations
- Support packages contain sensitive information (hostnames, IPs, configurations)
- Review contents before sharing if needed
- Delete local copies after uploading to support
- Use secure channels for transmission

### Performance Impact
- Package generation may temporarily impact cluster performance
- Large clusters can generate packages over 1GB
- Run during maintenance windows when possible
- Network bandwidth considerations for remote clusters

### File Management
- Files are saved in current directory by default
- Use `--file` to specify custom location
- Automatic timestamp prevents overwriting
- Clean up old support packages regularly

## Progress Indicators

The tool now shows progress during package generation:

```
⠋ Generating support package...
✓ Support package created successfully
  File: support-package-cluster-20250916-110539.tar.gz
  Size: 305.7 KB
```

## Troubleshooting

### Authentication Errors

If you get authentication errors, ensure correct credentials:

```bash
# Check your profile
redisctl profile list

# Use environment variables for testing
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_USER="admin@redis.local"
export REDIS_ENTERPRISE_PASSWORD="your_password"
export REDIS_ENTERPRISE_INSECURE="true"
```

### Large File Sizes

For very large support packages:

```bash
# Stream directly to compressed file
redisctl enterprise debug-info all --file >(gzip -9 > support.tar.gz)

# Split large files for upload
split -b 100M support-package.tar.gz support-part-
```

### Verify Package Contents

```bash
# List contents without extracting
tar -tzf support-package-cluster-*.tar.gz

# Extract specific files
tar -xzf support-package-cluster-*.tar.gz logs/

# View package info
gzip -l support-package-cluster-*.tar.gz
```

## Related Commands

- [Support Package Commands](support-package.md) - Enhanced support package workflow (Phase 2)
- [Logs Commands](logs.md) - View cluster logs directly
- [Stats Commands](stats.md) - Monitor performance metrics
- [Cluster Commands](cluster.md) - Check cluster health