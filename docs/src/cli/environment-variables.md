# Environment Variables

`redisctl` supports environment variables for configuration, allowing you to set credentials and options without modifying configuration files or using command-line flags. This is particularly useful for CI/CD pipelines, containerized deployments, and scripting.

## Redis Cloud Variables

### Authentication

| Variable | Description | Example |
|----------|-------------|---------|
| `REDIS_CLOUD_API_KEY` | API account key for authentication | `A3qcymrvqpn9rrgdt40s...` |
| `REDIS_CLOUD_API_SECRET` | API secret key for authentication | `S3s8ecrrnaguqkvwfvea...` |
| `REDIS_CLOUD_API_URL` | Custom API endpoint URL | `https://api.redislabs.com/v1` |

### Usage Example

```bash
# Set authentication credentials
export REDIS_CLOUD_API_KEY="your-account-key"
export REDIS_CLOUD_API_SECRET="your-secret-key"

# Optional: Use custom API endpoint
export REDIS_CLOUD_API_URL="https://api-staging.redislabs.com/v1"

# Run commands without specifying credentials
redisctl cloud subscription list
redisctl cloud database list --subscription-id 12345
```

## Redis Enterprise Variables

### Authentication and Connection

| Variable | Description | Example |
|----------|-------------|---------|
| `REDIS_ENTERPRISE_URL` | Cluster management API URL | `https://cluster.example.com:9443` |
| `REDIS_ENTERPRISE_USER` | Username for authentication | `admin@cluster.local` |
| `REDIS_ENTERPRISE_PASSWORD` | Password for authentication | `secure-password` |
| `REDIS_ENTERPRISE_INSECURE` | Skip TLS certificate verification | `true` or `false` |

### Usage Example

```bash
# Set connection and authentication
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="Redis123!"

# For self-signed certificates in development
export REDIS_ENTERPRISE_INSECURE="true"

# Run commands without specifying credentials
redisctl enterprise cluster info
redisctl enterprise database list
```

## General Configuration Variables

### Profile Selection

| Variable | Description | Example |
|----------|-------------|---------|
| `REDISCTL_PROFILE` | Default profile to use | `production`, `development` |

```bash
# Use specific profile by default
export REDISCTL_PROFILE="production"

# All commands will use the production profile
redisctl cloud subscription list
```

### Logging and Debugging

| Variable | Description | Example |
|----------|-------------|---------|
| `RUST_LOG` | Set logging level | `error`, `warn`, `info`, `debug`, `trace` |
| `RUST_BACKTRACE` | Show backtrace on panic | `1`, `full` |

```bash
# Enable debug logging
export RUST_LOG=debug
redisctl cloud subscription list

# Enable detailed debug logging for specific modules
export RUST_LOG=redisctl=debug,redis_cloud=trace

# Show full backtrace on errors
export RUST_BACKTRACE=1
redisctl cloud database get --database-id invalid
```

### Output Formatting

| Variable | Description | Example |
|----------|-------------|---------|
| `REDISCTL_OUTPUT` | Default output format | `json`, `yaml`, `table` |
| `NO_COLOR` | Disable colored output | `1` or any value |

```bash
# Set default output format
export REDISCTL_OUTPUT=yaml
redisctl cloud subscription list

# Disable colors in output
export NO_COLOR=1
redisctl cloud database list
```

## Docker Environment Configuration

When running `redisctl` in Docker containers, pass environment variables using `-e` flags:

```bash
# Single variable
docker run -e REDIS_CLOUD_API_KEY="your-key" \
  redisctl cloud account info

# Multiple variables
docker run \
  -e REDIS_CLOUD_API_KEY="your-key" \
  -e REDIS_CLOUD_API_SECRET="your-secret" \
  -e RUST_LOG=debug \
  redisctl cloud subscription list

# Using env file
docker run --env-file .env redisctl cloud database list
```

### Docker Compose Example

```yaml
version: '3.8'
services:
  redisctl:
    image: redisctl:latest
    environment:
      - REDIS_CLOUD_API_KEY=${REDIS_CLOUD_API_KEY}
      - REDIS_CLOUD_API_SECRET=${REDIS_CLOUD_API_SECRET}
      - REDIS_ENTERPRISE_URL=${REDIS_ENTERPRISE_URL:-https://localhost:9443}
      - REDIS_ENTERPRISE_USER=${REDIS_ENTERPRISE_USER:-admin@cluster.local}
      - REDIS_ENTERPRISE_PASSWORD=${REDIS_ENTERPRISE_PASSWORD}
      - REDIS_ENTERPRISE_INSECURE=true
      - RUST_LOG=info
    command: cloud subscription list
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Redis Management
on: [push]

jobs:
  manage-redis:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Run redisctl commands
        env:
          REDIS_CLOUD_API_KEY: ${{ secrets.REDIS_API_KEY }}
          REDIS_CLOUD_API_SECRET: ${{ secrets.REDIS_API_SECRET }}
          RUST_LOG: info
        run: |
          redisctl cloud subscription list
          redisctl cloud database list --subscription-id 12345
```

