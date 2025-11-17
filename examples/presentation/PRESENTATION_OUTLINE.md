# redisctl Presentation Outline

**Duration:** 20-25 minutes + Q&A
**Audience:** Technical team (engineers, architects)
**Goal:** Introduce redisctl as the first CLI tool for Redis management

> **📖 Primary Presentation Source:** Use `docs/src/walkthrough/` (mdBook) for the actual presentation.  
> This outline provides speaker notes, timing, and Q&A preparation.  
> The walkthrough is navigable, polished, and keeps content synced in one place.

---

## 1. Current State of Redis Management (3 min)

> **📖 Walkthrough:** [1. The Problem](../../docs/src/walkthrough/01-problem.md)

### Redis Cloud
- REST API exists but not widely used directly
- Primary interfaces:
  - **Web UI** - Point and click (not scriptable)
  - **Terraform Provider** - Good for IaC, not ad-hoc ops
- **Gap:** No CLI for day-to-day operations

### Redis Enterprise
- **rladmin** - Powerful but node-local CLI
  - Must SSH to cluster nodes
  - Text output (hard to parse)
  - Not cross-platform
  - Single cluster at a time
- **REST API** - Large, complex, poorly documented
  - ~100+ endpoints
  - Manual JSON construction
  - No tooling around it
- **Gap:** No remote CLI, no automation-friendly tool

### The Problem
**Before redisctl:**
- Support engineers → Manual UI clicking
- DevOps teams → Write fragile bash + curl + jq scripts
- Customers → Can't automate without Terraform
- Everyone → Reinvents the same wheel

**Demo:** Show `examples/presentation/01-before-redisctl.sh`
- Highlight painful curl + polling loops
- Emphasize manual JSON construction
- Show credential exposure

---

## 2. Enter redisctl (2 min)

> **📖 Walkthrough:** [2. Enter redisctl](../../docs/src/walkthrough/02-solution.md)

**The FIRST command-line tool for both Redis Cloud and Redis Enterprise**

### Key Value Propositions
- ✅ **Type-safe API clients** - No more parsing curl output
- ✅ **Async operation handling** - Automatic polling with `--wait`
- ✅ **Support package automation** - 10+ minutes → 30 seconds
- ✅ **Profile management** - Secure, multi-cluster
- ✅ **Structured output** - JSON, YAML, Table
- ✅ **Library-first** - Reusable components for ecosystem

**Demo:** Show `examples/presentation/02-after-redisctl.sh`
- One command vs 50 lines of bash
- Emphasize simplicity and safety

---

## 3. Installation & Setup (2 min)

> **📖 Walkthrough:** [3. Installation & Setup](../../docs/src/walkthrough/03-setup.md)

### Installation

```bash
# macOS/Linux
brew install joshrotenberg/brew/redisctl

# Or download binaries from GitHub releases
# Or use Docker
docker run ghcr.io/joshrotenberg/redisctl:latest --help
```

### Profile Setup

**Why Profiles?**
- Manage multiple clusters (dev, staging, prod)
- Secure credential storage (OS keyring)
- Override hierarchy: CLI flags → Env vars → Profile

**Quick Setup:**
```bash
# Cloud profile
redisctl profile set prod-cloud \
  --deployment-type cloud \
  --api-key $KEY \
  --api-secret $SECRET \
  --use-keyring

# Enterprise profile
redisctl profile set prod-cluster \
  --deployment-type enterprise \
  --url https://cluster.example.com:9443 \
  --username admin@cluster.local \
  --use-keyring

# List profiles
redisctl profile list
```

---

## 4. Four-Layer Architecture (5 min)

> **📖 Walkthrough:** [4. Raw API](../../docs/src/walkthrough/04-raw-api.md) → [5. Human-Friendly](../../docs/src/walkthrough/05-human-friendly.md) → [6. Workflows](../../docs/src/walkthrough/06-workflows.md) → [7. Advanced](../../docs/src/walkthrough/07-advanced.md)

### Layer 1: Raw API Access

**Use case:** Any endpoint, full control

```bash
# Direct REST calls to any endpoint
redisctl api cloud get /subscriptions
redisctl api cloud post /subscriptions/123/databases --data '{...}'

# Enterprise examples
redisctl api enterprise get /v1/cluster
redisctl api enterprise get /v1/bdbs
```

