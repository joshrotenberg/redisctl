# User & Account Management

User and provider account operations support async tracking for creation, updates, and deletion.

## User Operations

### Delete User
```bash
# Delete user with confirmation
redisctl cloud user delete 123 --wait

# Force delete without confirmation
redisctl cloud user delete 123 --force --wait
```

### List and Get Users
```bash
# List all users
redisctl cloud user list -o table

# Get specific user details
redisctl cloud user get 123
```

## Provider Account Operations

Provider accounts allow Redis Cloud to access your cloud provider resources.

### Create Provider Account

#### AWS Account
```bash
# Create AWS provider account
redisctl cloud provider-account create --file @aws-account.json --wait

# aws-account.json
{
  "name": "Production AWS",
  "provider": "AWS",
  "accessKeyId": "AKIAIOSFODNN7EXAMPLE",
  "accessSecretKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
}
```

#### GCP Account
```bash
# Create GCP provider account
redisctl cloud provider-account create --file @gcp-account.json --wait

# gcp-account.json (or use service account JSON directly)
{
  "name": "Production GCP",
  "provider": "GCP",
  "serviceAccountJson": "{\"type\":\"service_account\",...}"
}
```

#### Azure Account
```bash
# Create Azure provider account
redisctl cloud provider-account create --file @azure-account.json --wait

# azure-account.json
{
  "name": "Production Azure",
  "provider": "Azure",
  "subscriptionId": "12345678-1234-1234-1234-123456789012",
  "tenantId": "87654321-4321-4321-4321-210987654321",
  "clientId": "abcdef12-3456-7890-abcd-ef1234567890",
  "clientSecret": "super-secret-password"
}
```

### Update Provider Account
```bash
# Update provider account credentials
redisctl cloud provider-account update 456 \
  --file @updated-credentials.json --wait
```

### Delete Provider Account
```bash
# Delete provider account
redisctl cloud provider-account delete 456 --force --wait
```

## User Management Examples

### User Roles and Permissions

Users in Redis Cloud can have different roles:
- **Owner**: Full access to all resources
- **Admin**: Manage databases and users
- **Member**: Limited access based on permissions
- **Viewer**: Read-only access

### Create User with Specific Role
```bash
# Note: User creation typically doesn't support --wait as it's synchronous
redisctl cloud user create \
  --email "user@example.com" \
  --first-name "John" \
  --last-name "Doe" \
  --role "member"
```

### Managing User Alerts
```bash
# Update user alert preferences
redisctl cloud user update 123 \
  --alerts-email true \
  --alerts-sms false
```

## Provider Account Best Practices

### Security Guidelines

1. **Use IAM Roles**: Prefer IAM roles over access keys when possible
2. **Rotate Credentials**: Regularly update provider account credentials
3. **Minimum Permissions**: Grant only required permissions
4. **Audit Access**: Regular review provider account usage

### AWS IAM Policy Example
```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "ec2:*",
        "vpc:*",
        "cloudwatch:*"
      ],
      "Resource": "*"
    }
  ]
}
```

### GCP Service Account Setup
```bash
# Create service account
gcloud iam service-accounts create redis-cloud \
  --display-name="Redis Cloud Access"

# Grant required roles
gcloud projects add-iam-policy-binding PROJECT_ID \
  --member="serviceAccount:redis-cloud@PROJECT_ID.iam.gserviceaccount.com" \
  --role="roles/compute.admin"

# Create and download key
gcloud iam service-accounts keys create redis-cloud-key.json \
  --iam-account=redis-cloud@PROJECT_ID.iam.gserviceaccount.com

# Use key file with redisctl
redisctl cloud provider-account create --file @redis-cloud-key.json --wait
```

## Automation Examples

### Bulk User Management
```bash
#!/bin/bash
# Remove inactive users
INACTIVE_USERS=$(redisctl cloud user list \
  -q "[?lastLogin < '2024-01-01'].id" -o json)

for USER_ID in $INACTIVE_USERS; do
  echo "Deleting user $USER_ID"
  redisctl cloud user delete $USER_ID --force --wait
done
```

### Provider Account Rotation
```bash
#!/bin/bash
# Rotate AWS credentials
NEW_KEY=$(aws iam create-access-key --user-name redis-cloud)

cat > new-aws-account.json <<EOF
{
  "provider": "AWS",
  "accessKeyId": "$(echo $NEW_KEY | jq -r '.AccessKey.AccessKeyId')",
  "accessSecretKey": "$(echo $NEW_KEY | jq -r '.AccessKey.SecretAccessKey')"
}
EOF

redisctl cloud provider-account update 456 \
  --file @new-aws-account.json --wait

# Delete old access key
aws iam delete-access-key --access-key-id OLD_KEY_ID --user-name redis-cloud
```