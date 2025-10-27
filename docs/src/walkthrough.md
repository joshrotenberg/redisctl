# Complete Walkthrough & Presentation

This page provides a comprehensive walkthrough of redisctl, suitable for:
- **Team presentations** - Follow as a speaker script
- **Self-guided learning** - Work through at your own pace  
- **Onboarding** - Get new team members up to speed

## Presentation Materials

The complete presentation outline and supporting materials are available in the repository:

📁 **[examples/presentation/](https://github.com/joshrotenberg/redisctl/tree/main/examples/presentation)**

### Files

- **[PRESENTATION_OUTLINE.md](https://github.com/joshrotenberg/redisctl/blob/main/examples/presentation/PRESENTATION_OUTLINE.md)** - Complete 20-25 minute presentation
  - Slide-by-slide walkthrough
  - Speaking notes and timing
  - Code examples
  - Q&A preparation

- **[RLADMIN_COMPARISON.md](https://github.com/joshrotenberg/redisctl/blob/main/examples/presentation/RLADMIN_COMPARISON.md)** - Feature matrix comparing rladmin vs redisctl

- **Demo Scripts**:
  - `01-before-redisctl.sh` - The painful reality (curl + jq + polling)
  - `02-after-redisctl.sh` - The elegant solution
  - `03-demo-workflow.sh` - Complete feature showcase

## Quick Overview

### 1. The Problem (Why redisctl exists)

**Redis Cloud:**
- Web UI only (not scriptable)
- Terraform provider (good for IaC, not ad-hoc ops)
- No CLI for day-to-day operations

**Redis Enterprise:**
- **rladmin** - Node-local CLI, requires SSH, text output
- REST API - Large, complex, poorly documented
- No remote CLI, no automation-friendly tools

**Result:** Everyone writes fragile bash + curl + jq scripts

### 2. Enter redisctl

**The FIRST command-line tool for both Redis Cloud and Redis Enterprise**

Key features:
- Type-safe API clients
- Automatic async operation handling (`--wait`)
- Support package automation (10 min → 30 sec)
- Profile management with secure keyring
- Structured output (JSON, YAML, Table)
- Library-first architecture

### 3. Four-Layer Architecture

redisctl provides four layers of interaction:

**Layer 1: Raw API Access**
```bash
# Direct REST calls to any endpoint
redisctl api cloud get /subscriptions
redisctl api enterprise get /v1/cluster
```

**Layer 2: Human-Friendly Commands**
```bash
# Better UX with type-safe parameters
redisctl cloud subscription list -o table
redisctl enterprise database list
```

**Layer 3: Workflows**
```bash
# Multi-step orchestrated operations
redisctl enterprise workflow init-cluster \
  --cluster-name "production" \
  --username "admin@cluster.local"
```

**Layer 4: Specialized Tools**
```bash
# Support package automation
redisctl enterprise support-package cluster \
  --optimize \
  --upload
```

### 4. Advanced Features

- **JMESPath queries** - Filter and transform output
- **Log streaming** - Real-time with `--follow`
- **Multiple output formats** - JSON, YAML, Table
- **Profile system** - Manage multiple clusters
- **License management** - For Enterprise clusters

### 5. Library Architecture

redisctl is built as reusable libraries:

```
├── redisctl-config      # Profile/credential management
├── redis-cloud          # Cloud API client (21 handlers)
├── redis-enterprise     # Enterprise API client (29 handlers)
└── redisctl             # CLI binary
```

This enables:
- Terraform providers
- Custom automation tools
- Monitoring integrations
- Backup/migration utilities

## Hands-On Tutorial

### Prerequisites

For hands-on practice with Enterprise features:

```bash
# Start local Redis Enterprise cluster
docker compose up -d

# Verify it's running
docker compose ps
```

### Try the Demo Scripts

```bash
# Clone the repository
git clone https://github.com/joshrotenberg/redisctl
cd redisctl/examples/presentation

# Run the demos
./01-before-redisctl.sh   # See the problem
./02-after-redisctl.sh    # See the solution
./03-demo-workflow.sh     # Complete showcase
```

## Deep Dive Sections

For detailed information on specific topics:

- **Installation** - See [Installation Guide](./getting-started/installation.md)
- **Profile Setup** - See [Configuration Guide](./getting-started/configuration.md)
- **Cloud Operations** - See [Cloud Cookbook](./cookbook/README.md#redis-cloud-recipes)
- **Enterprise Operations** - See [Enterprise Cookbook](./cookbook/README.md#redis-enterprise-recipes)
- **API Reference** - See [Cloud Commands](./cloud/commands.md) and [Enterprise Commands](./enterprise/README.md)

## Presentation Tips

If you're using this as a presentation script:

1. **Start with the problem** - Show `01-before-redisctl.sh` first
2. **Emphasize "FIRST CLI"** - Neither Cloud nor Enterprise had one
3. **Demo the four layers** - Show progression from raw to workflows
4. **Highlight killer feature** - Support package automation
5. **End with library vision** - Platform, not just a CLI

**Timing:** 20-25 minutes + Q&A

## Questions & Feedback

- **Issues**: [GitHub Issues](https://github.com/joshrotenberg/redisctl/issues)
- **Discussions**: Start a discussion in the repository
- **API Docs**: [docs.rs/redisctl](https://docs.rs/redisctl)

## What's Next?

After this walkthrough, explore:

- **[Cookbook](./cookbook/README.md)** - Task-oriented recipes
- **[Cloud Reference](./cloud/overview.md)** - Complete Cloud API coverage
- **[Enterprise Reference](./enterprise/overview.md)** - Complete Enterprise API coverage
- **[Developer Guide](./developer/library-usage.md)** - Build with redisctl libraries