**When to use:**
- Endpoint not wrapped yet
- Need exact API behavior
- Testing/debugging

### Layer 2: Human-Friendly Commands

**Use case:** Day-to-day operations, better UX

```bash
# Cloud operations
redisctl cloud subscription list -o table
redisctl cloud database get 12345 -o json

# Enterprise operations  
redisctl enterprise cluster get
redisctl enterprise database list -o table
redisctl enterprise node list
```

**Benefits:**
- Type-safe parameters
- Automatic error handling
- Progress indicators
- Better output formatting

**Live Demo:**
```bash
# Show table output
redisctl enterprise database list -o table

# Show JSON with JMESPath filtering
redisctl enterprise database list \
  -q "[?status=='active'].{name:name,memory:memory_size}"

# Show database details
redisctl enterprise database get 1
```

### Layer 3: Workflows

**Use case:** Multi-step orchestrated operations

**Example: Cluster Initialization**
```bash
# Before: 10+ manual steps
# 1. rladmin cluster create
# 2. Set admin password
# 3. Accept license
# 4. Create default database
# 5. Verify everything
# ...

# With redisctl: One command
redisctl enterprise workflow init-cluster \
  --cluster-name "production" \
  --username "admin@cluster.local" \
  --password "SecurePassword123"

# Automatically handles:
# ✅ Bootstrap cluster
# ✅ Set credentials
# ✅ Accept license
# ✅ Create default DB
# ✅ Verify health
```

**Available Workflows:**
- `init-cluster` - Complete cluster setup
- `subscription-setup` (Cloud) - End-to-end subscription creation

**Future:** More workflows based on common patterns

### Layer 4: Specialized Tools

**Support Package Automation** (The killer feature)

**Before redisctl:**
1. SSH to cluster node (1 min)
2. Run `rladmin cluster debug_info` (2 min)
3. Find generated file (30 sec)
4. SCP to local machine (1 min)
5. Open Redis Support portal (1 min)
6. Click through upload form (3 min)
7. Wait for upload (2+ min)
**Total: 10+ minutes**

**With redisctl:**
```bash
# Generate, optimize, and upload in one command
redisctl enterprise support-package cluster \
  --optimize \
  --upload

# Total: 30 seconds
```

**Live Demo:**
```bash
# Show local generation
redisctl enterprise support-package cluster \
  --output ./demo-package.tar.gz

# Show optimized size
redisctl enterprise support-package cluster \
  --optimize \
  --output ./demo-package-small.tar.gz

# Compare sizes
ls -lh demo-package*.tar.gz
```

---

## 5. Recent UX Improvements (3 min)

> **📖 Walkthrough:** [5. Human-Friendly Commands](../../docs/src/walkthrough/05-human-friendly.md) (First-Class Parameters section)

### First-Class Parameters (NEW in v0.6.6)

**The Problem with JSON strings:**
- Easy to make syntax errors (missing quotes, commas, braces)
- No tab completion
- Hard to remember exact field names
- Verbose for simple operations

**Before: Complex JSON Required**

```bash
# Cloud database - the old way
redisctl cloud database create --subscription 123 \
  --data '{"name":"mydb","memoryLimitInGb":1,"protocol":"redis","replication":true,"dataPersistence":"aof","dataEvictionPolicy":"volatile-lru"}'

# Enterprise database - the old way  
redisctl enterprise database create \
  --data '{"name":"prod-db","memory_size":2147483648,"replication":true,"persistence":"aof","eviction_policy":"volatile-lru"}'
```

**After: Clean CLI Parameters**

```bash
# Cloud database - the new way (70% less typing!)
redisctl cloud database create --subscription 123 \
  --name mydb --memory 1 --replication \
  --data-persistence aof --eviction-policy volatile-lru

# Enterprise database - the new way
redisctl enterprise database create \
  --name prod-db --memory 2147483648 --replication \
  --persistence aof --eviction-policy volatile-lru

# Cloud subscription with smart defaults
redisctl cloud subscription create \
  --name prod-subscription \
  --payment-method marketplace \
  --memory-storage ram-and-flash \
  --data @subscription.json  # Still supports complex nested config
```

