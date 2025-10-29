# 8. Library Architecture

**redisctl isn't just a CLI - it's a platform**

## Current Structure

```
redisctl/
├── redisctl-config         # Profile & credential management
├── redis-cloud             # Cloud API client (21 handlers)
├── redis-enterprise        # Enterprise API client (29 handlers)
└── redisctl                # CLI binary (thin orchestration layer)
```

## Library-First Design

Each component is a **reusable library**:

### redisctl-config (v0.1.0)
- Profile management
- Credential storage (keyring)
- Environment variable expansion
- Platform-specific paths

**Use case:** Other tools can reuse our profile system

### redis-cloud
- 21 API handler modules
- 95%+ test coverage
- Type-safe request/response types
- Comprehensive error handling

**Use case:** Build Cloud management tools

### redis-enterprise
- 29 API handler modules
- 100% test coverage
- Streaming support
- Binary response handling

**Use case:** Build Enterprise management tools

## Recent Evolution

**Just extracted:** `redisctl-config` library

Before:
```
redisctl (monolithic binary)
```

After:
```
redisctl-config (library) reusable!
└── redisctl (uses library)
```

## Future Extractions (Issue #411)

**Planned:**
- `redisctl-workflows` - Orchestration library
- `redisctl-output` - Formatting utilities

This enables **any tool** to use our battle-tested components.

## Example: Using Libraries

```rust
use redisctl_config::Config;
use redis_enterprise::Client;

// Load profile
let config = Config::load()?;
let profile = config.get_profile("production")?;

// Create client
let client = Client::new(
    &profile.url,
    &profile.username,
    &profile.password
)?;

// Use API
let databases = client.database().list().await?;
```

## Enables the Ecosystem

### Terraform Provider
```rust
// terraform-provider-redis
use redis_enterprise::Client;
use redisctl_workflows::InitCluster;

// Reuse our battle-tested code
```

### Backup Tool
```rust
// redis-backup
use redisctl_config::Config;
use redis_enterprise::Client;

// Multi-cluster backups using profiles
```

### Monitoring Dashboard
```rust
// redis-monitor
use redis_enterprise::Client;

// Collect metrics from all clusters
```

### Custom Automation
```rust
// your-tool
use redis_cloud::Client;
use redisctl_workflows::SubscriptionSetup;

// Build custom workflows
```

## Why This Matters

**Not just a CLI** - Foundation for entire Redis Rust ecosystem

Terraform providers can use our API clients  
Monitoring tools can use our libraries  
Backup tools can use our workflows  
Custom tools can use proven components  

## Platform Vision

```
redisctl (the CLI)
    ↓
redisctl libraries
    ↓
Redis Rust Ecosystem
    ↓
Better tools for everyone
```

---

**Previous:** [7. Advanced Features](./07-advanced.md)  
**Next →** [9. Next Steps](./09-next-steps.md)

See [Developer Guide](../developer/library-usage.md) for details
