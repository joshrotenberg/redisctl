# redisctl

[![CI](https://github.com/joshrotenberg/redisctl/actions/workflows/ci.yml/badge.svg)](https://github.com/joshrotenberg/redisctl/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/redisctl.svg)](https://crates.io/crates/redisctl)
[![Documentation](https://docs.rs/redisctl/badge.svg)](https://docs.rs/redisctl)
[![License](https://img.shields.io/crates/l/redisctl.svg)](https://github.com/joshrotenberg/redisctl/blob/main/LICENSE-MIT)

A unified command-line interface for managing Redis deployments across Cloud and Enterprise.

## Overview

`redisctl` provides a single, consistent CLI for managing both Redis Cloud and Redis Enterprise deployments. It automatically routes commands to the appropriate API based on your configuration, eliminating the need for multiple tools.

### Key Features
- 🚀 **Unified Interface** - Single CLI for both Redis Cloud and Enterprise
- 🔄 **Smart Routing** - Auto-detects which API to use based on profiles
- 📦 **Complete API Coverage** - 95%+ Cloud API, 100% Enterprise API
- 🎨 **Clean Output** - GitHub CLI-style tables with automatic paging
- 🔐 **Profile Management** - Store multiple deployment credentials securely
- 📊 **Flexible Output** - JSON, YAML, or table format with JMESPath queries
- 🦀 **Rust Libraries** - Type-safe async clients for building custom tools

## Installation

### From crates.io (Recommended)
```bash
cargo install redisctl
```

### From GitHub Releases
Download pre-built binaries for your platform from the [releases page](https://github.com/joshrotenberg/redisctl/releases).

### Using Docker
```bash
docker run --rm joshrotenberg/redisctl:latest --help
```

### From Source
```bash
git clone https://github.com/joshrotenberg/redisctl.git
cd redisctl
cargo install --path crates/redisctl
```

## Quick Start

### 1. Configure Your Profile

```bash
# Interactive setup (recommended)
redisctl auth setup

# Or create profiles manually
redisctl profile set prod-cloud cloud \
  --api-key "your-key" \
  --api-secret "your-secret"

redisctl profile set prod-enterprise enterprise \
  --url https://cluster:9443 \
  --username admin@example.com \
  --password your-password

# Set default profile
redisctl profile default prod-cloud
```

### 2. Test Your Connection

```bash
# Test authentication
redisctl auth test

# View configuration
redisctl profile list
```

### 3. Start Managing Redis

```bash
# Cloud commands
redisctl cloud subscription list
redisctl cloud database list
redisctl cloud user list

# Enterprise commands
redisctl enterprise cluster info
redisctl enterprise database list
redisctl enterprise node list

# Smart commands (auto-detect deployment type)
redisctl database list --profile prod-cloud
redisctl user list --profile prod-enterprise
```

## Common Operations

### Database Management
```bash
# List databases with clean output
redisctl database list

# Get detailed database info
redisctl database get 123456

# Create a new database
redisctl database create \
  --name "cache-db" \
  --memory-limit 1024 \
  --modules search,json
```

### Backup and Restore
```bash
# Create backup
redisctl cloud backup create \
  --subscription-id 12345 \
  --database-id 67890

# List backups
redisctl cloud backup list \
  --subscription-id 12345
```

### User Management
```bash
# List users
redisctl user list

# Get user details
redisctl user get 456

# Create ACL rules
redisctl cloud acl create \
  --name "readonly" \
  --rule "+get +mget -flushdb"
```

### Raw API Access
```bash
# Direct API calls when needed
redisctl api cloud get /subscriptions
redisctl api enterprise get /v1/nodes
```

## Output Formats

```bash
# Table format (default - clean GitHub CLI style)
redisctl database list

# JSON format
redisctl database list -o json

# YAML format  
redisctl database list -o yaml

# JMESPath queries
redisctl database list -q "[?status=='active'].name"
```

## Environment Variables

Instead of profiles, you can use environment variables:

```bash
# Redis Cloud
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_API_SECRET="your-secret"

# Redis Enterprise
export REDIS_ENTERPRISE_URL="https://cluster:9443"
export REDIS_ENTERPRISE_USER="admin@example.com"
export REDIS_ENTERPRISE_PASSWORD="your-password"
export REDIS_ENTERPRISE_INSECURE="true"  # For self-signed certs
```

## For Developers

### Rust Client Libraries

This project provides comprehensive Rust client libraries:

```toml
[dependencies]
redis-cloud = "0.2"       # Redis Cloud REST API client
redis-enterprise = "0.2"  # Redis Enterprise REST API client
```

Features:
- Complete type safety with full API coverage
- Async/await with Tokio
- Builder patterns for easy configuration
- Comprehensive test coverage (90%+)

### Example Usage
```rust
use redis_cloud::CloudClient;
use redis_enterprise::EnterpriseClient;

// Cloud client
let cloud = CloudClient::builder()
    .api_key("key")
    .api_secret("secret")
    .build()?;

let subscriptions = cloud.subscriptions().list().await?;

// Enterprise client  
let enterprise = EnterpriseClient::builder()
    .url("https://cluster:9443")
    .username("admin")
    .password("pass")
    .build()?;

let cluster = enterprise.cluster().get().await?;
```

## Project Status

### Current State (v0.2.0)
- ✅ **Cloud API**: 95%+ coverage (21 handlers)
- ✅ **Enterprise API**: 100% coverage (29 handlers)
- ✅ **Test Coverage**: 90%+ with 500+ tests
- ✅ **Profile Management**: Secure credential storage
- ✅ **Smart Routing**: Automatic API detection
- ✅ **Clean Output**: GitHub CLI-style formatting

### Roadmap
- [ ] Interactive resource creation wizards
- [ ] Workflow automation commands
- [ ] Performance benchmarking tools
- [ ] Migration utilities
- [ ] TUI mode for monitoring

## Documentation

- [User Guide](https://joshrotenberg.github.io/redisctl)
- [API Reference](https://docs.rs/redisctl)
- [Examples](https://github.com/joshrotenberg/redisctl/tree/main/examples)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.