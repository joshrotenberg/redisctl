# Introduction

**redisctl** is a command-line tool for managing Redis Cloud and Redis Enterprise deployments.

## Features

redisctl provides:

- **Type-Safe API Clients** - Catch errors at compile time
- **Async Operation Handling** - Automatic polling with `--wait`
- **Support Package Automation** - 10+ minutes → 30 seconds
- **Profile Management** - Secure credential storage
- **Structured Output** - JSON, YAML, or Table with JMESPath
- **Library-First Architecture** - Reusable components
- **AI Integration** - Built-in [MCP server](./integrations/mcp.md) for Claude and other AI assistants

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
# Docker (quick start)
docker run ghcr.io/redis-developer/redisctl:latest --help

# macOS/Linux
brew install redis-developer/homebrew-tap/redisctl

# Or download from GitHub releases
```

See [Installation](./getting-started/installation.md) for all methods, or try the [Quick Start](./getting-started/quickstart.md) with Docker.

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

- [GitHub Issues](https://github.com/redis-developer/redisctl/issues)
- [API Documentation](https://docs.rs/redisctl)
- [Cookbook](./cookbook/README.md)
