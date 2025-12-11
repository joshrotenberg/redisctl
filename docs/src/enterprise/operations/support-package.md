# Support Package Commands (Phase 2)

Enhanced support package generation with improved UX, async operations, and intelligent defaults.

## Overview

The `support-package` command group provides a dedicated, user-friendly interface for generating Redis Enterprise support packages. This is the recommended way to collect diagnostic information for Redis Support tickets.

## Why Use Support Package Commands?

While `debug-info` commands provide the core functionality, `support-package` commands offer:
- **Better UX**: Clear progress indicators and helpful output
- **Smart defaults**: Automatic timestamps and intelligent file naming
- **Pre-flight checks**: Disk space and permission verification
- **Async support**: Handle long-running operations gracefully
- **Next steps**: Clear guidance on uploading to support

## Available Commands

### Generate Cluster Support Package

```bash
# Quick generation with all defaults
redisctl enterprise support-package cluster

# Custom output location
redisctl enterprise support-package cluster -o /tmp/support.tar.gz

# Skip pre-flight checks (not recommended)
redisctl enterprise support-package cluster --skip-checks

# Use new API endpoints (Redis Enterprise 7.4+)
redisctl enterprise support-package cluster --use-new-api

# Optimize package size (reduces by ~20-30%)
redisctl enterprise support-package cluster --optimize

# Show optimization details
redisctl enterprise support-package cluster --optimize --optimize-verbose

# Upload directly to Redis Support (Files.com)
export REDIS_ENTERPRISE_FILES_API_KEY="your-api-key"
redisctl enterprise support-package cluster --upload

# Upload without saving locally
redisctl enterprise support-package cluster --upload --no-save

# Optimize and upload in one command
redisctl enterprise support-package cluster --optimize --upload --no-save
```

**Example Output**:
```
Redis Enterprise Support Package
================================
Cluster: prod-cluster-01
Version: 7.2.4
Nodes: 3
Databases: 5

Output: ./support-package-cluster-20240115T143000.tar.gz

Generating support package...
⠋ Collecting cluster data...

✓ Support package created successfully
  File: support-package-cluster-20240115T143000.tar.gz
  Size: 487.3 MB
  Time: 154s

Next steps:
1. Upload to Redis Support: https://support.redis.com/upload
2. Reference your case number when uploading
3. Delete local file after upload to free space
```

### Generate Database Support Package

```bash
# Support package for specific database
redisctl enterprise support-package database 1

# Custom output with database name
redisctl enterprise support-package database 1 \
  -o production-db-issue.tar.gz

# For Active-Active database
redisctl enterprise support-package database 5 --use-new-api
```

**Example Output**:
```
Redis Enterprise Support Package
================================
Database: 1
Name: production-cache

Output: ./support-package-database-1-20240115T143000.tar.gz

Generating support package...
⠋ Collecting database 1 data...

✓ Database support package created successfully
  File: support-package-database-1-20240115T143000.tar.gz
  Size: 125.7 MB
  Time: 45s

Next steps:
1. Upload to Redis Support: https://support.redis.com/upload
2. Reference your case number when uploading
3. Delete local file after upload to free space
```

### Generate Node Support Package

```bash
# All nodes
redisctl enterprise support-package node

# Specific node
redisctl enterprise support-package node 2

# Custom output for node issue
redisctl enterprise support-package node 2 \
  -o node2-memory-issue.tar.gz
```

**Example Output**:
```
Redis Enterprise Support Package
================================
Node: 2
Address: 10.0.1.2

Output: ./support-package-node-2-20240115T143000.tar.gz

Generating support package...
⠋ Collecting node 2 data...

✓ Node support package created successfully
  File: support-package-node-2-20240115T143000.tar.gz
  Size: 89.3 MB
  Time: 32s

Next steps:
1. Upload to Redis Support: https://support.redis.com/upload
2. Reference your case number when uploading
3. Delete local file after upload to free space
```

## Package Optimization

Support packages can be large (500MB-2GB+). The `--optimize` flag reduces package size by 20-30% through:

- **Log truncation**: Keeps most recent 1000 lines per log file (configurable)
- **Redundant data removal**: Removes duplicate or unnecessary files
- **Nested archive cleanup**: Removes nested .gz files

### Basic Optimization

```bash
# Optimize with defaults
redisctl enterprise support-package cluster --optimize

# Customize log retention
redisctl enterprise support-package cluster --optimize --log-lines 5000

# Show detailed optimization stats
redisctl enterprise support-package cluster --optimize --optimize-verbose
```

