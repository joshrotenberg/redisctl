# CLI Examples

This page provides comprehensive examples demonstrating all features of the Redis Enterprise CLI.

## Cluster Management

### Getting Cluster Information

```bash
# Full cluster information in JSON
redis-enterprise cluster info

# Cluster name only using JMESPath
redis-enterprise cluster info --query 'name'

# Cluster info in YAML format
redis-enterprise cluster info --output yaml

# Cluster statistics
redis-enterprise cluster stats

# Stats for last hour
redis-enterprise cluster stats --interval 1h

# Specific metric with JMESPath
redis-enterprise cluster stats --query 'total_memory'
```

### Updating Cluster Configuration

```bash
# Update cluster name
redis-enterprise cluster update --name "Production Cluster"

# Update from JSON file
redis-enterprise cluster update --from-json cluster_config.json
```

## Database Operations

### Listing Databases

```bash
# List all databases (JSON by default)
redis-enterprise database list

# Table format for human readability
redis-enterprise database list --output table

# Filter active databases only
redis-enterprise database list --status active

# Sort by memory size
redis-enterprise database list --sort memory

# JMESPath query for specific fields
redis-enterprise database list --query '[].{name:name,port:port,status:status}'

# Find databases on specific port
redis-enterprise database list --query '[?port==`12000`].name'

# List databases with memory > 1GB
redis-enterprise database list --query '[?memory_size > `1073741824`].name'
```

### Creating Databases

```bash
# Basic database creation
redis-enterprise database create --name mydb --memory 1GB

# Specify port
redis-enterprise database create --name mydb --memory 1GB --port 12001

# With replication enabled
redis-enterprise database create --name prod-db --memory 2GB --replication

# With persistence (AOF)
redis-enterprise database create --name persistent-db --memory 1GB --persistence aof

# With eviction policy
redis-enterprise database create --name cache-db --memory 500MB --eviction-policy allkeys-lru

# Sharded database
redis-enterprise database create --name sharded-db --memory 4GB --shards 4

# Complete example with all options
redis-enterprise database create \
  --name production-db \
  --memory 4GB \
  --port 12000 \
  --replication \
  --persistence aof \
  --eviction-policy volatile-lru \
  --shards 2 \
  --password MySecretPassword123

# Create from JSON configuration
cat > database_config.json << EOF
{
  "name": "config-db",
  "memory_size": 1073741824,
  "port": 12002,
  "replication": true,
  "persistence": "aof"
}
EOF
redis-enterprise database create --from-json database_config.json
```

### Getting Database Information

```bash
# Get database by name
redis-enterprise database get mydb

# Get database by ID
redis-enterprise database get 1

# Get specific field with JMESPath
redis-enterprise database get mydb --query 'memory_size'

# Get endpoint information
redis-enterprise database get mydb --query 'endpoints[0].addr'

# Output in YAML
redis-enterprise database get mydb --output yaml
```

### Updating Databases

```bash
# Increase memory
redis-enterprise database update mydb --memory 2GB

# Change eviction policy
redis-enterprise database update mydb --eviction-policy volatile-lru

# Enable replication
redis-enterprise database update mydb --replication

# Update from JSON
cat > updates.json << EOF
{
  "memory_size": 2147483648,
  "eviction_policy": "volatile-lru"
}
EOF
redis-enterprise database update mydb --from-json updates.json
```

### Deleting Databases

```bash
# Delete with confirmation prompt
redis-enterprise database delete mydb

# Skip confirmation
redis-enterprise database delete mydb --yes
```

### Database Statistics

```bash
# Current statistics
redis-enterprise database stats mydb

# Stats for last hour
redis-enterprise database stats mydb --interval 1h

# Specific metric
redis-enterprise database stats mydb --query 'ops_per_sec'

# Multiple metrics
redis-enterprise database stats mydb --query '{ops:ops_per_sec,memory:used_memory}'
```

