# Phase 2: Command Coverage Audit

## Redis Cloud API Coverage

| API Category | API Method | Current CLI Command | Status | Notes |
|--------------|------------|-------------------|---------|--------|
| **Subscriptions** | | | | |
| | list() | ✅ `cloud subscription list` | ✅ Implemented | |
| | get() | ✅ `cloud subscription show` | ✅ Implemented | |
| | create() | ⚠️ `cloud subscription create` | ⚠️ Basic | Needs more options |
| | update() | ⚠️ `cloud subscription update` | ⚠️ Basic | Only name |
| | delete() | ❌ `cloud subscription delete` | ❌ Not implemented | |
| | databases() | ❌ | ❌ Missing | Should list databases in subscription |
| | pricing() | ❌ | ❌ Missing | |
| | payment_methods() | ❌ | ❌ Missing | |
| | cloud_accounts() | ❌ | ❌ Missing | |
| | get_cidr_whitelist() | ❌ | ❌ Missing | |
| | update_cidr_whitelist() | ❌ | ❌ Missing | |
| | get_vpc_peerings() | ❌ | ❌ Missing | |
| **Databases** | | | | |
| | list() | ✅ `cloud database list` | ✅ Implemented | Lists across all subscriptions |
| | show() | ✅ `cloud database show` | ✅ Implemented | |
| | create() | ⚠️ `cloud database create` | ⚠️ Redirect | Redirects to subscription context |
| | update() | ⚠️ `cloud database update` | ⚠️ Basic | Only name/memory |
| | delete() | ⚠️ `cloud database delete` | ⚠️ Basic | No --force flag |
| | backup() | ❌ | ❌ Not implemented | |
| | import() | ❌ | ❌ Not implemented | |
| **Accounts** | | | | |
| | list() | ✅ `cloud account list` | ✅ Implemented | |
| | show() | ✅ `cloud account show` | ✅ Implemented | |
| | info() | ❌ | ❌ Missing | Different from show |
| | owner() | ❌ | ❌ Missing | |
| | users() | ❌ | ❌ Missing | |
| | payment_methods() | ❌ | ❌ Missing | |
| **Users** | | | | |
| | list() | ✅ `cloud user list` | ✅ Implemented | |
| | show() | ✅ `cloud user show` | ✅ Implemented | |
| | create() | ❌ `cloud user create` | ❌ Not implemented | Shows "not yet implemented" |
| | update() | ❌ `cloud user update` | ❌ Not implemented | Shows "not yet implemented" |
| | delete() | ❌ `cloud user delete` | ❌ Not implemented | Shows "not yet implemented" |
| **Regions** | | | | |
| | list() | ✅ `cloud region list` | ✅ Implemented | |
| **Tasks** | | | | |
| | list() | ✅ `cloud task list` | ✅ Implemented | |
| | show() | ✅ `cloud task show` | ✅ Implemented | |
| | wait() | ❌ | ❌ Missing | Should add --wait flag |
| **ACLs** | | | | |
| | list() | ❌ `cloud acl` | ❌ Not implemented | Shows "not yet implemented" |
| | get() | ❌ | ❌ Missing | |
| | create() | ❌ | ❌ Missing | |
| | update() | ❌ | ❌ Missing | |
| | delete() | ❌ | ❌ Missing | |
| | list_users() | ❌ | ❌ Missing | |
| | list_roles() | ❌ | ❌ Missing | |
| | list_redis_rules() | ❌ | ❌ Missing | |
| **Peering** | | | | |
| | list() | ❌ | ❌ Missing | No peering command at all |
| | create() | ❌ | ❌ Missing | |
| | get() | ❌ | ❌ Missing | |
| | delete() | ❌ | ❌ Missing | |
| **Transit Gateway** | | | | |
| | list() | ❌ | ❌ Missing | No transit-gateway command |
| | get_attachment() | ❌ | ❌ Missing | |
| | create_attachment() | ❌ | ❌ Missing | |
| | delete_attachment() | ❌ | ❌ Missing | |
| | list_invitations() | ❌ | ❌ Missing | |
| | accept_invitation() | ❌ | ❌ Missing | |
| | reject_invitation() | ❌ | ❌ Missing | |
| **Cloud Accounts** | | | | |
| | list() | ❌ | ❌ Missing | Different from Account |
| | get() | ❌ | ❌ Missing | |
| | create() | ❌ | ❌ Missing | |
| | update() | ❌ | ❌ Missing | |
| | delete() | ❌ | ❌ Missing | |
| **Backups** | | | | |
| | list() | ❌ | ❌ Missing | No backup command |
| | create() | ❌ | ❌ Missing | |
| | get() | ❌ | ❌ Missing | |
| | restore() | ❌ | ❌ Missing | |
| | delete() | ❌ | ❌ Missing | |
| **Fixed Plans** | | | | |
| | list() | ❌ | ❌ Missing | No fixed command |
| | get() | ❌ | ❌ Missing | |
| | plans() | ❌ | ❌ Missing | |
| **Flexible Plans** | | | | |
| | list() | ❌ | ❌ Missing | No flexible command |
| | get() | ❌ | ❌ Missing | |
| | create() | ❌ | ❌ Missing | |
| | update() | ❌ | ❌ Missing | |
| | delete() | ❌ | ❌ Missing | |
| **CRDB** | | | | |
| | list() | ❌ | ❌ Missing | No crdb command |
| | get() | ❌ | ❌ Missing | |
| | create() | ❌ | ❌ Missing | |
| | update() | ❌ | ❌ Missing | |
| | delete() | ❌ | ❌ Missing | |
| | get_regions() | ❌ | ❌ Missing | |
| | add_region() | ❌ | ❌ Missing | |
| | remove_region() | ❌ | ❌ Missing | |
| **API Keys** | | | | |
| | list() | ❌ | ❌ Missing | No api-key command |
| | get() | ❌ | ❌ Missing | |
| | create() | ❌ | ❌ Missing | |
| | update() | ❌ | ❌ Missing | |
| | delete() | ❌ | ❌ Missing | |
| | regenerate() | ❌ | ❌ Missing | |
| | enable() | ❌ | ❌ Missing | |
| | disable() | ❌ | ❌ Missing | |
| **Metrics** | | | | |
| | database() | ❌ | ❌ Missing | No metrics command |
| | subscription() | ❌ | ❌ Missing | |
| **Logs** | | | | |
| | database() | ❌ | ❌ Missing | No logs command |
| | system() | ❌ | ❌ Missing | |
| | session() | ❌ | ❌ Missing | |
| **Private Service Connect** | | | | |
| | list() | ❌ | ❌ Missing | No psc command |
| | get() | ❌ | ❌ Missing | |
| | create() | ❌ | ❌ Missing | |
| | update() | ❌ | ❌ Missing | |
| | delete() | ❌ | ❌ Missing | |

