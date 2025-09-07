# Subscription Management

Subscription operations support full async tracking for both regular and fixed subscriptions.

## Regular Subscriptions

### Create Subscription
```bash
redisctl cloud subscription create --data @subscription.json --wait

# With custom timeout for complex configurations
redisctl cloud subscription create --data @complex-sub.json \
  --wait --wait-timeout 1200
```

### Update Subscription
```bash
redisctl cloud subscription update 12345 --data @updates.json \
  --wait --wait-interval 5
```

### Delete Subscription
```bash
redisctl cloud subscription delete 12345 --force --wait
```

## Fixed Subscriptions

Fixed subscriptions follow the same async patterns:

```bash
# Create fixed subscription
redisctl cloud fixed-subscription create --data @fixed-sub.json --wait

# Update fixed subscription
redisctl cloud fixed-subscription update 12345 --data @updates.json --wait

# Delete fixed subscription
redisctl cloud fixed-subscription delete 12345 --force --wait
```

## Subscription Configuration Examples

### Basic Subscription
```json
{
  "name": "dev-subscription",
  "cloudProviders": [
    {
      "provider": "AWS",
      "regions": [
        {
          "region": "us-east-1",
          "networking": {
            "deploymentCIDR": "10.0.0.0/24"
          }
        }
      ]
    }
  ]
}
```

### Multi-Region Subscription
```json
{
  "name": "global-subscription",
  "cloudProviders": [
    {
      "provider": "AWS",
      "regions": [
        {
          "region": "us-east-1",
          "networking": {
            "deploymentCIDR": "10.0.0.0/24"
          }
        },
        {
          "region": "eu-west-1",
          "networking": {
            "deploymentCIDR": "10.1.0.0/24"
          }
        }
      ]
    }
  ]
}
```

### Fixed Subscription Plan
```json
{
  "name": "fixed-plan-subscription",
  "planId": "fixed-100gb",
  "paymentMethod": "credit-card",
  "cloudProviders": [
    {
      "provider": "GCP",
      "regions": [
        {
          "region": "us-central1"
        }
      ]
    }
  ]
}
```

## Monitoring Subscription Operations

```bash
# List all subscriptions with status
redisctl cloud subscription list -o table

# Get specific subscription details
redisctl cloud subscription get 12345

# Check pending operations
redisctl cloud task list --status pending
```

## Best Practices

### Subscription Lifecycle
1. Create subscription with appropriate timeout
2. Wait for subscription to be active
3. Create databases within subscription
4. Monitor usage and costs
5. Update as needed with async tracking

### Automation Example
```bash
#!/bin/bash
# Create subscription and databases
SUB_RESPONSE=$(redisctl cloud subscription create \
  --data @subscription.json --wait -o json)

SUB_ID=$(echo "$SUB_RESPONSE" | jq -r '.id')

# Create multiple databases
for db in database1.json database2.json database3.json; do
  redisctl cloud database create --subscription-id $SUB_ID \
    --data @$db --wait &
done

wait
echo "Subscription $SUB_ID ready with all databases!"
```