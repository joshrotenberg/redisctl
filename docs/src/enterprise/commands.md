# Redis Enterprise Commands

Redis Enterprise commands are organized into three layers:

## 1. Human-Friendly Commands

High-level commands with typed parameters and structured output.

```bash
redisctl enterprise <resource> <action> [options]
```

See [Human-Friendly Commands](./human-commands.md) for the complete reference.

## 2. Raw API Access

Direct access to any REST endpoint when you need full control.

```bash
redisctl api enterprise <method> <path> [options]
```

See [Raw API Access](./api-access.md) for details.

## 3. Workflows (Coming Soon)

Multi-step orchestrated operations for complex tasks:
- Cluster bootstrap and setup
- Node addition and removal
- Database migration workflows
- Upgrade procedures

## Quick Reference

### Most Common Commands

```bash
# Cluster
redisctl enterprise cluster info
redisctl enterprise cluster update --name "Production"

# Databases
redisctl enterprise database list
redisctl enterprise database get 1
redisctl enterprise database create --name "cache"

# Nodes
redisctl enterprise node list
redisctl enterprise node get 1

# Direct API
redisctl api enterprise get /v1/cluster
redisctl api enterprise post /v1/bdbs --data @database.json
```