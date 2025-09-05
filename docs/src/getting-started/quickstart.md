# Quick Start

This guide will get you running your first commands in 5 minutes.

## Step 1: Configure Credentials

Choose one method:

### Option A: Environment Variables (Quickest)

```bash
# For Redis Cloud
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_API_SECRET="your-secret"

# For Redis Enterprise
export REDIS_ENTERPRISE_URL="https://cluster:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"
```

### Option B: Configuration File

Create `~/.config/redisctl/config.toml`:

```toml
[profiles.default]
deployment_type = "cloud"  # or "enterprise"
api_key = "your-key"
api_secret = "your-secret"
```

## Step 2: Test Connection

```bash
# For Cloud
redisctl api cloud get /account

# For Enterprise
redisctl api enterprise get /v1/cluster
```

## Step 3: Common Commands

### List Resources

```bash
# List all databases
redisctl database list

# List in table format
redisctl database list -o table

# Filter active databases only
redisctl database list -q "[?status=='active']"
```

### Get Details

```bash
# Get database details
redisctl database get 12345

# Get as YAML
redisctl database get 12345 -o yaml
```

### Direct API Access

```bash
# Any Cloud API endpoint
redisctl api cloud get /subscriptions
redisctl api cloud get /subscriptions/12345/databases

# Any Enterprise API endpoint
redisctl api enterprise get /v1/bdbs
redisctl api enterprise get /v1/nodes
```

## Step 4: Explore More

### Cloud Operations

```bash
# Cloud-specific commands
redisctl cloud subscription list
redisctl cloud database list --subscription-id 12345
```

### Enterprise Operations

```bash
# Enterprise-specific commands
redisctl enterprise cluster info
redisctl enterprise database list
redisctl enterprise node list
```

## Output Options

```bash
# JSON (default)
redisctl database list

# Table format
redisctl database list -o table

# YAML
redisctl database list -o yaml

# Filter with JMESPath
redisctl database list -q "[].{name:name,memory:memory_size}"
```

## What's Next?

- [Redis Cloud Guide](../cloud/overview.md) - Cloud-specific operations
- [Redis Enterprise Guide](../enterprise/overview.md) - Enterprise-specific operations
- [Examples](../cloud/examples.md) - More detailed examples