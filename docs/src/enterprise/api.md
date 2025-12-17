# Enterprise API Layer

Direct REST access to the Redis Enterprise API for scripting and automation.

## Overview

The API layer lets you call any Redis Enterprise REST endpoint directly. It's like a smart curl with:
- Automatic authentication
- Profile support
- Output formatting
- SSL handling

## Usage

```bash
redisctl api enterprise <method> <endpoint> [options]
```

**Methods:** `get`, `post`, `put`, `delete`

## Examples

### GET Requests

```bash
# Cluster info
redisctl api enterprise get /v1/cluster

# List nodes
redisctl api enterprise get /v1/nodes

# List databases
redisctl api enterprise get /v1/bdbs

# Get specific database
redisctl api enterprise get /v1/bdbs/1

# Get users
redisctl api enterprise get /v1/users
```

### POST Requests

```bash
# Create database
redisctl api enterprise post /v1/bdbs -d '{
  "name": "mydb",
  "memory_size": 1073741824
}'

# Create user
redisctl api enterprise post /v1/users -d @user.json
```

### PUT Requests

```bash
# Update database
redisctl api enterprise put /v1/bdbs/1 -d '{
  "memory_size": 2147483648
}'

# Update cluster
redisctl api enterprise put /v1/cluster -d '{"name": "new-name"}'
```

### DELETE Requests

```bash
# Delete database
redisctl api enterprise delete /v1/bdbs/1

# Remove node
redisctl api enterprise delete /v1/nodes/3
```

## Options

| Option | Description |
|--------|-------------|
| `-d, --data <JSON>` | Request body (inline or @file) |
| `-o, --output <FORMAT>` | Output format (json, yaml, table) |
| `-q, --query <JMESPATH>` | Filter output |

## Common Endpoints

### Cluster
- `GET /v1/cluster` - Cluster info
- `PUT /v1/cluster` - Update cluster
- `GET /v1/cluster/stats/last` - Cluster stats
- `GET /v1/cluster/certificates` - Certificates

### Nodes
- `GET /v1/nodes` - List nodes
- `GET /v1/nodes/{id}` - Get node
- `PUT /v1/nodes/{id}` - Update node
- `DELETE /v1/nodes/{id}` - Remove node
- `GET /v1/nodes/{id}/stats/last` - Node stats

### Databases
- `GET /v1/bdbs` - List databases
- `POST /v1/bdbs` - Create database
- `GET /v1/bdbs/{id}` - Get database
- `PUT /v1/bdbs/{id}` - Update database
- `DELETE /v1/bdbs/{id}` - Delete database
- `GET /v1/bdbs/{id}/stats/last` - Database stats

### Users & Roles
- `GET /v1/users` - List users
- `POST /v1/users` - Create user
- `GET /v1/roles` - List roles
- `GET /v1/redis_acls` - List ACLs

### Active-Active
- `GET /v1/crdbs` - List CRDBs
- `POST /v1/crdbs` - Create CRDB
- `GET /v1/crdb_tasks` - List tasks

### Logs & Alerts
- `GET /v1/logs` - Get logs
- `GET /v1/cluster/alerts` - Get alerts

### Debug & Support
- `GET /v1/debuginfo/all` - Full debug info (binary)
- `GET /v1/debuginfo/node/{id}` - Node debug info

## Scripting Examples

### Export Cluster Config

```bash
# Save cluster configuration
redisctl api enterprise get /v1/cluster > cluster-config.json
redisctl api enterprise get /v1/bdbs > databases.json
redisctl api enterprise get /v1/users > users.json
```

### Bulk Database Creation

```bash
# Create multiple databases
for name in cache sessions analytics; do
  redisctl api enterprise post /v1/bdbs -d "{
    \"name\": \"$name\",
    \"memory_size\": 1073741824
  }"
done
```

### Health Check

```bash
#!/bin/bash
# Check cluster health via API

STATUS=$(redisctl api enterprise get /v1/cluster -q 'status')

if [ "$STATUS" != "active" ]; then
  echo "Cluster unhealthy: $STATUS"
  exit 1
fi

# Check nodes
redisctl api enterprise get /v1/nodes -q '[].{id: uid, status: status}' -o table
```

### Watch Stats

```bash
# Poll stats every 5 seconds
while true; do
  redisctl api enterprise get /v1/cluster/stats/last \
    -q '{cpu:cpu_user,memory:free_memory}'
  sleep 5
done
```

## Binary Responses

Some endpoints return binary data (tar.gz):

```bash
# Download debug info
redisctl api enterprise get /v1/debuginfo/all --output debug.tar.gz
```

## When to Use API Layer

**Use API layer when:**
- Endpoint isn't wrapped in human commands
- You need exact control over the request
- Building automation scripts
- Exploring the API

**Use human commands when:**
- There's a command for what you need
- You want ergonomic flags
- You prefer structured output

## API Documentation

The cluster provides built-in API docs at:
`https://your-cluster:9443/v1/swagger-ui/index.html`
