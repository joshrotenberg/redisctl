# Subscriptions

Manage Redis Cloud subscriptions.

## Commands

| Command | Description |
|---------|-------------|
| `list` | List all subscriptions |
| `get` | Get subscription details |
| `create` | Create a new subscription |
| `update` | Update subscription settings |
| `delete` | Delete a subscription |

## List Subscriptions

```bash
redisctl cloud subscription list
```

### Examples

```bash
# List all subscriptions
redisctl cloud subscription list

# Get just names
redisctl cloud subscription list -o json -q '[].name'

# Filter active only
redisctl cloud subscription list -o json -q '[?status == `active`]'
```

## Get Subscription

```bash
redisctl cloud subscription get <subscription-id>
```

## Create Subscription

```bash
redisctl cloud subscription create \
  --name "my-subscription" \
  --cloud-provider AWS \
  --region us-east-1 \
  --wait
```

### Required Options

| Option | Description |
|--------|-------------|
| `--name` | Subscription name |
| `--cloud-provider` | AWS, GCP, or Azure |
| `--region` | Cloud region |

## Delete Subscription

```bash
redisctl cloud subscription delete <subscription-id> --wait
```

!!! warning
    Deleting a subscription removes all databases within it.
