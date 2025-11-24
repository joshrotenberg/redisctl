# Architecture

redisctl is built as a collection of reusable Rust libraries with a thin CLI layer on top.

## Workspace Structure

```
redisctl/
├── crates/
│   ├── redisctl-config/     # Profile and credential management
│   ├── redis-cloud/         # Cloud API client library
│   ├── redis-enterprise/    # Enterprise API client library
│   └── redisctl/           # CLI application
└── docs/                   # mdBook documentation
```

## Library Layers

### redisctl-config

Profile and credential management:
- Configuration file parsing
- Secure credential storage (OS keyring)
- Environment variable handling

```rust
use redisctl_config::{Config, Profile};

let config = Config::load()?;
let profile = config.get_profile("production")?;
```

### redis-cloud

Redis Cloud API client:
- 21 handler modules
- 95%+ API coverage
- Async/await support

```rust
use redis_cloud::CloudClient;

let client = CloudClient::new(api_key, api_secret)?;
let subscriptions = client.subscriptions().list().await?;
```

### redis-enterprise

Redis Enterprise API client:
- 29 handler modules
- 100% API coverage
- Support for binary responses (debug info, support packages)

```rust
use redis_enterprise::EnterpriseClient;

let client = EnterpriseClient::new(url, username, password)?;
let cluster = client.cluster().get().await?;
```

### redisctl

CLI application:
- Command parsing (clap)
- Output formatting
- Workflow orchestration

## Design Principles

### Library-First
The API clients are independent libraries that can be used by other tools (Terraform providers, monitoring dashboards, etc.).

### Type-Safe
All API responses are deserialized into Rust structs, catching errors at compile time.

### Handler Pattern
Each API resource has a handler module with methods for CRUD operations:

```rust
// Handler pattern
client.databases().list().await?;
client.databases().get(id).await?;
client.databases().create(config).await?;
```

### Async Operations
Built on Tokio for async I/O. Long-running operations return task IDs with optional polling.

## Error Handling

- **Libraries**: Use `thiserror` for typed errors
- **CLI**: Use `anyhow` for context-rich messages

```rust
// Library error
#[derive(Error, Debug)]
pub enum CloudError {
    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Authentication failed")]
    Auth,
}

// CLI error handling
let result = client.databases().get(id).await
    .context("Failed to fetch database")?;
```

## Output System

Three-tier output formatting:
1. **JSON** (default) - Machine-readable
2. **YAML** - Human-readable structured
3. **Table** - Human-readable tabular

JMESPath queries filter output before formatting.

## Future Libraries

Planned extractions:
- `redisctl-workflows` - Reusable workflow orchestration
- `redisctl-output` - Consistent output formatting
