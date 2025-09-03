# Summary

[Introduction](./introduction.md)

# Getting Started
- [Installation](./getting-started/installation.md)
- [Docker Environment](./getting-started/docker.md)
- [Authentication](./getting-started/authentication.md)
- [Quick Start](./getting-started/quickstart.md)

# CLI Usage
- [Installation](./cli/installation.md)
- [Docker Testing](./cli/docker.md)
- [Configuration](./cli/configuration.md)
- [Commands](./cli/commands.md)
- [Output Formats](./cli/output-formats.md)
- [Examples](./cli/examples.md)
- [Detailed Examples](./cli/examples-detailed.md)

# CLI Command Reference

- [Overview](./cli-reference/index.md)
- [Profile Commands](./cli-reference/profile/README.md)
  - [list](./cli-reference/profile/list.md)
  - [get](./cli-reference/profile/get.md)
  - [set](./cli-reference/profile/set.md)
  - [remove](./cli-reference/profile/remove.md)
  - [default](./cli-reference/profile/default.md)
  
- [Smart Commands]()
  - [database](./cli-reference/smart/database.md)
  - [user](./cli-reference/smart/user.md)
  - [cluster](./cli-reference/smart/cluster.md)
  - [account](./cli-reference/smart/account.md)

- [Cloud Commands](./cli-reference/cloud/README.md)
  - [api](./cli-reference/cloud/api.md)
  - [subscription](./cli-reference/cloud/subscription.md)
  - [database](./cli-reference/cloud/database.md)
  - [user](./cli-reference/cloud/user.md)
  - [account](./cli-reference/cloud/account.md)
  - [backup](./cli-reference/cloud/backup.md)
  - [acl](./cli-reference/cloud/acl.md)
  - [api-key](./cli-reference/cloud/api-key.md)
  - [billing](./cli-reference/cloud/billing.md)
  - [cloud-account](./cli-reference/cloud/cloud-account.md)
  - [crdb](./cli-reference/cloud/crdb.md)
  - [logs](./cli-reference/cloud/logs.md)
  - [metrics](./cli-reference/cloud/metrics.md)
  - [peering](./cli-reference/cloud/peering.md)
  - [region](./cli-reference/cloud/region.md)
  - [sso](./cli-reference/cloud/sso.md)
  - [task](./cli-reference/cloud/task.md)
  - [transit-gateway](./cli-reference/cloud/transit-gateway.md)
  
- [Enterprise Commands](./cli-reference/enterprise/README.md)
  - [api](./cli-reference/enterprise/api.md)
  - [bootstrap](./cli-reference/enterprise/bootstrap.md)
  - [cluster](./cli-reference/enterprise/cluster.md)
  - [database](./cli-reference/enterprise/database.md)
  - [user](./cli-reference/enterprise/user.md)
  - [role](./cli-reference/enterprise/role.md)
  - [node](./cli-reference/enterprise/node.md)
  - [module](./cli-reference/enterprise/module.md)
  - [actions](./cli-reference/enterprise/actions.md)
  - [alert](./cli-reference/enterprise/alert.md)
  - [crdb](./cli-reference/enterprise/crdb.md)
  - [license](./cli-reference/enterprise/license.md)
  - [logs](./cli-reference/enterprise/logs.md)
  - [stats](./cli-reference/enterprise/stats.md)

# Libraries
- [Overview](./libraries/README.md)
  - [Redis Cloud Library](./libraries/redis-cloud.md)
  - [Redis Enterprise Library](./libraries/redis-enterprise.md)

# Workflows
- [Cluster Bootstrap](./workflows/cluster-bootstrap.md)
- [Database Creation](./workflows/database-creation.md)
- [High Availability Setup](./workflows/ha-setup.md)
- [Module Configuration](./workflows/module-config.md)
- [Backup & Restore](./workflows/backup-restore.md)

# API Client Reference

- [Implemented Endpoints Overview](./api/endpoints-overview.md)
- [Core Endpoints]()
  - [Cluster](./api/cluster.md)
  - [Databases (BDB)](./api/databases.md)
  - [Nodes](./api/nodes.md)
  
- [User Management](./api/user-management.md)
  - [Users](./api/users.md)
  - [Roles](./api/roles.md)
  - [LDAP Mappings](./api/ldap.md)
  
- [Security](./api/security.md)
  - [Redis ACLs](./api/redis-acls.md)
  - [OCSP](./api/ocsp.md)
  - [Certificates](./api/certificates.md)
  
- [Monitoring](./api/monitoring.md)
  - [Alerts](./api/alerts.md)
  - [Statistics](./api/stats.md)
  - [Logs](./api/logs.md)
  - [Usage Reports](./api/usage-reports.md)
  
- [Database Features](./api/database-features.md)
  - [Modules](./api/modules.md)
  - [Shards](./api/shards.md)
  - [Endpoints](./api/endpoints.md)
  - [Proxies](./api/proxies.md)
  
- [Advanced Operations](./api/advanced.md)
  - [Active-Active (CRDB)](./api/crdb.md)
  - [CRDB Tasks](./api/crdb-tasks.md)
  - [Migrations](./api/migrations.md)
  - [Backup/Restore](./api/backup.md)
  
- [Cluster Management](./api/cluster-management.md)
  - [Bootstrap](./api/bootstrap.md)
  - [Services](./api/services.md)
  - [License](./api/license.md)
  - [Suffixes](./api/suffixes.md)
  
- [Diagnostics](./api/diagnostics-index.md)
  - [Debug Info](./api/debuginfo.md)
  - [Diagnostics](./api/diagnostics.md)
  - [Job Scheduler](./api/job-scheduler.md)

# REST API Reference

- [Authentication](./rest-api/auth.md)
- [Common Parameters](./rest-api/common.md)
- [Error Codes](./rest-api/errors.md)

- [Core Resources]()
  - [Cluster](./rest-api/cluster.md)
  - [Databases (BDB)](./rest-api/databases.md)
  - [Nodes](./rest-api/nodes.md)
  - [Users](./rest-api/users.md)
  - [Roles](./rest-api/roles.md)

- [Management Resources]()
  - [Alerts](./rest-api/alerts.md)
  - [Modules](./rest-api/modules.md)
  - [Statistics](./rest-api/stats.md)
  - [Logs](./rest-api/logs.md)

- [Security Resources]()
  - [Redis ACLs](./rest-api/redis-acls.md)
  - [LDAP](./rest-api/ldap.md)
  - [OCSP](./rest-api/ocsp.md)

- [Database Resources]()
  - [Shards](./rest-api/shards.md)
  - [Endpoints](./rest-api/endpoints.md)
  - [Proxies](./rest-api/proxies.md)

- [Advanced Resources]()
  - [CRDB](./rest-api/crdb.md)
  - [CRDB Tasks](./rest-api/crdb-tasks.md)
  - [Migrations](./rest-api/migrations.md)
  - [Actions](./rest-api/actions.md)

- [Cluster Operations]()
  - [Bootstrap](./rest-api/bootstrap.md)
  - [CM Settings](./rest-api/cm-settings.md)
  - [Services](./rest-api/services.md)
  - [License](./rest-api/license.md)
  - [Suffixes](./rest-api/suffixes.md)

- [Diagnostics]()
  - [Debug Info](./rest-api/debuginfo.md)
  - [Diagnostics](./rest-api/diagnostics.md)
  - [Usage Reports](./rest-api/usage-reports.md)
  - [JSON Schema](./rest-api/jsonschema.md)

[API Coverage](./coverage.md)
