# Profiles & Authentication

Profiles store connection details and credentials for your Redis deployments. You can have multiple profiles for different environments (dev, staging, production).

## Quick Setup

### Redis Cloud

```bash
# Using environment variables (simplest)
export REDIS_CLOUD_API_KEY="your-api-key"
export REDIS_CLOUD_SECRET_KEY="your-secret-key"

# Or create a profile
redisctl profile set cloud-prod \
  --deployment-type cloud \
  --api-key "your-api-key" \
  --api-secret "your-secret-key"
```

### Redis Enterprise

```bash
# Using environment variables
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"
export REDIS_ENTERPRISE_INSECURE="true"  # for self-signed certs

# Or create a profile
redisctl profile set enterprise-prod \
  --deployment-type enterprise \
  --url "https://cluster.example.com:9443" \
  --username "admin@cluster.local" \
  --password "your-password" \
  --insecure
```

## Getting Credentials

### Redis Cloud API Keys

1. Log in to [app.redislabs.com](https://app.redislabs.com)
2. Click your name → Account Settings → API Keys
3. Click "Add API Key" and give it a name
4. Copy both the Account key and Secret (you won't see the secret again!)

### Redis Enterprise

- **URL**: `https://cluster-fqdn:9443`
- **Username**: Configured during setup (often `admin@cluster.local`)
- **Password**: Set during cluster bootstrap

## Profile Management

```bash
# List all profiles
redisctl profile list

# Show profile details
redisctl profile get cloud-prod

# Set default profile
redisctl profile set-default cloud-prod

# Delete a profile
redisctl profile delete old-profile

# Use a specific profile
redisctl --profile cloud-prod cloud database list
```

## Credential Storage Options

### 1. Environment Variables (CI/CD)

Best for automation and CI/CD pipelines:

```bash
# Cloud
export REDIS_CLOUD_API_KEY="..."
export REDIS_CLOUD_SECRET_KEY="..."

# Enterprise
export REDIS_ENTERPRISE_URL="..."
export REDIS_ENTERPRISE_USER="..."
export REDIS_ENTERPRISE_PASSWORD="..."
```

### 2. OS Keyring (Recommended for Local)

Store credentials securely in your operating system's keychain:

```bash
# Requires secure-storage feature
cargo install redisctl --features secure-storage

# Create profile with keyring storage
redisctl profile set production \
  --deployment-type cloud \
  --api-key "your-key" \
  --api-secret "your-secret" \
  --use-keyring
```

Your config file will contain references, not actual secrets:

```toml
[profiles.production]
deployment_type = "cloud"
api_key = "keyring:production-api-key"
api_secret = "keyring:production-api-secret"
```

### 3. Configuration File (Development Only)

For development, credentials can be stored in `~/.config/redisctl/config.toml`:

```toml
default_profile = "dev"

[profiles.dev]
deployment_type = "cloud"
api_key = "your-api-key"
api_secret = "your-secret-key"
```

**Warning**: This stores credentials in plaintext. Use keyring or environment variables for production.

## Configuration File Location

- **Linux/macOS**: `~/.config/redisctl/config.toml`
- **Windows**: `%APPDATA%\redis\redisctl\config.toml`

## Override Hierarchy

Settings are resolved in this order (later overrides earlier):

1. Configuration file (profile settings)
2. Environment variables
3. Command-line flags

```bash
# Profile says cloud-prod, but CLI overrides
redisctl --profile cloud-prod --api-key "different-key" cloud database list
```

## Security Best Practices

1. **Use OS keyring** for local development
2. **Use environment variables** for CI/CD
3. **Never commit credentials** to version control
4. **Set file permissions**: `chmod 600 ~/.config/redisctl/config.toml`
5. **Rotate credentials regularly**
6. **Use read-only API keys** when write access isn't needed

## Troubleshooting

### Authentication Failed

```bash
# Check what credentials are being used
redisctl profile get

# Enable debug logging
RUST_LOG=debug redisctl api cloud get /
```

### Profile Not Found

```bash
# List available profiles
redisctl profile list

# Check config file exists
cat ~/.config/redisctl/config.toml
```

### Certificate Errors (Enterprise)

For self-signed certificates:

```bash
# Via environment
export REDIS_ENTERPRISE_INSECURE="true"

# Or in profile
redisctl profile set myprofile --insecure
```
