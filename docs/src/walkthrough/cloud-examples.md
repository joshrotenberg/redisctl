# Cloud Quick Examples

Examples showing the three-tier model for Redis Cloud.

## API Layer

Direct REST access:

```bash
# Get account info
redisctl api cloud get /account

# List subscriptions
redisctl api cloud get /subscriptions

# Create a database (raw JSON)
redisctl api cloud post /subscriptions/123/databases -d '{...}'
```

## Human Commands

Type-safe operations:

```bash
# List databases
redisctl cloud database list

# Get database details
redisctl cloud database get 123456 789

# Create a database with parameters
redisctl cloud database create --subscription-id 123456 --name mydb --memory 1024
```

## Workflows

Multi-step operations:

```bash
# Set up a complete subscription with VPC peering
redisctl cloud workflow subscription-setup --name production --region us-east-1
```
