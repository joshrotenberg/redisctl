# Quick Start

This guide will get you running your first commands in 5 minutes.

## Redis Cloud Quick Start

### Step 1: Get Your API Credentials

1. Log in to [Redis Cloud Console](https://app.redislabs.com)
2. Go to **Account Settings** → **Access Management** → **API Keys**
3. Click **Generate API Key**
4. Copy both the **API Key** and **Secret Key**

### Step 2: Configure redisctl

Choose your preferred method:

#### Environment Variables (Quick)

```bash
export REDIS_CLOUD_API_KEY="your-api-key-here"
export REDIS_CLOUD_API_SECRET="your-secret-key-here"
```

#### Profile with Secure Storage (Recommended)

```bash
redisctl profile set production \
  --deployment-type cloud \
  --api-key "your-api-key" \
  --api-secret "your-secret-key" \
  --use-keyring
```

Using `--use-keyring` stores credentials in your OS keychain.

### Step 3: Test Connection

```bash
# Quick test
redisctl api cloud get /

# List subscriptions
redisctl cloud subscription list -o table
```

### Step 4: Common Operations

```bash
# List databases
redisctl cloud database list --subscription-id 123456

# Create a database with auto-wait
redisctl cloud database create \
  --subscription-id 123456 \
  --data '{"name": "my-db", "memoryLimitInGb": 1}' \
  --wait

# Get database details
redisctl cloud database get 123456 789 \
  -q '{name: name, status: status}'
```

---

## Redis Enterprise Quick Start

### Step 1: Get Your Cluster Credentials

For a fresh Redis Enterprise installation:
- **URL:** `https://cluster-fqdn:9443`
- **Username:** Configured during setup (often `admin@cluster.local`)
- **Password:** Set during cluster bootstrap

### Step 2: Configure redisctl

#### Environment Variables

```bash
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"

# For self-signed certificates
export REDIS_ENTERPRISE_INSECURE="true"
```

#### Profile Configuration

```bash
redisctl profile set production \
  --deployment-type enterprise \
  --url "https://cluster.example.com:9443" \
  --username "admin@cluster.local" \
  --use-keyring
```

### Step 3: Test Connection

```bash
# Get cluster info
redisctl enterprise cluster get -o json -q 'name'

# List databases
redisctl enterprise database list -o table
```

### Step 4: Common Operations

```bash
# Get cluster status
redisctl enterprise cluster get

# Create a database
redisctl enterprise database create \
  --name "my-app-db" \
  --memory-size "1GB" \
  --port 12000

# Generate support package
redisctl enterprise support-package cluster \
  --output ./support-package.tar.gz

# With optimization (20-30% smaller)
redisctl enterprise support-package cluster \
  --optimize \
  --output ./support-package.tar.gz
```

### Use Workflows

For a fresh installation:

```bash
redisctl enterprise workflow init-cluster \
  --cluster-name "production-cluster" \
  --username "admin@cluster.local" \
  --password "YourSecurePassword"
```

This handles bootstrap, auth, license, and creates a default database.

---

## Advanced Features

### Output Formats

```bash
# JSON (default)
redisctl enterprise database list

# Table
redisctl enterprise database list -o table

# YAML
redisctl enterprise database list -o yaml
```

### JMESPath Queries

```bash
# Get only names
redisctl enterprise database list -q "[].name"

# Active databases with specific fields
redisctl enterprise database list \
  -q "[?status=='active'].{name:name,memory:memory_size}"
```

### Raw API Access

```bash
# Any Cloud endpoint
redisctl api cloud get /subscriptions

# Any Enterprise endpoint
redisctl api enterprise get /v1/cluster
```

---

## Troubleshooting

### 401 Unauthorized

Check your credentials:

```bash
redisctl profile get
echo $REDIS_CLOUD_API_KEY
```

### Connection Refused

Verify cluster URL and network:

```bash
curl -k https://cluster.example.com:9443/v1/cluster
ping cluster.example.com
```

### Profile Not Found

List and create profiles:

```bash
redisctl profile list
redisctl profile set default --deployment-type cloud
```

---

## Next Steps

- [Cookbook Recipes](../cookbook/README.md) - Practical examples
- [Redis Cloud Guide](../cloud/overview.md) - Cloud operations
- [Redis Enterprise Guide](../enterprise/overview.md) - Enterprise operations
- [Configuration Guide](./configuration.md) - Advanced profiles
