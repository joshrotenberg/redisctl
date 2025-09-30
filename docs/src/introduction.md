# Introduction

`redisctl` is a unified command-line tool for managing Redis Cloud and Redis Enterprise deployments through their REST APIs.

## Why redisctl?

- **Single Tool** - One CLI for both Cloud and Enterprise deployments
- **Explicit Commands** - Clear separation between Cloud and Enterprise operations
- **Multiple Interfaces** - Raw API access, human-friendly commands, and orchestrated workflows
- **Flexible Output** - JSON, YAML, or formatted tables with JMESPath filtering

## Command Layers

The CLI provides three layers of interaction:

1. **Raw API Access** - Direct REST calls to any endpoint
2. **Human-Friendly Commands** - Typed wrappers around common operations  
3. **Workflows** - Multi-step orchestrated operations (coming soon)

## Installation

Get started quickly with Homebrew:

```bash
brew install joshrotenberg/brew/redisctl
```

Or see [Installation](./getting-started/installation.md) for other methods.

## Quick Example

```bash
# Configure your profile
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_API_SECRET="your-secret"

# List all databases
redisctl cloud database list

# Get specific database details
redisctl cloud database get 12345

# Direct API call
redisctl api cloud get /subscriptions
```

## Next Steps

- [Installation](./getting-started/installation.md) - Get redisctl installed
- [Configuration](./getting-started/configuration.md) - Set up your profiles
- [Quick Start](./getting-started/quickstart.md) - Your first commands