# Network Connectivity

Network connectivity operations support async tracking for VPC Peering, Private Service Connect (PSC), and Transit Gateway (TGW) configurations.

## VPC Peering

### Regular VPC Peering
```bash
# Create VPC peering
redisctl cloud connectivity vpc-peering create 12345 \
  --data @peering.json --wait

# Update VPC peering
redisctl cloud connectivity vpc-peering update 12345 67890 \
  --data @updates.json --wait

# Delete VPC peering
redisctl cloud connectivity vpc-peering delete 12345 67890 \
  --force --wait
```

### Active-Active VPC Peering
```bash
# Create Active-Active VPC peering
redisctl cloud connectivity vpc-peering create-aa 12345 \
  --data @aa-peering.json --wait

# Update Active-Active VPC peering
redisctl cloud connectivity vpc-peering update-aa 12345 67890 \
  --data @updates.json --wait

# Delete Active-Active VPC peering
redisctl cloud connectivity vpc-peering delete-aa 12345 67890 \
  --force --wait
```

## Private Service Connect (GCP)

### Regular PSC
```bash
# Create PSC
redisctl cloud connectivity psc create 12345 \
  --data @psc.json --wait

# Update PSC
redisctl cloud connectivity psc update 12345 67890 \
  --data @updates.json --wait

# Delete PSC
redisctl cloud connectivity psc delete 12345 67890 \
  --force --wait
```

### Active-Active PSC
```bash
# Create Active-Active PSC
redisctl cloud connectivity psc create-aa 12345 \
  --data @aa-psc.json --wait

# Update Active-Active PSC
redisctl cloud connectivity psc update-aa 12345 67890 \
  --data @updates.json --wait

# Delete Active-Active PSC
redisctl cloud connectivity psc delete-aa 12345 67890 \
  --force --wait
```

## Transit Gateway (AWS)

### Regular Transit Gateway
```bash
# Create Transit Gateway
redisctl cloud connectivity tgw create 12345 \
  --data @tgw.json --wait

# Attach Transit Gateway
redisctl cloud connectivity tgw attach 12345 67890 \
  --data @attach.json --wait

# Detach Transit Gateway
redisctl cloud connectivity tgw detach 12345 67890 \
  --force --wait

# Delete Transit Gateway
redisctl cloud connectivity tgw delete 12345 67890 \
  --force --wait
```

### Active-Active Transit Gateway
```bash
# Create Active-Active TGW
redisctl cloud connectivity tgw create-aa 12345 \
  --data @aa-tgw.json --wait

# Attach Active-Active TGW
redisctl cloud connectivity tgw attach-aa 12345 67890 \
  --data @attach.json --wait

# Detach Active-Active TGW
redisctl cloud connectivity tgw detach-aa 12345 67890 \
  --force --wait
```

## Configuration Examples

### VPC Peering Configuration
```json
{
  "vpcId": "vpc-12345678",
  "vpcCidr": "10.0.0.0/16",
  "awsAccountId": "123456789012",
  "region": "us-east-1"
}
```

### PSC Configuration
```json
{
  "projectId": "my-gcp-project",
  "network": "default",
  "serviceAttachmentName": "redis-psc-attachment"
}
```

### Transit Gateway Configuration
```json
{
  "transitGatewayId": "tgw-12345678",
  "attachmentId": "tgw-attach-87654321",
  "region": "us-west-2",
  "cidrs": ["10.0.0.0/16", "10.1.0.0/16"]
}
```

## Network Connectivity Best Practices

### Setup Order
1. Create subscription in target region
2. Configure network connectivity
3. Wait for connectivity to be established
4. Create databases
5. Test connectivity from your VPC

### Monitoring Connectivity
```bash
# List all VPC peerings
redisctl cloud connectivity vpc-peering list 12345

# Check PSC status
redisctl cloud connectivity psc list 12345

# Monitor TGW attachments
redisctl cloud connectivity tgw list 12345
```

### Automation Example
```bash
#!/bin/bash
# Setup complete network infrastructure
SUBSCRIPTION_ID=12345

# Create VPC peering
redisctl cloud connectivity vpc-peering create $SUBSCRIPTION_ID \
  --data @vpc-peering.json --wait

# Get peering ID
PEERING_ID=$(redisctl cloud connectivity vpc-peering list $SUBSCRIPTION_ID \
  -q "[0].id" -o json)

# Wait for peering to be active
while true; do
  STATUS=$(redisctl cloud connectivity vpc-peering get $SUBSCRIPTION_ID $PEERING_ID \
    -q "status" -o json)
  if [ "$STATUS" = "active" ]; then
    break
  fi
  sleep 10
done

echo "Network connectivity established!"
```

## Troubleshooting

### Common Issues
- **Timeout on creation**: Network operations can take 10-15 minutes in some regions
- **CIDR conflicts**: Ensure no overlapping CIDRs between your VPC and Redis deployment
- **Permissions**: Verify IAM roles and permissions for cross-account access
- **Region mismatch**: Ensure subscription and network resources are in the same region