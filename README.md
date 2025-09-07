# redisctl

A unified CLI for Redis Cloud and Redis Enterprise REST APIs with comprehensive async operation support.

## Features

- 🚀 **Unified Interface** - Single CLI for both Redis Cloud and Redis Enterprise
- ⏳ **Async Operations** - Full support for long-running operations with `--wait` flags
- 🔄 **Smart Routing** - Automatically detects which API to use based on context
- 📊 **Multiple Output Formats** - JSON, YAML, and Table output with JMESPath filtering
- 🔐 **Secure Configuration** - Profile-based auth with environment variable support
- 🌐 **Comprehensive Coverage** - Full API coverage for both platforms

## Installation

```bash
# Install from source
cargo install --path crates/redisctl

# Or install from crates.io
cargo install redisctl

# Or download pre-built binaries from GitHub releases
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

### Database Operations

```bash
# List databases
redisctl database list

# Get specific database
redisctl database get 12345

# Create database with async wait
redisctl cloud database create --data @database.json --wait

# Update database with custom timeout
redisctl cloud database update 12345 --data @updates.json --wait --wait-timeout 300

# Delete database with force and wait
redisctl cloud database delete 12345 --force --wait
```

### Subscription Management

```bash
# List all subscriptions
redisctl cloud subscription list

# Create subscription and wait for completion
redisctl cloud subscription create --data @subscription.json --wait

# Update subscription with progress tracking
redisctl cloud subscription update 67890 --data @updates.json --wait --wait-interval 5

# Delete subscription
redisctl cloud subscription delete 67890 --force --wait
```

### ACL Management

```bash
# Redis ACL Rules
redisctl cloud acl create-redis-rule --name "read-only" --rule "+@read" --wait
redisctl cloud acl update-redis-rule --id 123 --rule "+@write" --wait
redisctl cloud acl delete-redis-rule --id 123 --force --wait

# ACL Roles
redisctl cloud acl create-role --name "app-role" --redis-rules "[{\"rule_id\": 123}]" --wait
redisctl cloud acl update-role --id 456 --name "updated-role" --wait
redisctl cloud acl delete-role --id 456 --force --wait

# ACL Users
redisctl cloud acl create-acl-user --name "app-user" --role "app-role" --password "secure123" --wait
redisctl cloud acl update-acl-user --id 789 --role "admin" --wait
redisctl cloud acl delete-acl-user --id 789 --force --wait
```

### Network Connectivity

```bash
# VPC Peering
redisctl cloud connectivity vpc-peering create 12345 --data @peering.json --wait
redisctl cloud connectivity vpc-peering list 12345
redisctl cloud connectivity vpc-peering delete 12345 67890 --force --wait

# Private Service Connect (GCP)
redisctl cloud connectivity psc create 12345 --data @psc.json --wait
redisctl cloud connectivity psc update 12345 67890 --data @updates.json --wait
redisctl cloud connectivity psc delete 12345 67890 --force --wait

# Transit Gateway (AWS)
redisctl cloud connectivity tgw create 12345 --data @tgw.json --wait
redisctl cloud connectivity tgw attach 12345 67890 --data @attach.json --wait
redisctl cloud connectivity tgw detach 12345 67890 --force --wait
```

### User and Account Management

```bash
# User operations
redisctl cloud user list
redisctl cloud user get 123
redisctl cloud user delete 123 --force --wait

# Provider account operations
redisctl cloud provider-account create --file @aws-account.json --wait
redisctl cloud provider-account update 456 --file @updates.json --wait
redisctl cloud provider-account delete 456 --force --wait
```

### Fixed Plans Management

```bash
# Fixed databases
redisctl cloud fixed-database create --subscription-id 12345 --data @fixed-db.json --wait
redisctl cloud fixed-database update --subscription-id 12345 --database-id 67890 --data @updates.json --wait
redisctl cloud fixed-database delete --subscription-id 12345 --database-id 67890 --force --wait

# Fixed subscriptions
redisctl cloud fixed-subscription create --data @fixed-sub.json --wait
redisctl cloud fixed-subscription update 12345 --data @updates.json --wait
redisctl cloud fixed-subscription delete 12345 --force --wait
```

### Direct API Access

```bash
# Any endpoint, any method
redisctl api cloud get /subscriptions
redisctl api cloud post /subscriptions --data @subscription.json
redisctl api enterprise get /v1/cluster
redisctl api enterprise put /v1/bdbs/1 --data @database.json

# With output formatting and filtering
redisctl api cloud get /databases -o json -q "[?status=='active']"
```

## Async Operations Support

The `--wait` flag is now supported across all creation, update, and deletion operations:

### Wait Flag Options

- `--wait` - Wait for operation to complete (default timeout: 600s)
- `--wait-timeout <seconds>` - Custom timeout duration
- `--wait-interval <seconds>` - Polling interval (default: 10s)

### Supported Operations

#### Cloud Operations with --wait Support
- ✅ Subscriptions (create, update, delete)
- ✅ Databases (create, update, delete, import, backup, migrate)
- ✅ Active-Active databases (create, update, delete)
- ✅ VPC Peering (create, update, delete)
- ✅ Private Service Connect (create, update, delete)
- ✅ Transit Gateway (create, attach, detach, delete)
- ✅ ACL Rules, Roles, and Users (create, update, delete)
- ✅ User management (delete)
- ✅ Provider accounts (create, update, delete)
- ✅ Fixed databases (create, update, delete)
- ✅ Fixed subscriptions (create, update, delete)

## Command Structure

```
redisctl
├── api              # Raw API access (any endpoint)
│   ├── cloud        # Direct Cloud API calls
│   └── enterprise   # Direct Enterprise API calls
├── cloud            # Cloud-specific commands
│   ├── subscription # Subscription management
│   ├── database     # Database operations
│   ├── acl         # ACL management (rules, roles, users)
│   ├── connectivity # Network connectivity (VPC, PSC, TGW)
│   ├── user        # User management
│   ├── provider-account # Cloud provider accounts
│   ├── fixed-database # Fixed plan databases
│   └── fixed-subscription # Fixed plan subscriptions
├── enterprise       # Enterprise-specific commands
│   ├── cluster     # Cluster management
│   ├── database    # Database operations
│   ├── node        # Node management
│   └── user        # User management
├── database        # Smart commands (work with both)
├── user           # Smart user commands
└── profile        # Manage configuration profiles
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

For comprehensive documentation, see the [User Guide](https://docs.rs/redisctl).

- **Getting Started** - Installation, configuration, first commands
- **Redis Cloud** - Cloud-specific operations and API reference
- **Redis Enterprise** - Enterprise-specific operations and API reference  
- **Examples** - Common use cases and patterns
- **API Reference** - Complete command reference

## Development

This project provides Rust client libraries for both APIs:

```toml
[dependencies]
redis-cloud = "0.2"       # Redis Cloud API client
redis-enterprise = "0.2"  # Redis Enterprise API client
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/joshrotenberg/redisctl.git
cd redisctl

# Build all components
cargo build --release

# Run tests
cargo test --workspace

# Install locally
cargo install --path crates/redisctl
```

## Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.