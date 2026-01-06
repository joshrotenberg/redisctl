# Networking Commands

VPC peering and private connectivity for Redis Cloud.

## VPC Peering

### List Peerings

```bash
redisctl cloud vpc-peering list --subscription-id 123456
```

### Create Peering

```bash
redisctl cloud vpc-peering create --subscription-id 123456 --data '{
  "region": "us-east-1",
  "awsAccountId": "123456789012",
  "vpcId": "vpc-abc123",
  "vpcCidr": "10.0.0.0/16"
}' --wait
```

### Delete Peering

```bash
redisctl cloud vpc-peering delete --subscription-id 123456 --peering-id 789 --wait
```

## Private Service Connect (GCP)

### List PSC Endpoints

```bash
redisctl cloud psc list --subscription-id 123456
```

### Create PSC Endpoint

```bash
redisctl cloud psc create --subscription-id 123456 --data '{
  "gcpProjectId": "my-project",
  "gcpNetworkName": "my-network"
}' --wait
```

## Transit Gateway (AWS)

### List Attachments

```bash
redisctl cloud transit-gateway list --subscription-id 123456
```

## Common Patterns

### Check Peering Status

```bash
redisctl cloud vpc-peering list --subscription-id 123456 -o json -q '[].{
  id: id,
  status: status,
  vpcId: vpcId
}'
```

### Wait for Peering Active

```bash
redisctl cloud vpc-peering create \
  --subscription-id 123456 \
  --data @peering.json \
  --wait \
  --wait-timeout 600
```

## Raw API Access

```bash
# VPC Peerings
redisctl api cloud get /subscriptions/123456/peerings

# Private endpoints
redisctl api cloud get /subscriptions/123456/privateEndpoints
```

## Related

- [Subscriptions](subscriptions.md) - Subscription management
- [VPC Peering Cookbook](../../cookbook/cloud/vpc-peering.md) - Step-by-step guide
