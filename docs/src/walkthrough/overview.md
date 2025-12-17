# Overview & Concepts

## What is redisctl?

redisctl is a command-line tool for managing Redis Cloud and Redis Enterprise deployments. It provides type-safe API clients, async operation handling, and a library-first architecture.

## The Problem

Before redisctl, managing Redis deployments meant:

- **Manual UI clicking** - No way to script operations
- **Fragile bash scripts** - curl with hardcoded endpoints and manual JSON parsing
- **Polling loops** - Writing custom logic to wait for async operations
- **Credential exposure** - Passwords on command lines or in plaintext
- **Reinventing the wheel** - Every team writing the same scripts

## The Three-Tier Model

redisctl provides three levels of interaction:

### 1. API Layer

Direct REST access for scripting and automation. Think of it as a smart curl replacement.

```bash
# Any endpoint, any method
redisctl api cloud get /subscriptions
redisctl api enterprise post /v1/bdbs -d @database.json
```

**Use when:**
- Building automation scripts
- Accessing endpoints not yet wrapped in commands
- You need exact control over requests

### 2. Human Commands

Type-safe, ergonomic commands with named parameters and built-in help.

```bash
redisctl cloud database create \
  --subscription 123456 \
  --data @database.json \
  --wait

redisctl enterprise database list -o table
```

**Use when:**
- Day-to-day operations
- Interactive use
- You want `--help` and validation

### 3. Workflows

Multi-step operations that handle sequencing, polling, and error recovery.

```bash
redisctl cloud workflow subscription-setup \
  --name production \
  --region us-east-1

redisctl enterprise workflow init-cluster \
  --cluster-name prod \
  --username admin@cluster.local
```

**Use when:**
- Setting up new resources
- Operations that require multiple API calls
- You want automatic waiting and error handling

## Common Features

All commands share:

- **Output formats** - JSON (default), YAML, or table
- **JMESPath queries** - Filter and transform output with `-q`
- **Async handling** - `--wait` flag for operations that return task IDs
- **Profile support** - Multiple credential sets for different environments

## Getting Help

```bash
# General help
redisctl --help

# Command help
redisctl cloud --help
redisctl cloud database --help
redisctl cloud database create --help

# List all commands
redisctl cloud database list --help
```

## Next Steps

- [Cloud Quick Examples](./cloud-examples.md) - See the three tiers in action
- [Enterprise Quick Examples](./enterprise-examples.md) - Enterprise-specific examples
