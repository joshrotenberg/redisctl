# Redis Cloud Commands

Redis Cloud commands are organized into three layers:

## 1. Human-Friendly Commands

High-level commands with typed parameters and structured output.

```bash
redisctl cloud <resource> <action> [options]
```

See [Human-Friendly Commands](./human-commands.md) for the complete reference.

## 2. Raw API Access

Direct access to any REST endpoint when you need full control.

```bash
redisctl api cloud <method> <path> [options]
```

See [Raw API Access](./api-access.md) for details.

## 3. Workflows (Coming Soon)

Multi-step orchestrated operations for complex tasks:
- Database migration workflows
- Backup and restore procedures
- Cluster setup automation

## Quick Reference

### Most Common Commands

```bash
# Subscriptions
redisctl cloud subscription list
redisctl cloud subscription get <id>

# Databases
redisctl cloud database list --subscription-id <id>
redisctl cloud database get --subscription-id <id> --database-id <id>

# Direct API
redisctl api cloud get /subscriptions
redisctl api cloud post /subscriptions/<id>/databases --data @database.json
```