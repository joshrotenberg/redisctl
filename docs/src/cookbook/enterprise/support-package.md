# Generate and Upload a Support Package

⏱️ **Time:** 10-15 minutes  
📋 **Prerequisites:**
- Redis Enterprise cluster running
- redisctl installed and configured
- (Optional) Files.com account for upload ([sign up](https://www.files.com/))

## Quick Command

Generate support package for entire cluster:

```bash
redisctl enterprise support-package cluster \
  --file /tmp/support-package.tar.gz
```

## What is a Support Package?

A support package is a comprehensive diagnostic bundle containing:
- Cluster configuration and logs
- Database configurations and statistics
- Node health and metrics
- Network configuration
- Redis server logs

Used for troubleshooting with Redis support or internal diagnostics.

## Step-by-Step Guide

### 1. Generate Basic Support Package

Create a support package for the entire cluster:

```bash
redisctl enterprise support-package cluster \
  --file /tmp/cluster-support-$(date +%Y%m%d).tar.gz
```

**What you should see:**
```
Generating support package...
Support package saved to: /tmp/cluster-support-20251007.tar.gz
Size: 45.2 MB
```

### 2. Generate for Specific Database

Create a package for just one database (smaller, faster):

```bash
redisctl enterprise support-package database \
  --database-id 1 \
  --file /tmp/db1-support.tar.gz
```

### 3. Optimize Before Upload

Reduce package size for faster upload:

```bash
redisctl enterprise support-package database \
  --database-id 1 \
  --optimize \
  --file /tmp/db1-optimized.tar.gz
```

**What `--optimize` does:**
- Compresses logs more aggressively
- Excludes large binary dumps
- Typically 50-70% smaller
- Still contains all diagnostic info

### 4. Upload to Files.com

#### One-Time Setup

Set up your Files.com API key:

```bash
# Store securely in keyring (recommended)
redisctl files-key set --use-keyring

# Or set as environment variable
export FILES_API_KEY="your-api-key"
```

#### Generate and Upload

Create package and upload in one command:

```bash
redisctl enterprise support-package database \
  --database-id 1 \
  --optimize \
  --upload \
  --no-save
```

**Flags explained:**
- `--upload`: Upload to Files.com after generation
- `--no-save`: Don't save locally (only upload)
- `--optimize`: Reduce size before upload

**What you should see:**
```
Generating support package...
Optimizing package...
Uploading to Files.com...
✓ Uploaded: /support-packages/db1-20251007-abc123.tar.gz
URL: https://yourcompany.files.com/file/support-packages/db1-20251007-abc123.tar.gz
```

## Advanced Usage

### Generate with Custom Filters

Exclude certain log types:

```bash
redisctl enterprise support-package database \
  --database-id 1 \
  --file /tmp/filtered-support.tar.gz
```

### Automated Uploads

Schedule regular support package uploads:

```bash
#!/bin/bash
# upload-support-package.sh

DATE=$(date +%Y%m%d-%H%M%S)
DB_ID=$1

redisctl enterprise support-package database \
  --database-id "$DB_ID" \
  --optimize \
  --upload \
  --no-save \
  -o json | tee /var/log/support-upload-$DATE.log
```

Run via cron:
```bash
# Daily at 2 AM for database 1
0 2 * * * /usr/local/bin/upload-support-package.sh 1
```

### Share with Redis Support

Generate and get sharable link:

```bash
RESULT=$(redisctl enterprise support-package cluster \
  --optimize \
  --upload \
  --no-save \
  -o json)

URL=$(echo "$RESULT" | jq -r '.upload_url')
echo "Share this URL with Redis Support:"
echo "$URL"
```

## Common Issues

### Package Generation Times Out

```
Error: Support package generation timed out
```

**Solution:** Use optimize flag to reduce generation time:
```bash
redisctl enterprise support-package cluster \
  --optimize \
  --file /tmp/support.tar.gz
```

### Upload Fails

```
Error: Failed to upload to Files.com: 401 Unauthorized
```

**Solution:** Verify API key:
```bash
# Check current configuration
redisctl files-key get

# Re-enter API key
redisctl files-key set --use-keyring
```

### Insufficient Disk Space

```
Error: Not enough disk space
```

**Solution:** Use `--optimize` or clean up old packages:
```bash
# Find old packages
find /tmp -name "*support*.tar.gz" -mtime +7

# Use optimization
redisctl enterprise support-package cluster \
  --optimize \
  --file /tmp/support.tar.gz
```

### Database Not Found

```
Error: Database with ID 999 not found
```

**Solution:** List available databases:
```bash
redisctl enterprise database list -o table -q '[].{id: uid, name: name}'
```

## Package Size Reference

Typical sizes (uncompressed / compressed):

| Scope | Uncompressed | Compressed | Optimized |
|-------|--------------|------------|-----------|
| Single small DB | 100-200 MB | 40-80 MB | 15-30 MB |
| Single large DB | 500 MB-2 GB | 200-800 MB | 50-200 MB |
| Entire cluster | 1-10 GB | 500 MB-3 GB | 200 MB-1 GB |

## What's Inside?

A support package typically contains:

```
support-package/
├── cluster/
│   ├── cluster-config.json
│   ├── cluster-logs/
│   └── cluster-stats.json
├── databases/
│   ├── db-1/
│   │   ├── config.json
│   │   ├── stats.json
│   │   └── redis-logs/
│   └── db-2/...
├── nodes/
│   ├── node-1/
│   │   ├── system-info.json
│   │   ├── network-config.json
│   │   └── logs/
│   └── node-2/...
└── metadata.json
```

## Next Steps

- 📊 [Monitor Cluster Health](../../enterprise/monitoring/cluster-health.md) - Proactive monitoring
- 🔍 [Troubleshooting Guide](../../guides/troubleshooting.md) - Common issues and solutions
- 🛠️ [Node Management](../../enterprise/operations/node-management.md) - Manage cluster nodes

## See Also

- [Support Package Command Reference](../../enterprise/operations/support-package.md) - Complete command documentation
- [Files.com Integration Guide](../../common-features/secure-storage.md#filescom-integration) - API key management
- [Redis Enterprise Support](https://redis.io/support/) - Contact Redis support
