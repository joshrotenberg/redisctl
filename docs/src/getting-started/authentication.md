# Authentication

`redisctl` supports authentication for both Redis Cloud and Redis Enterprise deployments. This guide covers how to obtain and configure authentication credentials for each deployment type.

## Redis Cloud Authentication

Redis Cloud uses API key authentication with two components:
- **API Key** (Account Key) - Public identifier for your account
- **API Secret** (Secret Key) - Private secret for authentication

### Obtaining Cloud API Credentials

1. **Log in to Redis Cloud Console**
   - Navigate to [app.redislabs.com](https://app.redislabs.com)
   - Sign in with your account credentials

2. **Access API Keys**
   - Click on your name in the top-right corner
   - Select "Account Settings"
   - Navigate to the "API Keys" tab

3. **Create New API Key**
   - Click "Add API Key"
   - Provide a descriptive name (e.g., "redisctl-cli")
   - Select appropriate permissions:
     - **Read-Only**: For viewing resources only
     - **Read-Write**: For full management capabilities
   - Click "Create"

4. **Save Credentials**
   - Copy the **Account key** (this is your API key)
   - Copy the **Secret** (shown only once!)
   - Store these securely - you won't be able to see the secret again

### Configuring Cloud Authentication

#### Using Profiles (Recommended)

```bash
# Create a profile with your credentials
redisctl profile set my-cloud \
  --deployment-type cloud \
  --api-key "your-account-key" \
  --api-secret "your-secret-key"

# Set as default profile
redisctl profile default my-cloud

# Test authentication
redisctl cloud account info
```

#### Using Environment Variables

```bash
# Export credentials
export REDIS_CLOUD_API_KEY="your-account-key"
export REDIS_CLOUD_API_SECRET="your-secret-key"

# Optional: Set custom API URL
export REDIS_CLOUD_API_URL="https://api.redislabs.com/v1"

# Test authentication
redisctl cloud account info
```

#### Using Command-Line Flags

```bash
# Pass credentials directly (not recommended for production)
redisctl cloud subscription list \
  --api-key "your-account-key" \
  --api-secret "your-secret-key"
```

## Redis Enterprise Authentication

Redis Enterprise uses basic authentication with username and password. The username typically follows an email format.

### Default Credentials

For new Redis Enterprise installations:
- **Username**: `admin@cluster.local`
- **Password**: Set during cluster setup

### Obtaining Enterprise Credentials

1. **During Cluster Setup**
   - Credentials are set during initial cluster configuration
   - Default username is typically `admin@cluster.local`

2. **From Cluster Administrator**
   - Contact your Redis Enterprise administrator
   - Request appropriate user credentials
   - Ensure your user has necessary permissions

3. **Creating New Users** (if you're an admin)
   ```bash
   # Using redisctl to create a new user
   redisctl enterprise user create \
     --email "newuser@example.com" \
     --password "secure-password" \
     --role "admin"
   ```

### Configuring Enterprise Authentication

#### Using Profiles (Recommended)

```bash
# Create a profile for Enterprise cluster
redisctl profile set my-enterprise \
  --deployment-type enterprise \
  --url "https://cluster.example.com:9443" \
  --username "admin@cluster.local" \
  --password "your-password"

# For self-signed certificates, add --insecure
redisctl profile set dev-cluster \
  --deployment-type enterprise \
  --url "https://localhost:9443" \
  --username "admin@cluster.local" \
  --password "your-password" \
  --insecure

# Test authentication
redisctl enterprise cluster info
```

#### Using Environment Variables

```bash
# Export credentials
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"

# For self-signed certificates
export REDIS_ENTERPRISE_INSECURE="true"

# Test authentication
redisctl enterprise cluster info
```

## Security Best Practices

### 1. Never Hardcode Credentials

```bash
# Bad - credentials in shell history
redisctl cloud subscription list --api-key "abc123" --api-secret "xyz789"

# Good - use profiles or environment variables
redisctl cloud subscription list
```

### 2. Use Environment Variables in CI/CD

```yaml
# GitHub Actions example
env:
  REDIS_CLOUD_API_KEY: ${{ secrets.REDIS_API_KEY }}
  REDIS_CLOUD_API_SECRET: ${{ secrets.REDIS_API_SECRET }}

# GitLab CI example
variables:
  REDIS_CLOUD_API_KEY: ${REDIS_API_KEY}
  REDIS_CLOUD_API_SECRET: ${REDIS_API_SECRET}
```

### 3. Rotate Credentials Regularly

```bash
# Cloud: Create new API key periodically
# 1. Create new key in Redis Cloud Console
# 2. Update profile with new credentials
redisctl profile set production \
  --api-key "new-key" \
  --api-secret "new-secret"

# 3. Test new credentials
redisctl cloud account info

# 4. Delete old key from Redis Cloud Console
```

### 4. Use Minimal Required Permissions

For Redis Cloud API keys:
- **Read-Only** for monitoring and reporting tasks
- **Read-Write** only when management operations are needed
- **Subscription-specific** keys when available

For Redis Enterprise users:
- Use role-based access control (RBAC)
- Assign minimum required role
- Create service-specific users

### 5. Secure Storage of Credentials

```bash
# Use secure secret management tools
# Example with 1Password CLI
export REDIS_CLOUD_API_KEY=$(op read "op://vault/redis-cloud/api-key")
export REDIS_CLOUD_API_SECRET=$(op read "op://vault/redis-cloud/api-secret")

# Example with AWS Secrets Manager
export REDIS_CLOUD_API_KEY=$(aws secretsmanager get-secret-value \
  --secret-id redis/cloud/api-key \
  --query SecretString --output text)
```

## Testing Authentication

### Verify Cloud Authentication

```bash
# Test with account info
redisctl cloud account info

# If successful, you'll see account details
# {
#   "accountId": 12345,
#   "name": "My Company",
#   ...
# }

# Test with a simple list operation
redisctl cloud subscription list
```

### Verify Enterprise Authentication

```bash
# Test with cluster info
redisctl enterprise cluster info

# If successful, you'll see cluster details
# {
#   "name": "my-cluster",
#   "version": "7.4.2",
#   ...
# }

# Test with node list
redisctl enterprise node list
```

### Authentication Test Command

```bash
# Test current profile authentication
redisctl auth test

# Test specific profile
redisctl auth test --profile production

# Verbose output for debugging
RUST_LOG=debug redisctl auth test
```

## Troubleshooting Authentication Issues

### Redis Cloud

#### Invalid API Key or Secret

```bash
# Error: Authentication failed: 401 Unauthorized
# Solution: Verify credentials are correct
redisctl profile get my-cloud

# Re-enter credentials if needed
redisctl profile set my-cloud \
  --api-key "correct-key" \
  --api-secret "correct-secret"
```

#### API Key Lacks Permissions

```bash
# Error: Forbidden: API key lacks required permissions
# Solution: Create new key with appropriate permissions in Redis Cloud Console
```

### Redis Enterprise

#### Connection Refused

```bash
# Error: Connection refused
# Solution: Verify URL and port
curl -k https://cluster.example.com:9443/v1/cluster

# Check if cluster is accessible
ping cluster.example.com
```

#### Invalid Credentials

```bash
# Error: Authentication failed: 401 Unauthorized
# Solution: Verify username and password
redisctl profile set my-enterprise \
  --username "correct-user@domain.com" \
  --password "correct-password"
```

#### SSL Certificate Issues

```bash
# Error: Certificate verify failed
# Solution for development/testing only:
export REDIS_ENTERPRISE_INSECURE=true

# Or update profile:
redisctl profile set dev-cluster --insecure
```

## Multi-Factor Authentication (MFA)

### Redis Cloud with SSO

If your Redis Cloud account uses SSO with MFA:
1. API keys bypass SSO/MFA for programmatic access
2. Create API keys through the web console after SSO login
3. Use API keys with `redisctl` as normal

### Redis Enterprise with LDAP

If your Redis Enterprise cluster uses LDAP with MFA:
1. Local users (non-LDAP) can be created for CLI access
2. Contact your administrator to create a service account
3. Use the service account credentials with `redisctl`

## See Also

- [Configuration](../cli/configuration.md) - Profile management and configuration
- [Profile Commands](../cli-reference/profile/README.md) - Profile command reference
- [Quick Start](./quickstart.md) - Getting started with redisctl