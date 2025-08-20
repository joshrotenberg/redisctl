# redisctl

A unified command-line interface for managing Redis deployments across Cloud and Enterprise.

## Overview

`redisctl` is a comprehensive CLI and Rust library that unifies management of both Redis Cloud and Redis Enterprise deployments. It automatically detects which API to use based on your configuration profile or explicit command selection, providing a consistent interface for all Redis management tasks.

## Features

### Core Capabilities
- **Unified Interface** - Single CLI for both Redis Cloud and Enterprise
- **Smart Command Routing** - Automatically routes commands based on deployment type
- **Profile Management** - Save and switch between multiple Redis deployments
- **Multiple Output Formats** - JSON, YAML, and Table output with JMESPath queries
- **Type-Safe Rust Libraries** - Build custom tools with our async Rust client libraries
- **Comprehensive API Coverage** - Full implementation of both Cloud and Enterprise REST APIs

### Advanced Features
- **Workflow Commands** - High-level operations for complex multi-step tasks
- **Cluster Initialization** - Bootstrap and configure new Enterprise clusters
- **Migration Tools** - Move databases between Cloud and Enterprise
- **Backup & Restore** - Automated backup management and recovery
- **Docker Integration** - Easy local testing with Redis Enterprise
- **Raw API Access** - Direct access to any API endpoint

## Installation

### From Source
```bash
# Clone and build
git clone https://github.com/redis-field-engineering/redisctl.git
cd redisctl
cargo build --release

# Install to PATH
cargo install --path crates/redisctl

# Or use the binary directly
./target/release/redisctl --help
```

### Platform-Specific Binaries
```bash
# Build Cloud-only binary (smaller size)
cargo build --release --features cloud-only --bin redis-cloud

# Build Enterprise-only binary (smaller size)
cargo build --release --features enterprise-only --bin redis-enterprise

# Build unified binary (default, includes both)
cargo build --release --bin redisctl
```

### Using Docker (for Enterprise testing)
```bash
# Start Redis Enterprise cluster
make docker-up

# Access the CLI
make docker-cli

# Run tests
make docker-test

# Clean up
make docker-down
```

## Quick Start

### 1. Configure Authentication

#### Redis Cloud
```bash
# Using environment variables
export REDIS_CLOUD_API_KEY="your-api-key"
export REDIS_CLOUD_API_SECRET="your-api-secret"

# Or using profiles
redisctl profile set prod-cloud \
  --deployment-type cloud \
  --api-key YOUR_KEY \
  --api-secret YOUR_SECRET
```

#### Redis Enterprise
```bash
# Using environment variables
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@example.com"
export REDIS_ENTERPRISE_PASSWORD="your-password"

# Or using profiles
redisctl profile set prod-enterprise \
  --deployment-type enterprise \
  --url https://cluster:9443 \
  --username admin \
  --password secret
```

### 2. Basic Usage

```bash
# List all profiles
redisctl profile list

# Set default profile
redisctl profile default prod-cloud

# Explicit deployment commands
redisctl cloud subscription list
redisctl enterprise cluster info

# Smart routing (auto-detects based on profile)
redisctl database list --profile prod-cloud
redisctl user list --profile prod-enterprise

# Query and format output
redisctl database list -o json | jq '.[] | .name'
redisctl database list -q "[?status=='active'].name" -o yaml
```

### 3. Common Workflows

```bash
# Initialize a new Enterprise cluster
redisctl enterprise bootstrap create-cluster \
  --name "my-cluster" \
  --username admin@example.com \
  --accept-eula

# Create a database
redisctl database create \
  --name "my-database" \
  --memory-limit 1024 \
  --modules search,json

# Backup and restore
redisctl database backup --id 1
redisctl database restore --id 1 --backup-id latest
```

## Architecture

### Workspace Structure
```
redisctl/
├── crates/
│   ├── redis-cloud/         # Cloud API client library
│   ├── redis-enterprise/    # Enterprise API client library
│   ├── redis-common/        # Shared utilities
│   └── redisctl/           # Unified CLI application
├── docs/                    # Documentation (mdBook)
├── tests/                   # Integration tests
└── examples/               # Usage examples
```

### Key Components

#### Libraries
- **redis-cloud** - Complete Redis Cloud REST API client
  - All Cloud API endpoints implemented
  - Async/await with Tokio
  - Full type safety with Rust
  
- **redis-enterprise** - Complete Redis Enterprise REST API client
  - All Enterprise API endpoints implemented
  - Support for cluster management, CRDB, modules
  - Bootstrap and initialization workflows

- **redis-common** - Shared utilities
  - Configuration and profile management
  - Output formatting (JSON, YAML, Table)
  - JMESPath query engine
  - Error handling

#### CLI Application
- **redisctl** - Unified command-line interface
  - Smart command routing
  - Profile-based configuration
  - High-level workflows
  - Interactive mode (planned)

## Development

### Building
```bash
# Run tests
cargo test --workspace

# Run with all features
cargo test --all-features

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

### Documentation
```bash
# Build documentation
cd docs && mdbook build

# Serve documentation locally
cd docs && mdbook serve

# Generate API docs
cargo doc --no-deps --open
```

### Contributing
Please see our [Contributing Guide](CONTRIBUTING.md) for details on:
- Code style and standards
- Testing requirements
- Pull request process
- Issue reporting

## API Coverage

### Redis Cloud
- ✅ Subscriptions (list, get, create, update, delete)
- ✅ Databases (full CRUD operations)
- ✅ Cloud Accounts (AWS, GCP, Azure integration)
- ✅ Users & ACLs
- ✅ Backup & Import
- ✅ VPC Peering
- ✅ Transit Gateway
- 🚧 Active-Active databases
- 🚧 SAML SSO configuration

### Redis Enterprise
- ✅ Cluster management
- ✅ Database (BDB) operations
- ✅ Users & roles
- ✅ Modules management
- ✅ Bootstrap & initialization
- ✅ Backup & restore
- 🚧 CRDB (Active-Active)
- 🚧 LDAP integration
- 🚧 Certificates (OCSP)

## Roadmap

See our [GitHub Issues](https://github.com/redis-field-engineering/redisctl/issues) for the complete roadmap. Key priorities:

1. **Phase 1** - Core functionality
   - Complete API coverage for both platforms
   - Comprehensive test suite
   - CI/CD automation

2. **Phase 2** - Enhanced workflows
   - Cluster initialization workflows
   - Migration tools (Cloud ↔ Enterprise)
   - Disaster recovery automation

3. **Phase 3** - Advanced features
   - Interactive TUI mode
   - Plugin system
   - Terraform provider
   - Kubernetes operator

## Support

- **Issues**: [GitHub Issues](https://github.com/redis-field-engineering/redisctl/issues)
- **Documentation**: [Online Docs](https://redis-field-engineering.github.io/redisctl/)
- **Examples**: See the [examples/](examples/) directory

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.