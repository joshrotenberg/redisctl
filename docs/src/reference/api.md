# API Reference

Complete reference for direct API access using redisctl.

## Overview

The `api` command provides direct access to REST endpoints for both Redis Cloud and Redis Enterprise APIs.

```bash
redisctl api <deployment> <method> <path> [OPTIONS]
```

## Command Structure

### Deployment Types
- `cloud` - Redis Cloud API
- `enterprise` - Redis Enterprise API

### HTTP Methods
- `get` - HTTP GET request
- `post` - HTTP POST request
- `put` - HTTP PUT request
- `patch` - HTTP PATCH request
- `delete` - HTTP DELETE request

### Path Format
- Must start with `/`
- Can include path parameters
- Query parameters via `--query-params`

## Options

| Option | Description | Example |
|--------|-------------|---------|
| `--data <JSON>` | Request body (inline or @file) | `--data @payload.json` |
| `--query-params <PARAMS>` | URL query parameters | `--query-params "limit=10&offset=0"` |
| `--headers <HEADERS>` | Additional HTTP headers | `--headers "X-Custom: value"` |
| `-o, --output <FORMAT>` | Output format | `-o json` |
| `-q, --query <JMESPATH>` | JMESPath filter | `-q "[].name"` |
| `--profile <NAME>` | Use specific profile | `--profile prod` |

## Redis Cloud API

### Base URL
`https://api.redislabs.com/v1`

### Authentication
- Header: `x-api-key: <api-key>`
- Header: `x-api-secret-key: <secret-key>`

### Common Endpoints

#### Account
```bash
# Get account info
redisctl api cloud get /

# Get payment methods
redisctl api cloud get /payment-methods

# Get regions
redisctl api cloud get /regions
```

#### Subscriptions
```bash
# List subscriptions
redisctl api cloud get /subscriptions

# Get specific subscription
redisctl api cloud get /subscriptions/123456

# Create subscription
redisctl api cloud post /subscriptions --data @subscription.json

# Update subscription
redisctl api cloud put /subscriptions/123456 --data '{"name": "New Name"}'

# Delete subscription
redisctl api cloud delete /subscriptions/123456
```

#### Databases
```bash
# List databases
redisctl api cloud get /subscriptions/123456/databases

# Get database
redisctl api cloud get /subscriptions/123456/databases/789

# Create database
redisctl api cloud post /subscriptions/123456/databases --data @database.json

# Update database
redisctl api cloud put /subscriptions/123456/databases/789 --data '{"memoryLimitInGb": 8}'

# Delete database
redisctl api cloud delete /subscriptions/123456/databases/789
```

#### VPC Peering
```bash
# List VPC peerings
redisctl api cloud get /subscriptions/123456/peerings

# Create VPC peering
redisctl api cloud post /subscriptions/123456/peerings --data @vpc.json

# Get peering status
redisctl api cloud get /subscriptions/123456/peerings/abc123

# Delete peering
redisctl api cloud delete /subscriptions/123456/peerings/abc123
```

#### Tasks
```bash
# Get task status
redisctl api cloud get /tasks/task-123

# List tasks
redisctl api cloud get /tasks --query-params "status=processing"
```

#### ACL
```bash
# List ACL rules
redisctl api cloud get /subscriptions/123456/databases/789/acl/rules

# Create ACL rule
redisctl api cloud post /subscriptions/123456/databases/789/acl/rules --data @rule.json

# List ACL users  
redisctl api cloud get /subscriptions/123456/databases/789/acl/users

# Create ACL user
redisctl api cloud post /subscriptions/123456/databases/789/acl/users --data @user.json
```

### Response Codes

| Code | Meaning | Action |
|------|---------|--------|
| 200 | Success | Request completed |
| 201 | Created | Resource created |
| 202 | Accepted | Async operation started |
| 400 | Bad Request | Check request format |
| 401 | Unauthorized | Check API credentials |
| 403 | Forbidden | Check permissions |
| 404 | Not Found | Verify resource exists |
| 409 | Conflict | Resource state conflict |
| 429 | Rate Limited | Retry after delay |
| 500 | Server Error | Contact support |

## Redis Enterprise API

### Base URL
`https://<cluster-address>:9443`

