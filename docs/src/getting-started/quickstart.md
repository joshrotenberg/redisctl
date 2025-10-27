# Quick Start

⏱️ **Time to first command:** 5 minutes

This guide will get you running redisctl commands quickly.

```admonish tip title="Choose Your Path"
Pick the section that matches what you're managing:
- **[Redis Cloud](#redis-cloud-quick-start)** - Cloud subscriptions and databases
- **[Redis Enterprise](#redis-enterprise-quick-start)** - On-premise clusters
```

---

## Redis Cloud Quick Start

### Step 1: Get Your API Credentials

```admonish info title="Finding Your API Keys"
1. Log in to [Redis Cloud Console](https://app.redislabs.com)
2. Go to **Account Settings** → **Access Management** → **API Keys**
3. Click **Generate API Key**
4. Copy both the **API Key** and **Secret Key**
```

### Step 2: Configure redisctl

Choose your preferred method:

#### Option A: Environment Variables (Quick)

```bash
export REDIS_CLOUD_API_KEY="your-api-key-here"
export REDIS_CLOUD_API_SECRET="your-secret-key-here"
```

```admonish warning
Environment variables are convenient but credentials remain in shell history. For production use, consider using profiles with keyring storage.
```

#### Option B: Profile with Secure Storage (Recommended)

```bash
# Store credentials in OS keyring
redisctl profile set production \
  --deployment-type cloud \
  --api-key "your-api-key" \
  --api-secret "your-secret-key" \
  --use-keyring

# Verify it works
redisctl profile get production
```

```admonish success title="Secure by Default"
Using `--use-keyring` stores credentials in your operating system's secure credential store (Keychain on macOS, Credential Manager on Windows, Secret Service on Linux).
```

### Step 3: Test Connection

```bash
# Quick connection test
redisctl api cloud get /

# List your subscriptions
redisctl cloud subscription list -o table
```

```admonish example title="Expected Output"
\`\`\`
┌────────┬─────────────────┬────────┬──────────┐
│ ID     │ Name            │ Status │ Provider │
├────────┼─────────────────┼────────┼──────────┤
│ 123456 │ my-subscription │ active │ AWS      │
└────────┴─────────────────┴────────┴──────────┘
\`\`\`
```

### Step 4: Common Cloud Operations

#### List Databases

```bash
# All databases in a subscription
redisctl cloud database list --subscription-id 123456

# Filter active databases only
redisctl cloud database list \
  --subscription-id 123456 \
  -q "[?status=='active']"
```

#### Create a Database

```bash
# Create with automatic wait for completion
redisctl cloud database create \
  --subscription-id 123456 \
  --data '{
    "name": "my-database",
    "memoryLimitInGb": 1,
    "protocol": "redis"
  }' \
  --wait
```

```admonish tip title="The --wait Flag"
Adding `--wait` automatically polls the operation until completion. No more manual polling loops!
```

#### Get Database Details

```bash
# JSON output with specific fields
redisctl cloud database get 123456 789 \
  -q '{name: name, status: status, endpoint: publicEndpoint}'
```

### Next Steps for Cloud Users

```admonish info title="Learn More"
- 📖 [Create Your First Database](../cookbook/cloud/create-first-database.md) - Detailed walkthrough
- 🔐 [Configure ACL Security](../cookbook/cloud/configure-acls.md) - Set up access control
- 🌐 [Setup VPC Peering](../cookbook/cloud/setup-vpc-peering.md) - Private connectivity
- 📚 [All Cloud Commands](../cloud/commands.md) - Complete reference
```

---

## Redis Enterprise Quick Start

### Step 1: Get Your Cluster Credentials

```admonish info title="Default Credentials"
For a fresh Redis Enterprise installation:
- **URL:** `https://cluster-fqdn:9443`
- **Username:** Configured during setup (often `admin@cluster.local`)
- **Password:** Set during cluster bootstrap
```

### Step 2: Configure redisctl

#### Option A: Environment Variables

```bash
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"

# For self-signed certificates
export REDIS_ENTERPRISE_INSECURE="true"
```

```admonish warning title="SSL Verification"
Only use `REDIS_ENTERPRISE_INSECURE=true` for testing. In production, configure proper SSL certificates.
```

#### Option B: Profile Configuration

```bash
# Create a profile
redisctl profile set production \
  --deployment-type enterprise \
  --url "https://cluster.example.com:9443" \
  --username "admin@cluster.local" \
  --use-keyring

# Enter password when prompted (stored securely)
```

### Step 3: Test Connection

```bash
# Get cluster information
redisctl enterprise cluster get -o json -q 'name'

# List databases
redisctl enterprise database list -o table
```

```admonish example title="Expected Output"
\`\`\`
┌────┬──────────────┬────────┬─────────────┐
│ ID │ Name         │ Status │ Memory (GB) │
├────┼──────────────┼────────┼─────────────┤
│ 1  │ default-db   │ active │ 1.0         │
│ 2  │ cache-db     │ active │ 0.5         │
└────┴──────────────┴────────┴─────────────┘
\`\`\`
```

### Step 4: Common Enterprise Operations

#### Get Cluster Status

```bash
# Cluster health overview
redisctl enterprise cluster get

# Node status
redisctl enterprise node list -o table
```

