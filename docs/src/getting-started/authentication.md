# Authentication

`redisctl` supports authentication for both Redis Cloud and Redis Enterprise deployments.

## Redis Cloud

Redis Cloud uses API key authentication:
- **API Key** - Your account key (public identifier)
- **API Secret** - Your secret key (keep this private!)

### Getting Your API Keys

1. Log in to [app.redislabs.com](https://app.redislabs.com)
2. Click your name → Account Settings → API Keys
3. Click "Add API Key" and give it a name
4. Copy both the Account key and Secret (you won't see the secret again!)

### Setting Up Authentication

Use environment variables:

```bash
export REDIS_CLOUD_API_KEY="your-account-key"
export REDIS_CLOUD_API_SECRET="your-secret-key"

# Test it works
redisctl api cloud get /account
```

Or create a configuration file at `~/.config/redisctl/config.toml`:

```toml
[profiles.cloud]
deployment_type = "cloud"
api_key = "your-account-key"
api_secret = "your-secret-key"
```

## Redis Enterprise

Redis Enterprise uses basic authentication with username/password.

### Default Credentials

- **Username**: `admin@cluster.local` (default)
- **Password**: Set during cluster setup

### Setting Up Authentication

Use environment variables:

```bash
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"

# For self-signed certificates
export REDIS_ENTERPRISE_INSECURE="true"

# Test it works
redisctl api enterprise get /v1/cluster
```

Or add to `~/.config/redisctl/config.toml`:

```toml
[profiles.enterprise]
deployment_type = "enterprise"
url = "https://cluster.example.com:9443"
username = "admin@cluster.local"
password = "your-password"
insecure = true  # For self-signed certs
```

## Security Tips

1. **Never commit credentials** - Use environment variables or secure vaults
2. **Use read-only API keys** when possible for Cloud
3. **Rotate credentials regularly**
4. **Set file permissions**: `chmod 600 ~/.config/redisctl/config.toml`

## Troubleshooting

### Authentication Failed

Check your credentials:
```bash
# Enable debug logging to see what's happening
RUST_LOG=debug redisctl api cloud get /account
```

### Connection Refused

Verify the URL and port are correct:
```bash
curl -k https://your-cluster:9443/v1/cluster
```

### Certificate Errors

For development/testing with self-signed certificates:
```bash
export REDIS_ENTERPRISE_INSECURE=true
```

## See Also

- [Configuration](../cli/configuration.md) - Profile management
- [Environment Variables](../cli/environment-variables.md) - All supported variables