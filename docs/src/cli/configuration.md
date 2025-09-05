# Configuration

`redisctl` uses a profile-based configuration system that allows you to manage multiple Redis Cloud and Redis Enterprise deployments from a single tool. Each profile stores connection settings, authentication credentials, and deployment preferences.

## Configuration File Locations

The configuration file location varies by platform:

- **Linux**: `~/.config/redisctl/config.toml`
- **macOS**: `~/.config/redisctl/config.toml` (preferred) or `~/Library/Application Support/com.redis.redisctl/config.toml`
- **Windows**: `%APPDATA%\redis\redisctl\config.toml`

> **Note**: On macOS, the Linux-style `~/.config/` path is supported for better cross-platform consistency and developer experience.

## Configuration File Structure

The configuration file uses TOML format with the following structure:

```toml
# Optional: Set the default profile
default_profile = "production"

# Define profiles
[profiles.production]
deployment_type = "cloud"
api_key = "your-api-key"
api_secret = "your-api-secret"
api_url = "https://api.redislabs.com/v1"  # Optional, uses default if not specified

[profiles.dev-cluster]
deployment_type = "enterprise"
url = "https://dev-cluster.example.com:9443"
username = "admin@cluster.local"
password = "secure-password"
insecure = true  # Allow self-signed certificates

[profiles.staging]
deployment_type = "cloud"
api_key = "staging-key"
api_secret = "staging-secret"
```

## Environment Variable Expansion

Profile configurations support environment variable expansion, allowing you to keep sensitive credentials out of configuration files. This is especially useful for CI/CD pipelines, containerized deployments, and team environments.

### Syntax

- `\${VAR}` - Expands to the value of environment variable `VAR`. Fails if not set.
- `\${VAR:-default}` - Expands to the value of `VAR` if set, otherwise uses `default`.

### Examples

#### Basic Environment Variable Usage

```toml
[profiles.cloud-dev]
deployment_type = "cloud"
api_key = "${REDIS_CLOUD_API_KEY}"
api_secret = "${REDIS_CLOUD_API_SECRET}"
api_url = "${REDIS_CLOUD_API_URL:-https://api.redislabs.com/v1}"
```

#### Multi-Environment Setup

```toml
# Development profile using environment-specific variables
[profiles.dev]
deployment_type = "cloud"
api_key = "${DEV_REDIS_API_KEY}"
api_secret = "${DEV_REDIS_API_SECRET}"

# Staging profile with defaults
[profiles.staging]
deployment_type = "cloud"
api_key = "${STAGING_API_KEY:-test-key}"
api_secret = "${STAGING_API_SECRET:-test-secret}"
api_url = "${STAGING_API_URL:-https://api-staging.redislabs.com/v1}"

# Production profile
[profiles.production]
deployment_type = "cloud"
api_key = "${PROD_API_KEY}"
api_secret = "${PROD_API_SECRET}"
```

#### Enterprise Cluster Configuration

```toml
[profiles.enterprise-local]
deployment_type = "enterprise"
url = "${REDIS_ENTERPRISE_URL:-https://localhost:9443}"
username = "${REDIS_ENTERPRISE_USER:-admin@cluster.local}"
password = "${REDIS_ENTERPRISE_PASSWORD}"
insecure = "${REDIS_ENTERPRISE_INSECURE:-true}"
```

## Profile Management Commands

`redisctl` provides several commands to manage profiles:

### List All Profiles

```bash
# Show all configured profiles
redisctl profile list

# Example output:
# Available profiles:
#   production (cloud) - default
#   dev-cluster (enterprise)
#   staging (cloud)
```

### Set a Profile

```bash
# Create or update a Redis Cloud profile
redisctl profile set my-cloud \
  --deployment-type cloud \
  --api-key "your-api-key" \
  --api-secret "your-api-secret"

# Create or update a Redis Enterprise profile
redisctl profile set my-enterprise \
  --deployment-type enterprise \
  --url "https://cluster.example.com:9443" \
  --username "admin@cluster.local" \
  --password "secure-password" \
  --insecure
```

### Get Profile Details

```bash
# Display a specific profile's configuration
redisctl profile get production

# Example output:
# Profile: production
# Deployment Type: cloud
# API URL: https://api.redislabs.com/v1
# API Key: ********key (hidden)
```

### Set Default Profile

```bash
# Set the default profile for all commands
redisctl profile default production

# Verify the default
redisctl profile list
```

### Remove a Profile

```bash
# Delete a profile from configuration
redisctl profile remove old-profile
```

## Configuration Precedence

`redisctl` resolves configuration values in the following order (highest to lowest priority):

1. **Command-line flags** - Directly specified in the command
2. **Environment variables** - Set in the shell environment
3. **Profile settings** - Defined in the configuration file
4. **Default values** - Built-in defaults

### Example Precedence

```bash
# Profile defines: api_key = "profile-key"
# Environment has: REDIS_CLOUD_API_KEY="env-key"
# Command uses: --api-key "cli-key"

# Result: "cli-key" is used (command-line flag wins)
redisctl cloud subscription list --api-key "cli-key"

# Without --api-key flag, "env-key" would be used (environment variable)
REDIS_CLOUD_API_KEY="env-key" redisctl cloud subscription list

# Without environment variable or flag, "profile-key" would be used
redisctl cloud subscription list
```

## Advanced Configuration Examples

### Multi-Region Cloud Setup

