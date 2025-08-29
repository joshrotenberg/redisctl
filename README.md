# redisctl

[![CI](https://github.com/joshrotenberg/redisctl/actions/workflows/ci.yml/badge.svg)](https://github.com/joshrotenberg/redisctl/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/redisctl.svg)](https://crates.io/crates/redisctl)
[![Documentation](https://docs.rs/redisctl/badge.svg)](https://docs.rs/redisctl)
[![License](https://img.shields.io/crates/l/redisctl.svg)](https://github.com/joshrotenberg/redisctl/blob/main/LICENSE-MIT)

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
- **Cluster Initialization** - Bootstrap and configure new Enterprise clusters
- **Backup & Restore** - Automated backup management and recovery
- **VPC Peering & Transit Gateway** - Complete networking management for Cloud
- **ACL Management** - Database access control and security rules
- **Docker Integration** - Easy local testing with Redis Enterprise
- **Raw API Access** - Direct access to any API endpoint

## Installation

### CLI Tool (Recommended for most users)

#### Install from crates.io
```bash
# Install the latest stable release
cargo install redisctl
```

#### Install from GitHub releases
Pre-built binaries are available for multiple platforms on our [releases page](https://github.com/joshrotenberg/redisctl/releases).

#### Docker
```bash
# Run using Docker
docker run --rm joshrotenberg/redisctl:latest --help
```

### Rust Libraries (For developers building custom tools)

This project also provides **comprehensive Rust client libraries** for both Redis Cloud and Enterprise REST APIs:

```toml
# Add to your Cargo.toml
[dependencies]
redis-cloud = "0.1.0"       # Full Redis Cloud REST API client
redis-enterprise = "0.1.0"  # Full Redis Enterprise REST API client
```

These libraries offer:
- **100% API coverage** - Every documented endpoint implemented
- **Full type safety** - Strongly typed request/response structures
- **Async/await** - Modern async Rust with Tokio
- **Builder patterns** - Ergonomic client configuration
- **Comprehensive testing** - Battle-tested with 500+ tests

Perfect for building custom automation, integrations, or management tools.

### From Source
```bash
# Clone and build
git clone https://github.com/joshrotenberg/redisctl.git
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

### Using Docker Hub Image

The official Docker images are available on Docker Hub at [joshrotenberg/redisctl](https://hub.docker.com/r/joshrotenberg/redisctl):

#### Available Tags
- `latest` - Latest stable release
- `v0.1.0`, `v0.2.0`, etc. - Specific version tags
- `main` - Latest development build from main branch

```bash
# Pull the latest image
docker pull joshrotenberg/redisctl:latest

# Or pull a specific version
docker pull joshrotenberg/redisctl:v0.1.0

# Run a command directly
docker run --rm joshrotenberg/redisctl:latest --help

# Use with environment variables for Redis Cloud
docker run --rm \
  -e REDIS_CLOUD_API_KEY="your-key" \
  -e REDIS_CLOUD_API_SECRET="your-secret" \
  joshrotenberg/redisctl:latest cloud subscription list

# Use with environment variables for Redis Enterprise
docker run --rm \
  -e REDIS_ENTERPRISE_URL="https://your-cluster:9443" \
  -e REDIS_ENTERPRISE_USER="admin@example.com" \
  -e REDIS_ENTERPRISE_PASSWORD="your-password" \
  -e REDIS_ENTERPRISE_INSECURE="true" \
  joshrotenberg/redisctl:latest enterprise cluster info

# Run interactively with a shell
docker run -it --rm \
  -e REDIS_ENTERPRISE_URL="https://your-cluster:9443" \
  -e REDIS_ENTERPRISE_USER="admin@example.com" \
  -e REDIS_ENTERPRISE_PASSWORD="your-password" \
  --entrypoint /bin/sh \
  joshrotenberg/redisctl:latest
```

### Local Development with Docker Compose
```bash
# Start Redis Enterprise cluster with initialization
docker compose up -d

# Check cluster status
docker compose logs init-cluster

# Watch logs
docker compose logs -f

# Clean up
docker compose down -v
```

## Quick Start

### 1. Configure Authentication

#### Option 1: Interactive Setup Wizard (Recommended)
```bash
# Launch guided setup for any deployment type
redisctl auth setup

# Test your authentication
redisctl auth test
```

The interactive setup wizard will:
- Guide you through credential collection
- Test authentication during setup
- Create and save working profiles
- Set up your first profile as default

#### Option 2: Manual Profile Creation

##### Redis Cloud
```bash
# Create a Cloud profile manually
redisctl profile set prod-cloud cloud \
  --api-key "your-api-key" \
  --api-secret "your-api-secret"

# Set as default profile
redisctl profile default prod-cloud
```

##### Redis Enterprise
```bash
# Create an Enterprise profile manually
redisctl profile set prod-enterprise enterprise \
  --url https://cluster:9443 \
  --username admin@example.com \
  --password your-password
```

#### Option 3: Environment Variables
```bash
# Redis Cloud
export REDIS_CLOUD_API_KEY="your-api-key"
export REDIS_CLOUD_API_SECRET="your-api-secret"

# Redis Enterprise
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@example.com"  
export REDIS_ENTERPRISE_PASSWORD="your-password"

# Test authentication works
redisctl auth test
```

### 2. Verify Your Setup

```bash
# Test authentication for any profile or environment vars
redisctl auth test
redisctl auth test --profile prod-cloud

# View your configuration
redisctl config show

# Validate all profiles  
redisctl config validate

# Find your config file location
redisctl config path
```

### 3. Basic Usage

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

### 4. Common Workflows

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

# Cloud-specific workflows
redisctl cloud backup create --subscription-id 123 --database-id 456
redisctl cloud peering create --subscription-id 123 --region us-east-1 --provider aws
redisctl cloud acl create --subscription-id 123 --database-id 456 --name readonly-rule

# Enterprise workflows  
redisctl enterprise database backup --database-id 1
redisctl enterprise database import --database-id 1 --source-uri redis://source:6379
```

## Architecture

### Workspace Structure
```
redisctl/
├── crates/
│   ├── redis-cloud/         # Cloud API client library
│   ├── redis-enterprise/    # Enterprise API client library
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

# Pre-commit hooks (recommended)
./scripts/install-hooks.sh
```

### Contributing
Please see our [Contributing Guide](CONTRIBUTING.md) for details on:
- Code style and standards
- Testing requirements
- Pull request process
- Issue reporting

## API Coverage

### Redis Cloud API (95%+ Coverage)
- ✅ **Core Operations**: Subscriptions, Databases, Users, Payment Methods
- ✅ **Security**: ACLs, API Keys, Redis Rules, SSO/SAML Integration
- ✅ **Networking**: VPC Peering, Transit Gateway, Private Service Connect
- ✅ **Data Management**: Backup/Restore, Import/Export, Active-Active (CRDB)
- ✅ **Monitoring**: Metrics, Logs, Tasks, Alerts
- ✅ **Cloud Integration**: AWS, GCP, Azure Cloud Accounts
- ✅ **Billing**: Invoices, Payment Methods, Cost Analysis
- ✅ **21 Handler Modules** with 200+ API endpoints implemented

### Redis Enterprise API (100% Coverage)
- ✅ **Cluster Operations**: Bootstrap, Join, Management, Recovery
- ✅ **Database Management**: Full BDB lifecycle, Actions, Stats, Shards
- ✅ **Security**: Users, Roles, LDAP, Redis ACLs, OCSP
- ✅ **Active-Active**: CRDB management, Tasks, Multi-region
- ✅ **Monitoring**: Alerts, Stats, Logs, Diagnostics
- ✅ **Advanced Features**: Modules, Proxies, Services, Migrations
- ✅ **29 Handler Modules** covering all documented REST API endpoints

## Roadmap

See our [GitHub Issues](https://github.com/joshrotenberg/redisctl/issues) for the complete roadmap.

### ✅ **Phase 1** - Raw API Access (Complete)
   - Redis Cloud API coverage (95%+)
   - Redis Enterprise API coverage (100%)
   - Comprehensive test suite (500+ tests)
   - CI/CD automation with pre-commit hooks
   - Published to crates.io as v0.1.0

### ✅ **Phase 2** - Human-Friendly Commands (Complete)
   - Enhanced command interface with smart routing
   - Consistent --force flags and output formatting
   - JMESPath queries and multiple output formats
   - Major Cloud API categories now supported

### 🚧 **Phase 3** - Workflow Commands (In Progress)
   - High-level operations for complex multi-step tasks
   - Migration tools (Cloud ↔ Enterprise)
   - Cluster initialization workflows
   - Disaster recovery automation

### 🔮 **Phase 4** - Advanced Features (Planned)
   - Interactive TUI mode
   - Plugin system
   - Terraform provider integration
   - Kubernetes operator

## Rust Library Usage

For developers who want to build their own tools, our libraries provide complete, type-safe access to Redis Cloud and Enterprise APIs:

Add to your `Cargo.toml`:
```toml
[dependencies]
redis-cloud = "0.1.0"        # For Cloud API
redis-enterprise = "0.1.0"   # For Enterprise API
```

### Quick Example
```rust
use redis_cloud::CloudClient;
use redis_enterprise::EnterpriseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Redis Cloud API client
    let cloud = CloudClient::new("api_key", "api_secret")?;
    
    // List all databases in a subscription
    let databases = cloud.database().list(subscription_id).await?;
    
    // Create a new database
    let new_db = cloud.database()
        .create(subscription_id, CreateDatabaseRequest {
            name: "production-cache".to_string(),
            memory_limit_in_gb: 10.0,
            // ... other settings
        })
        .await?;
    
    // Redis Enterprise API client
    let enterprise = EnterpriseClient::builder()
        .url("https://cluster:9443")
        .username("admin@example.com")
        .password("secure_password")
        .insecure(false)  // Set true for self-signed certs
        .build()?;
    
    // Get cluster information
    let cluster = enterprise.cluster().get().await?;
    
    // Create a database
    let db = enterprise.database()
        .create(CreateDatabaseRequest {
            name: "mydb".to_string(),
            memory_size: 1073741824,  // 1GB in bytes
            // ... other settings
        })
        .await?;
    
    Ok(())
}
```

### Library Features
- **Comprehensive handlers** for all API endpoints (subscriptions, databases, users, ACLs, etc.)
- **Builder patterns** for complex request construction
- **Error handling** with detailed context and retry logic
- **Both typed and untyped** responses (use `.raw()` methods for `serde_json::Value`)
- **Extensive documentation** on [docs.rs](https://docs.rs/redis-cloud) and [docs.rs](https://docs.rs/redis-enterprise)

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
```bash
# Clone the repository
git clone https://github.com/joshrotenberg/redisctl.git
cd redisctl

# Run tests
cargo test --workspace

# Run with local development
cargo run -- --help

# Start local Redis Enterprise for testing
docker compose up -d
```

### Release Process
This project uses automated releases via:
- **release-plz** - Automated version management and changelog generation
- **cargo-dist** - Cross-platform binary builds
- **Conventional Commits** - Semantic versioning from commit messages

For details, see our [Release Process Documentation](docs/RELEASE_PROCESS.md).

## Support

- **Issues**: [GitHub Issues](https://github.com/joshrotenberg/redisctl/issues)
- **Documentation**: [docs.rs/redisctl](https://docs.rs/redisctl/)
- **Examples**: See the [examples/](examples/) directory
- **Crates.io**: [crates.io/crates/redisctl](https://crates.io/crates/redisctl)
- **Docker Hub**: [joshrotenberg/redisctl](https://hub.docker.com/r/joshrotenberg/redisctl)

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.