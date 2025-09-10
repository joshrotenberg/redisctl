# Configuration File

Complete reference for the redisctl configuration file format and options.

## File Location

The configuration file is stored at:

- **Linux/macOS**: `~/.config/redisctl/config.toml`
- **Windows**: `%APPDATA%\redis\redisctl\config.toml`

View the exact path:
```bash
redisctl profile path
```

## File Format

The configuration file uses TOML format:

```toml
# Default profile to use when none specified
default_profile = "production"

# Profile definitions
[profiles.production]
deployment_type = "cloud"
api_key = "your-api-key"
api_secret = "your-api-secret"
api_url = "https://api.redislabs.com/v1"

[profiles.enterprise-local]
deployment_type = "enterprise"
url = "https://localhost:9443"
username = "admin@cluster.local"
password = "your-password"
insecure = true
```

## Profile Configuration

### Cloud Profile

All available options for Redis Cloud profiles:

```toml
[profiles.cloud-example]
# Required: Deployment type
deployment_type = "cloud"

# Required: API credentials
api_key = "A3qcymrvqpn9rrgdt40sv5f9yfxob26vx64hwddh8vminqnkgfq"
api_secret = "S3s8ecrrnaguqkvwfvealoe3sn25zqs4wc4lwgo4rb0ud3qm77c"

# Optional: API endpoint (defaults to production)
api_url = "https://api.redislabs.com/v1"

# Optional: Custom timeout (seconds)
timeout = 30

# Optional: Retry configuration
max_retries = 3
retry_delay = 1
```

### Enterprise Profile

All available options for Redis Enterprise profiles:

```toml
[profiles.enterprise-example]
# Required: Deployment type
deployment_type = "enterprise"

# Required: Cluster URL
url = "https://cluster.example.com:9443"

# Required: Authentication
username = "admin@example.com"
password = "secure-password"

# Optional: Allow self-signed certificates
insecure = false

# Optional: Custom timeout (seconds)
timeout = 60

# Optional: Client certificate authentication
client_cert = "/path/to/client.crt"
client_key = "/path/to/client.key"

# Optional: Custom CA certificate
ca_cert = "/path/to/ca.crt"
```

## Environment Variable Expansion

The configuration file supports environment variable expansion using `${VAR}` syntax:

### Basic Expansion

```toml
[profiles.production]
deployment_type = "cloud"
api_key = "${REDIS_CLOUD_API_KEY}"
api_secret = "${REDIS_CLOUD_API_SECRET}"
```

### With Default Values

```toml
[profiles.staging]
deployment_type = "cloud"
api_key = "${STAGING_API_KEY}"
api_secret = "${STAGING_API_SECRET}"
# Use production URL if STAGING_API_URL not set
api_url = "${STAGING_API_URL:-https://api.redislabs.com/v1}"
```

### Complex Example

```toml
default_profile = "${REDISCTL_DEFAULT_PROFILE:-development}"

[profiles.development]
deployment_type = "cloud"
api_key = "${DEV_API_KEY}"
api_secret = "${DEV_API_SECRET}"
api_url = "${DEV_API_URL:-https://api.redislabs.com/v1}"

[profiles.production]
deployment_type = "cloud"
api_key = "${PROD_API_KEY}"
api_secret = "${PROD_API_SECRET}"
api_url = "${PROD_API_URL:-https://api.redislabs.com/v1}"

[profiles."${DYNAMIC_PROFILE_NAME:-custom}"]
deployment_type = "${DYNAMIC_DEPLOYMENT:-cloud}"
api_key = "${DYNAMIC_API_KEY}"
api_secret = "${DYNAMIC_API_SECRET}"
```

## Multiple Profiles

### Organizing by Environment

```toml
# Development environments
[profiles.dev-cloud]
deployment_type = "cloud"
api_key = "${DEV_CLOUD_KEY}"
api_secret = "${DEV_CLOUD_SECRET}"

[profiles.dev-enterprise]
deployment_type = "enterprise"
url = "https://dev-cluster:9443"
username = "dev-admin"
password = "${DEV_ENTERPRISE_PASSWORD}"
insecure = true

# Staging environments
[profiles.staging-cloud]
deployment_type = "cloud"
api_key = "${STAGING_CLOUD_KEY}"
api_secret = "${STAGING_CLOUD_SECRET}"

[profiles.staging-enterprise]
deployment_type = "enterprise"
url = "https://staging-cluster:9443"
username = "staging-admin"
password = "${STAGING_ENTERPRISE_PASSWORD}"

# Production environments
[profiles.prod-cloud]
deployment_type = "cloud"
api_key = "${PROD_CLOUD_KEY}"
api_secret = "${PROD_CLOUD_SECRET}"

[profiles.prod-enterprise]
deployment_type = "enterprise"
url = "https://prod-cluster:9443"
username = "prod-admin"
password = "${PROD_ENTERPRISE_PASSWORD}"
```

### Organizing by Region

```toml
[profiles.us-east-1]
deployment_type = "cloud"
api_key = "${US_EAST_API_KEY}"
api_secret = "${US_EAST_SECRET}"

[profiles.eu-west-1]
deployment_type = "cloud"
api_key = "${EU_WEST_API_KEY}"
api_secret = "${EU_WEST_SECRET}"

[profiles.ap-southeast-1]
deployment_type = "cloud"
api_key = "${APAC_API_KEY}"
api_secret = "${APAC_SECRET}"
```

## Advanced Configuration

