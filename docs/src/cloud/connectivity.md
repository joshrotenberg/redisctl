# Network Connectivity

Configure VPC peering, Private Service Connect, and Transit Gateway connections for secure network access.

## VPC Peering

### List VPC Peerings

```bash
redisctl cloud connectivity list-vpc --subscription-id <ID>
```

### Create VPC Peering

```bash
redisctl cloud connectivity create-vpc --subscription-id <ID> --data @vpc.json --wait
```

**Example Payload:**
```json
{
  "region": "us-east-1",
  "awsAccountId": "123456789012",
  "vpcId": "vpc-0abc123def456",
  "vpcCidr": "10.0.0.0/16"
}
```

### Delete VPC Peering

```bash
redisctl cloud connectivity delete-vpc --subscription-id <ID> --peering-id <PEERING_ID> --wait
```

## Private Service Connect (GCP)

### Create PSC

```bash
redisctl cloud connectivity create-psc --subscription-id <ID> --data @psc.json --wait
```

## Transit Gateway (AWS)

### Create Transit Gateway Attachment

```bash
redisctl cloud connectivity create-tgw --subscription-id <ID> --data @tgw.json --wait
```

## Common Patterns

### Verify Connectivity

```bash
# List all connections
redisctl cloud connectivity list-vpc --subscription-id 123456

# Check connection status
redisctl cloud connectivity get-vpc --subscription-id 123456 --peering-id abc123 \
  -q "status"
```

## Troubleshooting

- Ensure CIDR blocks don't overlap
- Verify cloud provider permissions
- Check security group rules allow Redis ports