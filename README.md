# redisctl

[![Crates.io](https://img.shields.io/crates/v/redisctl.svg)](https://crates.io/crates/redisctl)
[![Documentation](https://docs.rs/redisctl/badge.svg)](https://docs.rs/redisctl)
[![CI](https://github.com/joshrotenberg/redisctl/actions/workflows/ci.yml/badge.svg)](https://github.com/joshrotenberg/redisctl/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/joshrotenberg/redisctl#license)

A unified command-line interface for managing Redis Cloud and Redis Enterprise deployments through their REST APIs.

## Features

- **Unified Interface** - Single CLI for both Redis Cloud and Redis Enterprise
- **Async Operations** - Full support for long-running operations with `--wait` flags  
- **Explicit Commands** - Clear separation between Cloud and Enterprise operations
- **Multiple Output Formats** - JSON, YAML, and Table output with JMESPath filtering
- **Profile Management** - Secure credential storage with environment variable support

## Installation

### Homebrew (macOS/Linux)
```bash
brew install joshrotenberg/brew/redisctl
```

### From Binary Releases
Download the latest release for your platform from the [releases page](https://github.com/joshrotenberg/redisctl/releases).

### From Cargo
```bash
cargo install redisctl
```

### Using Docker
```bash
docker run --rm joshrotenberg/redisctl --help
```

## Quick Start

### 1. Configure Authentication

```bash
# Redis Cloud
export REDIS_CLOUD_API_KEY="your-api-key"
export REDIS_CLOUD_SECRET_KEY="your-secret-key"

# Redis Enterprise
export REDIS_ENTERPRISE_URL="https://your-cluster:9443"
export REDIS_ENTERPRISE_USER="your-email@example.com"
export REDIS_ENTERPRISE_PASSWORD="your-password"
```

Or use profiles for multiple environments:

```bash
# Create a profile with plaintext storage (default)
redisctl profile set cloud-prod \
  --deployment cloud \
  --api-key "$REDIS_CLOUD_API_KEY" \
  --api-secret "$REDIS_CLOUD_SECRET_KEY"

# Create a profile with secure OS keyring storage (recommended)
redisctl profile set cloud-secure \
  --deployment cloud \
  --api-key "$REDIS_CLOUD_API_KEY" \
  --api-secret "$REDIS_CLOUD_SECRET_KEY" \
  --use-keyring  # Requires 'secure-storage' feature

# Use the profile
redisctl --profile cloud-prod cloud database list
```

### Profile Storage

Profiles are stored in:
- **Linux/macOS**: `~/.config/redisctl/config.toml`
- **Windows**: `%APPDATA%\redis\redisctl\config.toml`

### Secure Credential Storage

When compiled with the `secure-storage` feature, redisctl can store sensitive credentials in your OS keyring:

- **macOS**: Keychain
- **Windows**: Windows Credential Store
- **Linux**: Secret Service (GNOME Keyring, KWallet)

To use secure storage:
```bash
# Install with secure storage support
cargo install redisctl --features secure-storage

# Create profile with keyring storage
redisctl profile set prod \
  --deployment cloud \
  --api-key "your-key" \
  --api-secret "your-secret" \
  --use-keyring

# The config file will contain references like:
# api_key = "keyring:prod-api-key"
# api_secret = "keyring:prod-api-secret"
```

Example configuration file:
```toml
default_profile = "cloud-prod"

[profiles.cloud-prod]
deployment_type = "cloud"
api_key = "${REDIS_CLOUD_API_KEY}"
api_secret = "${REDIS_CLOUD_API_SECRET}"

[profiles.enterprise-dev]
deployment_type = "enterprise"
url = "https://localhost:9443"
username = "admin@redis.local"
password = "${REDIS_ENTERPRISE_PASSWORD}"
insecure = true
```

### 2. Verify Setup & First Commands

```bash
# Test your connection
redisctl api cloud get /        # For Cloud
redisctl api enterprise get /v1/cluster  # For Enterprise

# View your configuration
redisctl profile list           # Show all profiles
redisctl profile path           # Show config file location

# Common first commands
redisctl cloud database list      # List Cloud databases
redisctl cloud subscription list  # List Cloud subscriptions
redisctl enterprise node list      # List Enterprise nodes

# Get detailed output
redisctl cloud database get 12345 --output table  # Table format
redisctl cloud database list -o json | jq         # JSON with jq

# Create resources and wait for completion
redisctl cloud database create --data @database.json --wait
```

## Using with Docker

```bash
# Run commands directly with Docker
docker run --rm \
  -e REDIS_CLOUD_API_KEY \
  -e REDIS_CLOUD_API_SECRET \
  joshrotenberg/redisctl:latest \
  cloud subscription list

# Use local config file
docker run --rm \
  -v ~/.config/redisctl:/root/.config/redisctl:ro \
  joshrotenberg/redisctl:latest \
  database list

# Development environment with test cluster
docker compose up -d                    # Start Redis Enterprise cluster
docker compose --profile cli up cli     # Interactive CLI session
docker compose --profile test up test   # Run test suite
docker compose down -v                  # Clean up
```

See the [Docker documentation](https://joshrotenberg.com/redisctl/getting-started/docker.html) for advanced usage.

## Documentation

For comprehensive documentation including:
- Detailed configuration options
- Complete command reference
- Async operation handling
- Output formatting and filtering
- Troubleshooting guides

Visit the [full documentation](https://joshrotenberg.github.io/redisctl/).

## Development

### Building from Source
```bash
git clone https://github.com/joshrotenberg/redisctl.git
cd redisctl
cargo build --release
```

### Running Tests
```bash
cargo test --workspace
```

## Contributing

Contributions are welcome! Please see the [contributing guidelines](https://joshrotenberg.github.io/redisctl/developer/contributing.html) in the documentation.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.