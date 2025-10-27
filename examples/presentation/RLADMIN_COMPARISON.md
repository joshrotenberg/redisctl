# rladmin vs redisctl Enterprise Feature Comparison

## Overview

**rladmin** is Redis Enterprise's built-in cluster management CLI that runs locally on cluster nodes. It provides powerful low-level operations, interactive mode with tab completion, and direct node access.

**redisctl** provides remote cluster management via REST API with rich output formats, cross-platform support, and full scriptability for modern DevOps workflows.

## Key Architecture Differences

| Aspect | rladmin | redisctl |
|--------|---------|----------|
| **Access Method** | Node-local (SSH required) | Remote REST API |
| **Installation** | Pre-installed on cluster nodes | Single binary, any platform |
| **Network** | No network needed | Requires HTTPS to cluster |
| **Authentication** | Node access = implicit auth | API credentials required |
| **Output** | Text only (parsable) | JSON, YAML, Table |
| **Mode** | Interactive + CLI | CLI only |

## Comprehensive Feature Matrix

### Cluster Management

| Feature | rladmin | redisctl enterprise | Notes |
|---------|---------|---------------------|-------|
| **Cluster Creation** | ✅ `cluster create` | ✅ `workflow init-cluster` | redisctl orchestrates multi-step setup |
| **Cluster Info** | ✅ `info cluster` | ✅ `cluster get` | Both show config details |
| **Cluster Status** | ✅ `status` | ✅ `cluster get` + `stats` | rladmin has detailed tables |
| **Cluster Config** | ✅ `cluster config` | ✅ `cluster update` | Both support tuning |
| **License Management** | ✅ `cluster license` | ✅ `license set/get` | Tie |
| **Debug Info** | ✅ `cluster debug_info` | ✅ `support-package cluster` | redisctl adds upload automation |

### Node Management

| Feature | rladmin | redisctl enterprise | Notes |
|---------|---------|---------------------|-------|
| **List Nodes** | ✅ `status` (nodes section) | ✅ `node list` | Both show all nodes |
| **Node Info** | ✅ `info node <id>` | ✅ `node get <id>` | Both show config |
| **Node IP Config** | ✅ `node <id> addr set` | ❌ Not exposed | rladmin only |
| **External Addr** | ✅ `node <id> external_addr` | ❌ Not exposed | rladmin only |
| **Recovery Path** | ✅ `node <id> recovery_path` | ❌ Not exposed | rladmin only |
| **Node Enslave** | ✅ `node <id> enslave` | ❌ Not exposed | rladmin only |
| **Node Snapshots** | ✅ `node <id> snapshot` | ❌ Not exposed | rladmin only |
| **Node Remove** | ✅ `node <id> remove` | ❌ Not exposed | rladmin only |
| **Maintenance Mode** | ✅ `node <id> maintenance_mode` | ❌ Not exposed | rladmin only |

### Database Operations

| Feature | rladmin | redisctl enterprise | Notes |
|---------|---------|---------------------|-------|
| **List Databases** | ✅ `status` (databases) | ✅ `database list` | Both show all DBs |
| **Database Info** | ✅ `info db <id>` | ✅ `database get <id>` | Both detailed |
| **Create Database** | ✅ `bdb create` | ✅ `database create` | Tie |
| **Update Database** | ✅ `bdb update` | ✅ `database update` | Tie |
| **Delete Database** | ✅ `bdb delete` | ✅ `database delete` | Tie |
| **Database Stats** | ⚠️ Via `status` | ✅ `database stats <id>` | redisctl more detailed |
| **Restart Database** | ✅ `restart db <id>` | ❌ Not exposed | rladmin only |
| **Upgrade Database** | ✅ `upgrade db <id>` | ❌ Not exposed | rladmin only |
| **Recover Database** | ✅ `recover db <id>` | ❌ Not exposed | rladmin only |
| **Database Backup** | ✅ Via config | ✅ `database backup` | Both support |
| **Database Restore** | ✅ Via config | ✅ `database restore` | Both support |

### Shard Operations

