# Appendix: rladmin vs redisctl

## Overview

**rladmin** is Redis Enterprise's built-in CLI for node-local cluster management.  
**redisctl** provides remote cluster management via REST API.

They're **complementary tools** - use both!

## Quick Comparison

| Feature | rladmin | redisctl enterprise |
|---------|---------|---------------------|
| Deployment | Pre-installed on nodes | Single binary, any platform |
| Access | Local (SSH required) | Remote (REST API) |
| Platforms | Linux only (on nodes) | macOS, Windows, Linux |
| Output | Text only | JSON, YAML, Table |
| Scripting | Text parsing required | Native JSON |
| Multi-cluster | One at a time | Profile system |

## Where rladmin Excels

**Local node operations** - Direct node access  
**No network dependency** - Works when API is down  
**Low-level operations** - Node-specific commands  
**Already installed** - No extra setup  

## Where redisctl Excels

**Remote management** - No SSH required  
**Structured output** - JSON/YAML for automation  
**Cross-platform** - Works on developer laptops  
**Multi-cluster** - Profile system  
**Rich features** - JMESPath, workflows, support packages  
**Better UX** - Progress indicators, `--wait` for async ops  

## Example Comparison

### Get Database Info

**rladmin approach:**
```bash
# SSH to node
ssh admin@cluster-node

# Get info (text output)
rladmin info bdb 1 | grep memory | awk '{print $2}'
```

**redisctl approach:**
```bash
# From your laptop (no SSH)
redisctl enterprise database get 1 -q 'memory_size'
```

### Support Package

**rladmin approach:**
```bash
# 1. SSH to node
ssh admin@cluster-node

# 2. Generate
rladmin cluster debug_info

# 3. SCP to local
scp admin@node:/tmp/debug*.tar.gz ./

# 4. Manually upload via web UI
# (10+ minutes)
```

**redisctl approach:**
```bash
# One command from laptop
redisctl enterprise support-package cluster --optimize --upload
# (30 seconds)
```

## Use Cases

### Use rladmin when:
- SSH'd into a cluster node
- Need low-level node operations
- API is unavailable
- Debugging directly on nodes

### Use redisctl when:
- Remote management from laptop
- CI/CD automation
- Managing multiple clusters
- Need structured output
- Building custom tools

## Best Practice

**Use both:**
- **redisctl** for day-to-day ops, automation, remote management
- **rladmin** for emergency troubleshooting, node-level operations

## Full Comparison

See [RLADMIN_COMPARISON.md](https://github.com/joshrotenberg/redisctl/blob/main/examples/presentation/RLADMIN_COMPARISON.md) for detailed feature matrix with 30+ comparisons.

---

**Back:** [9. Next Steps](./09-next-steps.md)  
**[Return to Start](./README.md)**