### Waiting for Database Status

```bash
# Wait for database to become active (default 300s timeout)
redis-enterprise database wait mydb --status active

# Custom timeout
redis-enterprise database wait mydb --status active --timeout 60

# Wait for deletion to complete
redis-enterprise database delete mydb --yes
redis-enterprise database wait mydb --status deleted --timeout 120
```

## Using Workflows

### Database Type Presets

```bash
# High-performance cache
redis-enterprise workflow create-database --name cache-db --db-type cache

# Persistent storage
redis-enterprise workflow create-database --name persistent-db --db-type persistent

# Full-text search
redis-enterprise workflow create-database --name search-db --db-type search

# Time-series data
redis-enterprise workflow create-database --name timeseries-db --db-type timeseries

# JSON documents
redis-enterprise workflow create-database --name json-db --db-type json

# Graph database
redis-enterprise workflow create-database --name graph-db --db-type graph
```

### Cluster Initialization

```bash
# Initialize a new cluster
redis-enterprise workflow init-cluster \
  --name "Production Cluster" \
  --username "admin@company.com" \
  --password "SecurePassword123!" \
  --accept-eula
```

## Node Management

### Listing Nodes

```bash
# List all nodes
redis-enterprise node list

# Table format
redis-enterprise node list --output table

# Specific fields with JMESPath
redis-enterprise node list --query '[].{id:uid,role:role,status:status}'

# Find master node
redis-enterprise node list --query '[?role==`master`].uid'
```

### Getting Node Information

```bash
# Get node by ID
redis-enterprise node get 1

# Specific field
redis-enterprise node get 1 --query 'total_memory'

# Node addresses
redis-enterprise node get 1 --query 'addr'
```

### Node Statistics

```bash
# Current stats
redis-enterprise node stats 1

# CPU usage
redis-enterprise node stats 1 --query '{cpu_user:cpu_user,cpu_system:cpu_system}'

# Memory usage
redis-enterprise node stats 1 --query '{used:used_memory,free:free_memory}'
```

## User Management

### Listing Users

```bash
# List all users
redis-enterprise user list

# Table format
redis-enterprise user list --output table

# Filter by role
redis-enterprise user list --query '[?role==`admin`].email'

# Find specific user
redis-enterprise user list --query '[?email==`user@company.com`]'
```

### Creating Users

```bash
# Basic user creation
redis-enterprise user create --email user@company.com --password SecurePass123

# Admin user
redis-enterprise user create --email admin@company.com --password AdminPass123 --role admin

# DB viewer role
redis-enterprise user create --email viewer@company.com --password ViewerPass123 --role db_viewer
```

### Getting User Information

```bash
# Get user by ID
redis-enterprise user get 1

# Get user role
redis-enterprise user get 1 --query 'role'
```

## Role and ACL Management

### Listing Roles

```bash
# List custom roles
redis-enterprise role list

# Include built-in roles
redis-enterprise role list --include-builtin

# Filter admin roles
redis-enterprise role list --query '[?management==`admin`].name'
```

### Creating Roles

```bash
# Create custom role
redis-enterprise role create \
  --name db-admin \
  --management admin \
  --data-access read-write

# From JSON configuration
cat > role_config.json << EOF
{
  "name": "custom-role",
  "management": "db_member",
  "data_access": "read-only"
}
EOF
redis-enterprise role create --from-json role_config.json
```

### Managing Role Users

```bash
# List users with specific role
redis-enterprise role users admin

# Get user emails only
redis-enterprise role users db-admin --query '[].email'
```

## Module Management

### Listing Modules

```bash
# List all modules
redis-enterprise module list

# Get module names and versions
redis-enterprise module list --query '[].{name:module_name,version:version}'

# Filter by module type
redis-enterprise module list --query '[?module_name==`search`]'
```

### Uploading Modules

