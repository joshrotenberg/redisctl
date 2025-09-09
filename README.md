# redisctl

[![Crates.io](https://img.shields.io/crates/v/redisctl.svg)](https://crates.io/crates/redisctl)
[![Downloads](https://img.shields.io/crates/d/redisctl.svg)](https://crates.io/crates/redisctl)
[![Documentation](https://docs.rs/redisctl/badge.svg)](https://docs.rs/redisctl)
[![GitHub Release](https://img.shields.io/github/v/release/joshrotenberg/redisctl)](https://github.com/joshrotenberg/redisctl/releases)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/joshrotenberg/redisctl#license)
[![CI](https://github.com/joshrotenberg/redisctl/actions/workflows/ci.yml/badge.svg)](https://github.com/joshrotenberg/redisctl/actions/workflows/ci.yml)
[![Security Audit](https://github.com/joshrotenberg/redisctl/actions/workflows/security.yml/badge.svg)](https://github.com/joshrotenberg/redisctl/actions/workflows/security.yml)
[![Docker](https://img.shields.io/docker/v/joshrotenberg/redisctl?label=docker)](https://hub.docker.com/r/joshrotenberg/redisctl)

A unified CLI for Redis Cloud and Redis Enterprise REST APIs with comprehensive async operation support.

## Features

- **Unified Interface** - Single CLI for both Redis Cloud and Redis Enterprise
- **Async Operations** - Full support for long-running operations with `--wait` flags
- **Smart Routing** - Automatically detects which API to use based on context
- **Multiple Output Formats** - JSON, YAML, and Table output with JMESPath filtering
- **Secure Configuration** - Profile-based auth with environment variable support
- **Comprehensive Coverage** - Full API coverage for both platforms

## Installation

```bash
# Install from crates.io
cargo install redisctl

# Or build from source
git clone https://github.com/joshrotenberg/redisctl.git
cd redisctl
cargo install --path crates/redisctl
```

### Shell Completions

`redisctl` can generate shell completions for various shells. To install them:

#### Bash
```bash
# Generate and install completion
redisctl completions bash > ~/.local/share/bash-completion/completions/redisctl

# Or for system-wide installation (requires sudo)
redisctl completions bash | sudo tee /usr/share/bash-completion/completions/redisctl
```

#### Zsh
```bash
# Generate completion to a directory in your $fpath
redisctl completions zsh > ~/.zsh/completions/_redisctl

# Add this to your ~/.zshrc if not already present
fpath=(~/.zsh/completions $fpath)
autoload -U compinit && compinit
```

#### Fish
```bash
# Generate completion
redisctl completions fish > ~/.config/fish/completions/redisctl.fish
```

#### PowerShell
```powershell
# Generate completion
redisctl completions powershell > $PROFILE.CurrentUserAllHosts

# Or add to your profile
redisctl completions powershell >> $PROFILE
```

#### Elvish
```bash
# Generate completion
redisctl completions elvish > ~/.elvish/lib/redisctl.elv

# Add to rc.elv
echo "use redisctl" >> ~/.elvish/rc.elv
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

# Update database with wait
redisctl cloud database update 12345 --data @update.json --wait

# Different output formats with filtering
redisctl database list -o yaml | yq '.[] | select(.name == "prod")'

# Delete database with force and wait
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