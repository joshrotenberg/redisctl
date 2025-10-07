# Setup VPC Peering

Time: 15-20 minutes  
Prerequisites:
- Redis Cloud subscription with database
- AWS/GCP/Azure VPC to peer with
- Network admin access to your cloud provider
- redisctl configured with Cloud credentials

## What is VPC Peering?

VPC Peering creates a private network connection between your Redis Cloud subscription and your application's VPC, eliminating public internet exposure and reducing latency.

## Quick Command

If you already have your VPC details:

```bash
redisctl cloud connectivity vpc-peering create \
  --subscription-id YOUR_SUB_ID \
  --data '{
    "provider_name": "AWS",
    "aws_account_id": "123456789012",
    "vpc_id": "vpc-abc123",
    "vpc_cidr": "10.0.0.0/16",
    "region": "us-east-1"
  }' \
  --wait
```

## Step-by-Step Guide

### 1. Get Your Subscription Details

First, identify which subscription to peer:

```bash
redisctl cloud subscription list -o table -q '[].{id: id, name: name, region: "deployment.regions[0].region"}'
```

**Example output:**
```
┌────┬──────────────┬───────────┐
│ id │ name         │ region    │
├────┼──────────────┼───────────┤
│ 42 │ production   │ us-east-1 │
└────┴──────────────┴───────────┘
```

### 2. Gather Your VPC Information

You'll need these details from your cloud provider:

**For AWS:**
- AWS Account ID (12-digit number)
- VPC ID (starts with `vpc-`)
- VPC CIDR block (e.g., `10.0.0.0/16`)
- Region (must match Redis Cloud region)

**For GCP:**
- GCP Project ID
- Network name
- Region

**For Azure:**
- Subscription ID (Azure subscription, not Redis)
- Resource group
- VNet name
- Region

### 3. Create VPC Peering Request

#### AWS Example

```bash
redisctl cloud connectivity vpc-peering create \
  --subscription-id 42 \
  --data '{
    "provider_name": "AWS",
    "aws_account_id": "123456789012",
    "vpc_id": "vpc-abc123def",
    "vpc_cidr": "10.0.0.0/16",
    "region": "us-east-1"
  }' \
  --wait \
  --wait-timeout 600
```

#### GCP Example

```bash
redisctl cloud connectivity vpc-peering create \
  --subscription-id 42 \
  --data '{
    "provider_name": "GCP",
    "gcp_project_id": "my-project-123",
    "network_name": "my-vpc-network",
    "gcp_redis_project_id": "redis-project-456",
    "gcp_redis_network_name": "redis-network",
    "region": "us-central1"
  }' \
  --wait
```

#### Azure Example

```bash
redisctl cloud connectivity vpc-peering create \
  --subscription-id 42 \
  --data '{
    "provider_name": "Azure",
    "azure_subscription_id": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
    "resource_group": "my-resource-group",
    "vnet_name": "my-vnet",
    "region": "East US"
  }' \
  --wait
```

**What you should see:**

```json
{
  "taskId": "xyz789...",
  "status": "processing"
}
...
VPC Peering created successfully!
{
  "vpc_peering_id": 123,
  "status": "pending-acceptance",
  "provider_name": "AWS",
  "aws_peering_id": "pcx-abc123def"
}
```

### 4. Accept the Peering Connection (AWS Only)

For AWS, you must accept the peering request in your AWS console:

```bash
# Get the AWS peering connection ID
redisctl cloud connectivity vpc-peering get \
  --subscription-id 42 \
  --peering-id 123 \
  -q 'aws_peering_id'

# Output: "pcx-abc123def"
```

**In AWS Console:**
1. Go to VPC Dashboard
2. Click "Peering Connections"
3. Find connection `pcx-abc123def`
4. Click "Actions" > "Accept Request"

**Via AWS CLI:**
```bash
aws ec2 accept-vpc-peering-connection \
  --vpc-peering-connection-id pcx-abc123def \
  --region us-east-1
```

### 5. Update Route Tables

Add routes to your VPC route tables to direct Redis traffic through the peering connection:

```bash
# Get Redis Cloud CIDR
redisctl cloud subscription get \
  --subscription-id 42 \
  -q 'deployment.regions[0].networking.cidr'

# Output: "172.31.0.0/24"
```