### Authentication
- Basic Auth: `username:password`
- Header: `Authorization: Basic <base64>`

### Common Endpoints

#### Cluster
```bash
# Get cluster info
redisctl api enterprise get /v1/cluster

# Update cluster
redisctl api enterprise put /v1/cluster --data '{"name": "Production"}'

# Get cluster policy
redisctl api enterprise get /v1/cluster/policy

# Update policy
redisctl api enterprise put /v1/cluster/policy --data @policy.json
```

#### Databases (BDB)
```bash
# List databases
redisctl api enterprise get /v1/bdbs

# Get database
redisctl api enterprise get /v1/bdbs/1

# Create database
redisctl api enterprise post /v1/bdbs --data @bdb.json

# Update database
redisctl api enterprise put /v1/bdbs/1 --data '{"memory_size": 10737418240}'

# Delete database
redisctl api enterprise delete /v1/bdbs/1
```

#### Nodes
```bash
# List nodes
redisctl api enterprise get /v1/nodes

# Get node
redisctl api enterprise get /v1/nodes/1

# Update node
redisctl api enterprise put /v1/nodes/1 --data '{"rack_id": "rack-1"}'

# Node actions
redisctl api enterprise post /v1/nodes/1/actions/check
```

#### Users & RBAC
```bash
# List users
redisctl api enterprise get /v1/users

# Create user
redisctl api enterprise post /v1/users --data @user.json

# Get user
redisctl api enterprise get /v1/users/1

# Update user
redisctl api enterprise put /v1/users/1 --data '{"name": "Updated Name"}'

# Delete user
redisctl api enterprise delete /v1/users/1

# List roles
redisctl api enterprise get /v1/roles
```

#### Statistics
```bash
# Cluster stats
redisctl api enterprise get /v1/cluster/stats/last

# Database stats
redisctl api enterprise get /v1/bdbs/stats/last

# Node stats
redisctl api enterprise get /v1/nodes/stats/last

# Shard stats
redisctl api enterprise get /v1/shards/stats/last
```

#### Modules
```bash
# List modules
redisctl api enterprise get /v1/modules

# Upload module (requires multipart)
# Use module command instead: redisctl enterprise module upload --file module.zip

# Get module
redisctl api enterprise get /v1/modules/1

# Delete module
redisctl api enterprise delete /v1/modules/1
```

#### Logs
```bash
# Get cluster logs
redisctl api enterprise get /v1/logs --query-params "limit=100"

# Filter logs by time
redisctl api enterprise get /v1/logs --query-params "stime=2024-01-01T00:00:00Z&etime=2024-01-02T00:00:00Z"
```

### API Versions

Redis Enterprise supports both v1 and v2 endpoints:

| Version | Status | Usage |
|---------|--------|-------|
| v1 | Stable | Most operations |
| v2 | Preview | New features, async operations |

```bash
# v1 endpoint
redisctl api enterprise get /v1/bdbs

# v2 endpoint (if available)
redisctl api enterprise get /v2/bdbs
```

## Query Parameters

Common query parameters across APIs:

| Parameter | Description | Example |
|-----------|-------------|---------|
| `limit` | Max results | `limit=50` |
| `offset` | Skip results | `offset=100` |
| `sort` | Sort field | `sort=name` |
| `order` | Sort order | `order=desc` |
| `fields` | Select fields | `fields=name,status` |
| `filter` | Filter results | `filter=status:active` |

## Request Body Formats

### JSON Payload
```bash
# Inline JSON
redisctl api cloud post /path --data '{"key": "value"}'

# From file
redisctl api cloud post /path --data @payload.json

# From stdin
echo '{"key": "value"}' | redisctl api cloud post /path --data @-
```

### Complex Examples

#### Create Database with Full Configuration
```json
{
  "name": "production-cache",
  "memoryLimitInGb": 16,
  "protocol": "redis",
  "port": 10000,
  "throughputMeasurement": {
    "by": "operations-per-second",
    "value": 100000
  },
  "replication": true,
  "dataPersistence": "aof-every-1-second",
  "dataEvictionPolicy": "allkeys-lru",
  "modules": [
    {"name": "RedisJSON"},
    {"name": "RediSearch"}
  ],
  "alerts": [
    {"name": "dataset-size", "value": 80}
  ],
  "backup": {
    "interval": 6,
    "enabled": true
  }
}
```

