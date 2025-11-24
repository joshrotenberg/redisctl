# Redis Enterprise Overview

Redis Enterprise is Redis's self-managed database platform for on-premises or cloud deployments. redisctl provides complete CLI access to the REST API.

## Three-Tier Access

### 1. API Layer
Direct REST access for scripting and automation:
```bash
redisctl api enterprise get /v1/cluster
redisctl api enterprise post /v1/bdbs -d @database.json
```

### 2. Commands
Human-friendly commands for day-to-day operations:
```bash
redisctl enterprise cluster get
redisctl enterprise database create --name mydb --memory-size 1073741824
```

### 3. Workflows
Multi-step operations:
```bash
redisctl enterprise workflow init-cluster --name prod --nodes 3
```

## Key Concepts

### Cluster
The cluster is the top-level container that spans multiple nodes. It manages:
- Node membership
- Resource allocation
- Policies and certificates
- License

### Nodes
Physical or virtual machines running Redis Enterprise. Each node provides:
- CPU and memory resources
- Network connectivity
- Storage for persistence

### Databases (BDBs)
Databases run across the cluster. Each database has:
- Memory allocation
- Sharding configuration
- Replication settings
- Modules (RedisJSON, RediSearch, etc.)

## Authentication

Redis Enterprise uses basic authentication:

```bash
# Environment variables
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"
export REDIS_ENTERPRISE_INSECURE="true"  # for self-signed certs

# Or profile
redisctl profile set enterprise \
  --deployment-type enterprise \
  --url "https://cluster.example.com:9443" \
  --username "admin@cluster.local" \
  --password "your-password" \
  --insecure
```

## Quick Examples

```bash
# Get cluster info
redisctl enterprise cluster get

# List databases
redisctl enterprise database list -o table

# Create database
redisctl enterprise database create \
  --name cache \
  --memory-size 1073741824

# Stream cluster stats
redisctl enterprise stats cluster --follow

# Generate support package
redisctl enterprise support-package cluster --upload
```

## Command Groups

- **[Cluster](./commands/cluster.md)** - Cluster config, certs, policies
- **[Databases](./commands/databases.md)** - Create, update, delete databases
- **[Nodes](./commands/nodes.md)** - Manage cluster nodes
- **[Access Control](./commands/access-control.md)** - Users, roles, LDAP
- **[Monitoring](./commands/monitoring.md)** - Stats, logs, alerts
- **[Active-Active](./commands/active-active.md)** - CRDB operations

## Operations

Special tools for cluster management:
- **[Support Package](./operations/support-package.md)** - Diagnostic packages
- **[License](./operations/license.md)** - License management
- **[Debug Info](./operations/debuginfo.md)** - Detailed diagnostics
- **[Diagnostics](./operations/diagnostics.md)** - Health checks
- **[Migrations](./operations/migration.md)** - Data migrations

## Next Steps

- [API Layer](./api.md) - Direct REST access
- [Workflows](./workflows.md) - Multi-step operations
- [Enterprise Cookbook](../cookbook/README.md) - Practical recipes