**AWS Example:**
```bash
aws ec2 create-route \
  --route-table-id rtb-abc123 \
  --destination-cidr-block 172.31.0.0/24 \
  --vpc-peering-connection-id pcx-abc123def
```

### 6. Update Security Groups

Allow inbound Redis traffic (port 6379 or your database port):

```bash
aws ec2 authorize-security-group-ingress \
  --group-id sg-abc123 \
  --protocol tcp \
  --port 6379 \
  --cidr 172.31.0.0/24
```

### 7. Verify the Connection

Check peering status:

```bash
redisctl cloud connectivity vpc-peering get \
  --subscription-id 42 \
  --peering-id 123 \
  -o json -q '{status: status, aws_peering_id: aws_peering_id}'
```

**Expected status:** `active`

### 8. Test Connectivity

Get your database endpoint and test from an instance in your VPC:

```bash
# Get database endpoint
redisctl cloud database get \
  --subscription-id 42 \
  --database-id 12345 \
  -q 'private_endpoint'

# From an EC2 instance in your VPC:
redis-cli -h redis-12345.internal.cloud.redislabs.com -p 12345 PING
```

## Advanced: Active-Active VPC Peering

For Active-Active (CRDB) databases, peer with each region:

```bash
# List Active-Active regions
redisctl cloud subscription get --subscription-id 42 \
  -q 'deployment.regions[].{region: region, cidr: networking.cidr}'

# Create peering for each region
redisctl cloud connectivity vpc-peering create-aa \
  --subscription-id 42 \
  --region-id 1 \
  --data '{
    "provider_name": "AWS",
    "aws_account_id": "123456789012",
    "vpc_id": "vpc-east-123",
    "vpc_cidr": "10.0.0.0/16",
    "region": "us-east-1"
  }' \
  --wait
```

## Using Configuration Files

For complex setups, use a JSON file:

```bash
cat > vpc-peering.json << 'EOF'
{
  "provider_name": "AWS",
  "aws_account_id": "123456789012",
  "vpc_id": "vpc-abc123def",
  "vpc_cidr": "10.0.0.0/16",
  "region": "us-east-1",
  "vpc_peering_name": "production-redis-peer"
}
EOF

redisctl cloud connectivity vpc-peering create \
  --subscription-id 42 \
  --data @vpc-peering.json \
  --wait
```

## Common Issues

### Peering Request Times Out

```
Error: VPC peering creation timed out
```

**Solution:** Check async operation status manually:
```bash
redisctl cloud action get --task-id xyz789...
```

### CIDR Overlap

```
Error: VPC CIDR blocks overlap
```

**Solution:** Redis Cloud and your VPC cannot have overlapping CIDR blocks. Either:
1. Choose a different CIDR for new subscription
2. Use a different VPC with non-overlapping CIDR

### Peering Stuck in "pending-acceptance"

**Solution:** For AWS, you must manually accept the peering request (see Step 4)

### Cannot Connect After Peering

**Troubleshooting checklist:**
1. Verify peering status is `active`
2. Check route tables have correct routes
3. Verify security groups allow Redis port
4. Ensure database has private endpoint enabled
5. Test from instance actually in the peered VPC

## Monitoring VPC Peering

List all peerings for a subscription:

```bash
redisctl cloud connectivity vpc-peering list \
  --subscription-id 42 \
  -o table \
  -q '[].{id: id, status: status, provider: provider_name, region: region}'
```

## Deleting VPC Peering

```bash
redisctl cloud connectivity vpc-peering delete \
  --subscription-id 42 \
  --peering-id 123 \
  --wait
```

This also removes the peering from your cloud provider.

## Next Steps

- [Configure ACL Security](configure-acls.md) - Secure your private database
- [Setup Private Service Connect](setup-psc.md) - Alternative private connectivity for GCP
- [Configure Transit Gateway](setup-tgw.md) - Multi-VPC connectivity for AWS
- [Monitor Performance](../common/monitor-performance.md) - Track latency improvements

## See Also

- [VPC Peering Command Reference](../../cloud/connectivity/vpc-peering.md) - Complete command documentation
- [Redis Cloud Networking Guide](https://redis.io/docs/latest/operate/rc/security/vpc-peering/) - Official docs
- [AWS VPC Peering](https://docs.aws.amazon.com/vpc/latest/peering/) - AWS documentation