### Team Shared Configuration

Create a shared base configuration:

```toml
# team-config.toml (checked into git)
[profiles.team-base]
deployment_type = "cloud"
api_url = "https://api.redislabs.com/v1"

# Local overrides (not in git)
# ~/.config/redisctl/config.toml
[profiles.team]
deployment_type = "cloud"
api_url = "https://api.redislabs.com/v1"
api_key = "${MY_API_KEY}"
api_secret = "${MY_API_SECRET}"
```

### CI/CD Configuration

```toml
# CI/CD specific profiles
[profiles.ci-test]
deployment_type = "cloud"
api_key = "${CI_TEST_API_KEY}"
api_secret = "${CI_TEST_API_SECRET}"
api_url = "${CI_API_URL:-https://api.redislabs.com/v1}"

[profiles.ci-deploy]
deployment_type = "enterprise"
url = "${CI_CLUSTER_URL}"
username = "${CI_USERNAME}"
password = "${CI_PASSWORD}"
insecure = true  # CI uses self-signed certs
```

## Security Considerations

### File Permissions

Set restrictive permissions on the configuration file:

```bash
# Linux/macOS
chmod 600 ~/.config/redisctl/config.toml

# Verify permissions
ls -la ~/.config/redisctl/config.toml
# Should show: -rw-------
```

### Credential Storage Best Practices

1. **Never commit credentials to version control**
   ```bash
   # .gitignore
   config.toml
   *.secret
   ```

2. **Use environment variables for sensitive data**
   ```toml
   [profiles.secure]
   deployment_type = "cloud"
   api_key = "${REDIS_API_KEY}"  # Set in environment
   api_secret = "${REDIS_API_SECRET}"  # Set in environment
   ```

3. **Integrate with secret managers**
   ```bash
   # Set environment variables from secret manager
   export REDIS_API_KEY=$(vault kv get -field=api_key secret/redis)
   export REDIS_API_SECRET=$(vault kv get -field=api_secret secret/redis)
   ```

## Migration from Other Formats

### From Environment Variables Only

If currently using only environment variables:

```bash
# Create profile from environment
redisctl profile set migrated \
  --deployment cloud \
  --api-key "$REDIS_CLOUD_API_KEY" \
  --api-secret "$REDIS_CLOUD_API_SECRET"
```

### From JSON Configuration

Convert JSON to TOML:

```bash
# old-config.json
{
  "profiles": {
    "production": {
      "type": "cloud",
      "apiKey": "key",
      "apiSecret": "secret"
    }
  }
}

# Convert to config.toml
[profiles.production]
deployment_type = "cloud"
api_key = "key"
api_secret = "secret"
```

## Validation

### Check Configuration

```bash
# Validate profile configuration
redisctl profile show production

# Test authentication
redisctl auth test --profile production

# List all profiles
redisctl profile list
```

### Common Issues

**Invalid TOML syntax**
```toml
# Wrong - missing quotes
[profiles.prod]
deployment_type = cloud  # Should be "cloud"

# Correct
[profiles.prod]
deployment_type = "cloud"
```

**Environment variable not found**
```toml
# This will fail if MY_VAR is not set
api_key = "${MY_VAR}"

# Use default value to prevent failure
api_key = "${MY_VAR:-default-key}"
```

**Profile name with special characters**
```toml
# Use quotes for profile names with special characters
[profiles."prod-us-east-1"]
deployment_type = "cloud"
```

## Backup and Recovery

### Backup Configuration

```bash
# Backup current configuration
cp ~/.config/redisctl/config.toml ~/.config/redisctl/config.toml.backup

# Backup with timestamp
cp ~/.config/redisctl/config.toml \
   ~/.config/redisctl/config.toml.$(date +%Y%m%d_%H%M%S)
```

### Restore Configuration

```bash
# Restore from backup
cp ~/.config/redisctl/config.toml.backup ~/.config/redisctl/config.toml

# Verify restoration
redisctl profile list
```

## Example Configurations

### Minimal Configuration

```toml
# Minimal working configuration
[profiles.default]
deployment_type = "cloud"
api_key = "your-key"
api_secret = "your-secret"
```

### Full-Featured Configuration

```toml
# Complete example with all features
default_profile = "production"

# Production Cloud
[profiles.production]
deployment_type = "cloud"
api_key = "${PROD_API_KEY}"
api_secret = "${PROD_API_SECRET}"
api_url = "${PROD_API_URL:-https://api.redislabs.com/v1}"

# Staging Cloud with defaults
[profiles.staging]
deployment_type = "cloud"
api_key = "${STAGING_API_KEY}"
api_secret = "${STAGING_API_SECRET}"
api_url = "https://api.redislabs.com/v1"

# Development Enterprise
[profiles.dev-enterprise]
deployment_type = "enterprise"
url = "https://dev-cluster:9443"
username = "admin@dev.local"
password = "${DEV_PASSWORD}"
insecure = true

# DR Enterprise with client certs
[profiles.dr-enterprise]
deployment_type = "enterprise"
url = "https://dr-cluster:9443"
username = "admin@dr.local"
password = "${DR_PASSWORD}"
client_cert = "/etc/ssl/client.crt"
client_key = "/etc/ssl/client.key"
ca_cert = "/etc/ssl/ca.crt"

# Local testing
[profiles.local]
deployment_type = "enterprise"
url = "https://localhost:9443"
username = "admin@cluster.local"
password = "test123"
insecure = true
```