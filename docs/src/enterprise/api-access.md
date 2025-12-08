# Raw API Access

Direct access to any Redis Enterprise REST API endpoint.

## Basic Usage

```bash
redisctl api enterprise <method> <path> [options]
```

Methods: `get`, `post`, `put`, `patch`, `delete`

## Examples

### GET Requests

```bash
# Get cluster information
redisctl api enterprise get /v1/cluster

# Get all databases
redisctl api enterprise get /v1/bdbs

# Get specific database
redisctl api enterprise get /v1/bdbs/1

# Get all nodes
redisctl api enterprise get /v1/nodes

# Get node statistics
redisctl api enterprise get /v1/nodes/1/stats

# Get with query parameters
redisctl api enterprise get "/v1/bdbs?fields=name,port,status"
```

### POST Requests

```bash
# Create database (with JSON file)
redisctl api enterprise post /v1/bdbs \
  --data @database.json

# Create database (with inline JSON)
redisctl api enterprise post /v1/bdbs \
  --data '{
    "name": "my-database",
    "memory_size": 10737418240,
    "port": 12000,
    "replication": true
  }'

# Bootstrap cluster
redisctl api enterprise post /v1/bootstrap \
  --data '{
    "action": "create_cluster",
    "cluster": {
      "name": "my-cluster"
    },
    "credentials": {
      "username": "admin@cluster.local",
      "password": "admin-password"
    }
  }'
```

### PUT Requests

```bash
# Update database configuration
redisctl api enterprise put /v1/bdbs/1 \
  --data '{"memory_size": 21474836480}'

# Update cluster settings
redisctl api enterprise put /v1/cluster \
  --data '{"name": "Production Cluster"}'
```

### DELETE Requests

```bash
# Delete database
redisctl api enterprise delete /v1/bdbs/1

# Remove node from cluster
redisctl api enterprise delete /v1/nodes/3
```

## Common Endpoints

### Cluster Management
- `/v1/cluster` - Cluster information and settings
- `/v1/bootstrap` - Bootstrap operations
- `/v1/license` - License management
- `/v1/ocsp` - OCSP configuration
- `/v1/cm_settings` - Cluster manager settings

### Database Operations (BDB)
- `/v1/bdbs` - Database list and creation
- `/v1/bdbs/{id}` - Database details and management
- `/v1/bdbs/{id}/actions` - Database actions (flush, restart)
- `/v1/bdbs/{id}/stats` - Database statistics

### Node Management
- `/v1/nodes` - Node list
- `/v1/nodes/{id}` - Node details
- `/v1/nodes/{id}/actions` - Node actions
- `/v1/nodes/{id}/stats` - Node statistics

### User & Access Control
- `/v1/users` - User management
- `/v1/roles` - Role definitions
- `/v1/acl_roles` - Redis ACL roles
- `/v1/ldap_mappings` - LDAP integration

### Sharding & Replication
- `/v1/shards` - Shard management
- `/v1/shards/{id}/actions` - Shard operations
- `/v1/bdbs/{id}/endpoints` - Database endpoints

### Active-Active (CRDB)
- `/v1/crdbs` - CRDB list and creation
- `/v1/crdbs/{id}` - CRDB management
- `/v1/crdbs/{id}/participating_clusters` - Participating clusters

### Monitoring & Alerts
- `/v1/bdbs/{id}/alerts` - Database alerts
- `/v1/nodes/{id}/alerts` - Node alerts
- `/v1/cluster/alerts` - Cluster alerts
- `/v1/logs` - System logs

### Modules
- `/v1/modules` - Module management
- `/v1/modules/{id}` - Module details

## Working with Certificates

For self-signed certificates:

```bash
# Allow insecure connections
export REDIS_ENTERPRISE_INSECURE=true

# Or use system certificate store
redisctl api enterprise get /v1/cluster \
  --ca-cert /path/to/ca.crt
```

## Pagination

Many endpoints support pagination:

```bash
# Get databases with pagination
redisctl api enterprise get "/v1/bdbs?offset=0&limit=10"

# Get next page
redisctl api enterprise get "/v1/bdbs?offset=10&limit=10"
```

## Filtering Results

```bash
# Get only specific fields
redisctl api enterprise get "/v1/bdbs?fields=name,port,status"

# Filter with JMESPath after retrieval
redisctl api enterprise get /v1/bdbs \
  -q "[?status=='active'].{name:name,port:port}"
```

## Async Operations

Some operations return task IDs:

```bash
# Create database (returns task)
TASK_ID=$(redisctl api enterprise post /v1/bdbs \
  --data @database.json \
  -q "task_id")

# Check task status
redisctl api enterprise get /v1/tasks/$TASK_ID

# Wait for completion
while [ "$(redisctl api enterprise get /v1/tasks/$TASK_ID -q status)" != "completed" ]; do
  sleep 2
done
```

## Error Handling

```bash
# Check HTTP status
if ! redisctl api enterprise get /v1/bdbs/999; then
  echo "Database not found"
fi

# Get detailed error information
redisctl api enterprise get /v1/bdbs/999 --verbose
```

## Tips

1. Use `/v1/swagger` to get API documentation
2. Most IDs in Enterprise are integers (unlike Cloud's UUIDs)
3. Memory sizes are in bytes
4. Use `--insecure` for development clusters with self-signed certs
5. Check the [Redis Enterprise API docs](https://docs.redis.com/latest/rs/references/rest-api/) for detailed endpoint information