| Feature | rladmin | redisctl enterprise | Notes |
|---------|---------|---------------------|-------|
| **List Shards** | ✅ `status` (shards section) | ✅ `shard list` | Both show all shards |
| **Shard Info** | ✅ Via `status` | ✅ `shard get <id>` | Both detailed |
| **Failover Shard** | ✅ `failover shard <id>` | ❌ Not exposed | rladmin only |
| **Migrate Shard** | ✅ `migrate shard <id>` | ❌ Not exposed | rladmin only |
| **Migrate All Slaves** | ✅ `migrate all_slave_shards` | ❌ Not exposed | rladmin only |
| **Migrate All Masters** | ✅ `migrate all_master_shards` | ❌ Not exposed | rladmin only |

### Endpoint Management

| Feature | rladmin | redisctl enterprise | Notes |
|---------|---------|---------------------|-------|
| **List Endpoints** | ✅ `status` (endpoints) | ✅ Implied in DB list | rladmin more explicit |
| **Bind Endpoint** | ✅ `bind endpoint <id>` | ❌ Not exposed | rladmin only |
| **Endpoint Policy** | ✅ `bind endpoint <id> policy` | ❌ Not exposed | rladmin only |
| **Migrate Endpoints** | ✅ `migrate endpoint_to_shards` | ❌ Not exposed | rladmin only |

### DNS & Networking

| Feature | rladmin | redisctl enterprise | Notes |
|---------|---------|---------------------|-------|
| **List Suffixes** | ✅ `suffix list` | ❌ Not exposed | rladmin only |
| **Add Suffix** | ✅ `suffix add` | ❌ Not exposed | rladmin only |
| **Delete Suffix** | ✅ `suffix delete` | ❌ Not exposed | rladmin only |

### Verification & Validation

| Feature | rladmin | redisctl enterprise | Notes |
|---------|---------|---------------------|-------|
| **Balance Report** | ✅ `verify balance` | ❌ Not exposed | rladmin only |
| **Rack Aware Check** | ✅ `verify rack_aware` | ❌ Not exposed | rladmin only |

### Configuration Tuning

| Feature | rladmin | redisctl enterprise | Notes |
|---------|---------|---------------------|-------|
| **Tune Database** | ✅ `tune db <id> <param>` | ⚠️ Via `database update` | rladmin more granular |
| **Tune Proxy** | ✅ `tune proxy <param>` | ⚠️ Via API | rladmin more direct |
| **Tune Cluster** | ✅ `tune cluster <param>` | ⚠️ Via `cluster update` | rladmin more granular |
| **Tune Node** | ✅ `tune node <id> <param>` | ⚠️ Via `node update` | rladmin more granular |

### Output & Scripting

| Feature | rladmin | redisctl enterprise | Winner |
|---------|---------|---------------------|---------|
| **Output Format** | Text tables | JSON, YAML, Table | redisctl |
| **Structured Output** | ❌ Parse text | ✅ Native JSON/YAML | redisctl |
| **JMESPath Queries** | ❌ Not supported | ✅ Built-in `-q` flag | redisctl |
| **Progress Indicators** | ❌ No feedback | ✅ Spinners with `--wait` | redisctl |
| **Interactive Mode** | ✅ With tab completion | ❌ CLI only | rladmin |
| **Batch Commands** | ✅ Non-interactive mode | ✅ All commands | Tie |
| **Auto-confirmation** | ✅ `-y` flag | ✅ `--force` where needed | Tie |

### Authentication & Multi-Cluster

| Feature | rladmin | redisctl enterprise | Winner |
|---------|---------|---------------------|---------|
| **Local Auth** | ✅ Node access = auth | ❌ Requires API creds | rladmin |
| **Remote Auth** | N/A | ✅ Username/password | redisctl |
| **Keyring Storage** | N/A | ✅ OS keyring integration | redisctl |
| **Multi-cluster** | ❌ One node at a time | ✅ Profile system | redisctl |
| **Profile Management** | N/A | ✅ Secure storage | redisctl |