```bash
# Upload module file
redis-enterprise module upload --file redisgraph.zip

# Upload custom module
redis-enterprise module upload --file custom-module.zip
```

### Module Operations

```bash
# Get module info
redis-enterprise module get search

# Update module
redis-enterprise module update search --description "Full-text search module"

# Delete module
redis-enterprise module delete old-module --yes
```

## License Management

### License Information

```bash
# Get current license
redis-enterprise license get

# Expiration date only
redis-enterprise license get --query 'expiration_date'

# License type
redis-enterprise license get --query 'type'
```

### License Usage

```bash
# Current usage
redis-enterprise license usage

# Shards usage
redis-enterprise license usage --query 'shards_used'

# Memory usage
redis-enterprise license usage --query 'memory_used'

# Usage percentage
redis-enterprise license usage --query '{shards_pct:shards_used/shards_limit,memory_pct:memory_used/memory_limit}'
```

### Updating License

```bash
# Update with key string
redis-enterprise license update --key "LICENSE-KEY-HERE"

# Update from file
redis-enterprise license update --file license.key

# Validate before installing
redis-enterprise license validate --key "NEW-LICENSE-KEY"
redis-enterprise license validate --file new_license.key
```

## Bootstrap Operations

### Checking Bootstrap Status

```bash
# Current status
redis-enterprise bootstrap status

# Status only
redis-enterprise bootstrap status --query 'status'

# Check if completed
redis-enterprise bootstrap status --query 'status' | grep -q completed && echo "Bootstrap complete"
```

### Raw Bootstrap Operations

```bash
# Send raw bootstrap command
redis-enterprise bootstrap raw --body '{"action":"create_cluster","cluster":{"name":"my-cluster"}}'

# From JSON file
cat > bootstrap.json << EOF
{
  "action": "create_cluster",
  "cluster": {
    "name": "production-cluster"
  }
}
EOF
redis-enterprise bootstrap raw --from-json bootstrap.json
```

## Configuration Management

### Managing Profiles

```bash
# List all profiles
redis-enterprise config list

# Set default profile
redis-enterprise config set --url https://cluster.redis.com:9443 --username admin@redis.com

# Create named profile
redis-enterprise config set --profile production \
  --url https://prod.redis.com:9443 \
  --username admin@company.com \
  --insecure

# Get profile details
redis-enterprise config get --profile production

# Remove profile
redis-enterprise config remove --profile old-cluster
```

### Using Profiles

```bash
# Use default profile
redis-enterprise cluster info

# Use specific profile
redis-enterprise --profile production cluster info

# Override profile settings
redis-enterprise --profile production --insecure cluster info
```

## Raw API Access

### GET Requests

```bash
# Get cluster info
redis-enterprise api get /v1/cluster

# Get all databases
redis-enterprise api get /v1/bdbs

# Get specific database
redis-enterprise api get /v1/bdbs/1

# With JMESPath query
redis-enterprise api get /v1/bdbs --query '[].name'
```

### POST Requests

```bash
# Create database
redis-enterprise api post /v1/bdbs --body '{"name":"api-db","memory_size":1073741824}'

# From file
cat > request.json << EOF
{
  "name": "api-created-db",
  "memory_size": 1073741824,
  "port": 12003
}
EOF
redis-enterprise api post /v1/bdbs --from-json request.json
```

### PUT Requests

```bash
# Update database
redis-enterprise api put /v1/bdbs/1 --body '{"memory_size":2147483648}'

# Update cluster
redis-enterprise api put /v1/cluster --body '{"name":"Updated Cluster"}'
```

### DELETE Requests

```bash
# Delete database
redis-enterprise api delete /v1/bdbs/5

# Delete user
redis-enterprise api delete /v1/users/10
```

## Advanced JMESPath Queries

### Complex Filtering

```bash
# Databases with specific conditions
redis-enterprise database list --query \
  '[?memory_size > `1073741824` && status==`active`].name'

# Multi-condition node query
redis-enterprise node list --query \
  '[?status==`active` && role==`master`].{id:uid,memory:total_memory}'
```

