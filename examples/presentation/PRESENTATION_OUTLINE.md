# redisctl Presentation Outline

**Duration:** 20-25 minutes + Q&A
**Audience:** Technical team (engineers, architects)
**Goal:** Introduce redisctl as the first CLI tool for Redis management

---

## 1. Current State of Redis Management (3 min)

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

## 5. Advanced Features (3 min)

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

## 6. rladmin Comparison (2 min)

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

## 7. Library Architecture (2 min)

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

## 8. Metrics & Status (1 min)

### Current State

- ✅ **50+ API handlers** (21 Cloud + 29 Enterprise)
- ✅ **85%+ test coverage** (production quality)
- ✅ **4 crates published** to crates.io
- ✅ **Cross-platform** (macOS, Linux, Windows)
- ✅ **Docker images** on ghcr.io
- ✅ **Homebrew tap** for easy installation

### Release Status

- **Current version:** v0.6.5
- **Automated releases:** GitHub Actions + cargo-dist
- **Distribution:** crates.io, GitHub releases, Docker Hub, Homebrew

---

## 9. Demo Time (3 min)

**Run:** `examples/presentation/03-demo-workflow.sh`

Showcases:
1. Profile management
2. Cluster operations
3. Database management
4. Structured output + JMESPath
5. Support package generation
6. Raw API access

---

## 10. Roadmap & Future (2 min)

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

## 11. Call to Action (1 min)

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
- v0.6.5, used in production
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

## Key Takeaways (Summary Slide)

1. **First CLI tool** for Redis Cloud and Enterprise
2. **Eliminates fragile scripts** - One command vs 50 lines of bash
3. **Four-layer architecture** - Raw API → Human-friendly → Workflows → Tools
4. **Production ready** - 85%+ coverage, cross-platform, v0.6.5
5. **Library-first** - Foundation for Redis Rust ecosystem
6. **Automation-friendly** - JSON output, JMESPath, profiles
7. **Support tools** - 30-second support packages vs 10+ minutes

**redisctl: Making Redis operations scriptable, automatable, and enjoyable** 🚀
