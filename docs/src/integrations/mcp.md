# MCP Server (AI Integration)

redisctl includes a built-in [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server that enables AI assistants like Claude to manage your Redis deployments through natural language.

## Overview

The MCP server exposes redisctl functionality as tools that AI systems can discover and invoke. This allows you to:

- Ask an AI to "list all my Redis databases"
- Request "create a new 256MB database called cache-db"
- Query "what's the status of my cluster nodes"

All operations use your existing redisctl profiles for authentication.

## Quick Start

```bash
# Start the MCP server (read-only mode, safe for exploration)
redisctl -p my-profile mcp serve

# Enable write operations (create, update, delete)
redisctl -p my-profile mcp serve --allow-writes

# List available tools
redisctl mcp tools
```

## Configuring Claude Desktop

Add the following to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "redisctl": {
      "command": "/path/to/redisctl",
      "args": ["-p", "my-profile", "mcp", "serve"]
    }
  }
}
```

For write operations:
```json
{
  "mcpServers": {
    "redisctl": {
      "command": "/path/to/redisctl",
      "args": ["-p", "my-profile", "mcp", "serve", "--allow-writes"]
    }
  }
}
```

## Configuring Claude Code

Add to your project's `.mcp.json` or global MCP settings:

```json
{
  "mcpServers": {
    "redisctl": {
      "command": "redisctl",
      "args": ["-p", "my-profile", "mcp", "serve", "--allow-writes"]
    }
  }
}
```

## Available Tools

### Redis Cloud Tools

| Tool | Description |
|------|-------------|
| `cloud_account_get` | Get account information |
| `cloud_subscriptions_list` | List all subscriptions |
| `cloud_subscription_get` | Get subscription details |
| `cloud_databases_list` | List databases in a subscription |
| `cloud_database_get` | Get database details |
| `cloud_tasks_list` | List recent async tasks |
| `cloud_task_get` | Get task status |

### Redis Enterprise Tools

| Tool | Description |
|------|-------------|
| `enterprise_cluster_get` | Get cluster information |
| `enterprise_nodes_list` | List all cluster nodes |
| `enterprise_node_get` | Get node details |
| `enterprise_databases_list` | List all databases |
| `enterprise_database_get` | Get database details |
| `enterprise_database_stats` | Get database statistics |
| `enterprise_database_create` | Create a new database *(write)* |
| `enterprise_shards_list` | List all shards |
| `enterprise_alerts_list` | List active alerts |
| `enterprise_logs_get` | Get cluster event logs |
| `enterprise_license_get` | Get license information |

## Example Conversations

Once configured, you can interact naturally with your Redis infrastructure:

> **You**: What databases do I have in my enterprise cluster?
>
> **Claude**: *uses enterprise_databases_list*
> You have 2 databases:
> - `default-db` (uid: 1) - 1GB, active
> - `cache-db` (uid: 2) - 256MB, active

> **You**: Create a new database called session-store with 512MB
>
> **Claude**: *uses enterprise_database_create*
> Created database `session-store` (uid: 3) with 512MB memory. Status: active.

> **You**: Show me any active alerts
>
> **Claude**: *uses enterprise_alerts_list*
> No active alerts in your cluster.

## Security Considerations

### Read-Only Mode (Default)

By default, the MCP server runs in read-only mode. This prevents any destructive operations and is recommended for:

- Exploring your infrastructure
- Monitoring and reporting
- Learning about your deployments

### Write Mode

Use `--allow-writes` only when you need to create or modify resources. Consider:

- Using separate profiles for read-only vs write access
- Running write-enabled servers only in development environments
- Reviewing AI-suggested changes before confirming

### Profile-Based Authentication

The MCP server uses your existing redisctl profiles, which means:

- Credentials are never exposed to the AI
- You control which environments are accessible
- Standard profile security applies (keyring support, etc.)

## Troubleshooting

### Server won't start

```bash
# Check your profile works
redisctl -p my-profile enterprise cluster get

# Verify MCP feature is enabled
redisctl mcp tools
```

### Claude can't find the server

1. Ensure the path to redisctl is absolute in your config
2. Restart Claude Desktop after config changes
3. Check Claude's MCP logs for connection errors

### Operations timing out

The MCP server inherits redisctl's timeout settings. For slow operations:

```bash
# Enterprise operations may need longer timeouts
redisctl -p my-profile mcp serve --allow-writes
```

## Protocol Details

The MCP server uses:

- **Transport**: stdio (standard input/output)
- **Protocol Version**: 2024-11-05
- **Capabilities**: Tools only (no resources or prompts currently)

For MCP protocol details, see the [MCP Specification](https://spec.modelcontextprotocol.io/).
