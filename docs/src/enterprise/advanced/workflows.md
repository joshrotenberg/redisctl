# Enterprise Workflows

Workflows are multi-step operations that automate complex Redis Enterprise management tasks. They combine multiple API calls, handle asynchronous operations, and provide progress feedback.

## Available Workflows

### List Workflows

```bash
# List all available workflows
redisctl enterprise workflow list

# JSON output for scripting
redisctl enterprise workflow list --output json
```

### Initialize Cluster

The `init-cluster` workflow automates the complete setup of a new Redis Enterprise cluster, including bootstrapping and optional database creation.

```bash
# Initialize with default settings
redisctl enterprise workflow init-cluster \
  --username "admin@cluster.local" \
  --password "YourSecurePassword"

# Initialize with custom cluster name and database
redisctl enterprise workflow init-cluster \
  --name "production-cluster" \
  --username "admin@redis.local" \
  --password "YourSecurePassword" \
  --database-name "my-database" \
  --database-memory-gb 2

# Skip database creation
redisctl enterprise workflow init-cluster \
  --username "admin@cluster.local" \
  --password "YourSecurePassword" \
  --skip-database
```

#### Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `--name` | Cluster name | `redis-cluster` |
| `--username` | Admin username | `admin@redis.local` |
| `--password` | Admin password (required) | - |
| `--skip-database` | Skip creating default database | `false` |
| `--database-name` | Name for default database | `default-db` |
| `--database-memory-gb` | Memory size in GB for database | `1` |
| `--wait` | Wait for operations to complete | `true` |
| `--wait-timeout` | Maximum wait time in seconds | `600` |

#### What it does

1. **Checks cluster status** - Verifies if cluster needs initialization
2. **Bootstraps cluster** - Creates cluster with specified name and credentials
3. **Waits for stabilization** - Ensures cluster is ready for operations
4. **Creates database** (optional) - Sets up initial database with specified configuration
5. **Verifies connectivity** - Tests database with PING command

## Output Formats

Workflows support structured output for automation:

```bash
# JSON output
redisctl enterprise workflow init-cluster \
  --username "admin@cluster.local" \
  --password "Redis123" \
  --output json

# YAML output
redisctl enterprise workflow init-cluster \
  --username "admin@cluster.local" \
  --password "Redis123" \
  --output yaml
```

Example JSON output:
```json
{
  "success": true,
  "message": "Cluster initialized successfully",
  "outputs": {
    "cluster_name": "redis-cluster",
    "username": "admin@cluster.local",
    "database_created": true,
    "database_name": "default-db"
  }
}
```

## Docker Development

For testing workflows with Docker:

```bash
# Start Redis Enterprise container
docker compose up -d

# Wait for container to be ready
sleep 10

# Initialize cluster
redisctl enterprise workflow init-cluster \
  --username "admin@cluster.local" \
  --password "Redis123"

# Clean up
docker compose down -v
```

## Environment Variables

Workflows respect standard environment variables:

```bash
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_INSECURE="true"

# Password can be set via environment
export REDIS_ENTERPRISE_INIT_PASSWORD="Redis123"

redisctl enterprise workflow init-cluster \
  --username "admin@cluster.local"
```

## Error Handling

Workflows provide clear error messages and maintain partial progress:

- If cluster is already initialized, workflow reports success without re-bootstrapping
- If database creation fails, cluster remains initialized and can be managed manually
- Network failures include retry logic with configurable timeouts

## Future Workflows

Additional workflows are planned for common operations:

- **upgrade-cluster** - Orchestrate cluster version upgrades
- **backup-restore** - Automated backup and restore operations
- **migrate-database** - Database migration between clusters
- **security-hardening** - Apply security best practices

See the Workflows Feature Guide for architectural details and information about creating custom workflows.