### GitLab CI

```yaml
redis-management:
  image: redisctl:latest
  variables:
    REDIS_CLOUD_API_KEY: ${REDIS_API_KEY}
    REDIS_CLOUD_API_SECRET: ${REDIS_API_SECRET}
    REDISCTL_OUTPUT: json
  script:
    - redisctl cloud subscription list
    - redisctl cloud database list --subscription-id 12345
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any
    environment {
        REDIS_CLOUD_API_KEY = credentials('redis-api-key')
        REDIS_CLOUD_API_SECRET = credentials('redis-api-secret')
        RUST_LOG = 'info'
    }
    stages {
        stage('List Resources') {
            steps {
                sh 'redisctl cloud subscription list'
                sh 'redisctl cloud database list --subscription-id 12345'
            }
        }
    }
}
```

## Environment Variable Precedence

Environment variables are evaluated in the following order:

1. **Command-line flags** (highest priority)
2. **Environment variables**
3. **Profile configuration**
4. **Default values** (lowest priority)

### Example

```bash
# Profile has: api_key = "profile-key"
# Environment has: REDIS_CLOUD_API_KEY="env-key"
# Command has: --api-key "cli-key"

# Uses "cli-key" (command-line wins)
redisctl cloud account info --api-key "cli-key"

# Uses "env-key" (environment variable)
redisctl cloud account info

# Unset environment variable to use profile value
unset REDIS_CLOUD_API_KEY
redisctl cloud account info  # Uses "profile-key"
```

## Security Considerations

### 1. Avoid Exposing Secrets in Logs

```bash
# Bad - secrets visible in process list
REDIS_CLOUD_API_SECRET="secret" redisctl cloud account info

# Good - export separately
export REDIS_CLOUD_API_SECRET="secret"
redisctl cloud account info
```

### 2. Use Secure Secret Management

```bash
# Load from secure vault (1Password example)
export REDIS_CLOUD_API_KEY=$(op read "op://vault/redis/api-key")
export REDIS_CLOUD_API_SECRET=$(op read "op://vault/redis/api-secret")

# Load from AWS Secrets Manager
export REDIS_CLOUD_API_KEY=$(aws secretsmanager get-secret-value \
  --secret-id redis/api-key --query SecretString --output text)

# Load from HashiCorp Vault
export REDIS_CLOUD_API_KEY=$(vault kv get -field=api_key secret/redis)
```

### 3. Clear Sensitive Variables After Use

```bash
# Set variables for session
export REDIS_CLOUD_API_KEY="sensitive-key"
export REDIS_CLOUD_API_SECRET="sensitive-secret"

# Use the CLI
redisctl cloud subscription list

# Clear variables when done
unset REDIS_CLOUD_API_KEY
unset REDIS_CLOUD_API_SECRET
```

### 4. Use Environment Files Carefully

```bash
# Create env file with restricted permissions
cat > .env << EOF
REDIS_CLOUD_API_KEY=your-key
REDIS_CLOUD_API_SECRET=your-secret
EOF
chmod 600 .env

# Source the file
source .env

# Or use with Docker
docker run --env-file .env redisctl:latest cloud account info

# Never commit .env files
echo ".env" >> .gitignore
```

## Troubleshooting

### Variables Not Being Recognized

```bash
# Check if variable is set
echo $REDIS_CLOUD_API_KEY

# Check all Redis-related variables
env | grep REDIS

# Enable debug logging to see what's being used
RUST_LOG=debug redisctl cloud account info
```

### Incorrect Variable Values

```bash
# Check for extra spaces or quotes
export REDIS_CLOUD_API_KEY="key-without-quotes"  # Correct
export REDIS_CLOUD_API_KEY='"key-with-quotes"'   # Wrong - includes quotes

# Verify variable contents
echo "[$REDIS_CLOUD_API_KEY]"  # Brackets help show spaces
```

### Profile Override Issues

```bash
# Force use of environment variables over profile
unset REDISCTL_PROFILE  # Don't use any profile
export REDIS_CLOUD_API_KEY="override-key"
export REDIS_CLOUD_API_SECRET="override-secret"

# Now only environment variables will be used
redisctl cloud account info
```

## Quick Reference

### Minimal Cloud Setup

```bash
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_API_SECRET="your-secret"
redisctl cloud subscription list
```

### Minimal Enterprise Setup

```bash
export REDIS_ENTERPRISE_URL="https://cluster:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="password"
redisctl enterprise cluster info
```

### Debug Mode

```bash
export RUST_LOG=debug
export RUST_BACKTRACE=1
redisctl cloud database get --database-id 12345
```

## See Also

- [Configuration](./configuration.md) - Profile-based configuration
- [Authentication](../getting-started/authentication.md) - Setting up authentication
- [Docker Environment](./docker.md) - Running in Docker containers