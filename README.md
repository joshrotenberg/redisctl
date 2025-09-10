# redisctl

[![Crates.io](https://img.shields.io/crates/v/redisctl.svg)](https://crates.io/crates/redisctl)
[![Documentation](https://docs.rs/redisctl/badge.svg)](https://docs.rs/redisctl)
[![CI](https://github.com/joshrotenberg/redisctl/actions/workflows/ci.yml/badge.svg)](https://github.com/joshrotenberg/redisctl/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/joshrotenberg/redisctl#license)

A unified command-line interface for managing Redis Cloud and Redis Enterprise deployments through their REST APIs.

## Features

- **Unified Interface** - Single CLI for both Redis Cloud and Redis Enterprise
- **Async Operations** - Full support for long-running operations with `--wait` flags
- **Smart Routing** - Automatically detects which API to use based on context
- **Multiple Output Formats** - JSON, YAML, and Table output with JMESPath filtering
- **Profile Management** - Secure credential storage with environment variable support

## Installation

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
# Create a profile
redisctl profile set cloud-prod \
  --cloud-api-key="$REDIS_CLOUD_API_KEY" \
  --cloud-secret-key="$REDIS_CLOUD_SECRET_KEY"

# Use the profile
redisctl --profile cloud-prod cloud database list
```

Profiles are stored in:
- **Linux/macOS**: `~/.config/redisctl/config.toml`
- **Windows**: `%APPDATA%\redis\redisctl\config.toml`

### 2. Basic Commands

```bash
# List databases (auto-detects deployment type from profile)
redisctl database list

# Get database details with formatted output
redisctl database get 12345 --output table

# Create a database and wait for completion
redisctl cloud database create --data @database.json --wait

# Direct API access for any endpoint
redisctl api cloud get /subscriptions/88449/databases
```

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