### Transformations

```bash
# Calculate total memory across databases
redis-enterprise database list --query 'sum([].memory_size)'

# Get min/max values
redis-enterprise database list --query \
  '{min:min([].memory_size),max:max([].memory_size)}'

# Sort and limit
redis-enterprise database list --query \
  'sort_by(@, &memory_size) | [:5].name'
```

### Nested Queries

```bash
# Database endpoints
redis-enterprise database get mydb --query \
  'endpoints[].{addr:addr,port:port}'

# Module capabilities
redis-enterprise module list --query \
  '[].{name:module_name,caps:capabilities[].name}'
```

## Output Format Examples

### JSON (Default)

```bash
redis-enterprise cluster info
# Output: {"uid":1,"name":"my-cluster","status":"active",...}
```

### YAML

```bash
redis-enterprise cluster info --output yaml
# Output:
# uid: 1
# name: my-cluster
# status: active
# ...
```

### Table

```bash
redis-enterprise database list --output table
# Output:
# +-----+-----------+--------+----------+-------+
# | UID | Name      | Status | Memory   | Port  |
# +-----+-----------+--------+----------+-------+
# | 1   | test-db   | active | 104857600| 12000 |
# | 2   | cache-db  | active | 524288000| 12001 |
# +-----+-----------+--------+----------+-------+
```

## Environment Variables

```bash
# Set default connection
export REDIS_ENTERPRISE_URL="https://cluster.redis.com:9443"
export REDIS_ENTERPRISE_USER="admin@company.com"
export REDIS_ENTERPRISE_PASSWORD="SecurePassword123"

# Use default profile
export REDIS_ENTERPRISE_PROFILE="production"

# Enable verbose logging
export RUST_LOG="debug"

# Now commands use these defaults
redis-enterprise cluster info
```

## Scripting Examples

### Batch Database Creation

```bash
#!/bin/bash
for i in {1..5}; do
  redis-enterprise database create \
    --name "db-$i" \
    --memory 100MB \
    --port $((12000 + $i))
done
```

### Monitor Database Status

```bash
#!/bin/bash
while true; do
  status=$(redis-enterprise database get mydb --query 'status')
  echo "Database status: $status"
  if [ "$status" = "active" ]; then
    echo "Database is ready!"
    break
  fi
  sleep 5
done
```

### Export Configuration

```bash
#!/bin/bash
# Export all database configurations
redis-enterprise database list --query '[].name' | \
while read -r db_name; do
  redis-enterprise database get "$db_name" > "${db_name}_config.json"
done
```

### Health Check Script

```bash
#!/bin/bash
# Check cluster health
cluster_status=$(redis-enterprise cluster info --query 'status')
if [ "$cluster_status" != "active" ]; then
  echo "WARNING: Cluster status is $cluster_status"
  exit 1
fi

# Check all databases
redis-enterprise database list --query '[?status!=`active`].name' | \
while read -r db_name; do
  echo "WARNING: Database $db_name is not active"
done
```

## Tips and Best Practices

1. **Use Profiles**: Save frequently used cluster configurations as profiles
2. **JMESPath Queries**: Filter output at the source to reduce data transfer
3. **Output Formats**: Use `--output table` for human reading, JSON for scripting
4. **Wait Commands**: Always use wait after create/delete operations in scripts
5. **Environment Variables**: Set defaults to avoid repeating credentials
6. **Verbose Mode**: Use `--verbose` or `RUST_LOG=debug` for troubleshooting
7. **JSON Files**: Store complex configurations in JSON files for repeatability
8. **Error Handling**: Check exit codes in scripts (0 = success)
9. **Batch Operations**: Use loops and JSON configs for bulk operations
10. **Security**: Never hardcode passwords in scripts; use environment variables or config files