### Optimization Output

```
Optimization: 487.3 MB → 358.2 MB (26.5% reduction)

Files processed: 847
Files truncated: 142
Files removed: 23
```

### When to Use Optimization

**Use optimization when:**
- Package size exceeds upload limits
- Network bandwidth is limited
- Storage space is constrained
- Only recent log data is needed

**Skip optimization when:**
- Full historical logs are needed for issue diagnosis
- Investigating intermittent issues from the past
- Redis Support specifically requests unoptimized packages

## Direct Upload to Redis Support

Upload support packages directly to Files.com for Redis Support tickets, eliminating manual upload steps.

### Setup Files.com API Key

Get your Files.com API key from Redis Support, then configure it:

```bash
# Option 1: Environment variable (recommended for CI/CD)
export REDIS_ENTERPRISE_FILES_API_KEY="your-api-key"

# Option 2: Secure keyring storage (requires secure-storage feature)
redisctl files-key set "$REDIS_ENTERPRISE_FILES_API_KEY" --use-keyring

# Option 3: Global config file (plaintext)
redisctl files-key set "$REDIS_ENTERPRISE_FILES_API_KEY" --global

# Option 4: Per-profile config
redisctl files-key set "$REDIS_ENTERPRISE_FILES_API_KEY" --profile enterprise-prod
```

### Upload Commands

```bash
# Generate and upload
redisctl enterprise support-package cluster --upload

# Upload without local copy (saves disk space)
redisctl enterprise support-package cluster --upload --no-save

# Optimize before upload (recommended)
redisctl enterprise support-package cluster --optimize --upload --no-save

# Database-specific package
redisctl enterprise support-package database 1 --optimize --upload
```

### Upload Output

```
Generating support package...
Uploading to Files.com: /RLEC_Customers/Uploads/support-package-cluster-20240115T143000.tar.gz
Size: 358234567 bytes

✓ Support package created successfully
  Uploaded to: RLEC_Customers/Uploads/support-package-cluster-20240115T143000.tar.gz
  Size: 341.7 MB
  Time: 124s
```

### API Key Priority

The Files.com API key is resolved in this order:

1. `REDIS_ENTERPRISE_FILES_API_KEY` environment variable
2. Profile-specific `files_api_key` in config
3. Global `files_api_key` in config  
4. System keyring (if secure-storage feature enabled)
5. `REDIS_FILES_API_KEY` environment variable (fallback)

### Secure API Key Storage

With the `secure-storage` feature, API keys are stored in your OS keyring:

- **macOS**: Keychain
- **Windows**: Credential Manager
- **Linux**: Secret Service (GNOME Keyring, KWallet)

```bash
# Install with secure storage
cargo install redisctl --features secure-storage

# Store key securely
redisctl files-key set "$REDIS_ENTERPRISE_FILES_API_KEY" --use-keyring

# Verify storage
redisctl files-key get
# Output: Key found in keyring: your-ke...key4

# Remove when no longer needed
redisctl files-key remove --keyring
```

The config file only stores a reference:
```toml
files_api_key = "keyring:files-api-key"
```

## Pre-flight Checks

The command automatically performs safety checks before generating packages:

### Disk Space Check
```
Warning: Low disk space detected (< 1GB available)
Continue anyway? (y/N):
```

### File Overwrite Protection
```
Warning: File support-package.tar.gz already exists
Overwrite? (y/N):
```

### Permission Verification
```
Error: Cannot write to directory /restricted/path
Please choose a different location or check permissions
```

To skip all checks (not recommended for production):
```bash
redisctl enterprise support-package cluster --skip-checks
```

## Async Operations

For large clusters, support package generation can take several minutes:

### With Wait (Default)
```bash
# Wait for completion with default timeout (10 minutes)
redisctl enterprise support-package cluster --wait

# Custom timeout (30 minutes for very large clusters)
redisctl enterprise support-package cluster --wait --wait-timeout 1800
```

### Without Wait
```bash
# Start generation and return immediately
redisctl enterprise support-package cluster --no-wait

# Output:
# Task ID: abc123-def456-789
# Check status: redisctl enterprise support-package status abc123-def456-789
```

### Check Status
```bash
redisctl enterprise support-package status abc123-def456-789

# Output:
# Support Package Generation Status
# =================================
# Task ID: abc123-def456-789
# Status: in_progress
# Progress: 65%
# Message: Collecting node 3 data...
```

