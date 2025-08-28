# redisctl CLI Reference

Complete command reference for the Redis unified CLI tool.

## Command Categories

### [Main Commands](README.md)
The main redisctl commands and global options.

### [Cloud Commands](cloud/README.md)
Commands for managing Redis Cloud deployments:
- [API Access](cloud/api.md)
- [Subscriptions](cloud/subscription.md)
- [Databases](cloud/database.md)
- [Users](cloud/user.md)
- [ACLs](cloud/acl.md)
- [Backups](cloud/backup.md)
- [VPC Peering](cloud/peering.md)
- [Transit Gateway](cloud/transit-gateway.md)
- [Metrics](cloud/metrics.md)
- [Logs](cloud/logs.md)
- [SSO/SAML](cloud/sso.md)
- [Billing](cloud/billing.md)
- [And more...](cloud/)

### [Enterprise Commands](enterprise/README.md)
Commands for managing Redis Enterprise deployments:
- [API Access](enterprise/api.md)
- [Clusters](enterprise/cluster.md)
- [Databases](enterprise/database.md)
- [Nodes](enterprise/node.md)
- [Users](enterprise/user.md)
- [Bootstrap](enterprise/bootstrap.md)
- [Modules](enterprise/module.md)
- [Roles](enterprise/role.md)
- [License](enterprise/license.md)
- [Alerts](enterprise/alert.md)
- [Stats](enterprise/stats.md)
- [Logs](enterprise/logs.md)
- [And more...](enterprise/)

### [Profile Management](profile/README.md)
Commands for managing configuration profiles:
- [List Profiles](profile/list.md)
- [Set Profile](profile/set.md)
- [Get Profile](profile/get.md)
- [Remove Profile](profile/remove.md)
- [Set Default](profile/default.md)

### [Authentication Management](auth/README.md)
Commands for testing and setting up authentication:
- [Test Authentication](auth/test.md)
- [Interactive Setup](auth/setup.md)

### [Configuration Management](config/README.md)
Commands for inspecting and validating configuration:
- [Show Configuration](config/show.md)
- [Configuration Path](config/path.md)
- [Validate Configuration](config/validate.md)

### [Smart-Routed Commands](smart/)
Commands that automatically detect deployment type:
- [Database Operations](smart/database.md)
- [Cluster Operations](smart/cluster.md)
- [User Operations](smart/user.md)
- [Account Operations](smart/account.md)

## Environment Variables

### Redis Cloud
- `REDIS_CLOUD_API_KEY` - API key for authentication
- `REDIS_CLOUD_API_SECRET` - API secret for authentication
- `REDIS_CLOUD_API_URL` - Custom API URL (optional)

### Redis Enterprise
- `REDIS_ENTERPRISE_URL` - Cluster API URL
- `REDIS_ENTERPRISE_USER` - Username for authentication
- `REDIS_ENTERPRISE_PASSWORD` - Password for authentication
- `REDIS_ENTERPRISE_INSECURE` - Allow insecure TLS (true/false)

### General
- `REDISCTL_PROFILE` - Default profile to use
- `RUST_LOG` - Logging level (error, warn, info, debug, trace)

## Output Formats

All commands support multiple output formats via the `-o/--output` flag:
- `json` - JSON format (default)
- `yaml` - YAML format
- `table` - Human-readable table format

## Query Filtering

Use JMESPath queries with the `-q/--query` flag to filter output:
```bash
redisctl database list -q "[?status=='active'].name"
redisctl user list -q "[].{email:email,role:role}"
```

## Examples

See the [examples directory](../../examples/) for common usage patterns.
