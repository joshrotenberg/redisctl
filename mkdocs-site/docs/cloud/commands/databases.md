# Database Commands

Manage databases within Redis Cloud subscriptions.

## List Databases

```bash
redisctl cloud database list --subscription-id <id>
```

### Examples

```bash
# List all databases in a subscription
redisctl cloud database list --subscription-id 123456

# As JSON
redisctl cloud database list --subscription-id 123456 -o json

# Just names and endpoints
redisctl cloud database list --subscription-id 123456 -o json -q '[].{
  name: name,
  endpoint: publicEndpoint
}'
```

## Get Database Details

```bash
redisctl cloud database get <subscription-id> <database-id>
```

### Examples

```bash
# Full details
redisctl cloud database get 123456 789

# Connection info
redisctl cloud database get 123456 789 -o json -q '{
  endpoint: publicEndpoint,
  password: password
}'

# Memory and status
redisctl cloud database get 123456 789 -o json -q '{
  name: name,
  memory_gb: memoryLimitInGb,
  status: status
}'
```

## Create Database

```bash
redisctl cloud database create \
  --subscription-id 123456 \
  --name mydb \
  --memory-limit-in-gb 1 \
  --wait
```

### Options

| Option | Description |
|--------|-------------|
| `--subscription-id` | Subscription ID (required) |
| `--name` | Database name |
| `--memory-limit-in-gb` | Memory size in GB |
| `--data-eviction-policy` | Eviction policy |
| `--replication` | Enable replication |
| `--wait` | Wait for completion |
| `--data` | Full JSON configuration |

### Create with Full Config

```bash
redisctl cloud database create \
  --subscription-id 123456 \
  --data '{
    "name": "cache",
    "memoryLimitInGb": 2,
    "dataEvictionPolicy": "volatile-lru",
    "replication": true
  }' \
  --wait
```

## Update Database

```bash
redisctl cloud database update <subscription-id> <database-id> \
  --memory-limit-in-gb 2 \
  --wait
```

### Scale Memory

```bash
redisctl cloud database update 123456 789 \
  --memory-limit-in-gb 4 \
  --wait
```

## Delete Database

```bash
redisctl cloud database delete <subscription-id> <database-id> --wait
```

!!! warning
    This permanently deletes the database. Add `--wait` to confirm deletion completes.

## Common Queries

### Get Connection String

```bash
ENDPOINT=$(redisctl cloud database get 123456 789 -o json -q 'publicEndpoint')
PASSWORD=$(redisctl cloud database get 123456 789 -o json -q 'password')
echo "redis://default:$PASSWORD@$ENDPOINT"
```

### Find All Databases Across Subscriptions

```bash
for sub in $(redisctl cloud subscription list -o json -q '[].id' | jq -r '.[]'); do
  echo "=== Subscription $sub ==="
  redisctl cloud database list --subscription-id "$sub" -o json -q '[].name'
done
```

### Database Size Summary

```bash
redisctl cloud database list --subscription-id 123456 -o json -q '[].{
  name: name,
  memory_gb: memoryLimitInGb,
  status: status
}'
```

## Raw API Access

```bash
# All databases in subscription
redisctl api cloud get /subscriptions/123456/databases

# Specific database
redisctl api cloud get /subscriptions/123456/databases/789
```

## Related Commands

- [Subscriptions](subscriptions.md) - Manage subscriptions
- [Access Control](access-control.md) - Users and ACLs
- [Tasks](tasks.md) - Monitor async operations
