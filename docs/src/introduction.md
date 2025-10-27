# Introduction

**redisctl** is the first command-line tool for managing Redis Cloud and Redis Enterprise deployments.

## The Problem

Before redisctl existed, managing Redis deployments meant:

- Manual UI clicking with no scriptability
- Writing fragile bash scripts with curl and jq
- Manual polling loops to wait for operations
- Credential exposure on command lines
- Every operator reinventing the same scripts

## The Solution

redisctl provides a first-class CLI experience with:

- **Type-Safe API Clients** - Catch errors at compile time
- **Async Operation Handling** - Automatic polling with `--wait`
- **Support Package Automation** - 10+ minutes → 30 seconds
- **Profile Management** - Secure credential storage
- **Structured Output** - JSON, YAML, or Table with JMESPath
- **Library-First Architecture** - Reusable components

## Quick Example

```bash
# Configure once
redisctl profile set prod --api-key $KEY --api-secret $SECRET

# Create subscription and wait
redisctl cloud subscription create \
  --name prod \
  --cloud-provider AWS \
  --region us-east-1 \
  --wait

# Everything just works
redisctl cloud database create --subscription $SUB --name mydb --wait
```

## Installation

```bash
# macOS/Linux
brew install joshrotenberg/brew/redisctl

# Or download from GitHub releases
# Or use Docker
docker run ghcr.io/joshrotenberg/redisctl:latest --help
```

See [Installation](./getting-started/installation.md) for all methods.

## Quick Start Paths

- **Redis Cloud Users**: Start with [Cloud Quick Start](./getting-started/quickstart.md#redis-cloud-quick-start)
- **Redis Enterprise Users**: Start with [Enterprise Quick Start](./getting-started/quickstart.md#redis-enterprise-quick-start)  
- **Developers**: Check out [Library Documentation](./developer/library-usage.md)

## Architecture

redisctl is built as reusable libraries:

- `redisctl-config` - Profile and credential management
- `redis-cloud` - Cloud API client (21 handlers, 95%+ coverage)
- `redis-enterprise` - Enterprise API client (29 handlers, 100% coverage)
- `redisctl` - CLI binary (thin orchestration layer)

This enables Terraform providers, backup tools, monitoring dashboards, and more.

## Next Steps

1. [Install redisctl](./getting-started/installation.md)
2. [Configure profiles](./getting-started/configuration.md)
3. [Try the quickstart](./getting-started/quickstart.md)
4. [Explore the cookbook](./cookbook/README.md)

## Need Help?

- [GitHub Issues](https://github.com/joshrotenberg/redisctl/issues)
- [API Documentation](https://docs.rs/redisctl)
- [Cookbook](./cookbook/README.md)