**Key Improvements:**
- ✅ **70% less typing** for common operations
- ✅ **No JSON syntax errors** - CLI validates parameters
- ✅ **Tab completion** for parameter names
- ✅ **Clear error messages** - "unknown flag --nmae" vs silent JSON typo
- ✅ **Smart defaults** - protocol=redis, eviction-policy=volatile-lru
- ✅ **Backwards compatible** - `--data` still works for advanced configs
- ✅ **Parameter override** - Mix `--data` with flags for flexibility

**Live Demo:**
```bash
# Show help with all new parameters
redisctl enterprise database create --help

# Create a simple database - no JSON needed!
redisctl enterprise database create \
  --name demo-db \
  --memory 1073741824 \
  --replication \
  --dry-run

# Compare to the old way
echo '{"name":"demo-db","memory_size":1073741824,"replication":true}' | \
  redisctl enterprise database create --data @- --dry-run
```

---

## 6. Advanced Features (3 min)

### JMESPath Query Support

**Built-in filtering and transformation:**

```bash
# Extract specific fields
redisctl enterprise database list -q "[].name"

# Filter and reshape
redisctl enterprise database list \
  -q "[?status=='active'].{name:name,mem:memory_size,port:port}"

# Count databases
redisctl enterprise database list -q "length(@)"

# Complex queries
redisctl cloud subscription list \
  -q "[?status=='active' && cloudProviders[0].provider=='AWS'].name"
```

### Output Formats

```bash
# JSON for scripts/pipelines
redisctl enterprise database list -o json | jq '.[] | .name'

# YAML for config files
redisctl enterprise database get 1 -o yaml > db-config.yaml

# Table for humans
redisctl enterprise database list -o table
```

### Log Streaming

```bash
# Real-time log following
redisctl enterprise logs list --follow --interval 2

# With filtering
redisctl enterprise logs list --follow -q "[?level=='error']"
```

### License Management

```bash
# Check current license
redisctl enterprise license get

# Set new license
redisctl enterprise license set --key "license-string"
```

---

## 7. rladmin Comparison (2 min)

> **📖 Walkthrough:** [Appendix: rladmin Comparison](../../docs/src/walkthrough/rladmin-comparison.md)

**Quick comparison:** (Show `RLADMIN_COMPARISON.md`)

### Where redisctl Excels Over rladmin

| Feature | rladmin | redisctl |
|---------|---------|----------|
| Remote management | ❌ Must SSH | ✅ REST API |
| Cross-platform | ❌ Linux only | ✅ Mac/Win/Linux |
| Structured output | ❌ Text parsing | ✅ JSON/YAML/Table |
| Multi-cluster | ❌ One at a time | ✅ Profile system |
| Automation | ⚠️ Shell scripting | ✅ Native JSON |
| Support packages | ⚠️ Manual upload | ✅ One command |

**Key Message:** Complementary tools
- **rladmin** for local node ops
- **redisctl** for remote management and automation

---

## 8. Library Architecture (2 min)

> **📖 Walkthrough:** [8. Library Architecture](../../docs/src/walkthrough/08-libraries.md)

**redisctl isn't just a CLI - it's a platform**

### Current Libraries

```
redisctl/
├── redisctl-config      # Profile/credential management
├── redis-cloud          # Cloud API client (21 handlers)
├── redis-enterprise     # Enterprise API client (29 handlers)
└── redisctl             # CLI binary (thin layer)
```

### Recent Evolution

**Just extracted:** `redisctl-config` (v0.1.0)
- Profile management
- Credential storage (keyring)
- Environment variable expansion
- **Use case:** Other tools can reuse our profile system

