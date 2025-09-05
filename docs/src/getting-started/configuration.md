# Configuration

`redisctl` can be configured using either a configuration file or environment variables.

## Configuration File

Create `~/.config/redisctl/config.toml`:

```toml
# Set default profile
default_profile = "cloud-prod"

# Redis Cloud Profile
[profiles.cloud-prod]
deployment_type = "cloud"
api_key = "your-account-key"
api_secret = "your-secret-key"
api_url = "https://api.redislabs.com/v1"  # Optional, this is the default

# Redis Enterprise Profile
[profiles.enterprise-dev]
deployment_type = "enterprise"
url = "https://localhost:9443"
username = "admin@cluster.local"
password = "your-password"
insecure = true  # Allow self-signed certificates

# You can have multiple profiles
[profiles.cloud-staging]
deployment_type = "cloud"
api_key = "staging-key"
api_secret = "staging-secret"
```

### File Locations

- **Linux/macOS**: `~/.config/redisctl/config.toml`
- **Windows**: `%APPDATA%\redis\redisctl\config.toml`

### Security

Set appropriate file permissions:

```bash
chmod 600 ~/.config/redisctl/config.toml
```

## Environment Variables

Alternatively, use environment variables:

### Cloud Variables

```bash
export REDIS_CLOUD_API_KEY="your-account-key"
export REDIS_CLOUD_API_SECRET="your-secret-key"
export REDIS_CLOUD_API_URL="https://api.redislabs.com/v1"  # Optional
```

### Enterprise Variables

```bash
export REDIS_ENTERPRISE_URL="https://cluster:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"
export REDIS_ENTERPRISE_INSECURE="true"  # For self-signed certs
```

## Getting Your Credentials

### Redis Cloud

1. Log in to [Redis Cloud Console](https://app.redislabs.com)
2. Go to **Account Settings** → **API Keys**
3. Click **Add API Key**
4. Copy the Account Key and Secret

### Redis Enterprise

Get credentials from your cluster administrator or use the default:
- Username: `admin@cluster.local`
- Password: Set during cluster setup

## Testing Your Configuration

```bash
# Test Cloud connection
redisctl api cloud get /account

# Test Enterprise connection
redisctl api enterprise get /v1/cluster
```

## Using Profiles

```bash
# Use default profile
redisctl database list

# Use specific profile
redisctl database list --profile cloud-staging

# List all profiles
redisctl profile list
```

## Next Steps

- [Quick Start](./quickstart.md) - Start using redisctl