## List Available Packages

```bash
redisctl enterprise support-package list
```

**Note**: Most Redis Enterprise versions don't store generated packages on the server. This command is a placeholder for future functionality.

## Smart File Naming

The command uses intelligent defaults for file names:

| Type | Pattern | Example |
|------|---------|---------|
| Cluster | `support-package-cluster-{timestamp}.tar.gz` | `support-package-cluster-20240115T143000.tar.gz` |
| Database | `support-package-database-{uid}-{timestamp}.tar.gz` | `support-package-database-1-20240115T143000.tar.gz` |
| Node | `support-package-node-{uid}-{timestamp}.tar.gz` | `support-package-node-2-20240115T143000.tar.gz` |
| All Nodes | `support-package-nodes-{timestamp}.tar.gz` | `support-package-nodes-20240115T143000.tar.gz` |

Timestamps use ISO format for easy sorting: `YYYYMMDDTHHMMSS`

## Best Practices

### 1. Organized Collection
```bash
#!/bin/bash
# Create case-specific directory
CASE_ID="CASE-12345"
mkdir -p "./support-$CASE_ID"

# Collect all relevant packages
redisctl enterprise support-package cluster \
  -o "./support-$CASE_ID/cluster.tar.gz"

redisctl enterprise support-package database 1 \
  -o "./support-$CASE_ID/database-1.tar.gz"

# Create summary
echo "Case: $CASE_ID" > "./support-$CASE_ID/README.txt"
echo "Issue: Database 1 high latency" >> "./support-$CASE_ID/README.txt"
echo "Collected: $(date)" >> "./support-$CASE_ID/README.txt"
```

### 2. Automated Daily Collection
```bash
#!/bin/bash
# Daily support package collection for monitoring

OUTPUT_DIR="/backup/support-packages"
RETENTION_DAYS=7

# Generate with date-based naming
redisctl enterprise support-package cluster \
  -o "$OUTPUT_DIR/daily-$(date +%Y%m%d).tar.gz"

# Clean up old packages
find "$OUTPUT_DIR" -name "daily-*.tar.gz" \
  -mtime +$RETENTION_DAYS -delete
```

### 3. Pre-incident Collection
```bash
# Collect baseline before maintenance
redisctl enterprise support-package cluster \
  -o "baseline-pre-upgrade-$(date +%Y%m%d).tar.gz"

# Perform upgrade...

# Collect post-change package
redisctl enterprise support-package cluster \
  -o "post-upgrade-$(date +%Y%m%d).tar.gz"
```

## Integration with Support Workflow

### 1. Generate Package
```bash
redisctl enterprise support-package cluster
```

### 2. Verify Package
```bash
# Check file size and type
ls -lh support-package-*.tar.gz
file support-package-*.tar.gz

# Quick content verification
tar -tzf support-package-*.tar.gz | head -20
```

### 3. Upload to Support
- Navigate to https://support.redis.com/upload
- Select your case number
- Upload the tar.gz file directly
- Add description of the issue

### 4. Clean Up
```bash
# Remove local copy after successful upload
rm support-package-*.tar.gz
```

## Troubleshooting

### Package Generation Fails
```bash
# Check cluster connectivity
redisctl enterprise cluster get

# Verify credentials
redisctl profile list

# Try with explicit credentials
export REDIS_ENTERPRISE_URL="https://your-cluster:9443"
export REDIS_ENTERPRISE_USER="your-user"
export REDIS_ENTERPRISE_PASSWORD="your-password"
export REDIS_ENTERPRISE_INSECURE="true"
```

### Timeout Issues
```bash
# Increase timeout for large clusters
redisctl enterprise support-package cluster \
  --wait --wait-timeout 3600  # 1 hour
```

### Permission Denied
```bash
# Use a writable directory
redisctl enterprise support-package cluster \
  -o /tmp/support.tar.gz

# Or fix permissions
chmod 755 ./output-directory
```

## Comparison with debug-info

| Feature | debug-info | support-package |
|---------|------------|-----------------|
| Binary download | ✅ | ✅ |
| Progress indicators | ✅ | ✅ Enhanced |
| Pre-flight checks | ❌ | ✅ |
| Smart naming | Basic | Advanced |
| Async operations | ❌ | ✅ |
| Status checking | ❌ | ✅ |
| Clear next steps | ❌ | ✅ |
| Cluster info display | ❌ | ✅ |

