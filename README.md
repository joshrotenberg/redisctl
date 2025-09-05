# redisctl

A unified CLI for Redis Cloud and Redis Enterprise REST APIs.

## Installation

```bash
# Install from source
cargo install --path crates/redisctl

# Or install from crates.io (coming soon)
cargo install redisctl
```

## Quick Configuration

Create `~/.config/redisctl/config.toml`:

```toml
# Redis Cloud Profile
[profiles.cloud]
deployment_type = "cloud"
api_key = "your-account-key"          # From Redis Cloud console
api_secret = "your-secret-key"        # Keep this secret!

# Redis Enterprise Profile  
[profiles.enterprise]
deployment_type = "enterprise"
url = "https://your-cluster:9443"
username = "admin@example.com"
password = "your-password"
insecure = true                       # For self-signed certificates

# Set your default
default_profile = "cloud"
```

Or use environment variables:

```bash
# For Redis Cloud
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_API_SECRET="your-secret"

# For Redis Enterprise
export REDIS_ENTERPRISE_URL="https://cluster:9443"
export REDIS_ENTERPRISE_USER="admin@example.com"
export REDIS_ENTERPRISE_PASSWORD="your-password"
```

## Basic Usage

```bash
# List databases
redisctl database list

# Get specific database
redisctl database get 12345

# Direct API access
redisctl api cloud get /subscriptions
redisctl api enterprise get /v1/cluster

# Output formats
redisctl database list -o json
redisctl database list -o yaml
redisctl database list -o table

# Filter with JMESPath
redisctl database list -q "[?status=='active'].name"
```

## Command Structure

```
redisctl
├── api          # Raw API access (any endpoint)
├── cloud        # Cloud-specific commands
├── enterprise   # Enterprise-specific commands
├── database     # Smart commands (work with both)
└── profile      # Manage configuration profiles
```

## Documentation

For comprehensive documentation, see the [User Guide](https://docs.rs/redisctl).

- **Getting Started** - Installation, configuration, first commands
- **Redis Cloud** - Cloud-specific operations and API reference
- **Redis Enterprise** - Enterprise-specific operations and API reference
- **Examples** - Common use cases and patterns

## Development

This project provides Rust client libraries for both APIs:

```toml
[dependencies]
redis-cloud = "0.2"       # Redis Cloud API client
redis-enterprise = "0.2"  # Redis Enterprise API client
```

## License

MIT