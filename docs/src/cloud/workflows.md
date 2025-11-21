# Cloud Workflows

Multi-step operations that orchestrate multiple API calls with automatic polling and error handling.

## Available Workflows

### subscription-setup

Create a complete subscription with optional VPC peering and initial database.

```bash
redisctl cloud workflow subscription-setup \
  --name production \
  --cloud-provider aws \
  --region us-east-1 \
  --memory-limit-in-gb 10
```

**What it does:**
1. Creates the subscription
2. Waits for subscription to be active
3. Optionally sets up VPC peering
4. Optionally creates initial database
5. Returns connection details

**Options:**

| Option | Description |
|--------|-------------|
| `--name` | Subscription name |
| `--cloud-provider` | AWS, GCP, or Azure |
| `--region` | Cloud region |
| `--memory-limit-in-gb` | Total memory allocation |
| `--with-vpc-peering` | Set up VPC peering |
| `--vpc-id` | Your VPC ID (for peering) |
| `--vpc-cidr` | Your VPC CIDR (for peering) |
| `--create-database` | Create initial database |
| `--database-name` | Name for initial database |

**Example with VPC Peering:**

```bash
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

## When to Use Workflows

Use workflows when you need to:
- Perform multiple related operations in sequence
- Handle async operations with proper waiting
- Get a complete setup done in one command

Use individual commands when you need:
- Fine-grained control over each step
- Custom error handling
- Partial operations

## Creating Custom Workflows

For operations not covered by built-in workflows, you can script them:

```bash
#!/bin/bash
set -e

# Create subscription
SUB_ID=$(redisctl cloud subscription create \
  --data @subscription.json \
  --wait \
  -q 'id')

echo "Created subscription: $SUB_ID"

# Set up VPC peering
redisctl cloud vpc-peering create \
  --subscription-id $SUB_ID \
  --data @peering.json \
  --wait

echo "VPC peering created - accept in AWS console"

# Create database
DB_ID=$(redisctl cloud database create \
  --subscription-id $SUB_ID \
  --data @database.json \
  --wait \
  -q 'databaseId')

echo "Created database: $DB_ID"

# Get connection info
redisctl cloud database get \
  --subscription-id $SUB_ID \
  --database-id $DB_ID \
  -q '{endpoint: publicEndpoint, password: password}'
```

## Error Handling

Workflows handle errors gracefully:
- Failed steps report clear error messages
- Partial progress is preserved (you can resume manually)
- Resources created before failure remain (clean up if needed)

```bash
# If workflow fails, check what was created
redisctl cloud subscription list -o table
redisctl cloud vpc-peering list --subscription-id <ID>
```

## Comparison: Workflow vs Manual

**With workflow:**
```bash
redisctl cloud workflow subscription-setup \
  --name prod --cloud-provider aws --region us-east-1 \
  --memory-limit-in-gb 10 --create-database --database-name cache
```

**Manual equivalent:**
```bash
# 1. Create subscription
SUB=$(redisctl cloud subscription create --data '{...}' --wait)
SUB_ID=$(echo $SUB | jq -r '.id')

# 2. Wait for active status
while [ "$(redisctl cloud subscription get $SUB_ID -q 'status')" != "active" ]; do
  sleep 10
done

# 3. Create database
redisctl cloud database create --subscription-id $SUB_ID --data '{...}' --wait

# 4. Get connection info
redisctl cloud database list --subscription-id $SUB_ID
```

The workflow handles all the waiting, polling, and sequencing automatically.
