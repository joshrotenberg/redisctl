# redisctl

A unified CLI for Redis Cloud and Redis Enterprise REST APIs with comprehensive async operation support.

## Features

- 🚀 **Unified Interface** - Single CLI for both Redis Cloud and Redis Enterprise
- ⏳ **Async Operations** - Full support for long-running operations with `--wait` flags
- 🔄 **Smart Routing** - Automatically detects which API to use based on context
- 📊 **Multiple Output Formats** - JSON, YAML, and Table output with JMESPath filtering
- 🔐 **Secure Configuration** - Profile-based auth with environment variable support
- 🌐 **Comprehensive Coverage** - Full API coverage for both platforms
A unified CLI for Redis Cloud and Redis Enterprise REST APIs with comprehensive async operation support.

## Features

- 🚀 **Unified Interface** - Single CLI for both Redis Cloud and Redis Enterprise
- ⏳ **Async Operations** - Full support for long-running operations with `--wait` flags
- 🔄 **Smart Routing** - Automatically detects which API to use based on context
- 📊 **Multiple Output Formats** - JSON, YAML, and Table output with JMESPath filtering
- 🔐 **Secure Configuration** - Profile-based auth with environment variable support

## Installation

```bash
# Install from crates.io
cargo install redisctl

# Or build from source
git clone https://github.com/joshrotenberg/redisctl.git
cd redisctl
cargo install --path crates/redisctl
```

## Quick Start

### Configure Authentication

Create `~/.config/redisctl/config.toml`:

```toml
[profiles.cloud]
deployment_type = "cloud"
api_key = "your-api-key"
api_secret = "your-secret-key"

[profiles.enterprise]
deployment_type = "enterprise"
url = "https://cluster:9443"
username = "admin@example.com"
password = "your-password"

default_profile = "cloud"
```

Or use environment variables:

```bash
# Redis Cloud
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_API_SECRET="your-secret"

# Redis Enterprise
export REDIS_ENTERPRISE_URL="https://cluster:9443"
export REDIS_ENTERPRISE_USER="admin@example.com"
export REDIS_ENTERPRISE_PASSWORD="your-password"
```

### Basic Usage

### Database Operations

```bash
# List databases
redisctl database list

# Create database with async wait
redisctl cloud database create --data @database.json --wait

# Create database with async wait
redisctl cloud database create
# UpdateDifferent output
redisctl Deletedatabaselist-o yaml | yq '.[] | select(.name == "prod")' database with force and wait
redisctl cloud database delete 12345 --force --wait
```

## Output Formats

```bash
# JSON output (default)
redisctl database list -o json

# YAML output
redisctl database list -o yaml

# Human-readable table
redisctl database list -o table

# Filter with JMESPath
redisctl database list -q "[?status=='active'].{name: name, memory: memoryLimitInGb}"

# Combine with jq for advanced processing
redisctl database list -o json | jq '.[] | select(.name | contains("prod"))'
```

## Profile Management

```bash
# List all profiles
redisctl profile list

# Set default profile
redisctl profile default cloud-prod

# Get specific profile settings
redisctl profile get enterprise-dev

# Set profile values
redisctl profile set cloud-staging api_key "new-key"
redisctl profile set cloud-staging api_secret "new-secret"

# Remove profile
redisctl profile remove old-profile

# Use specific profile for a command
redisctl database list --profile cloud-staging
```

## Environment Variables

### Cloud Configuration
- `REDIS_CLOUD_API_KEY` - API key for authentication
- `REDIS_CLOUD_API_SECRET` - API secret for authentication
- `REDIS_CLOUD_API_URL` - Custom API URL (optional)

### Enterprise Configuration
- `REDIS_ENTERPRISE_URL` - Cluster API URL
- `REDIS_ENTERPRISE_USER` - Username for authentication
- `REDIS_ENTERPRISE_PASSWORD` - Password for authentication
- `REDIS_ENTERPRISE_INSECURE` - Allow insecure TLS (true/false)

### General Configuration
- `REDISCTL_PROFILE` - Default profile to use
- `RUST_LOG` - Logging level (error, warn, info, debug, trace)

## Documentation

For comprehensive documentation, see the [mdBook documentation](docs/):

- [Getting Started](docs/src/getting-started/index.md) - Installation and configuration
- [CLI Reference](docs/src/cli-reference/index.md) - Complete command reference
- [Async Operations](docs/src/features/async-operations.md) - Using `--wait` flags  
- [Examples](docs/src/examples/index.md) - Common use cases and patterns
- **API Reference** - Complete command reference

## Development

This project provides Rust client libraries for both APIs:

```toml
[dependencies]
redis-cloud = "0.2"       # Redis Cloud API client
redis-enterprise = "0.2"  # Redis Enterprise API client
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.