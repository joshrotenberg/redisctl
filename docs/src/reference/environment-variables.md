# Environment Variables

Complete reference of environment variables supported by `redisctl`.

## Redis Cloud

| Variable | Description | Example |
|----------|-------------|---------|
| `REDIS_CLOUD_API_KEY` | API account key | `A3qcymrvqpn9rr...` |
| `REDIS_CLOUD_API_SECRET` | API secret key | `S3s8ecrrnaguqk...` |
| `REDIS_CLOUD_API_URL` | API endpoint (optional) | `https://api.redislabs.com/v1` |

## Redis Enterprise

| Variable | Description | Example |
|----------|-------------|---------|
| `REDIS_ENTERPRISE_URL` | Cluster API URL | `https://cluster:9443` |
| `REDIS_ENTERPRISE_USER` | Username | `admin@cluster.local` |
| `REDIS_ENTERPRISE_PASSWORD` | Password | `your-password` |
| `REDIS_ENTERPRISE_INSECURE` | Allow self-signed certs | `true` or `false` |

## General

| Variable | Description | Example |
|----------|-------------|---------|
| `REDISCTL_PROFILE` | Default profile name | `production` |
| `REDISCTL_OUTPUT` | Default output format | `json`, `yaml`, `table` |
| `RUST_LOG` | Logging level | `error`, `warn`, `info`, `debug` |
| `NO_COLOR` | Disable colored output | `1` or any value |

## Usage Examples

### Basic Setup

```bash
# Redis Cloud
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_API_SECRET="your-secret"

# Redis Enterprise
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="password"
export REDIS_ENTERPRISE_INSECURE="true"
```

### Debugging

```bash
# Enable debug logging
export RUST_LOG=debug
redisctl api cloud get /account

# Trace specific modules
export RUST_LOG=redisctl=debug,redis_cloud=trace
```

### CI/CD

```yaml
# GitHub Actions
env:
  REDIS_CLOUD_API_KEY: ${{ secrets.REDIS_API_KEY }}
  REDIS_CLOUD_API_SECRET: ${{ secrets.REDIS_API_SECRET }}
```

## Precedence

Environment variables are overridden by:
1. Command-line flags (highest priority)
2. Configuration file settings

But override:
1. Default values (lowest priority)