# redisctl

A unified command-line interface for managing Redis deployments across Cloud and Enterprise.

## Overview

`redisctl` is a single CLI that can manage both Redis Cloud and Redis Enterprise deployments, automatically detecting which API to use based on your configuration profile or explicit command selection.

## Features

- **Unified Interface** - One CLI for both Redis Cloud and Enterprise
- **Smart Detection** - Automatically routes commands based on deployment type
- **Profile Management** - Manage multiple Redis deployments with saved profiles
- **Multiple Output Formats** - JSON, YAML, and Table output
- **JMESPath Queries** - Powerful filtering and transformation
- **Workflow Commands** - High-level operations for common tasks
- **Raw API Access** - Direct access to both Cloud and Enterprise APIs

## Installation

```bash
# Build from source
git clone https://github.com/redis-field-engineering/redisctl.git
cd redisctl
cargo build --release

# The binary will be at target/release/redisctl
./target/release/redisctl --help
```

## Quick Start

### Configure Profiles

```bash
# Redis Cloud profile
redisctl config set --profile prod-cloud --type cloud \
  --api-key YOUR_KEY --api-secret YOUR_SECRET

# Redis Enterprise profile  
redisctl config set --profile prod-enterprise --type enterprise \
  --url https://cluster:9443 --username admin --password secret

# Set default profile
export REDISCTL_PROFILE=prod-cloud
```

### Usage Examples

```bash
# Explicit deployment selection
redisctl cloud subscription list
redisctl enterprise cluster info

# Auto-detection based on command (when unambiguous)
redisctl subscription list  # Cloud-only command -> uses Cloud API
redisctl cluster info       # Enterprise-only command -> uses Enterprise API

# Profile-based routing
redisctl --profile prod-cloud database list    # Uses Cloud API
redisctl --profile prod-enterprise database list # Uses Enterprise API

# Error for ambiguous commands without explicit selection
redisctl database list  # Error: specify 'cloud' or 'enterprise'
```

## Architecture

This project consists of four main crates:

- **redis-cloud** - Redis Cloud API client library
- **redis-enterprise** - Redis Enterprise API client library  
- **redis-common** - Shared utilities (output, config, errors)
- **redisctl** - Unified CLI that orchestrates the above

## Development

See individual crate documentation:
- [Redis Cloud Library](./crates/redis-cloud/README.md)
- [Redis Enterprise Library](./crates/redis-enterprise/README.md)
- [CLI Documentation](./crates/redisctl/README.md)

## License

MIT OR Apache-2.0