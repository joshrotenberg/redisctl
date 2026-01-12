# Tools Reference

The redisctl MCP server exposes **65 tools** for managing Redis Cloud and Redis Enterprise deployments.

Tools marked with *(write)* require `--allow-writes` flag.

## Redis Cloud Tools (17 tools)

### Account & Infrastructure

| Tool | Description |
|------|-------------|
| `cloud_account_get` | Get account information |
| `cloud_payment_methods_get` | List all payment methods configured for your account |
| `cloud_database_modules_get` | List all available database modules (capabilities) |
| `cloud_regions_get` | Get available regions across cloud providers (AWS, GCP, Azure) |

### Pro Subscriptions

| Tool | Description |
|------|-------------|
| `cloud_subscriptions_list` | List all Pro subscriptions |
| `cloud_subscription_get` | Get Pro subscription details |
| `cloud_pro_subscription_create` | Create a new Pro subscription *(write)* |
| `cloud_pro_subscription_delete` | Delete a Pro subscription *(write)* |

### Essentials Subscriptions

| Tool | Description |
|------|-------------|
| `cloud_essentials_subscriptions_list` | List all Essentials subscriptions |
| `cloud_essentials_subscription_get` | Get Essentials subscription details |
| `cloud_essentials_subscription_create` | Create a new Essentials subscription *(write)* |
| `cloud_essentials_subscription_delete` | Delete an Essentials subscription *(write)* |
| `cloud_essentials_plans_list` | List available Essentials plans with pricing |

### Database & Task Operations

| Tool | Description |
|------|-------------|
| `cloud_databases_list` | List databases in a subscription |
| `cloud_database_get` | Get database details |
| `cloud_tasks_list` | List recent async tasks |
| `cloud_task_get` | Get task status |

## Redis Enterprise Tools (48 tools)

### Cluster Operations

| Tool | Description |
|------|-------------|
| `enterprise_cluster_get` | Get cluster information |
| `enterprise_cluster_stats` | Get cluster statistics |
| `enterprise_cluster_settings` | Get cluster settings |
| `enterprise_cluster_topology` | Get cluster topology |
| `enterprise_cluster_update` | Update cluster configuration *(write)* |

### Database Operations

| Tool | Description |
|------|-------------|
| `enterprise_databases_list` | List all databases |
| `enterprise_database_get` | Get database details |
| `enterprise_database_stats` | Get database statistics |
| `enterprise_database_metrics` | Get database performance metrics |
| `enterprise_database_create` | Create a new database *(write)* |
| `enterprise_database_update` | Update database configuration *(write)* |
| `enterprise_database_delete` | Delete a database *(write)* |
| `enterprise_database_flush` | Flush all data from database *(write)* |
| `enterprise_database_export` | Export database to external location *(write)* |
| `enterprise_database_import` | Import data into database *(write)* |
| `enterprise_database_backup` | Trigger database backup *(write)* |
| `enterprise_database_restore` | Restore database from backup *(write)* |

### Node Operations

| Tool | Description |
|------|-------------|
| `enterprise_nodes_list` | List all cluster nodes |
| `enterprise_node_get` | Get node details |
| `enterprise_node_stats` | Get node statistics |
| `enterprise_node_update` | Update node configuration *(write)* |
| `enterprise_node_remove` | Remove node from cluster *(write)* |

### Shard Operations

| Tool | Description |
|------|-------------|
| `enterprise_shards_list` | List all shards |
| `enterprise_shard_get` | Get shard details |

### Alert Operations

| Tool | Description |
|------|-------------|
| `enterprise_alerts_list` | List active alerts |
| `enterprise_alert_get` | Get alert details |

### User Management

| Tool | Description |
|------|-------------|
| `enterprise_users_list` | List all users |
| `enterprise_user_get` | Get user details |
| `enterprise_user_create` | Create a new user *(write)* |
| `enterprise_user_delete` | Delete a user *(write)* |

### Role Management

| Tool | Description |
|------|-------------|
| `enterprise_roles_list` | List all roles |
| `enterprise_role_get` | Get role details |
| `enterprise_role_create` | Create a new role *(write)* |
| `enterprise_role_delete` | Delete a role *(write)* |

### ACL Management

| Tool | Description |
|------|-------------|
| `enterprise_acls_list` | List all Redis ACLs |
| `enterprise_acl_get` | Get ACL details |
| `enterprise_acl_create` | Create a new Redis ACL *(write)* |
| `enterprise_acl_delete` | Delete a Redis ACL *(write)* |

### License & Modules

| Tool | Description |
|------|-------------|
| `enterprise_license_get` | Get license information |
| `enterprise_modules_list` | List available modules |
| `enterprise_module_get` | Get module details |

### Active-Active (CRDB)

| Tool | Description |
|------|-------------|
| `enterprise_crdbs_list` | List Active-Active databases |
| `enterprise_crdb_get` | Get Active-Active database details |
| `enterprise_crdb_update` | Update Active-Active database *(write)* |
| `enterprise_crdb_delete` | Delete Active-Active database *(write)* |

### Logs & Debug

| Tool | Description |
|------|-------------|
| `enterprise_logs_get` | Get cluster event logs |
| `enterprise_debuginfo_list` | List debug info tasks |
| `enterprise_debuginfo_status` | Get debug info task status |

## Tool Categories Summary

| Category | Read | Write | Total |
|----------|------|-------|-------|
| Cloud Account | 4 | 0 | 4 |
| Cloud Subscriptions | 5 | 4 | 9 |
| Cloud Databases | 4 | 0 | 4 |
| Enterprise Cluster | 4 | 1 | 5 |
| Enterprise Databases | 4 | 8 | 12 |
| Enterprise Nodes | 3 | 2 | 5 |
| Enterprise Shards | 2 | 0 | 2 |
| Enterprise Alerts | 2 | 0 | 2 |
| Enterprise Users | 2 | 2 | 4 |
| Enterprise Roles | 2 | 2 | 4 |
| Enterprise ACLs | 2 | 2 | 4 |
| Enterprise License/Modules | 3 | 0 | 3 |
| Enterprise Active-Active | 2 | 2 | 4 |
| Enterprise Logs/Debug | 3 | 0 | 3 |
| **Total** | **42** | **23** | **65** |

## Example Tool Usage

### Listing Databases

> **You**: What databases do I have?
>
> **AI**: *uses `enterprise_databases_list`*
>
> You have 2 databases:
> - `default-db` (uid: 1) - 1GB, active
> - `cache-db` (uid: 2) - 256MB, active

### Creating a Database

> **You**: Create a new database called session-store with 512MB
>
> **AI**: *uses `enterprise_database_create` with name="session-store", memory_size_mb=512*
>
> Created database `session-store` (uid: 3) with 512MB memory. Status: active.

### Checking License

> **You**: When does my license expire?
>
> **AI**: *uses `enterprise_license_get`*
>
> Your Trial license expires on February 11, 2026 (30 days remaining).
> - Shards in use: 1 of 4 (25%)
> - Features: bigstore enabled

### Viewing Cluster Health

> **You**: Show me cluster health
>
> **AI**: *uses `enterprise_cluster_get`, `enterprise_cluster_stats`, `enterprise_nodes_list`*
>
> **Cluster**: docker-cluster (v8.0.2-17)
>
> | Metric | Value |
> |--------|-------|
> | CPU Utilization | 4% |
> | Memory Available | 7.81 GB |
> | Nodes | 1 active |
> | Databases | 1 active |
> | Alerts | None |