**Future:** (Issue #411)
- `redisctl-workflows` - Orchestration library
- `redisctl-output` - Formatting utilities

### Why This Matters

**Enables the ecosystem:**
- Terraform providers using our API clients
- Backup tools using our workflows
- Monitoring dashboards using our libraries
- Custom automation using proven components

**Example use case:**
```rust
use redisctl_config::Config;
use redis_enterprise::Client;

// Any Rust tool can now use our battle-tested libraries
let config = Config::load()?;
let profile = config.get_profile("prod")?;
let client = Client::from_profile(&profile)?;
```

---

## 9. Metrics & Status (1 min)

### Current State

- ✅ **50+ API handlers** (21 Cloud + 29 Enterprise)
- ✅ **225 comprehensive CLI tests** (+217% coverage increase)
- ✅ **85%+ test coverage** (production quality)
- ✅ **First-class parameters** for common operations (NEW)
- ✅ **4 crates published** to crates.io
- ✅ **Cross-platform** (macOS, Linux, Windows)
- ✅ **Docker images** on ghcr.io
- ✅ **Homebrew tap** for easy installation

### Release Status

- **Current version:** v0.6.6
- **Automated releases:** GitHub Actions + cargo-dist
- **Distribution:** crates.io, GitHub releases, Docker Hub, Homebrew

---

## 10. Demo Time (3 min)

**Run:** `examples/presentation/03-demo-workflow.sh`

Showcases:
1. Profile management
2. Cluster operations
3. Database management
4. Structured output + JMESPath
5. Support package generation
6. Raw API access

---

## 11. Roadmap & Future (2 min)

> **📖 Walkthrough:** [9. Next Steps](../../docs/src/walkthrough/09-next-steps.md)

### Near Term
- Additional workflows (see issues #263-#268)
- Enhanced streaming (metrics, events - #405)
- Library extraction (workflows, output - #411)

### Long Term
- Terraform provider (using our libraries)
- Interactive REPL mode (#186)
- Enhanced monitoring integrations
- Community contributions

---

## 12. Call to Action (1 min)

### Try It

```bash
brew install joshrotenberg/brew/redisctl
redisctl --help
```

### Resources

- **GitHub:** https://github.com/joshrotenberg/redisctl
- **Docs:** https://docs.rs/redisctl
- **Issues:** https://github.com/joshrotenberg/redisctl/issues
- **Examples:** `examples/presentation/` in repo

### Feedback Welcome

- What workflows would help you?
- What features are missing?
- How can we improve the UX?

---

## Q&A Preparation

### Anticipated Questions

**Q: Why Rust?**
- Type safety catches errors at compile time
- Excellent async/await for API operations
- Zero-cost abstractions
- Great cargo ecosystem
- Cross-platform single binary

**Q: What about the Python SDK?**
- Different use case: SDK for application integration
- redisctl is for operations/automation
- Complementary, not competing

**Q: Can this replace Terraform?**
- No, complementary
- Terraform for IaC (desired state)
- redisctl for ad-hoc operations, troubleshooting, scripts
- Future: Terraform provider could use our libraries

**Q: Windows support?**
- Yes! Builds for Windows, macOS (Intel + Apple Silicon), Linux
- Keyring works on all platforms

**Q: How stable is it?**
- v0.6.6, used in production
- 85%+ test coverage
- Comprehensive wiremock tests
- Breaking changes follow semver

**Q: Can I contribute?**
- Yes! GitHub issues and PRs welcome
- Good first issues tagged
- Architecture designed for extensibility

**Q: What's the performance like?**
- Fast - Rust performance, async operations
- Efficient - Type-safe, no runtime overhead
- Scalable - Profile system for many clusters

---

**Q: Why first-class parameters instead of just JSON?**
- 70% reduction in typing for common operations
- No JSON syntax errors (closing braces, quotes, commas)
- Tab completion and shell history work better
- Better error messages (typo in --name vs typo in JSON key)
- Still supports --data for complex configs (backwards compatible)
- Follows industry best practices (kubectl, gh, aws cli all use flags)

---

## Key Takeaways (Summary Slide)

1. **First CLI tool** for Redis Cloud and Enterprise
2. **Eliminates fragile scripts** - One command vs 50 lines of bash
3. **Clean UX** - First-class parameters, no JSON required for common ops
4. **Four-layer architecture** - Raw API → Human-friendly → Workflows → Tools
5. **Production ready** - 225 tests, 85%+ coverage, cross-platform, v0.6.6
6. **Library-first** - Foundation for Redis Rust ecosystem
7. **Automation-friendly** - JSON output, JMESPath, profiles
8. **Support tools** - 30-second support packages vs 10+ minutes

**redisctl: Making Redis operations scriptable, automatable, and enjoyable** 🚀