```toml
# US Production
[profiles.us-prod]
deployment_type = "cloud"
api_key = "${US_PROD_API_KEY}"
api_secret = "${US_PROD_API_SECRET}"

# EU Production
[profiles.eu-prod]
deployment_type = "cloud"
api_key = "${EU_PROD_API_KEY}"
api_secret = "${EU_PROD_API_SECRET}"

# APAC Production
[profiles.apac-prod]
deployment_type = "cloud"
api_key = "${APAC_PROD_API_KEY}"
api_secret = "${APAC_PROD_API_SECRET}"

# Set default based on region
default_profile = "${DEFAULT_REGION:-us-prod}"
```

### Hybrid Cloud and Enterprise Setup

```toml
# Cloud deployment for managed databases
[profiles.cloud-managed]
deployment_type = "cloud"
api_key = "${CLOUD_API_KEY}"
api_secret = "${CLOUD_API_SECRET}"

# On-premise Enterprise cluster
[profiles.on-prem]
deployment_type = "enterprise"
url = "${ON_PREM_URL}"
username = "${ON_PREM_USER}"
password = "${ON_PREM_PASS}"
insecure = false

# Edge Enterprise deployment
[profiles.edge-cluster]
deployment_type = "enterprise"
url = "${EDGE_CLUSTER_URL}"
username = "${EDGE_USER:-admin@edge.local}"
password = "${EDGE_PASS}"
insecure = true
```

### Development Team Configuration

```toml
# Shared development environment
[profiles.dev-shared]
deployment_type = "enterprise"
url = "https://dev.redis.internal:9443"
username = "${USER}@redis.local"  # Uses system username
password = "${REDIS_DEV_PASSWORD}"
insecure = true

# Personal development stack
[profiles.dev-personal]
deployment_type = "enterprise"
url = "https://localhost:9443"
username = "admin@localhost"
password = "${LOCAL_REDIS_PASSWORD:-redis123}"
insecure = true

# CI/CD pipeline profile
[profiles.ci]
deployment_type = "${CI_DEPLOYMENT_TYPE:-cloud}"
api_key = "${CI_API_KEY}"
api_secret = "${CI_API_SECRET}"
api_url = "${CI_API_URL:-https://api.redislabs.com/v1}"
```

## Security Best Practices

### 1. Use Environment Variables for Secrets

Never commit credentials directly in configuration files. Always use environment variables:

```toml
# Good - credentials from environment
[profiles.production]
api_key = "${REDIS_API_KEY}"
api_secret = "${REDIS_API_SECRET}"

# Bad - hardcoded credentials
[profiles.production]
api_key = "actual-key-value"  # Don't do this!
api_secret = "actual-secret"   # Don't do this!
```

### 2. Secure Configuration Files

```bash
# Set appropriate permissions on configuration files
chmod 600 ~/.config/redisctl/config.toml

# Verify permissions
ls -la ~/.config/redisctl/config.toml
# Should show: -rw------- (only owner can read/write)
```

### 3. Use Separate Profiles for Different Environments

```toml
# Separate profiles prevent accidental cross-environment operations
[profiles.production]
deployment_type = "cloud"
api_key = "${PROD_KEY}"

[profiles.development]
deployment_type = "cloud"
api_key = "${DEV_KEY}"
```

### 4. Leverage Profile Defaults for Safety

```toml
# Default to read-only or development profile
default_profile = "dev-readonly"

[profiles.dev-readonly]
deployment_type = "cloud"
api_key = "${READONLY_API_KEY}"
api_secret = "${READONLY_API_SECRET}"
```

## Troubleshooting

### Profile Not Found

```bash
# Error: Profile 'myprofile' not found
# Solution: List available profiles and check spelling
redisctl profile list
```

### Environment Variable Not Set

```bash
# Error: Environment variable REDIS_API_KEY not set
# Solution: Export the variable or use default value
export REDIS_API_KEY="your-key"
# Or update config to use default:
# api_key = "${REDIS_API_KEY:-default-key}"
```

### Configuration File Not Found

```bash
# Create configuration directory and file
mkdir -p ~/.config/redisctl
touch ~/.config/redisctl/config.toml

# Or let redisctl create it
redisctl profile set default --deployment-type cloud
```

### Permission Denied

```bash
# Fix configuration file permissions
chmod 600 ~/.config/redisctl/config.toml

# Fix directory permissions
chmod 700 ~/.config/redisctl
```

## Migration from Environment Variables

If you're currently using environment variables exclusively, you can migrate to profiles while maintaining backward compatibility:

```toml
# Profile that reads from existing environment variables
[profiles.legacy]
deployment_type = "${REDIS_DEPLOYMENT_TYPE:-cloud}"
api_key = "${REDIS_CLOUD_API_KEY}"
api_secret = "${REDIS_CLOUD_API_SECRET}"
url = "${REDIS_ENTERPRISE_URL}"
username = "${REDIS_ENTERPRISE_USER}"
password = "${REDIS_ENTERPRISE_PASSWORD}"

# Set as default to maintain existing behavior
default_profile = "legacy"
```

This allows gradual migration to profile-based configuration while existing scripts continue to work.

## See Also

- [Authentication](../getting-started/authentication.md) - Detailed authentication setup
- [Profile Commands Reference](../cli-reference/profile/README.md) - Complete profile command documentation
- [Environment Variables](./environment-variables.md) - Full list of supported environment variables