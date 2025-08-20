# CLI Commands Reference

The Redis Enterprise CLI provides comprehensive commands for managing all aspects of your Redis Enterprise cluster. All commands support multiple output formats (JSON, YAML, Table) and JMESPath queries for filtering results.

## Global Options

These options can be used with any command:

```bash
--url <URL>              # Redis Enterprise cluster URL
--username <USERNAME>    # Username for authentication
--password <PASSWORD>    # Password for authentication
--profile <PROFILE>      # Use a saved configuration profile
--insecure              # Allow insecure TLS connections
--output <FORMAT>       # Output format: json, yaml, or table
--query <JMESPATH>      # JMESPath query to filter/transform output
--verbose               # Enable verbose logging
```

## Cluster Commands

Manage cluster configuration and monitor cluster health.

### cluster info
Get comprehensive cluster information including status, nodes, and databases.

```bash
redis-enterprise cluster info
redis-enterprise cluster info --query 'name'
redis-enterprise cluster info --output yaml
```

### cluster stats
Get cluster performance statistics with optional time intervals.

```bash
redis-enterprise cluster stats
redis-enterprise cluster stats --interval 1h
redis-enterprise cluster stats --query 'total_ops_per_sec'
```

### cluster update
Update cluster configuration settings.

```bash
redis-enterprise cluster update --name "Production Cluster"
redis-enterprise cluster update --from-json cluster_config.json
```

## Database Commands

Complete database lifecycle management with advanced configuration options.

### database list
List all databases with optional filtering and sorting.

```bash
redis-enterprise database list
redis-enterprise database list --status active
redis-enterprise database list --sort memory
redis-enterprise database list --query '[?status==`active`].name'
```

### database get
Get detailed information about a specific database.

```bash
redis-enterprise database get mydb
redis-enterprise database get 1
redis-enterprise database get mydb --query 'memory_size'
```

### database create
Create a new database with specified configuration.

```bash
# Basic creation
redis-enterprise database create --name mydb --memory 1GB

# Advanced creation with all options
redis-enterprise database create \
  --name production-db \
  --memory 4GB \
  --port 12000 \
  --replication \
  --persistence aof \
  --eviction-policy allkeys-lru \
  --shards 4

# Create from JSON configuration
redis-enterprise database create --from-json database_config.json
```

### database update
Update existing database configuration.

```bash
redis-enterprise database update mydb --memory 2GB
redis-enterprise database update mydb --eviction-policy volatile-lru
redis-enterprise database update mydb --from-json updates.json
```

### database delete
Delete a database with confirmation.

```bash
redis-enterprise database delete mydb
redis-enterprise database delete mydb --yes  # Skip confirmation
```

### database stats
Get database performance statistics.

```bash
redis-enterprise database stats mydb
redis-enterprise database stats mydb --interval 1h
redis-enterprise database stats mydb --query 'ops_per_sec'
```

### database wait
Wait for a database to reach a specific status.

```bash
redis-enterprise database wait mydb --status active --timeout 300
redis-enterprise database wait new-db --status active --timeout 60
```

## Node Commands

Monitor and manage cluster nodes.

### node list
List all nodes in the cluster.

```bash
redis-enterprise node list
redis-enterprise node list --query '[].{id:id,status:status,role:role}'
```

### node get
Get detailed information about a specific node.

```bash
redis-enterprise node get 1
redis-enterprise node get 2 --query 'total_memory'
```

### node stats
Get node performance statistics.

```bash
redis-enterprise node stats 1
redis-enterprise node stats 2 --output table
```

## User Commands

Manage user accounts and permissions.

### user list
List all users in the cluster.

```bash
redis-enterprise user list
redis-enterprise user list --query '[?role==`admin`].email'
```

### user get
Get detailed information about a specific user.

```bash
redis-enterprise user get 1
redis-enterprise user get 5 --query 'role'
```

### user create
Create a new user account.

```bash
redis-enterprise user create --email user@company.com --password secure123
redis-enterprise user create --email admin@company.com --password admin123 --role admin
```

## Bootstrap Commands

Initialize and monitor cluster bootstrap process.

### bootstrap status
Check the current bootstrap status of the cluster.

```bash
redis-enterprise bootstrap status
redis-enterprise bootstrap status --query 'status'
```

### bootstrap raw
Send raw bootstrap configuration to initialize cluster.

```bash
redis-enterprise bootstrap raw --body '{"action":"create_cluster","cluster":{"name":"my-cluster"}}'
redis-enterprise bootstrap raw --from-json bootstrap_config.json
```

## Module Commands

Manage Redis modules in the cluster.

### module list
List all available modules.

```bash
redis-enterprise module list
redis-enterprise module list --query '[].{name:module_name,version:version}'
```

### module get
Get information about a specific module.

```bash
redis-enterprise module get search
redis-enterprise module get json --query 'version'
```

### module upload
Upload a new module to the cluster.

```bash
redis-enterprise module upload --file redisgraph.zip
redis-enterprise module upload --file custom-module.zip
```

### module update
Update module configuration.

```bash
redis-enterprise module update search --description "Full-text search module"
redis-enterprise module update graph --from-json module_config.json
```

### module delete
Delete a module from the cluster.

```bash
redis-enterprise module delete graph
redis-enterprise module delete custom-module --yes
```

## Role Commands

Manage roles and access control lists (ACLs).

### role list
List all roles in the cluster.

```bash
redis-enterprise role list
redis-enterprise role list --include-builtin
redis-enterprise role list --query '[?management==`admin`].name'
```

### role get
Get detailed information about a specific role.

