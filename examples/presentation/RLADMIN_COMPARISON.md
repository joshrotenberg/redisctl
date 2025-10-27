# rladmin vs redisctl Enterprise Feature Comparison

## Overview

**rladmin** is Redis Enterprise's built-in cluster management CLI that runs locally on cluster nodes. It's powerful but node-local, lacks scriptability, and has limited output formatting.

**redisctl** provides remote cluster management via REST API with rich output formats, cross-platform support, and full scriptability.

## Feature Matrix

| Feature | rladmin | redisctl enterprise | Winner |
|---------|---------|---------------------|---------|
| **Deployment** |
| Installation | Pre-installed on nodes | Single binary, any platform | redisctl |
| Remote management | ❌ Must SSH to nodes | ✅ REST API from anywhere | redisctl |
| Cross-platform | ❌ Linux only (on nodes) | ✅ macOS, Linux, Windows | redisctl |
| Docker support | ❌ Requires node access | ✅ Direct API access | redisctl |
| **Cluster Management** |
| Cluster info | ✅ `rladmin info cluster` | ✅ `redisctl enterprise cluster get` | Tie |
| Node management | ✅ `rladmin node <id>` | ✅ `redisctl enterprise node list/get` | Tie |
| Cluster initialization | ✅ `rladmin cluster create` | ✅ `redisctl enterprise workflow init-cluster` | redisctl¹ |
| License management | ✅ `rladmin cluster license` | ✅ `redisctl enterprise license` | Tie |
| **Database Operations** |
| List databases | ✅ `rladmin status databases` | ✅ `redisctl enterprise database list` | Tie |
| Create database | ✅ `rladmin bdb create` | ✅ `redisctl enterprise database create` | Tie |
| Update database | ✅ `rladmin bdb update` | ✅ `redisctl enterprise database update` | Tie |
| Delete database | ✅ `rladmin bdb delete` | ✅ `redisctl enterprise database delete` | Tie |
| **Configuration** |
| Cluster settings | ✅ `rladmin cluster config` | ✅ `redisctl enterprise cluster update` | Tie |
| Node settings | ✅ `rladmin node config` | ✅ `redisctl enterprise node update` | Tie |
| **Monitoring** |
| Cluster stats | ⚠️ Limited output | ✅ `redisctl enterprise cluster stats` | redisctl |
| Database stats | ⚠️ Limited output | ✅ `redisctl enterprise database stats` | redisctl |
| Log streaming | ❌ View logs manually | ✅ `redisctl enterprise logs list --follow` | redisctl |
| Alerts | ✅ View/manage alerts | ✅ `redisctl enterprise alert list/update` | Tie |
| **Operations** |
| Backup | ✅ `rladmin backup` | ✅ `redisctl enterprise database backup` | Tie |
| Restore | ✅ `rladmin restore` | ✅ `redisctl enterprise database restore` | Tie |
| Failover | ✅ `rladmin failover` | ✅ `redisctl enterprise database failover` | Tie |
| Shard management | ✅ `rladmin shard` | ✅ `redisctl enterprise shard list/get` | Tie |
| **Support & Diagnostics** |
| Support package | ⚠️ Manual download | ✅ Generate + upload automation | redisctl² |
| Debug info | ✅ `rladmin cluster debug_info` | ✅ `redisctl enterprise support-package` | redisctl² |
| **Output & Scripting** |
| Output format | ❌ Text only | ✅ JSON, YAML, Table | redisctl |
| Structured output | ❌ Parse text | ✅ Native JSON/YAML | redisctl |
| JMESPath queries | ❌ Not supported | ✅ Built-in filtering | redisctl |
| Scriptability | ⚠️ Text parsing required | ✅ JSON output → easy parsing | redisctl |
| Progress indicators | ❌ No feedback | ✅ Built-in with `--wait` | redisctl |
| **Authentication** |
| Local auth | ✅ Node access = auth | ❌ Requires API credentials | rladmin |
| Remote auth | N/A | ✅ Username/password or keyring | redisctl |
| Multi-cluster | ❌ One node at a time | ✅ Profile system | redisctl |
| **Advanced Features** |
| Workflows | ❌ Manual multi-step | ✅ Orchestrated workflows | redisctl |
| Async operations | ❌ Manual polling | ✅ Automatic `--wait` | redisctl |
| Raw API access | ❌ Not applicable | ✅ `redisctl api enterprise` | redisctl |
| Profile management | ❌ Not applicable | ✅ Secure keyring storage | redisctl |
| **Integration** |
| CI/CD pipelines | ⚠️ Requires SSH + parsing | ✅ JSON output, no SSH | redisctl |
| Terraform | ❌ Not integrated | ✅ Libraries for provider³ | redisctl |
| Monitoring tools | ⚠️ Text scraping | ✅ JSON metrics | redisctl |
| Custom tools | ⚠️ Shell scripting | ✅ Rust library integration | redisctl |

