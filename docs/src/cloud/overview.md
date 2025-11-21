# Redis Cloud Overview

Redis Cloud is Redis's fully managed database service. redisctl provides complete CLI access to the Redis Cloud API.

## Three-Tier Access

### 1. API Layer
Direct REST access for scripting and automation:
```bash
redisctl api cloud get /subscriptions
redisctl api cloud post /subscriptions -d @subscription.json
```

### 2. Commands
Human-friendly commands for day-to-day operations:
```bash
redisctl cloud subscription list
redisctl cloud database create --subscription 123 --data @db.json --wait
```

### 3. Workflows
Multi-step operations:
```bash
redisctl cloud workflow subscription-setup --name prod --region us-east-1
```

## Key Concepts

### Subscriptions
Subscriptions are the top-level container for databases. They define:
- Cloud provider (AWS, GCP, Azure)
- Region
- Memory allocation
- Networking configuration

### Databases
Databases run within subscriptions. Each database has:
- Memory limit
- Modules (RedisJSON, RediSearch, etc.)
- Persistence settings
- Access credentials

### Tasks
Most operations are async and return task IDs. Use `--wait` to block until completion.

## Authentication

Redis Cloud uses API key authentication:

```bash
# Environment variables
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_SECRET_KEY="your-secret"

# Or profile
redisctl profile set cloud --deployment-type cloud --api-key "..." --api-secret "..."
```

Get your API keys from [app.redislabs.com](https://app.redislabs.com) → Account Settings → API Keys.

## Quick Examples

```bash
# List subscriptions
redisctl cloud subscription list -o table

# Create database and wait
redisctl cloud database create \
  --subscription 123456 \
  --data '{"name": "cache", "memoryLimitInGb": 1}' \
  --wait

# Get database connection info
redisctl cloud database get 123456:789 \
  -q '{endpoint: publicEndpoint, password: password}'

# Set up VPC peering
redisctl cloud vpc-peering create \
  --subscription 123456 \
  --data @peering.json \
  --wait
```

## Command Groups

- **[Databases](./commands/databases.md)** - Create, update, delete databases
- **[Subscriptions](./commands/subscriptions.md)** - Manage subscriptions
- **[Access Control](./commands/access-control.md)** - Users, roles, ACLs
- **[Networking](./commands/networking.md)** - VPC, PSC, Transit Gateway
- **[Tasks](./commands/tasks.md)** - Monitor async operations

## Next Steps

- [API Layer](./api.md) - Direct REST access
- [Workflows](./workflows.md) - Multi-step operations
- [Cloud Cookbook](../cookbook/README.md) - Practical recipes