```bash
redis-enterprise role get admin
redis-enterprise role get custom-role --query 'permissions'
```

### role create
Create a new custom role.

```bash
# Create with CLI arguments
redis-enterprise role create \
  --name db-admin \
  --management admin \
  --data-access read-write

# Create from JSON configuration
redis-enterprise role create --from-json role_config.json
```

### role update
Update an existing role.

```bash
redis-enterprise role update db-admin --data-access read-only
redis-enterprise role update custom-role --from-json updates.json
```

### role delete
Delete a custom role.

```bash
redis-enterprise role delete custom-role
redis-enterprise role delete old-role --yes
```

### role users
List users assigned to a specific role.

```bash
redis-enterprise role users admin
redis-enterprise role users db-admin --query '[].email'
```

## License Commands

Manage cluster licensing.

### license get
Get current license information.

```bash
redis-enterprise license get
redis-enterprise license get --query 'expiration_date'
```

### license usage
Check current resource usage against license limits.

```bash
redis-enterprise license usage
redis-enterprise license usage --query 'shards_used'
```

### license update
Update the cluster license.

```bash
redis-enterprise license update --key "LICENSE-KEY-STRING"
redis-enterprise license update --file license.key
```

### license validate
Validate a license key before installation.

```bash
redis-enterprise license validate --key "NEW-LICENSE-KEY"
redis-enterprise license validate --file new_license.key
```

### license cluster
Get license information from cluster configuration.

```bash
redis-enterprise license cluster
```

## Configuration Commands

Manage CLI configuration profiles.

### config list
List all saved configuration profiles.

```bash
redis-enterprise config list
```

### config set
Create or update a configuration profile.

```bash
redis-enterprise config set --profile production --url https://prod.redis.com:9443
redis-enterprise config set --profile staging --username admin@staging.com --insecure
```

### config get
Display a specific configuration profile.

```bash
redis-enterprise config get --profile production
redis-enterprise config get --profile default
```

### config remove
Remove a configuration profile.

```bash
redis-enterprise config remove --profile old-cluster
```

## API Commands

Direct access to REST API endpoints for advanced operations.

### api get
Send a GET request to any API endpoint.

```bash
redis-enterprise api get /v1/cluster
redis-enterprise api get /v1/bdbs
redis-enterprise api get /v1/nodes/1
```

### api post
Send a POST request with JSON body.

```bash
redis-enterprise api post /v1/bdbs --body '{"name":"test","memory_size":1073741824}'
redis-enterprise api post /v1/users --from-json user.json
```

### api put
Send a PUT request to update resources.

```bash
redis-enterprise api put /v1/bdbs/1 --body '{"memory_size":2147483648}'
redis-enterprise api put /v1/cluster --from-json cluster_update.json
```

### api delete
Send a DELETE request to remove resources.

```bash
redis-enterprise api delete /v1/bdbs/5
redis-enterprise api delete /v1/users/10
```

## Workflow Commands

High-level commands for common operations.

### workflow init-cluster
Initialize a new Redis Enterprise cluster.

```bash
redis-enterprise workflow init-cluster \
  --name "Production Cluster" \
  --username admin@redis.local \
  --password secure_password \
  --accept-eula
```

### workflow create-database
Create optimized databases for specific use cases.

```bash
# Cache database
redis-enterprise workflow create-database --name cache-db --db-type cache

# Search database with RediSearch
redis-enterprise workflow create-database --name search-db --db-type search

# Time-series database
redis-enterprise workflow create-database --name metrics-db --db-type timeseries

# Persistent database with AOF
redis-enterprise workflow create-database --name persistent-db --db-type persistent
```

## Output Examples

### JSON Output (Default)
```json
{
  "uid": 1,
  "name": "my-database",
  "status": "active",
  "memory_size": 1073741824
}
```

### YAML Output
```yaml
uid: 1
name: my-database
status: active
memory_size: 1073741824
```

### Table Output
```
+-----+-------------+--------+--------------+
| UID | Name        | Status | Memory Size  |
+-----+-------------+--------+--------------+
| 1   | my-database | active | 1073741824   |
+-----+-------------+--------+--------------+
```

## JMESPath Query Examples

Filter and transform output using JMESPath queries:

```bash
# Get only database names
redis-enterprise database list --query '[].name'

# Filter active databases
redis-enterprise database list --query '[?status==`active`]'

# Custom object structure
redis-enterprise database list --query '[].{name:name,port:port,memory:memory_size}'

# Get first item
redis-enterprise node list --query '[0]'

# Sort and limit
redis-enterprise database list --query 'sort_by(@, &memory_size) | [:3]'
```

## Environment Variables

Configure default values using environment variables:

```bash
export REDIS_ENTERPRISE_URL="https://cluster.redis.com:9443"
export REDIS_ENTERPRISE_USER="admin@company.com"
export REDIS_ENTERPRISE_PASSWORD="secure_password"
export REDIS_ENTERPRISE_PROFILE="production"
```

## Exit Codes

The CLI uses standard exit codes:

- `0`: Success
- `1`: General error
- `2`: Authentication failure
- `3`: Resource not found
- `4`: Validation error
- `5`: Timeout

## Tips and Best Practices

1. **Use Profiles**: Save cluster credentials in profiles for easy switching
2. **Query Filtering**: Use JMESPath queries to reduce output and focus on needed data
3. **JSON Configuration**: Use JSON files for complex configurations
4. **Automation**: Use `--yes` flags and exit codes for scripting
5. **Verbose Mode**: Use `--verbose` for debugging connection issues
6. **Wait Commands**: Use wait commands in scripts to ensure operations complete