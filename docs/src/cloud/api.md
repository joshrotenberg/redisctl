# Cloud API Layer

Direct REST access to the Redis Cloud API for scripting and automation.

## Overview

The API layer lets you call any Redis Cloud REST endpoint directly. It's like a smart curl with:
- Automatic authentication
- Profile support
- Output formatting

## Usage

```bash
redisctl api cloud <method> <endpoint> [options]
```

**Methods:** `get`, `post`, `put`, `delete`

## Examples

### GET Requests

```bash
# Account info
redisctl api cloud get /

# List subscriptions
redisctl api cloud get /subscriptions

# Get specific subscription
redisctl api cloud get /subscriptions/123456

# List databases
redisctl api cloud get /subscriptions/123456/databases

# Get specific database
redisctl api cloud get /subscriptions/123456/databases/789
```

### POST Requests

```bash
# Create subscription
redisctl api cloud post /subscriptions -d @subscription.json

# Create database
redisctl api cloud post /subscriptions/123456/databases -d '{
  "name": "mydb",
  "memoryLimitInGb": 1
}'
```

### PUT Requests

```bash
# Update database
redisctl api cloud put /subscriptions/123456/databases/789 -d '{
  "memoryLimitInGb": 2
}'
```

### DELETE Requests

```bash
# Delete database
redisctl api cloud delete /subscriptions/123456/databases/789

# Delete subscription
redisctl api cloud delete /subscriptions/123456
```

## Options

| Option | Description |
|--------|-------------|
| `-d, --data <JSON>` | Request body (inline or @file) |
| `-o, --output <FORMAT>` | Output format (json, yaml, table) |
| `-q, --query <JMESPATH>` | Filter output |

## Common Endpoints

### Account
- `GET /` - Account info
- `GET /payment-methods` - Payment methods
- `GET /regions` - Available regions

### Subscriptions
- `GET /subscriptions` - List all
- `POST /subscriptions` - Create
- `GET /subscriptions/{id}` - Get one
- `PUT /subscriptions/{id}` - Update
- `DELETE /subscriptions/{id}` - Delete

### Databases
- `GET /subscriptions/{id}/databases` - List
- `POST /subscriptions/{id}/databases` - Create
- `GET /subscriptions/{id}/databases/{dbId}` - Get
- `PUT /subscriptions/{id}/databases/{dbId}` - Update
- `DELETE /subscriptions/{id}/databases/{dbId}` - Delete

### ACL
- `GET /acl/users` - List users
- `GET /acl/roles` - List roles
- `GET /acl/redisRules` - List Redis rules

### Networking
- `GET /subscriptions/{id}/peerings` - VPC peerings
- `GET /subscriptions/{id}/privateServiceConnect` - PSC
- `GET /subscriptions/{id}/transitGateway` - Transit Gateway

### Tasks
- `GET /tasks` - List tasks
- `GET /tasks/{taskId}` - Get task

## Scripting Examples

### Create and Wait

```bash
# Create database
TASK_ID=$(redisctl api cloud post /subscriptions/123/databases \
  -d @database.json \
  -q 'taskId')

# Poll for completion
while true; do
  STATUS=$(redisctl api cloud get /tasks/$TASK_ID -q 'status')
  [ "$STATUS" = "processing-completed" ] && break
  [ "$STATUS" = "processing-error" ] && exit 1
  sleep 10
done

# Get result
redisctl api cloud get /tasks/$TASK_ID -q 'response.resourceId'
```

### Bulk Operations

```bash
# Get all database IDs and process each
for db in $(redisctl api cloud get /subscriptions/123/databases -q '[].databaseId' --raw); do
  redisctl api cloud get /subscriptions/123/databases/$db
done
```

### Export to File

```bash
# Save subscription config
redisctl api cloud get /subscriptions/123 > subscription.json

# Save all databases
redisctl api cloud get /subscriptions/123/databases > databases.json
```

## When to Use API Layer

**Use API layer when:**
- Endpoint isn't wrapped in human commands yet
- You need exact control over the request
- Building automation scripts
- Exploring the API

**Use human commands when:**
- There's a command for what you need
- You want built-in `--wait` support
- You prefer ergonomic flags over JSON

## API Documentation

Full API documentation: [Redis Cloud API Docs](https://api.redislabs.com/v1/swagger-ui.html)