## CI/CD Integration with JSON Output

The support-package commands fully support structured JSON output for automation and CI/CD pipelines.

### Basic JSON Output

```bash
# Generate package with JSON output
redisctl enterprise support-package cluster -o json

# Output:
{
  "success": true,
  "package_type": "cluster",
  "file_path": "support-package-cluster-20240115T143000.tar.gz",
  "file_size": 510234567,
  "file_size_display": "487.3 MB",
  "elapsed_seconds": 154,
  "cluster_name": "prod-cluster-01",
  "cluster_version": "7.2.4-92",
  "message": "Support package created successfully",
  "timestamp": "2024-01-15T14:32:34Z"
}
```

### CI/CD Script Examples

#### Automated Collection on Failure

```bash
#!/bin/bash
# collect-support-on-failure.sh

# Run tests
if ! ./run-tests.sh; then
  echo "Tests failed, collecting support package..."
  
  # Generate support package with JSON output
  result=$(redisctl enterprise support-package cluster -o json)
  
  # Check if successful
  if [ $(echo "$result" | jq -r '.success') = "true" ]; then
    file_path=$(echo "$result" | jq -r '.file_path')
    file_size=$(echo "$result" | jq -r '.file_size_display')
    
    echo "Support package created: $file_path ($file_size)"
    
    # Upload to artifact storage
    aws s3 cp "$file_path" "s3://support-packages/$(date +%Y%m%d)/"
    
    # Create support ticket
    curl -X POST https://support.redis.com/api/tickets \
      -H "Authorization: Bearer $SUPPORT_TOKEN" \
      -d @- <<EOF
{
  "title": "CI Test Failure - $(date)",
  "priority": "high",
  "attachment": "$file_path",
  "metadata": $(echo "$result" | jq -c .)
}
EOF
    
    # Clean up local file
    rm "$file_path"
  else
    echo "Failed to create support package"
    echo "$result" | jq -r '.error'
    exit 1
  fi
fi
```

#### GitHub Actions Integration

```yaml
name: Support Package Collection

on:
  workflow_dispatch:
  schedule:
    - cron: '0 0 * * 0'  # Weekly on Sunday

jobs:
  collect-support:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install redisctl
        run: |
          curl -L https://github.com/joshrotenberg/redisctl/releases/latest/download/redisctl-linux-amd64.tar.gz | tar xz
          sudo mv redisctl /usr/local/bin/
      
      - name: Configure Redis Enterprise credentials
        run: |
          redisctl profile set enterprise \
            --deployment enterprise \
            --url ${{ secrets.REDIS_ENTERPRISE_URL }} \
            --username ${{ secrets.REDIS_ENTERPRISE_USER }} \
            --password ${{ secrets.REDIS_ENTERPRISE_PASSWORD }} \
            --insecure
      
      - name: Collect support package
        id: support
        run: |
          # Generate package with JSON output
          OUTPUT=$(redisctl enterprise support-package cluster -o json)
          echo "$OUTPUT" > support-result.json
          
          # Extract key fields
          SUCCESS=$(echo "$OUTPUT" | jq -r '.success')
          FILE_PATH=$(echo "$OUTPUT" | jq -r '.file_path')
          FILE_SIZE=$(echo "$OUTPUT" | jq -r '.file_size_display')
          
          # Set outputs for next steps
          echo "success=$SUCCESS" >> $GITHUB_OUTPUT
          echo "file_path=$FILE_PATH" >> $GITHUB_OUTPUT
          echo "file_size=$FILE_SIZE" >> $GITHUB_OUTPUT
      
      - name: Upload artifact
        if: steps.support.outputs.success == 'true'
        uses: actions/upload-artifact@v4
        with:
          name: support-package-${{ github.run_id }}
          path: ${{ steps.support.outputs.file_path }}
          retention-days: 30
      
      - name: Create issue on large package
        if: steps.support.outputs.success == 'true'
        run: |
          FILE_SIZE_BYTES=$(jq -r '.file_size' support-result.json)
          
          # If package is over 1GB, create an issue
          if [ "$FILE_SIZE_BYTES" -gt 1073741824 ]; then
            gh issue create \
              --title "Large support package detected" \
              --body "Support package size: ${{ steps.support.outputs.file_size }}" \
              --label monitoring
          fi
```

#### Jenkins Pipeline

