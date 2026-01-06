# Profiles

Manage credentials for multiple Redis deployments.

## Why Profiles?

Instead of juggling environment variables or passing credentials on every command, profiles let you:

- Store credentials for multiple environments (dev, staging, prod)
- Switch between Redis Cloud and Enterprise deployments
- Keep credentials secure with OS keyring integration
- Share configuration (without secrets) across teams

## Creating Profiles

### Redis Cloud

``` bash
redisctl profile set my-cloud \
  --cloud-api-key "your-api-key" \
  --cloud-secret-key "your-secret-key"
```

### Redis Enterprise

``` bash
redisctl profile set my-enterprise \
  --enterprise-url "https://cluster.example.com:9443" \
  --enterprise-user "admin@cluster.local" \
  --enterprise-password "your-password"
```

### Combined Profile

A profile can have both Cloud and Enterprise credentials:

``` bash
redisctl profile set prod \
  --cloud-api-key "$CLOUD_KEY" \
  --cloud-secret-key "$CLOUD_SECRET" \
  --enterprise-url "https://prod-cluster:9443" \
  --enterprise-user "admin@cluster.local" \
  --enterprise-password "$ENT_PASSWORD"
```

## Using Profiles

### Per-Command

``` bash
redisctl --profile prod cloud subscription list
redisctl --profile dev enterprise cluster get
```

### Default Profile

``` bash
# Set the default
redisctl profile set-default prod

# Now commands use prod automatically
redisctl cloud subscription list
```

### Override with Environment

Environment variables override profile settings:

``` bash
# Profile says one thing, env var wins
export REDIS_CLOUD_API_KEY="override-key"
redisctl --profile prod cloud subscription list  # Uses override-key
```

## Managing Profiles

### List All Profiles

``` bash
redisctl profile list
```

```
Profiles:
  * prod (default)
    dev
    staging
```

### Show Profile Details

``` bash
redisctl profile show prod
```

### Delete a Profile

``` bash
redisctl profile delete dev
```

## Secure Storage

!!! danger "Default is Plaintext"
    By default, credentials are stored in plaintext. Use one of the secure options below for sensitive environments.

### Option 1: OS Keyring

Store credentials in macOS Keychain, Windows Credential Manager, or Linux Secret Service:

``` bash
# Requires the secure-storage feature
cargo install redisctl --features secure-storage

# Create profile with keyring storage
redisctl profile set prod \
  --cloud-api-key "$KEY" \
  --cloud-secret-key "$SECRET" \
  --use-keyring
```

The config file only stores a reference:

``` toml
[profiles.prod]
cloud_api_key = "keyring:prod-cloud-api-key"
cloud_secret_key = "keyring:prod-cloud-secret-key"
```

### Option 2: Environment References

Store references to environment variables:

``` bash
redisctl profile set prod \
  --cloud-api-key '${REDIS_CLOUD_API_KEY}' \
  --cloud-secret-key '${REDIS_CLOUD_SECRET_KEY}'
```

Variables are resolved at runtime. Great for CI/CD where secrets are injected.

## Configuration File Location

| Platform | Path |
|----------|------|
| Linux | `~/.config/redisctl/config.toml` |
| macOS | `~/.config/redisctl/config.toml` |
| Windows | `%APPDATA%\redis\redisctl\config.toml` |

## Example Configuration

``` toml
default_profile = "prod"

[profiles.prod]
cloud_api_key = "keyring:prod-api-key"
cloud_secret_key = "keyring:prod-secret-key"

[profiles.dev]
enterprise_url = "https://dev-cluster:9443"
enterprise_user = "admin@cluster.local"
enterprise_password = "${DEV_PASSWORD}"
enterprise_insecure = true

[profiles.local]
enterprise_url = "https://localhost:9443"
enterprise_user = "admin@redis.local"
enterprise_password = "Redis123!"
enterprise_insecure = true
```
