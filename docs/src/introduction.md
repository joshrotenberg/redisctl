# Introduction

**redisctl** is the **first** command-line tool for managing Redis Cloud and Redis Enterprise deployments. Before redisctl, operators had to use web UIs or write fragile bash scripts with curl and polling loops.

```admonish success title="The First CLI Tool"
redisctl eliminates the need for fragile bash + curl + jq scripts by providing type-safe API clients, automatic async operation handling, and a library-first architecture that enables the entire Redis Rust ecosystem.
```

## The Problem

Before redisctl existed, managing Redis deployments meant:

- 🖱️ **Manual UI clicking** - No scriptability or automation
- 📝 **Writing bash scripts** - Complex curl commands with jq parsing
- ⏰ **Manual polling** - Write your own loops to wait for operations
- 🔐 **Credential exposure** - API keys passed on command line
- 🔁 **Reinventing the wheel** - Every operator builds the same fragile scripts

```admonish example title="Before redisctl: The Reality"
Everyone wrote scripts like this:

\`\`\`bash
# The "best practice" before redisctl
curl -X POST https://api.redislabs.com/v1/subscriptions \\
  -H "x-api-key: $KEY" \\
  -H "x-api-secret-key: $SECRET" \\
  -d '{"name":"prod","cloudProviders":[...]}'

# Then poll for completion
while true; do
  STATUS=$(curl -s https://api.redislabs.com/v1/tasks/$TASK_ID | jq -r '.status')
  if [ "$STATUS" = "completed" ]; then break; fi
  sleep 2
done
\`\`\`

**Problems:** No type safety, fragile parsing, manual error handling, credential exposure, not portable.
```

## The Solution

redisctl provides a first-class CLI experience:

### ✨ Key Features

- **Type-Safe API Clients** - Catch errors at compile time, not runtime
- **Async Operation Handling** - Automatic polling with `--wait` flag
- **Support Package Automation** - 10+ minutes → 30 seconds
- **Profile Management** - Secure credential storage with OS keyring
- **Structured Output** - JSON, YAML, or Table with JMESPath filtering
- **Library-First Architecture** - Reusable components for the ecosystem

### 🎯 Three Command Layers

```admonish info title="Flexible Interface"
The CLI provides three layers of interaction to match your needs:

1. **Raw API Access** - Direct REST calls to any endpoint
2. **Human-Friendly Commands** - Type-safe wrappers around common operations
3. **Workflows** - Multi-step orchestrated operations

Choose the layer that fits your use case!
```

## Quick Example

```admonish example title="After redisctl: Elegant and Simple"
\`\`\`bash
# Configure once (secure keyring storage)
redisctl profile set prod --api-key $KEY --api-secret $SECRET --use-keyring

# Create subscription and wait for completion
redisctl cloud subscription create \\
  --name prod \\
  --cloud-provider AWS \\
  --region us-east-1 \\
  --wait

# Everything just works
redisctl cloud database create --subscription $SUB --name mydb --wait
redisctl enterprise support-package cluster --upload
\`\`\`

**One command replaces 50 lines of bash.**
```

## Installation

Get started quickly:

```bash
# macOS/Linux
brew install joshrotenberg/brew/redisctl

# Or download from GitHub releases
# Or use Docker
docker run ghcr.io/joshrotenberg/redisctl:latest --help
```

See [Installation](./getting-started/installation.md) for all methods.

## Quick Start Paths

Choose your path based on what you're managing:

```admonish tip title="Redis Cloud Users"
👉 Start with [Cloud Quick Start](./getting-started/quickstart.md#redis-cloud)

Learn how to manage subscriptions, databases, and networking through the CLI.
```

```admonish tip title="Redis Enterprise Users"
👉 Start with [Enterprise Quick Start](./getting-started/quickstart.md#redis-enterprise)

Learn how to manage clusters, databases, and generate support packages.
```

```admonish tip title="Developers & Integrators"
👉 Check out [Library Documentation](./developer/library-usage.md)

Learn how to use redisctl libraries in your own Rust applications.
```

## What Makes redisctl Different?

| Before redisctl | With redisctl |
|----------------|---------------|
| curl + jq + while loops | Single command with `--wait` |
| Manual JSON construction | Type-safe structs |
| Credentials in bash history | OS keyring integration |
| Copy-paste from web UI | Scriptable automation |
| Platform-specific scripts | Cross-platform binary |
| Everyone reinvents | Shared library ecosystem |

## Architecture

redisctl is built as **reusable libraries**:

- `redisctl-config` - Profile and credential management
- `redis-cloud` - Cloud API client (21 handlers, 95%+ test coverage)
- `redis-enterprise` - Enterprise API client (29 handlers, 100% test coverage)
- `redisctl` - CLI binary (thin orchestration layer)

This enables other tools to reuse battle-tested components for Terraform providers, backup tools, monitoring dashboards, and more.

```admonish info title="Library-First Design"
redisctl isn't just a CLI - it's a **platform** for building Redis automation tools in Rust.
```

## Next Steps

Ready to get started?

1. **[Install redisctl](./getting-started/installation.md)** - Get the CLI installed
2. **[Configure profiles](./getting-started/configuration.md)** - Set up your credentials
3. **[Try the quickstart](./getting-started/quickstart.md)** - Run your first commands
4. **[Explore the cookbook](./cookbook/README.md)** - Copy-paste ready recipes

```admonish question title="Need Help?"
- [GitHub Issues](https://github.com/joshrotenberg/redisctl/issues) - Report bugs or request features
- [API Documentation](https://docs.rs/redisctl) - Library API reference
- [Cookbook](./cookbook/README.md) - Practical examples
```