### Legend
- ✅ Full support
- ⚠️ Partial/limited support
- ❌ Not supported
- N/A Not applicable

### Footnotes

¹ **Cluster Init**: redisctl's workflow handles bootstrap + auth + license + default DB in one command. rladmin requires multiple steps.

² **Support Package**: redisctl can generate, optimize (20-30% compression), and upload directly to Redis Support (Files.com) in one command. rladmin requires manual download, then manual upload.

³ **Library Integration**: redisctl-config, redis-enterprise libraries can be used by Terraform providers or custom tools.

## Key Differentiators

### Where rladmin Wins

1. **Local Node Access** - Already installed, no extra setup
2. **Low-level Operations** - Direct node-level commands
3. **No Network Dependency** - Works when API is down

### Where redisctl Wins

1. **Remote Management** - No SSH required, manage from anywhere
2. **Structured Output** - JSON/YAML for automation
3. **Cross-Platform** - Works on developer laptops (macOS/Windows)
4. **Scriptability** - Built for CI/CD and automation
5. **Multi-Cluster** - Profile system for managing multiple clusters
6. **Rich Features** - JMESPath, workflows, automatic polling, support package automation
7. **Better UX** - Progress indicators, clear errors, `--wait` for async ops

## Use Cases

### Use rladmin when:
- You're SSH'd into a cluster node
- You need low-level node operations
- API is unavailable/broken
- You're debugging directly on nodes

### Use redisctl when:
- Remote cluster management from your laptop
- CI/CD pipeline automation
- Managing multiple clusters
- Need structured output for scripts
- Want rich formatting (tables, JSON, YAML)
- Building custom tools/integrations
- Generating support packages for Redis Support

## Example Comparison

### Task: Get Database Info and Extract Memory Size

**rladmin approach:**
```bash
# SSH to cluster node
ssh admin@cluster-node

# Get database info (text output)
rladmin info bdb 1

# Output is text - must parse manually
bdb:1
  status: active
  memory: 1073741824
  ...

# Parse with grep/awk/sed
rladmin info bdb 1 | grep memory | awk '{print $2}'
```

**redisctl approach:**
```bash
# From your laptop (no SSH)
redisctl enterprise database get 1 -o json -q 'memory_size'

# Output: 1073741824

# Or in table format for humans
redisctl enterprise database get 1 -o table
```

### Task: Generate and Upload Support Package

**rladmin approach:**
```bash
# 1. SSH to cluster node
ssh admin@cluster-node

# 2. Generate debug info
rladmin cluster debug_info

# 3. Find the generated file
ls -ltr /tmp/*.tar.gz | tail -1

# 4. SCP to local machine
scp admin@cluster-node:/tmp/debuginfo-*.tar.gz ./

# 5. Manually upload to Redis Support portal via web UI
# (10+ minutes of clicking)
```

**redisctl approach:**
```bash
# One command from your laptop
redisctl enterprise support-package cluster \
  --optimize \
  --upload

# Done in 30 seconds
```

## Conclusion

**rladmin** and **redisctl** are complementary tools:

- **rladmin** excels at local node management and low-level operations
- **redisctl** excels at remote management, automation, and modern DevOps workflows

For modern cloud-native operations, CI/CD pipelines, and multi-cluster management, **redisctl** provides a superior experience. For on-node troubleshooting and maintenance, **rladmin** remains valuable.

**Best practice**: Use both
- **redisctl** for day-to-day operations, automation, and remote management
- **rladmin** for emergency troubleshooting and node-level operations
