# 2. Enter redisctl

**The FIRST command-line tool for Redis Cloud and Enterprise**

## What is redisctl?

A unified CLI that eliminates fragile bash scripts with:

- Type-safe API clients
- Automatic async operation handling
- Support package automation
- Profile management with secure keyring
- Structured output (JSON, YAML, Table)
- Library-first architecture

## Before vs After

**Before: 50 Lines of Bash**
```bash
curl + jq + while loops + manual polling + text parsing
```

**After: One Simple Command**
```bash
# Clean CLI with first-class parameters (v0.6.6+)
redisctl cloud database create \
  --subscription 12345 \
  --name mydb \
  --memory 1 \
  --wait

# Or use JSON for complex configs
redisctl cloud database create \
  --subscription 12345 \
  --data '{"name": "mydb", "memoryLimitInGb": 1}' \
  --wait
```

## Key Benefits

**Support Engineers:**
- Remote cluster management (no SSH)
- Support package automation (10 min to 30 sec)
- Scriptable diagnostics

**DevOps Teams:**
- CI/CD integration (JSON output)
- Multi-cluster management (profiles)
- Automation-friendly

**Developers:**
- Reusable libraries
- Type-safe API clients
- Build custom tools

## Metrics

- 50+ API handlers (21 Cloud + 29 Enterprise)
- 225 comprehensive CLI tests (+217% coverage increase)
- 85%+ test coverage
- First-class parameters for common operations
- Cross-platform (macOS, Linux, Windows)
- v0.6.6 released and actively maintained

## The Impact

**One command replaces 50 lines of fragile bash**

```bash
redisctl cloud subscription list
redisctl enterprise database list
redisctl enterprise support-package cluster --upload
```

---

**Previous:** [1. The Problem](./01-problem.md)  
**Next:** [3. Installation & Setup](./03-setup.md)