### Advanced Features

| Feature | rladmin | redisctl enterprise | Winner |
|---------|---------|---------------------|---------|
| **Workflows** | ❌ Manual multi-step | ✅ Orchestrated | redisctl |
| **Async Operations** | ❌ Manual polling | ✅ Auto `--wait` | redisctl |
| **Raw API Access** | N/A | ✅ `api enterprise` layer | redisctl |
| **Support Package Upload** | ❌ Manual | ✅ Direct to Files.com | redisctl |
| **Package Optimization** | N/A | ✅ 20-30% compression | redisctl |

### Integration & Automation

| Feature | rladmin | redisctl enterprise | Winner |
|---------|---------|---------------------|---------|
| **CI/CD Pipelines** | ⚠️ SSH + text parsing | ✅ JSON output, no SSH | redisctl |
| **Terraform** | ❌ Not integrated | ✅ Library for providers | redisctl |
| **Monitoring Tools** | ⚠️ Text scraping | ✅ JSON metrics | redisctl |
| **Custom Tools** | ⚠️ Shell scripting | ✅ Rust library integration | redisctl |
| **Cross-platform** | ❌ Linux on nodes only | ✅ macOS, Linux, Windows | redisctl |

## Legend
- ✅ Full support
- ⚠️ Partial/limited support
- ❌ Not supported / Not exposed
- N/A Not applicable

## Key Insights from Testing

### rladmin Strengths (Verified)

1. **Low-level Node Operations** - Extensive node management (snapshots, maintenance mode, recovery paths)
2. **Shard Operations** - Direct shard failover and migration control
3. **Endpoint Management** - Granular endpoint binding and policy control
4. **DNS Management** - Suffix configuration for DNS
5. **Interactive Mode** - Tab completion and command help (Type `rladmin` then press Enter)
6. **Verification Tools** - Balance reports and rack-aware validation
7. **No Network Required** - Works when API is down/broken
8. **Direct Tuning** - Low-level parameter tuning for db/proxy/cluster/node

### redisctl Strengths (Verified)

1. **Remote Management** - No SSH required, manage from anywhere
2. **Structured Output** - JSON/YAML for automation, perfect for CI/CD
3. **Cross-Platform** - Works on developer laptops (macOS/Windows/Linux)
4. **JMESPath Filtering** - Query and filter output easily
5. **Workflows** - Multi-step orchestration (init-cluster, etc.)
6. **Async Handling** - Automatic polling with `--wait` flags
7. **Support Package Automation** - Generate + optimize + upload in one command
8. **Multi-Cluster** - Profile system for managing many clusters
9. **Better DevOps UX** - Progress indicators, clear errors, structured output

### API Coverage Gap

**Important Finding**: The REST API does not expose all operations that rladmin provides. Key gaps include:

- Node-level operations (maintenance mode, snapshots, recovery paths)
- Direct shard failover and migration
- Endpoint binding configuration
- DNS suffix management
- Cluster verification tools
- Database restart/upgrade/recover commands

This is by design - these are low-level operations typically performed on-node, not remotely.

## Use Cases

### Use rladmin when:

1. **Low-level maintenance** - Node snapshots, maintenance mode, recovery
2. **Shard operations** - Manual failover or migration of specific shards
3. **Emergency troubleshooting** - API is down, need direct node access
4. **DNS configuration** - Managing cluster DNS suffixes
5. **Interactive exploration** - Tab completion makes discovery easy
6. **Balance verification** - Checking shard distribution across nodes

### Use redisctl when:

1. **Remote management** - From your laptop, no SSH needed
2. **CI/CD automation** - JSON output, no text parsing
3. **Multi-cluster management** - Managing multiple Redis Enterprise clusters
4. **Support workflows** - Generating and uploading support packages
5. **Cross-platform work** - Running on macOS/Windows
6. **DevOps integration** - Terraform, monitoring, custom tools
7. **Day-to-day operations** - Creating/updating/deleting databases
8. **Structured data needs** - JMESPath queries, JSON for scripts

