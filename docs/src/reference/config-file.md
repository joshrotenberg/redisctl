# Configuration File

Complete reference for the `redisctl` configuration file.

## Location

The configuration file is located at:

- **Linux/macOS**: `~/.config/redisctl/config.toml`
- **Windows**: `%APPDATA%\redis\redisctl\config.toml`

## Format

The file uses TOML format with profiles:

```toml
# Optional: Set default profile
default_profile = "production"

# Profile definitions
[profiles.<name>]
deployment_type = "cloud" | "enterprise"
# ... deployment-specific settings
```

## Cloud Profile

```toml
[profiles.my-cloud]
deployment_type = "cloud"
api_key = "your-account-key"
api_secret = "your-secret-key"
api_url = "https://api.redislabs.com/v1"  # Optional
```

## Enterprise Profile

```toml
[profiles.my-enterprise]
deployment_type = "enterprise"
url = "https://cluster:9443"
username = "admin@cluster.local"
password = "your-password"
insecure = true  # Optional, for self-signed certificates
```

## Complete Example

```toml
# Set the default profile
default_profile = "cloud-prod"

# Production Cloud
[profiles.cloud-prod]
deployment_type = "cloud"
api_key = "prod-account-key"
api_secret = "prod-secret-key"

# Staging Cloud
[profiles.cloud-staging]
deployment_type = "cloud"
api_key = "staging-account-key"
api_secret = "staging-secret-key"
api_url = "https://api-staging.redislabs.com/v1"

# Development Enterprise
[profiles.enterprise-dev]
deployment_type = "enterprise"
url = "https://localhost:9443"
username = "admin@cluster.local"
password = "dev-password"
insecure = true

# Production Enterprise
[profiles.enterprise-prod]
deployment_type = "enterprise"
url = "https://redis-prod.internal:9443"
username = "admin@production.local"
password = "prod-password"
insecure = false
```

## Environment Variable Expansion

Configuration values can reference environment variables:

```toml
[profiles.dynamic]
deployment_type = "cloud"
api_key = "${REDIS_API_KEY}"
api_secret = "${REDIS_API_SECRET}"
api_url = "${REDIS_API_URL:-https://api.redislabs.com/v1}"
```

Syntax:
- `${VAR}` - Use environment variable, fail if not set
- `${VAR:-default}` - Use environment variable, or default value if not set

## Security

Set appropriate file permissions:

```bash
chmod 600 ~/.config/redisctl/config.toml
```

## Profile Management

```bash
# List profiles
redisctl profile list

# Use specific profile
redisctl database list --profile staging

# Set default profile
redisctl profile default production
```

Note: The `profile set`, `profile remove`, and `profile default` commands are not yet fully implemented.