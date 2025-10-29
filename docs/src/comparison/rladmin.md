# rladmin vs redisctl

This guide helps you understand when to use **rladmin** (Redis Enterprise's built-in CLI) versus **redisctl**.

## Quick Summary

| Aspect | rladmin | redisctl |
|--------|---------|----------|
| **Access** | Node-local (SSH required) | Remote REST API |
| **Installation** | Pre-installed on nodes | Single binary |
| **Platform** | Linux (nodes only) | macOS, Linux, Windows |
| **Output** | Text tables | JSON, YAML, Table |
| **Best For** | Low-level ops, troubleshooting | Automation, DevOps, CI/CD |

## When to Use Each Tool

### Use rladmin for:
- Low-level node operations (maintenance mode, snapshots)
- Direct shard and endpoint management
- Emergency troubleshooting when API is down
- Interactive exploration (tab completion)
- Operations not exposed via REST API

### Use redisctl for:
- Remote cluster management from your laptop
- CI/CD pipeline automation
- Multi-cluster management via profiles
- Structured output (JSON/YAML) for scripts
- Support package generation and upload
- Day-to-day database operations

## Key Architecture Differences

| Aspect | rladmin | redisctl |
|--------|---------|----------|
| **Access Method** | Node-local (SSH required) | Remote REST API |
| **Installation** | Pre-installed on cluster nodes | Single binary, any platform |
| **Network** | No network needed | Requires HTTPS to cluster |
| **Authentication** | Node access = implicit auth | API credentials required |
| **Output** | Text only (parsable) | JSON, YAML, Table |
| **Mode** | Interactive + CLI | CLI only |

## Feature Comparison by Category

### Cluster Management

| Feature | rladmin | redisctl |
|---------|---------|----------|
| Cluster creation | ✅ `cluster create` | ✅ `workflow init-cluster` |
| Cluster info | ✅ `info cluster` | ✅ `cluster get` |
| Cluster status | ✅ `status` | ✅ `cluster get` + `stats` |
| License management | ✅ `cluster license` | ✅ `license set/get` |
| Debug info | ✅ `cluster debug_info` | ✅ `support-package cluster` |

### Database Operations

| Feature | rladmin | redisctl |
|---------|---------|----------|
| List databases | ✅ `status` | ✅ `database list` |
| Create database | ✅ `bdb create` | ✅ `database create` |
| Update database | ✅ `bdb update` | ✅ `database update` |
| Delete database | ✅ `bdb delete` | ✅ `database delete` |
| Database stats | ⚠️ Via `status` | ✅ `database stats` |
| Restart database | ✅ `restart db` | ❌ Not exposed |
| Upgrade database | ✅ `upgrade db` | ❌ Not exposed |

### Node Operations

| Feature | rladmin | redisctl |
|---------|---------|----------|
| List nodes | ✅ `status` | ✅ `node list` |
| Node info | ✅ `info node` | ✅ `node get` |
| Maintenance mode | ✅ `node maintenance_mode` | ❌ Not exposed |
| Node snapshots | ✅ `node snapshot` | ❌ Not exposed |

### Output & Automation

| Feature | rladmin | redisctl |
|---------|---------|----------|
| JSON output | ❌ Text only | ✅ Native JSON |
| YAML output | ❌ Text only | ✅ Native YAML |
| JMESPath queries | ❌ Not supported | ✅ Built-in `-q` flag |
| Progress indicators | ❌ No feedback | ✅ Spinners with `--wait` |
| Interactive mode | ✅ Tab completion | ❌ CLI only |

**Legend:**
- ✅ Full support
- ⚠️ Partial support
- ❌ Not supported

## Example Comparisons

### Task: Get Database Memory Size

**rladmin approach:**
```bash
# SSH to cluster node
ssh admin@cluster-node

# Get database info (text output)
rladmin info db db:1

# Parse with grep/awk
rladmin info db db:1 | grep memory_size | awk '{print $2}'
```

**redisctl approach:**
```bash
# From your laptop (no SSH needed)
redisctl enterprise database get 1 -o json -q 'memory_size'
```

### Task: Generate Support Package

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

# 5. Upload to Redis Support via web UI (10+ minutes)
```

**redisctl approach:**
```bash
# One command from your laptop
redisctl enterprise support-package cluster --optimize --upload

# Done in 30 seconds
```

### Task: Interactive Exploration

**rladmin approach:**
```bash
# SSH to node
ssh admin@cluster-node

# Start interactive mode with tab completion
rladmin

rladmin> status <TAB>
rladmin> info cluster <TAB>
```

**redisctl approach:**
```bash
# No interactive mode (yet), but rich output
redisctl enterprise cluster get -o table
redisctl enterprise database list -o json | jq
```

## Why They're Complementary

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

## Best Practice

Use both tools together:

- **Primary tool: redisctl** for day-to-day operations, automation, CI/CD
- **Secondary tool: rladmin** for emergencies, low-level ops, troubleshooting

## API Coverage Gap

The REST API does not expose all operations that rladmin provides:

- Node-level operations (maintenance mode, snapshots, recovery paths)
- Direct shard failover and migration
- Endpoint binding configuration
- DNS suffix management
- Cluster verification tools
- Database restart/upgrade/recover commands

This is by design - these are low-level operations typically performed on-node, not remotely.

## Community Insights

Based on Stack Overflow and documentation research:

**rladmin strengths:**
- Tab completion makes it discoverable
- Comprehensive low-level operations
- Works when API is broken
- Fast for on-node troubleshooting

**Pain points:**
- Requires SSH access to nodes
- Text output requires parsing for automation
- No cross-platform support (Linux nodes only)

## Planned Features

We're adding rladmin-inspired features to redisctl:

- Interactive mode with tab completion ([#417](https://github.com/joshrotenberg/redisctl/issues/417))
- Cluster balance verification ([#418](https://github.com/joshrotenberg/redisctl/issues/418))
- Rack-aware verification ([#419](https://github.com/joshrotenberg/redisctl/issues/419))
- Comprehensive status command ([#420](https://github.com/joshrotenberg/redisctl/issues/420))

See [issue #416](https://github.com/joshrotenberg/redisctl/issues/416) for the full list.

## Conclusion

**rladmin and redisctl are complementary tools, not competitors.** They serve different purposes:

- Use **redisctl** for remote management, automation, and modern DevOps workflows
- Use **rladmin** for low-level operations, emergency troubleshooting, and on-node tasks

The best practice is to use both, letting each tool do what it does best.