#### Update Multiple Properties
```bash
redisctl api cloud put /subscriptions/123/databases/456 --data '{
  "memoryLimitInGb": 32,
  "throughputMeasurement": {
    "by": "operations-per-second",
    "value": 200000
  },
  "alerts": [
    {"name": "dataset-size", "value": 90},
    {"name": "throughput-higher-than", "value": 180000}
  ]
}'
```

## Response Handling

### Success Response
```bash
# Pretty print JSON
redisctl api cloud get /subscriptions -o json | jq .

# Extract specific fields
redisctl api cloud get /subscriptions -q "[].{id: id, name: name}"

# Table format
redisctl api cloud get /subscriptions -o table
```

### Error Response
```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Missing required field: name",
    "details": {
      "field": "name",
      "constraint": "required"
    }
  }
}
```

### Async Operations
```bash
# Create returns task ID
TASK_ID=$(redisctl api cloud post /subscriptions/123/databases \
  --data @database.json \
  -q "taskId")

# Poll task status
while true; do
  STATUS=$(redisctl api cloud get /tasks/$TASK_ID -q "status")
  if [ "$STATUS" = "completed" ]; then
    break
  elif [ "$STATUS" = "failed" ]; then
    echo "Task failed!"
    exit 1
  fi
  sleep 10
done
```

## Rate Limiting

Both APIs implement rate limiting:

### Redis Cloud
- Default: 100 requests per minute
- Burst: 150 requests
- Headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`

### Redis Enterprise
- Configurable per cluster
- Default: No rate limiting
- Can be enabled in cluster settings

### Handling Rate Limits
```bash
# Retry with exponential backoff
retry_with_backoff() {
  local max_attempts=5
  local attempt=0
  local delay=1
  
  while [ $attempt -lt $max_attempts ]; do
    if redisctl api cloud get /subscriptions; then
      return 0
    fi
    
    echo "Rate limited, waiting ${delay}s..."
    sleep $delay
    
    attempt=$((attempt + 1))
    delay=$((delay * 2))
  done
  
  return 1
}
```

## Pagination

Handle paginated results:

```bash
#!/bin/bash
# Fetch all pages

LIMIT=100
OFFSET=0
ALL_RESULTS=()

while true; do
  RESULTS=$(redisctl api cloud get /subscriptions \
    --query-params "limit=$LIMIT&offset=$OFFSET" \
    -o json)
  
  COUNT=$(echo "$RESULTS" | jq '. | length')
  
  if [ "$COUNT" -eq 0 ]; then
    break
  fi
  
  ALL_RESULTS+=("$RESULTS")
  OFFSET=$((OFFSET + LIMIT))
done

# Combine results
echo "${ALL_RESULTS[@]}" | jq -s 'flatten'
```

## Best Practices

1. **Use profiles** for credential management
2. **Handle errors** gracefully with proper error checking
3. **Implement retries** for transient failures
4. **Respect rate limits** with backoff strategies
5. **Use pagination** for large result sets
6. **Cache responses** when appropriate
7. **Log API calls** for audit trails
8. **Validate JSON** before sending
9. **Use query filters** to reduce response size
10. **Monitor API usage** to stay within limits

## Troubleshooting

### Debug API Calls
```bash
# Enable debug logging
RUST_LOG=debug redisctl api cloud get /subscriptions

# View request headers
RUST_LOG=trace redisctl api cloud get /subscriptions 2>&1 | grep -i header

# Test with curl
curl -H "x-api-key: $API_KEY" \
     -H "x-api-secret-key: $SECRET" \
     https://api.redislabs.com/v1/subscriptions
```

### Common Issues

**401 Unauthorized**
- Check API credentials
- Verify profile configuration
- Ensure credentials have necessary permissions

**404 Not Found**
- Verify endpoint path
- Check resource IDs
- Ensure API version is correct

**429 Rate Limited**
- Implement retry logic
- Add delays between requests
- Consider caching responses

**500 Server Error**
- Check API status page
- Retry with exponential backoff
- Contact support if persistent