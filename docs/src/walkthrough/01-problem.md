# 1. The Problem

**Before redisctl: How did we manage Redis deployments?**

## Redis Cloud

**What Exists:**
- Web UI - Point and click (not scriptable)
- Terraform Provider - Good for IaC, not ad-hoc operations
- REST API - Documented but no tooling around it

**The Gap:**
- No CLI for day-to-day operations
- No way to script common tasks
- Must use UI or write custom bash scripts

## Redis Enterprise

**What Exists:**
- **rladmin** - Powerful but limited
  - Must SSH to cluster nodes
  - Text output (hard to parse)
  - Not cross-platform (Linux only on nodes)
  - Single cluster at a time
- **REST API** - Large (~100+ endpoints), poorly documented
  - Manual JSON construction
  - No official tooling

**The Gap:**
- No remote management CLI
- No automation-friendly tools
- No multi-cluster support

## The Reality

**What everyone actually does:**

```bash
# The "best practice" before redisctl
curl -X POST https://api.redislabs.com/v1/subscriptions \
  -H "x-api-key: $KEY" \
  -H "x-api-secret-key: $SECRET" \
  -H "Content-Type: application/json" \
  -d '{"name":"prod","cloudProviders":[...]}'

# Then poll for completion
while true; do
  STATUS=$(curl -s https://api.redislabs.com/v1/tasks/$TASK_ID | jq -r '.status')
  if [ "$STATUS" = "completed" ]; then break; fi
  sleep 2
done
```

## Problems

1. No type safety - Typos cause runtime failures
2. Manual JSON - Error-prone, hard to maintain
3. Polling loops - Fragile, need manual error handling
4. Credential exposure - API keys in shell history
5. Not portable - Requires bash, curl, jq
6. No progress feedback - Silent failures

## The Core Problem

**Redis had ZERO command-line tools for Cloud or Enterprise management**

---

**Next:** [2. Enter redisctl](./02-solution.md)
