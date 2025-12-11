# Subscriptions

Manage Redis Cloud subscriptions - the containers for your databases and configuration.

## Commands

### List Subscriptions

List all subscriptions in your account.

```bash
redisctl cloud subscription list [OPTIONS]
```

**Options:**
- `-o, --output <FORMAT>` - Output format: json, yaml, or table (default: auto)
- `-q, --query <JMESPATH>` - JMESPath query to filter output

**Examples:**

```bash
# List all subscriptions
redisctl cloud subscription list

# Table format with specific fields
redisctl cloud subscription list -o table

# Get only subscription IDs and names
redisctl cloud subscription list -q "[].{id: id, name: name}"

# Filter by status
redisctl cloud subscription list -q "[?status=='active']"
```

### Get Subscription

Get details of a specific subscription.

```bash
redisctl cloud subscription get <ID> [OPTIONS]
```

**Arguments:**
- `<ID>` - Subscription ID

**Options:**
- `-o, --output <FORMAT>` - Output format: json, yaml, or table
- `-q, --query <JMESPATH>` - JMESPath query to filter output

**Examples:**

```bash
# Get subscription details
redisctl cloud subscription get 123456

# Get specific fields in YAML
redisctl cloud subscription get 123456 -o yaml -q "{name: name, status: status, databases: numberOfDatabases}"
```

### Create Subscription

Create a new subscription.

```bash
redisctl cloud subscription create --data <JSON> [OPTIONS]
```

**Options:**
- `--data <JSON>` - JSON payload (inline or @file.json)
- `--wait` - Wait for operation to complete
- `--wait-timeout <SECONDS>` - Maximum time to wait (default: 600)
- `--wait-interval <SECONDS>` - Polling interval (default: 10)

**Example Payload:**

```json
{
  "name": "Production Subscription",
  "cloudProvider": {
    "provider": "AWS",
    "regions": [
      {
        "region": "us-east-1",
        "multipleAvailabilityZones": true,
        "networking": {
          "deploymentCIDR": "10.0.0.0/24"
        }
      }
    ]
  },
  "databases": [
    {
      "name": "cache-db",
      "memoryLimitInGb": 1,
      "throughputMeasurement": {
        "by": "operations-per-second",
        "value": 10000
      }
    }
  ]
}
```

**Examples:**

```bash
# Create subscription from file
redisctl cloud subscription create --data @subscription.json

# Create and wait for completion
redisctl cloud subscription create --data @subscription.json --wait

# Create with inline JSON
redisctl cloud subscription create --data '{
  "name": "Test Subscription",
  "cloudProvider": {"provider": "AWS", "regions": [{"region": "us-east-1"}]}
}'
```

### Update Subscription

Update an existing subscription.

```bash
redisctl cloud subscription update <ID> --data <JSON> [OPTIONS]
```

**Arguments:**
- `<ID>` - Subscription ID

**Options:**
- `--data <JSON>` - JSON payload with updates
- `--wait` - Wait for operation to complete
- `--wait-timeout <SECONDS>` - Maximum time to wait
- `--wait-interval <SECONDS>` - Polling interval

**Examples:**

```bash
# Update subscription name
redisctl cloud subscription update 123456 --data '{"name": "New Name"}'

# Update payment method
redisctl cloud subscription update 123456 --data '{"paymentMethodId": 8840}' --wait
```

### Delete Subscription

Delete a subscription (requires all databases to be deleted first).

```bash
redisctl cloud subscription delete <ID> [OPTIONS]
```

**Arguments:**
- `<ID>` - Subscription ID

**Options:**
- `--wait` - Wait for deletion to complete
- `--wait-timeout <SECONDS>` - Maximum time to wait
- `--wait-interval <SECONDS>` - Polling interval

**Examples:**

```bash
# Delete subscription
redisctl cloud subscription delete 123456

# Delete and wait for completion
redisctl cloud subscription delete 123456 --wait
```

## Fixed Subscriptions

Fixed subscriptions offer reserved capacity with predictable pricing.

### List Fixed Subscriptions

```bash
redisctl cloud fixed-subscription list
```

### Get Fixed Subscription

```bash
redisctl cloud fixed-subscription get <ID>
```

### Create Fixed Subscription

```bash
redisctl cloud fixed-subscription create --data @fixed-subscription.json --wait
```

**Example Payload:**

```json
{
  "name": "Fixed Production",
  "plan": {
    "provider": "AWS",
    "region": "us-east-1",
    "size": "r5.xlarge"
  },
  "quantity": 2
}
```

## Related Commands

- Databases - Manage databases within subscriptions
- Network Connectivity - Configure VPC peering and private endpoints
- Provider Accounts - Manage cloud provider integrations

## Common Patterns

### List All Databases Across Subscriptions

```bash
# Get all subscription IDs
SUBS=$(redisctl cloud subscription list -q "[].id" | jq -r '.[]')

# List databases for each subscription
for sub in $SUBS; do
  echo "Subscription $sub:"
  redisctl cloud database list --subscription $sub
done
```

### Monitor Subscription Usage

```bash
# Get memory usage across all databases
redisctl cloud subscription get 123456 -q "databases[].{name: name, memory: memoryLimitInGb}" | \
  jq -r '.[] | "\(.name): \(.memory)GB"'
```

## Troubleshooting

### Common Issues

**"Subscription not found"**
- Verify the subscription ID is correct
- Check that your API key has access to the subscription

**"Cannot delete subscription with active databases"**
- Delete all databases first: `redisctl cloud database list --subscription <ID>`
- Then delete each database before deleting the subscription

**"Operation timeout"**
- Increase timeout: `--wait-timeout 1200`
- Check operation status: `redisctl cloud task get <TASK_ID>`

## API Reference

These commands use the following REST endpoints:
- `GET /v1/subscriptions` - List subscriptions
- `GET /v1/subscriptions/{id}` - Get subscription
- `POST /v1/subscriptions` - Create subscription
- `PUT /v1/subscriptions/{id}` - Update subscription
- `DELETE /v1/subscriptions/{id}` - Delete subscription

For direct API access, use: `redisctl api cloud get /subscriptions`