```groovy
pipeline {
  agent any
  
  stages {
    stage('Health Check') {
      steps {
        script {
          def clusterHealth = sh(
            script: 'redisctl enterprise cluster get -o json',
            returnStdout: true
          ).trim()
          
          def health = readJSON text: clusterHealth
          if (health.data.state != 'active') {
            echo "Cluster unhealthy, generating support package..."
            
            def supportResult = sh(
              script: 'redisctl enterprise support-package cluster -o json',
              returnStdout: true
            ).trim()
            
            def support = readJSON text: supportResult
            if (support.success) {
              archiveArtifacts artifacts: support.file_path
              
              // Send notification
              emailext (
                subject: "Redis Cluster Issue - Support Package Generated",
                body: """
                  Cluster State: ${health.data.state}
                  Support Package: ${support.file_path}
                  Size: ${support.file_size_display}
                  Generated at: ${support.timestamp}
                """,
                to: 'ops-team@company.com'
              )
            }
          }
        }
      }
    }
  }
}
```

#### Terraform Integration

```hcl
# Generate support package before infrastructure changes

resource "null_resource" "pre_change_support" {
  provisioner "local-exec" {
    command = <<-EOT
      # Generate support package and capture output
      OUTPUT=$(redisctl enterprise support-package cluster -o json)
      
      # Save to state bucket
      if [ $(echo "$OUTPUT" | jq -r '.success') = "true" ]; then
        FILE=$(echo "$OUTPUT" | jq -r '.file_path')
        aws s3 cp "$FILE" "s3://terraform-state/support-packages/pre-${timestamp()}/"
      fi
    EOT
  }
  
  triggers = {
    always_run = timestamp()
  }
}
```

### Parsing JSON Output in Different Languages

#### Python

```python
import json
import subprocess

# Generate support package
result = subprocess.run(
    ['redisctl', 'enterprise', 'support-package', 'cluster', '-o', 'json'],
    capture_output=True,
    text=True
)

# Parse JSON output
data = json.loads(result.stdout)

if data['success']:
    print(f"Package created: {data['file_path']}")
    print(f"Size: {data['file_size_display']}")
    print(f"Time taken: {data['elapsed_seconds']} seconds")
    
    # Upload to monitoring system
    metrics.send('support_package.size', data['file_size'])
    metrics.send('support_package.generation_time', data['elapsed_seconds'])
else:
    print(f"Error: {data.get('error', 'Unknown error')}")
```

#### Node.js

```javascript
const { exec } = require('child_process');
const fs = require('fs');

// Generate support package
exec('redisctl enterprise support-package cluster -o json', (error, stdout, stderr) => {
  if (error) {
    console.error(`Error: ${error.message}`);
    return;
  }
  
  const result = JSON.parse(stdout);
  
  if (result.success) {
    console.log(`Package created: ${result.file_path}`);
    console.log(`Size: ${result.file_size_display}`);
    
    // Upload to cloud storage
    uploadToS3(result.file_path).then(() => {
      // Clean up local file
      fs.unlinkSync(result.file_path);
    });
  }
});
```

### Monitoring and Alerting

```bash
#!/bin/bash
# monitor-support-package.sh

# Generate package and check size
result=$(redisctl enterprise support-package cluster -o json)

if [ $(echo "$result" | jq -r '.success') = "true" ]; then
  size_bytes=$(echo "$result" | jq -r '.file_size')
  elapsed=$(echo "$result" | jq -r '.elapsed_seconds')
  
  # Send metrics to monitoring system
  curl -X POST http://metrics.internal/api/v1/metrics \
    -H "Content-Type: application/json" \
    -d @- <<EOF
{
  "metrics": [
    {
      "name": "redis.support_package.size_bytes",
      "value": $size_bytes,
      "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    },
    {
      "name": "redis.support_package.generation_seconds",
      "value": $elapsed,
      "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    }
  ]
}
EOF
  
  # Alert if package is too large
  if [ "$size_bytes" -gt 2147483648 ]; then  # 2GB
    curl -X POST http://alerts.internal/api/v1/alert \
      -H "Content-Type: application/json" \
      -d "{\"severity\": \"warning\", \"message\": \"Large support package: $(echo "$result" | jq -r '.file_size_display')\"}"
  fi
fi
```

## Related Commands

- [Debug Info Commands](debuginfo.md) - Lower-level diagnostic collection
- Logs Commands - View logs without full package
- Cluster Commands - Check cluster health
- Database Commands - Database management