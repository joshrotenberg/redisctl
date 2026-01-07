# Architecture

How redisctl is structured and designed.

## Four-Layer Design

```
┌─────────────────────────────────────┐
│           Workflows                 │  Multi-step orchestration
├─────────────────────────────────────┤
│         Human Commands              │  Type-safe wrappers
├─────────────────────────────────────┤
│           Raw API                   │  Direct REST access
├─────────────────────────────────────┤
│           Profiles                  │  Credential management
└─────────────────────────────────────┘
```

| Layer | Purpose | Example |
|-------|---------|---------|
| **Profiles** | Credential management | `redisctl profile set prod` |
| **Raw API** | Direct REST access | `redisctl api cloud get /subscriptions` |
| **Human Commands** | Type-safe wrappers | `redisctl cloud database list` |
| **Workflows** | Multi-step operations | `redisctl cloud workflow subscription-setup` |

## Workspace Structure

```
redisctl/
├── crates/
│   ├── redisctl-config/     # Profile and credential management
│   ├── redis-cloud/         # Cloud API client library
│   ├── redis-enterprise/    # Enterprise API client library
│   └── redisctl/            # CLI application
│       ├── src/commands/    # Command implementations
│       ├── src/workflows/   # Multi-step workflows
│       └── tests/           # CLI tests
├── mkdocs-site/             # Documentation (you're reading it)
└── scripts/                 # Build automation
```

## Library-First Design

The CLI is a thin layer over reusable libraries:

```rust
// redis-cloud crate
let client = RedisCloudClient::new(api_key, secret_key);
let subscriptions = client.subscriptions().list().await?;

// redis-enterprise crate
let client = RedisEnterpriseClient::new(url, user, password);
let databases = client.databases().list().await?;
```

This enables:
- Terraform providers
- Backup tools
- Migration scripts
- Monitoring dashboards
- Custom automation

## API Client Architecture

### Redis Cloud

- **Auth:** `x-api-key` and `x-api-secret-key` headers
- **Base URL:** `https://api.redislabs.com/v1`
- **Async operations:** Most operations return task IDs

### Redis Enterprise

- **Auth:** Basic auth (username/password)
- **Base URL:** `https://cluster:9443/v1`
- **TLS:** Self-signed certs common (`--insecure`)

## Error Handling

- **Libraries:** Use `thiserror` for typed errors
- **CLI:** Use `anyhow` for context-rich messages

```rust
// Library error
#[derive(thiserror::Error, Debug)]
pub enum CloudError {
    #[error("API error: {0}")]
    Api(String),
    #[error("Authentication failed")]
    AuthFailed,
}

// CLI wraps with context
let result = client.databases().list().await
    .context("Failed to list databases")?;
```

## Output System

All commands support multiple output formats:

```rust
match output_format {
    OutputFormat::Table => print_table(&data),
    OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&data)?),
    OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&data)?),
}
```

JMESPath queries are applied before formatting.

## Async Operations

Centralized handling for Cloud API async operations:

```rust
pub async fn handle_async_response(
    client: &CloudClient,
    response: Value,
    wait: bool,
    timeout: Duration,
) -> Result<Value> {
    if !wait {
        return Ok(response);
    }

    let task_id = response["taskId"].as_str()?;
    poll_until_complete(client, task_id, timeout).await
}
```

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `reqwest` | HTTP client |
| `clap` | CLI parsing |
| `serde` | Serialization |
| `jmespath` | Query filtering |