## Example Comparisons

### Task 1: Get Database Memory Size

**rladmin approach:**
```bash
# SSH to cluster node
ssh admin@cluster-node

# Get database info (text output)
rladmin info db db:1

# Output is text - must parse manually
bdb:1
  memory_size: 1073741824
  ...

# Parse with grep/awk
rladmin info db db:1 | grep memory_size | awk '{print $2}'
```

**redisctl approach:**
```bash
# From your laptop (no SSH needed)
redisctl enterprise database get 1 -o json -q 'memory_size'

# Output: 1073741824
```

### Task 2: Interactive Cluster Exploration

**rladmin approach:**
```bash
# SSH to node
ssh admin@cluster-node

# Start interactive mode
rladmin

# Use tab completion
rladmin> status <TAB>
rladmin> info cluster <TAB>
```

**redisctl approach:**
```bash
# No interactive mode, but rich output
redisctl enterprise cluster get -o table
redisctl enterprise database list -o json | jq
```

### Task 3: Generate Support Package

**rladmin approach:**
```bash
# 1. SSH to cluster node
ssh admin@cluster-node

# 2. Generate debug info
rladmin cluster debug_info

# 3. Find the file
ls -ltr /tmp/*.tar.gz | tail -1

# 4. SCP to local machine
scp admin@cluster-node:/tmp/debuginfo-*.tar.gz ./

# 5. Upload to Redis Support via web UI
# (10+ minutes of manual clicking)
```

**redisctl approach:**
```bash
# One command from your laptop
redisctl enterprise support-package cluster \
  --optimize \
  --upload

# Done in 30 seconds
```

### Task 4: Manage Node Maintenance

**rladmin approach:**
```bash
# SSH to node
ssh admin@cluster-node

# Enter maintenance mode (migrates shards out)
rladmin node 1 maintenance_mode on

# Exit maintenance mode (restores shards)
rladmin node 1 maintenance_mode off
```

**redisctl approach:**
```bash
# Not exposed via REST API
# Must use rladmin
```

## Online Community Insights

Based on Stack Overflow and documentation searches:

1. **rladmin Usage Patterns**:
   - Commonly used in Docker setups: `docker exec -it redis-node rladmin`
   - Interactive mode preferred by operators: `rladmin` then tab completion
   - Scripting requires text parsing: output not structured
   - Located at `/opt/redislabs/bin/rladmin` on nodes

2. **Pain Points Mentioned**:
   - Requires SSH access to nodes
   - Text output requires parsing for automation
   - No cross-platform support (Linux nodes only)
   - Must be on the cluster node

3. **Strengths Mentioned**:
   - Tab completion makes it discoverable
   - Comprehensive low-level operations
   - Works when API is broken
   - Fast for on-node troubleshooting

## Conclusion

**rladmin** and **redisctl** are complementary tools with different design goals:

### rladmin: Node-Local Power Tool
- Designed for direct cluster node operations
- Provides low-level control (shards, endpoints, nodes)
- Interactive mode with tab completion
- Essential for emergency troubleshooting
- No network dependency

### redisctl: Remote DevOps Platform
- Designed for remote management and automation
- REST API based (works from anywhere)
- Structured output (JSON/YAML) for CI/CD
- Cross-platform (macOS, Windows, Linux)
- Multi-cluster profile management
- Modern DevOps workflows

### Best Practice: Use Both

- **Primary tool: redisctl** for day-to-day operations, automation, CI/CD
- **Secondary tool: rladmin** for emergencies, low-level ops, troubleshooting

### Feature Recommendations for redisctl

Based on this analysis, features worth considering for redisctl:

1. **Interactive mode** with tab completion (like rladmin)
2. **Balance verification** - Check shard distribution
3. **Cluster health checks** - Beyond basic status
4. **More granular tuning** - Expose low-level parameters where safe
5. **Support for node operations** where API permits

**Note**: Many rladmin operations (shard migration, endpoint binding, etc.) are intentionally not exposed via REST API for safety reasons. This is by design, not a limitation of redisctl.
