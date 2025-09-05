# Redis Cloud Overview

Redis Cloud is a fully-managed database-as-a-service offering. `redisctl` provides comprehensive access to the Redis Cloud REST API.

## Authentication

Redis Cloud uses API key authentication:

```bash
# Set credentials
export REDIS_CLOUD_API_KEY="your-account-key"
export REDIS_CLOUD_API_SECRET="your-secret-key"

# Test connection
redisctl api cloud get /account
```

## Command Structure

Redis Cloud commands follow this pattern:

```
redisctl cloud <resource> <action> [options]
```

Resources include:
- `subscription` - Manage subscriptions
- `database` - Manage databases
- `account` - Account information
- `user` - User management
- `acl` - Access control lists
- `backup` - Backup operations

## Common Operations

```bash
# List all subscriptions
redisctl cloud subscription list

# Get subscription details
redisctl cloud subscription get 12345

# List databases in a subscription
redisctl cloud database list --subscription-id 12345

# Get database details
redisctl cloud database get --subscription-id 12345 --database-id 67890
```

## Next Steps

- [Human-Friendly Commands](./human-commands.md) - High-level command reference
- [Raw API Access](./api-access.md) - Direct API endpoint access
- [Examples](./examples.md) - Real-world usage examples