## Redis Enterprise API Coverage

| API Category | API Method | Current CLI Command | Status | Notes |
|--------------|------------|-------------------|---------|--------|
| **Clusters** | | | | |
| | info() | ✅ `enterprise cluster info` | ✅ Implemented | |
| | nodes() | ✅ `enterprise cluster nodes` | ✅ Implemented | |
| | settings() | ✅ `enterprise cluster settings` | ✅ Implemented | |
| | update() | ✅ `enterprise cluster update` | ✅ Implemented | |
| **Databases** | | | | |
| | list() | ✅ `enterprise database list` | ✅ Implemented | |
| | show() | ✅ `enterprise database show` | ✅ Implemented | |
| | create() | ⚠️ `enterprise database create` | ⚠️ Basic | Needs more options |
| | update() | ⚠️ `enterprise database update` | ⚠️ Basic | Only name/memory |
| | delete() | ⚠️ `enterprise database delete` | ⚠️ Basic | No --force flag |
| | backup() | ✅ `enterprise database backup` | ✅ Implemented | |
| | import() | ✅ `enterprise database import` | ✅ Implemented | |
| **Nodes** | | | | |
| | list() | ✅ `enterprise node list` | ✅ Implemented | |
| | show() | ✅ `enterprise node show` | ✅ Implemented | |
| | update() | ⚠️ `enterprise node update` | ⚠️ Basic | Only external_addr |
| | add() | ❌ | ❌ Missing | |
| | remove() | ❌ | ❌ Missing | |
| **Users** | | | | |
| | list() | ✅ `enterprise user list` | ✅ Implemented | |
| | show() | ✅ `enterprise user show` | ✅ Implemented | |
| | create() | ⚠️ `enterprise user create` | ⚠️ Basic | Limited options |
| | update() | ⚠️ `enterprise user update` | ⚠️ Basic | Only email/password |
| | delete() | ⚠️ `enterprise user delete` | ⚠️ Basic | No --force flag |
| **Roles** | | | | |
| | list() | ✅ `enterprise role list` | ✅ Implemented | |
| | show() | ✅ `enterprise role show` | ✅ Implemented | |
| | create() | ⚠️ `enterprise role create` | ⚠️ Basic | |
| | update() | ⚠️ `enterprise role update` | ⚠️ Basic | |
| | delete() | ⚠️ `enterprise role delete` | ⚠️ Basic | No --force flag |
| **Modules** | | | | |
| | list() | ✅ `enterprise module list` | ✅ Implemented | |
| | show() | ✅ `enterprise module show` | ✅ Implemented | |
| | upload() | ❌ `enterprise module upload` | ❌ Not implemented | Shows "not yet implemented" |
| **License** | | | | |
| | info() | ✅ `enterprise license info` | ✅ Implemented | |
| | update() | ✅ `enterprise license update` | ✅ Implemented | |
| **Bootstrap** | | | | |
| | create() | ✅ `enterprise bootstrap create` | ✅ Implemented | |
| | join() | ❌ | ❌ Missing | |

## Summary

### Cloud API Coverage
- **Total API Methods**: ~100+
- **Fully Implemented**: ~15 (15%)
- **Partially Implemented**: ~8 (8%)
- **Not Implemented**: ~77 (77%)

### Enterprise API Coverage
- **Total API Methods**: ~30
- **Fully Implemented**: ~15 (50%)
- **Partially Implemented**: ~10 (33%)
- **Not Implemented**: ~5 (17%)

### Priority Missing Commands for Cloud
1. **Peering** - Critical for networking
2. **Transit Gateway** - Critical for networking
3. **Backups** - Critical for operations
4. **ACLs** - Critical for security
5. **CRDB** - Critical for Active-Active
6. **Metrics & Logs** - Important for monitoring
7. **API Keys** - Important for automation
8. **Cloud Accounts** - Important for multi-cloud

### Priority Missing Commands for Enterprise
1. **Node add/remove** - Critical for cluster management
2. **Module upload** - Important for custom modules
3. **Bootstrap join** - Important for cluster setup

## Next Steps

1. Implement missing critical commands
2. Enhance existing basic commands with more options
3. Add convenience flags (--wait, --force, --dry-run)
4. Improve error messages
5. Add progress indicators for long operations