# Secure Storage

redisctl supports secure credential storage using your operating system's native keyring service. This keeps your API keys and passwords encrypted instead of storing them as plaintext in configuration files.

## Overview

When secure storage is enabled, redisctl integrates with:
- **macOS**: Keychain
- **Windows**: Windows Credential Store
- **Linux**: Secret Service API (GNOME Keyring, KWallet, etc.)

## Installation

The secure storage feature is optional. To enable it, install redisctl with the `secure-storage` feature:

```bash
# Build from source with secure storage
cargo install redisctl --features secure-storage

# Or build locally
cargo build --release --features secure-storage
```

## Setting Up Secure Storage

### Store Credentials in Keyring

Use the `--use-keyring` flag when setting up profiles:

```bash
# Store Cloud API credentials securely
redisctl profile set cloud-prod \
  --deployment-type cloud \
  --api-key YOUR_API_KEY \
  --api-secret YOUR_SECRET_KEY \
  --use-keyring

# Store Enterprise credentials securely  
redisctl profile set enterprise-prod \
  --deployment-type enterprise \
  --url https://cluster.example.com:9443 \
  --username admin@cluster.local \
  --password YOUR_PASSWORD \
  --use-keyring
```

### Verify Secure Storage

```bash
# Check profile configuration
redisctl profile get cloud-prod

# Output shows keyring references instead of plaintext:
# Profile: cloud-prod
#   Deployment Type: cloud
#   API Key: keyring:cloud-prod-api-key
#   API Secret: keyring:cloud-prod-api-secret
```

## How It Works

When you use `--use-keyring`:

1. **Credentials are stored** in your OS keyring service
2. **Config file contains references** like `keyring:profile-field` instead of actual values
3. **At runtime**, redisctl retrieves credentials from the keyring
4. **No plaintext secrets** are written to disk

### Configuration File

Your config file (`~/.config/redisctl/config.toml`) will look like:

```toml
[profiles.cloud-prod]
deployment_type = "cloud"
api_key = "keyring:cloud-prod-api-key"
api_secret = "keyring:cloud-prod-api-secret"

[profiles.enterprise-prod]
deployment_type = "enterprise"
url = "https://cluster.example.com:9443"
username = "admin@cluster.local"
password = "keyring:enterprise-prod-password"
```

## Migration from Plaintext

### Migrate Existing Profiles

If you have existing profiles with plaintext credentials:

```bash
# Update existing profile to use keyring
redisctl profile set cloud-prod \
  --api-key YOUR_API_KEY \
  --api-secret YOUR_SECRET_KEY \
  --use-keyring

# The command will:
# 1. Store credentials in keyring
# 2. Update config to use keyring references
# 3. Remove plaintext from config file
```

### Bulk Migration Script

```bash
#!/bin/bash
# Migrate all profiles to secure storage

for profile in $(redisctl profile list | grep -v "Available profiles:" | awk '{print $1}'); do
  echo "Migrating profile: $profile"
  
  # Get current values (you'll need to provide these)
  # This is a manual step for security reasons
  read -p "Enter API key for $profile: " api_key
  read -s -p "Enter API secret for $profile: " api_secret
  echo
  
  # Update profile with keyring storage
  redisctl profile set "$profile" \
    --api-key "$api_key" \
    --api-secret "$api_secret" \
    --use-keyring
done
```

## Credential Resolution Order

redisctl resolves credentials in this order:

1. **Environment variables** (highest priority)
   - `REDIS_CLOUD_API_KEY`, `REDIS_ENTERPRISE_PASSWORD`, etc.
2. **Keyring** (if value starts with `keyring:`)
3. **Config file plaintext** (lowest priority)

### Environment Variable Override

Environment variables always take precedence:

```bash
# Temporarily override keyring-stored credentials
export REDIS_CLOUD_API_KEY="temporary-key"
redisctl cloud subscription list  # Uses env var, not keyring
```

## Security Best Practices

### Do's

- **Use unique credentials** per environment (dev, staging, prod)
- **Enable keyring storage** for all production profiles
- **Regularly rotate** API keys and passwords
- **Set appropriate permissions** on config file (0600)
- **Use environment variables** in CI/CD pipelines

### Don'ts

- **Don't commit** config files with plaintext credentials
- **Don't share** keyring-stored credentials between users
- **Don't use** the same credentials across environments
- **Don't disable** secure storage for production profiles

## Troubleshooting

### Keyring Service Not Available

If you get a keyring error:

```bash
Error: No keyring service available
```

**Solutions:**

1. **Linux**: Install a keyring service
   ```bash
   # Ubuntu/Debian
   sudo apt-get install gnome-keyring
   
   # Fedora/RHEL
   sudo dnf install gnome-keyring
   ```

2. **macOS**: Keychain should be available by default

3. **Windows**: Credential Store should be available by default

4. **Fallback**: Use environment variables instead
   ```bash
   export REDIS_CLOUD_API_KEY="your-key"
   export REDIS_CLOUD_API_SECRET="your-secret"
   ```

### Permission Denied

If you can't access stored credentials:

```bash
Error: Failed to access keyring: Permission denied
```

**Solutions:**

1. **Unlock your keyring** (Linux)
2. **Check Keychain Access** permissions (macOS)
3. **Run as the same user** who stored the credentials

### Lost Keyring Access

If you lose access to your keyring:

1. **Reset the profile** with new credentials:
   ```bash
   redisctl profile set cloud-prod \
     --api-key NEW_KEY \
     --api-secret NEW_SECRET \
     --use-keyring
   ```

2. **Or switch to plaintext** (not recommended for production):
   ```bash
   redisctl profile set cloud-prod \
     --api-key NEW_KEY \
     --api-secret NEW_SECRET
   ```

## Platform-Specific Notes

### macOS

- Credentials are stored in the login keychain
- You may be prompted to allow access on first use
- Use Keychain Access app to manage stored credentials

### Windows

- Credentials are stored in Windows Credential Manager
- Access via: Control Panel → Credential Manager → Windows Credentials
- Look for entries starting with "redisctl:"

### Linux

- Requires a Secret Service provider (GNOME Keyring, KWallet, etc.)
- May need to unlock keyring on login
- Use `seahorse` or similar tools to manage credentials

## Comparison: Secure vs Plaintext

| Aspect | Secure Storage | Plaintext |
|--------|---------------|-----------|
| Security | Encrypted in OS keyring | Visible in config file |
| Portability | Tied to user account | Config file portable |
| CI/CD | Use env vars | Can use config file |
| Setup | Requires keyring service | No dependencies |
| Recommended for | Production, personal use | Development only |

## Advanced Usage

### Custom Keyring Service Name

The service name in the keyring is "redisctl" by default. This cannot be customized in the current version.

### Keyring Entry Format

Keyring entries are stored as:
- Service: `redisctl`
- Account: `profile-field` (e.g., `cloud-prod-api-key`)
- Password: The actual credential value

### Direct Keyring Access

You can directly manage keyring entries using OS tools:

```bash
# macOS - using security command
security find-generic-password -s "redisctl" -a "cloud-prod-api-key"

# Linux - using secret-tool
secret-tool lookup service redisctl account cloud-prod-api-key

# Windows - PowerShell
$cred = Get-StoredCredential -Target "redisctl:cloud-prod-api-key"
$cred.GetNetworkCredential().Password
```

## Next Steps

- Set up profiles with secure storage: [Configuration](../getting-started/configuration.md)
- Learn about environment variables: [Environment Variables](../reference/environment-variables.md)
- Explore security best practices: [Security](../reference/security.md)