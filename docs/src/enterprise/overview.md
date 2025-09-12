# Redis Enterprise Overview

Redis Enterprise is a self-managed database platform that can be deployed on-premises or in your cloud account. `redisctl` provides comprehensive access to the Redis Enterprise REST API.

## Authentication

Redis Enterprise uses basic authentication:

```bash
# Set credentials
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"
export REDIS_ENTERPRISE_INSECURE="true"  # For self-signed certificates

# Test connection
redisctl api enterprise get /v1/cluster
```

## Command Structure

Redis Enterprise commands follow this pattern:

```
redisctl enterprise <resource> <action> [options]
```

Resources include:
- `cluster` - Cluster management
- `database` - Database operations
- `node` - Node management
- `user` - User management
- `role` - Role-based access control
- `alert` - Alert configuration
- `workflow` - Multi-step automated operations

## Common Operations

```bash
# Get cluster information
redisctl enterprise cluster info

# List all databases
redisctl enterprise database list

# Get database details
redisctl enterprise database get 1

# List nodes
redisctl enterprise node list

# Initialize a new cluster (workflow)
redisctl enterprise workflow init-cluster \
  --username "admin@cluster.local" \
  --password "SecurePassword"
```

## Next Steps

- [Human-Friendly Commands](./human-commands.md) - High-level command reference
- [Workflows](./workflows.md) - Automated multi-step operations
- [Raw API Access](./api-access.md) - Direct API endpoint access
- [Examples](./examples.md) - Real-world usage examples