# Cloud Quick Examples

Examples showing the three-tier model for Redis Cloud.

## Setup

```bash
# Set credentials
export REDIS_CLOUD_API_KEY="your-api-key"
export REDIS_CLOUD_SECRET_KEY="your-secret-key"

# Or use Docker
alias redisctl='docker run --rm \
  -e REDIS_CLOUD_API_KEY \
  -e REDIS_CLOUD_SECRET_KEY \
  redis-developer/redisctl'
```

## API Layer Examples

Direct REST access for scripting:

```bash
# Get account info
redisctl api cloud get /

# List all subscriptions
redisctl api cloud get /subscriptions

# Get specific subscription
redisctl api cloud get /subscriptions/123456

# List databases in subscription
redisctl api cloud get /subscriptions/123456/databases

# Create database (returns task ID)
redisctl api cloud post /subscriptions/123456/databases -d '{
  "name": "cache",
  "memoryLimitInGb": 1
}'

# Check task status
redisctl api cloud get /tasks/abc-123-def
```

## Human Command Examples

Type-safe operations for daily use:

```bash
# List subscriptions with table output
redisctl cloud subscription list -o table

# Get subscription details
redisctl cloud subscription get 123456

# List databases with specific fields
redisctl cloud database list --subscription 123456 \
  -q "[].{name:name,status:status,memory:memoryLimitInGb}" \
  -o table

# Create database and wait for completion
redisctl cloud database create \
  --subscription 123456 \
  --name sessions \
  --memory 2 \
  --wait

# Get connection details
redisctl cloud database get 123456:789 \
  -q '{endpoint: publicEndpoint, password: password}'

# Update database memory
redisctl cloud database update 123456:789 \
  --data '{"memoryLimitInGb": 4}' \
  --wait

# Delete database
redisctl cloud database delete 123456:789 --wait
```

## Workflow Examples

Multi-step operations:

```bash
# Set up complete subscription with VPC peering
redisctl cloud workflow subscription-setup \
  --name production \
  --cloud-provider aws \
  --region us-east-1 \
  --memory-limit-in-gb 10 \
  --with-vpc-peering \
  --vpc-id vpc-abc123 \
  --vpc-cidr 10.0.0.0/16 \
  --create-database \
  --database-name cache
```

This single command:
1. Creates the subscription
2. Waits for it to be active
3. Sets up VPC peering
4. Creates the initial database
5. Returns connection details

## Common Patterns

### Get Database Connection String

```bash
DB=$(redisctl cloud database get 123456:789)
ENDPOINT=$(echo $DB | jq -r '.publicEndpoint')
PASSWORD=$(echo $DB | jq -r '.password')
echo "redis://:$PASSWORD@$ENDPOINT"
```

### List All Databases Across Subscriptions

```bash
for sub in $(redisctl cloud subscription list -q '[].id' | jq -r '.[]'); do
  echo "=== Subscription $sub ==="
  redisctl cloud database list --subscription $sub -o table
done
```

### Wait for Operation with Custom Polling

```bash
TASK_ID=$(redisctl cloud database create \
  --subscription 123456 \
  --data @database.json \
  -q 'taskId')

while true; do
  STATUS=$(redisctl cloud task get $TASK_ID -q 'status')
  echo "Status: $STATUS"
  
  case $STATUS in
    "processing-completed") 
      echo "Database created!"
      redisctl cloud task get $TASK_ID -q 'response.resourceId'
      break 
      ;;
    "processing-error") 
      echo "Failed!"
      redisctl cloud task get $TASK_ID -q 'response.error'
      exit 1 
      ;;
    *) sleep 10 ;;
  esac
done
```

### Export Configuration

```bash
# Backup subscription and database configs
SUB_ID=123456
redisctl cloud subscription get $SUB_ID > subscription-$SUB_ID.json
redisctl cloud database list --subscription $SUB_ID > databases-$SUB_ID.json
```

## Next Steps

- [Enterprise Quick Examples](./enterprise-examples.md) - Enterprise-specific examples
- [Cloud Overview](../cloud/overview.md) - Full Cloud documentation
- [Cloud Cookbook](../cookbook/README.md) - Practical recipes
