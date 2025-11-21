# Cloud Networking

Configure VPC peering, Private Service Connect (PSC), and Transit Gateway for Redis Cloud.

## VPC Peering

Connect your Redis Cloud subscription to your VPC.

### List VPC Peerings

```bash
redisctl cloud vpc-peering list --subscription <ID>
```

### Get VPC Peering

```bash
redisctl cloud vpc-peering get --subscription <ID> --peering-id <PEERING_ID>
```

### Create VPC Peering

```bash
redisctl cloud vpc-peering create --subscription <ID> --data '{
  "region": "us-east-1",
  "awsAccountId": "123456789012",
  "vpcId": "vpc-abc123",
  "vpcCidr": "10.0.0.0/16"
}' --wait
```

### Delete VPC Peering

```bash
redisctl cloud vpc-peering delete --subscription <ID> --peering-id <PEERING_ID> --wait
```

### AWS Setup

After creating the peering in redisctl:

1. Get the peering request ID from the response
2. In AWS Console, go to VPC → Peering Connections
3. Accept the peering request
4. Update route tables to route traffic to Redis Cloud CIDR

## Private Service Connect (GCP)

### Create PSC Service

```bash
redisctl cloud psc create-service --subscription <ID> --data '{
  "region": "us-central1"
}' --wait
```

### Create PSC Endpoint

```bash
redisctl cloud psc create-endpoint --subscription <ID> --data '{
  "serviceId": "psc-123",
  "endpointName": "redis-endpoint"
}' --wait
```

### List PSC Services

```bash
redisctl cloud psc list-services --subscription <ID>
```

## Transit Gateway (AWS)

### Create Transit Gateway Attachment

```bash
redisctl cloud tgw create --subscription <ID> --data '{
  "region": "us-east-1",
  "transitGatewayId": "tgw-abc123",
  "cidrs": ["10.0.0.0/16"]
}' --wait
```

### List Transit Gateway Attachments

```bash
redisctl cloud tgw list --subscription <ID>
```

### Delete Transit Gateway Attachment

```bash
redisctl cloud tgw delete --subscription <ID> --tgw-id <TGW_ID> --wait
```

## CIDR Allowlist

Control which IP ranges can access your subscription.

### Get CIDR Allowlist

```bash
redisctl cloud subscription get-cidr --subscription <ID>
```

### Update CIDR Allowlist

```bash
redisctl cloud subscription update-cidr --subscription <ID> --data '{
  "cidrIps": ["10.0.0.0/16", "192.168.1.0/24"],
  "securityGroupIds": ["sg-abc123"]
}'
```

## Examples

### Set Up AWS VPC Peering

```bash
# Create peering
PEERING=$(redisctl cloud vpc-peering create \
  --subscription 123456 \
  --data '{
    "region": "us-east-1",
    "awsAccountId": "123456789012",
    "vpcId": "vpc-abc123",
    "vpcCidr": "10.0.0.0/16"
  }' --wait)

echo "Accept peering request in AWS Console"
echo "Peering ID: $(echo $PEERING | jq -r '.vpcPeeringId')"
```

### List All Network Connections

```bash
# VPC peerings
redisctl cloud vpc-peering list --subscription 123456 -o table

# PSC services  
redisctl cloud psc list-services --subscription 123456 -o table

# Transit gateways
redisctl cloud tgw list --subscription 123456 -o table
```

## Active-Active Networking

For Active-Active subscriptions, use the `--active-active` flag:

```bash
redisctl cloud vpc-peering create-active-active \
  --subscription <ID> \
  --region us-east-1 \
  --data '{...}' --wait
```

## Troubleshooting

### Peering Stuck in Pending

- Ensure you've accepted the peering request in your cloud console
- Verify the VPC CIDR doesn't overlap with Redis Cloud CIDR
- Check IAM permissions for peering operations

### Cannot Connect After Peering

- Update route tables in your VPC
- Check security group rules allow Redis ports (default: 10000+)
- Verify DNS resolution if using private endpoints

## API Reference

These commands use the following REST endpoints:
- `GET/POST /v1/subscriptions/{id}/peerings` - VPC peering
- `GET/POST /v1/subscriptions/{id}/privateServiceConnect` - PSC
- `GET/POST /v1/subscriptions/{id}/transitGateway` - Transit Gateway

For direct API access: `redisctl api cloud get /subscriptions/123456/peerings`
