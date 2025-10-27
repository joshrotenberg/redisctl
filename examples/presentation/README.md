# redisctl Presentation Demo Scripts

Demo scripts for team presentation on redisctl - the first CLI tool for Redis Cloud and Enterprise.

## Setup

1. **Start Docker environment**:
   ```bash
   cd ../../  # Back to repo root
   docker compose up -d
   ```

2. **Wait for initialization** (about 30 seconds):
   ```bash
   docker compose logs -f redis-enterprise-init
   ```

3. **Verify connection**:
   ```bash
   cargo run -- enterprise cluster get -o json -q 'name'
   ```

## Demo Scripts

### 01-before-redisctl.sh

Shows the painful reality BEFORE redisctl:
- Complex curl commands with authentication
- Manual JSON construction
- jq parsing gymnastics
- Polling loops for async operations
- Credential exposure on command line
- No error handling or progress indicators

**Run**:
```bash
./01-before-redisctl.sh
```

**Key Message**: "This is what everyone had to do before redisctl existed"

### 02-after-redisctl.sh

Shows the elegant solution AFTER redisctl:
- Simple, readable commands
- Type-safe API calls
- Automatic async operation handling
- Secure credential storage
- Structured output (JSON, YAML, Table)
- Built-in JMESPath query support

**Run**:
```bash
./02-after-redisctl.sh
```

**Key Message**: "One command replaces 50 lines of fragile bash"

### 03-demo-workflow.sh

Complete demo showcasing all key features:
- Profile management
- Cluster information
- Database operations
- Structured output + JMESPath
- Support package automation
- Raw API access
- Library-first architecture

**Run**:
```bash
./03-demo-workflow.sh
```

**Key Message**: "Production-ready tool that enables the entire Redis Rust ecosystem"

## Presentation Flow

### 1. Introduction (2 min)
- The problem: Redis has ZERO CLI tooling
- The solution: redisctl - first-class CLI
- The impact: Foundation for Redis Rust ecosystem

### 2. The "Before" (3 min)
- Run `01-before-redisctl.sh`
- Highlight the pain points
- Emphasize manual polling loops

### 3. The "After" (3 min)
- Run `02-after-redisctl.sh`
- Show the contrast
- Emphasize simplicity and safety

### 4. Live Demo (8 min)
- Run `03-demo-workflow.sh`
- Highlight key features:
  - Profile management
  - Structured output
  - Support packages
  - Library architecture

### 5. Architecture Deep Dive (3 min)
- Library-first design
- `redisctl-config` extraction
- Future: workflows + output libraries
- Enables: Terraform, backup tools, monitoring

### 6. Q&A (5 min)
- Why Rust? Type safety, async, performance
- Windows support? Yes, cross-platform
- When available? Now! v0.6.5 released
- How to contribute? GitHub, issues welcome

## Key Metrics to Emphasize

- ✅ **FIRST** CLI tool for Redis Cloud and Enterprise
- ✅ 50+ API handlers (comprehensive coverage)
- ✅ 85%+ test coverage (production quality)
- ✅ 21 Cloud + 29 Enterprise endpoints
- ✅ Support packages: 10+ minutes → 30 seconds
- ✅ Library architecture: Reusable components

## Demo Environment Details

The Docker environment provides:
- Redis Enterprise cluster (localhost:9443)
- 3 test databases (default-db, persistent-db, cache-db)
- Full API access for testing
- Credentials: admin@redis.local / Redis123!

## Troubleshooting

**Docker not responding?**
```bash
docker compose down -v
docker compose up -d
docker compose logs -f
```

**Connection refused?**
```bash
# Wait for healthy status
docker compose ps
# Should show "healthy" status
```

**Profile not found?**
```bash
# Check profile configuration
redisctl profile list

# Set default Enterprise profile
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_USER="admin@redis.local"
export REDIS_ENTERPRISE_PASSWORD="Redis123!"
export REDIS_ENTERPRISE_INSECURE="true"
```

## Post-Presentation

After the presentation:
1. Gather feedback
2. Update issues with team suggestions
3. Plan next steps based on interest
4. Share recording and slides

## Resources

- GitHub: https://github.com/joshrotenberg/redisctl
- Issues: https://github.com/joshrotenberg/redisctl/issues
- Docs: https://docs.rs/redisctl
- Releases: https://github.com/joshrotenberg/redisctl/releases
