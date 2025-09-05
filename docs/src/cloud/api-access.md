# Raw API Access

Direct access to any Redis Cloud REST API endpoint.

## Basic Usage

```bash
redisctl api cloud <method> <path> [options]
```

Methods: `get`, `post`, `put`, `patch`, `delete`

## Examples

### GET Requests

```bash
# Get account information
redisctl api cloud get /account

# Get all subscriptions
redisctl api cloud get /subscriptions

# Get specific subscription
redisctl api cloud get /subscriptions/12345

# Get databases in subscription
redisctl api cloud get /subscriptions/12345/databases

# Get with query parameters
redisctl api cloud get "/subscriptions?limit=10&offset=20"
```

### POST Requests

```bash
# Create database (with JSON file)
redisctl api cloud post /subscriptions/12345/databases \
  --data @database.json

# Create database (with inline JSON)
redisctl api cloud post /subscriptions/12345/databases \
  --data '{
    "name": "my-database",
    "memoryLimitInGb": 1,
    "modules": ["RedisJSON", "RediSearch"]
  }'

# Create with data from stdin
echo '{"name": "test"}' | redisctl api cloud post /subscriptions/12345/databases \
  --data @-
```

### PUT Requests

```bash
# Update database
redisctl api cloud put /subscriptions/12345/databases/67890 \
  --data '{"memoryLimitInGb": 2}'
```

### DELETE Requests

```bash
# Delete database
redisctl api cloud delete /subscriptions/12345/databases/67890
```

## Request Options

### Headers

```bash
# Add custom headers
redisctl api cloud get /account \
  --header "X-Request-ID: abc123" \
  --header "X-Custom: value"
```

### Output Control

```bash
# Get raw response body only
redisctl api cloud get /account --raw

# Include response headers
redisctl api cloud get /account --include-headers

# Verbose output (shows request details)
redisctl api cloud get /account --verbose
```

## Working with Files

### Request Body from File

```bash
# JSON file
redisctl api cloud post /subscriptions/12345/databases \
  --data @create-database.json

# YAML file (converted to JSON)
redisctl api cloud post /subscriptions/12345/databases \
  --data @create-database.yaml
```

### Save Response to File

```bash
# Save response
redisctl api cloud get /subscriptions > subscriptions.json

# Pretty print and save
redisctl api cloud get /subscriptions | jq '.' > subscriptions.json
```

## Common Endpoints

### Account & Billing
- `/account` - Account information
- `/payment-methods` - Payment methods
- `/cloud-accounts` - Cloud provider accounts

### Subscriptions
- `/subscriptions` - List subscriptions
- `/subscriptions/{id}` - Subscription details
- `/subscriptions/{id}/databases` - Databases in subscription
- `/subscriptions/{id}/pricing` - Pricing information

### Databases
- `/subscriptions/{sub}/databases` - List databases
- `/subscriptions/{sub}/databases/{db}` - Database details
- `/subscriptions/{sub}/databases/{db}/backup` - Backup operations
- `/subscriptions/{sub}/databases/{db}/import` - Import data

### Users & Access
- `/users` - User management
- `/roles` - Role definitions
- `/subscriptions/{id}/redis-acl` - ACL rules

### Operations
- `/tasks/{id}` - Task status
- `/logs` - System logs
- `/metrics` - Performance metrics

## Error Handling

API errors are returned with appropriate HTTP status codes:

```bash
# Check response code
redisctl api cloud get /invalid-endpoint
# Error: 404 Not Found

# Verbose mode shows full error
redisctl api cloud get /invalid-endpoint --verbose
# Shows full error response with details
```

## Tips

1. Use `--data @-` to read from stdin for piping
2. Use `-q` with JMESPath to filter responses
3. Use `--raw` to get just the response body for scripting
4. Check the [Redis Cloud API docs](https://api.redislabs.com/v1/swagger-ui/) for endpoint details