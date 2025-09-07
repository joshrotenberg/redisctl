# Profile Management

Profiles allow you to manage multiple Redis Cloud and Enterprise environments with different credentials and settings.

## Profile Configuration

Profiles are stored in `~/.config/redisctl/config.toml`:

```toml
default_profile = "cloud-prod"

[profiles.cloud-dev]
deployment_type = "cloud"
api_key = "dev-key-123"
api_secret = "dev-secret-456"
api_url = "https://api.redislabs.com/v1"

[profiles.cloud-prod]
deployment_type = "cloud"
api_key = "prod-key-789"
api_secret = "prod-secret-012"

[profiles.enterprise-local]
deployment_type = "enterprise"
url = "https://localhost:9443"
username = "admin@cluster.local"
password = "localpass"
insecure = true

[profiles.enterprise-prod]
deployment_type = "enterprise"
url = "https://redis-cluster.company.com:9443"
username = "admin@company.com"
password = "prodpass"
```

## Profile Commands

### List Profiles
```bash
# Show all configured profiles
redisctl profile list

# Example output:
# Available profiles:
#   cloud-dev (cloud)
# * cloud-prod (cloud) [default]
#   enterprise-local (enterprise)
#   enterprise-prod (enterprise)
```

### Set Default Profile
```bash
# Set default profile
redisctl profile default cloud-prod

# Verify
redisctl profile list
```

### Get Profile Details
```bash
# Show profile configuration (passwords are masked)
redisctl profile get cloud-dev

# Example output:
# Profile: cloud-dev
# Type: cloud
# API Key: dev-key-123
# API Secret: ****
# API URL: https://api.redislabs.com/v1
```

### Set Profile Values
```bash
# Update API key
redisctl profile set cloud-dev api_key "new-key-123"

# Update API secret
redisctl profile set cloud-dev api_secret "new-secret-456"

# Update Enterprise URL
redisctl profile set enterprise-prod url "https://new-cluster:9443"
```

### Remove Profile
```bash
# Remove a profile
redisctl profile remove old-profile
```

## Using Profiles

### Explicit Profile Selection
```bash
# Use specific profile for a command
redisctl database list --profile cloud-dev

# Override default profile
redisctl --profile enterprise-prod cluster info
```

### Profile Resolution Order

redisctl resolves profiles in this order:
1. `--profile` command-line flag
2. `REDISCTL_PROFILE` environment variable
3. `default_profile` in config file
4. Error if no profile can be determined

## Environment Variable Support

### Variable Expansion in Profiles

Profiles support environment variable expansion:

```toml
[profiles.cloud-dynamic]
deployment_type = "cloud"
api_key = "${REDIS_CLOUD_KEY}"
api_secret = "${REDIS_CLOUD_SECRET}"
api_url = "${REDIS_API_URL:-https://api.redislabs.com/v1}"
```

### Setting Variables
```bash
# Set environment variables
export REDIS_CLOUD_KEY="my-api-key"
export REDIS_CLOUD_SECRET="my-secret"
export REDIS_API_URL="https://custom-api.example.com"

# Use profile with variable expansion
redisctl database list --profile cloud-dynamic
```

### Default Values
```toml
# Use default if variable not set
api_url = "${REDIS_API_URL:-https://api.redislabs.com/v1}"
username = "${REDIS_USER:-admin@cluster.local}"
```

## Advanced Profile Management

### Multiple Environments
```toml
# Development environments
[profiles.dev-us]
deployment_type = "cloud"
api_key = "${DEV_US_KEY}"
api_secret = "${DEV_US_SECRET}"

[profiles.dev-eu]
deployment_type = "cloud"
api_key = "${DEV_EU_KEY}"
api_secret = "${DEV_EU_SECRET}"

# Staging environments
[profiles.staging-us]
deployment_type = "cloud"
api_key = "${STAGING_US_KEY}"
api_secret = "${STAGING_US_SECRET}"

# Production environments
[profiles.prod-us]
deployment_type = "cloud"
api_key = "${PROD_US_KEY}"
api_secret = "${PROD_US_SECRET}"

[profiles.prod-eu]
deployment_type = "cloud"
api_key = "${PROD_EU_KEY}"
api_secret = "${PROD_EU_SECRET}"
```

### Profile Switching Script
```bash
#!/bin/bash
# Switch between environments
ENV=$1
REGION=$2

case "$ENV" in
  dev|staging|prod)
    redisctl profile default "${ENV}-${REGION}"
    echo "Switched to ${ENV}-${REGION}"
    ;;
  *)
    echo "Usage: $0 [dev|staging|prod] [us|eu]"
    exit 1
    ;;
esac
```

### CI/CD Integration
```yaml
# GitHub Actions example
jobs:
  deploy:
    steps:
      - name: Configure Redis Profile
        run: |
          mkdir -p ~/.config/redisctl
          cat > ~/.config/redisctl/config.toml <<EOF
          [profiles.ci]
          deployment_type = "cloud"
          api_key = "${{ secrets.REDIS_API_KEY }}"
          api_secret = "${{ secrets.REDIS_API_SECRET }}"
          EOF
          
      - name: Deploy Database
        run: |
          redisctl --profile ci database create \
            --subscription-id ${{ vars.SUBSCRIPTION_ID }} \
            --data @database.json --wait
```

## Security Best Practices

### Protecting Credentials

1. **Never commit credentials**: Keep config.toml in .gitignore
2. **Use environment variables**: Store secrets in environment
3. **Restrict file permissions**: 
   ```bash
   chmod 600 ~/.config/redisctl/config.toml
   ```
4. **Rotate credentials regularly**: Update API keys periodically

### Secure Profile Template
```toml
# Use environment variables for sensitive data
[profiles.secure]
deployment_type = "cloud"
api_key = "${REDIS_CLOUD_API_KEY}"
api_secret = "${REDIS_CLOUD_API_SECRET}"

# Store in secure vault
# export REDIS_CLOUD_API_KEY=$(vault read -field=key secret/redis)
# export REDIS_CLOUD_API_SECRET=$(vault read -field=secret secret/redis)
```

### Profile Audit
```bash
#!/bin/bash
# Audit profile usage
echo "Profile Audit Report"
echo "==================="

for profile in $(redisctl profile list | grep -E '^\s+' | awk '{print $1}'); do
  echo -e "\nProfile: $profile"
  echo "Last used: $(grep -l "profile.*$profile" ~/.bash_history | tail -1)"
  
  # Check for hardcoded credentials
  if grep -q "api_key = \"" ~/.config/redisctl/config.toml; then
    echo "WARNING: Hardcoded credentials detected!"
  fi
done
```