#### Create a Database

```bash
# Create a simple database
redisctl enterprise database create \
  --name "my-app-db" \
  --memory-size "1GB" \
  --port 12000
```

#### Generate Support Package

```admonish tip title="Support Package Magic"
This is one of redisctl's most powerful features. What used to take 10+ minutes of clicking and uploading now takes 30 seconds.
```

```bash
# Generate and save locally
redisctl enterprise support-package cluster \
  --output ./support-package.tar.gz

# Generate with compression (20-30% smaller)
redisctl enterprise support-package cluster \
  --optimize \
  --output ./support-package.tar.gz

# Generate and upload directly to Redis Support
export REDIS_ENTERPRISE_FILES_API_KEY="your-files-api-key"
redisctl enterprise support-package cluster \
  --optimize \
  --upload
```

### Step 5: Use Workflows

```admonish info title="Workflows = Multi-Step Operations"
Workflows handle complex operations that require multiple API calls and waiting for completions.
```

#### Initialize a New Cluster

For a fresh Redis Enterprise installation:

```bash
redisctl enterprise workflow init-cluster \
  --cluster-name "production-cluster" \
  --username "admin@cluster.local" \
  --password "YourSecurePassword"
```

This workflow will:
1. ✅ Bootstrap the cluster
2. ✅ Configure authentication
3. ✅ Accept license terms
4. ✅ Create default database
5. ✅ Verify everything works

### Next Steps for Enterprise Users

```admonish info title="Learn More"
- 📖 [Create a Database](../cookbook/enterprise/create-database.md) - Detailed guide
- 🏥 [Cluster Health Check](../cookbook/enterprise/cluster-health.md) - Monitoring
- 🔧 [Configure Replication](../cookbook/enterprise/configure-replication.md) - High availability
- 📦 [Support Package Guide](../cookbook/enterprise/support-package.md) - Automation
- 📚 [All Enterprise Commands](../enterprise/commands.md) - Complete reference
```

---

## Advanced Features

### Output Formats

```bash
# JSON (default) - great for piping to jq
redisctl enterprise database list

# Table - human-readable
redisctl enterprise database list -o table

# YAML - configuration files
redisctl enterprise database list -o yaml
```

### JMESPath Queries

Filter and transform output with powerful queries:

```bash
# Get only database names
redisctl enterprise database list -q "[].name"

# Get active databases with specific fields
redisctl enterprise database list \
  -q "[?status=='active'].{name:name,memory:memory_size}"

# Count databases
redisctl enterprise database list -q "length(@)"
```

```admonish tip title="JMESPath is Powerful"
Learn more at [jmespath.org](https://jmespath.org) - it works with any JSON output.
```

### Raw API Access

Need something not covered by human-friendly commands?

```bash
# Any Cloud endpoint
redisctl api cloud get /subscriptions
redisctl api cloud post /subscriptions/123/databases --data '{...}'

# Any Enterprise endpoint
redisctl api enterprise get /v1/cluster
redisctl api enterprise get /v1/bdbs/1
```

```admonish info title="Four-Layer Architecture"
1. **Raw API** - Direct REST access (use `api` command)
2. **Human-Friendly** - Typed commands (use `cloud`/`enterprise` commands)
3. **Workflows** - Multi-step operations (use `workflow` command)
4. **Support Tools** - Specialized utilities (use `support-package` command)

Choose the layer that fits your need!
```

---

## Troubleshooting

### Connection Issues

```admonish warning title="401 Unauthorized"
**Problem:** Invalid API credentials

**Solution:**
\`\`\`bash
# Check your current profile
redisctl profile get

# Verify environment variables
echo $REDIS_CLOUD_API_KEY
echo $REDIS_ENTERPRISE_URL

# Test with explicit credentials
redisctl api cloud get / --api-key "your-key" --api-secret "your-secret"
\`\`\`
```

```admonish warning title="Connection Refused"
**Problem:** Can't reach Redis Enterprise cluster

**Solution:**
\`\`\`bash
# Verify URL and port
curl -k https://cluster.example.com:9443/v1/cluster

# Check network connectivity
ping cluster.example.com

# Verify SSL settings
export REDIS_ENTERPRISE_INSECURE="true"  # For self-signed certs
\`\`\`
```

### Profile Issues

```admonish warning title="Profile Not Found"
**Problem:** No default profile configured

**Solution:**
\`\`\`bash
# List all profiles
redisctl profile list

# Set a default profile
redisctl profile set default --deployment-type cloud

# Or use explicit profile
redisctl --profile production cloud database list
\`\`\`
```

---

## What's Next?

```admonish success title="You're Ready!"
You've learned the basics of redisctl. Now explore deeper:

📚 **Learn by Example**
- [Cookbook Recipes](../cookbook/README.md) - Copy-paste ready examples

🔍 **Deep Dives**
- [Redis Cloud Guide](../cloud/overview.md) - Cloud operations
- [Redis Enterprise Guide](../enterprise/overview.md) - Enterprise operations

🛠️ **Advanced**
- [Configuration Guide](./configuration.md) - Profile management
- [Library Usage](../developer/library-usage.md) - Use in your own code
```
