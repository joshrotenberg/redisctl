# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**redisctl** is a unified CLI tool for managing both Redis Cloud and Redis Enterprise deployments through their REST APIs. A single binary that intelligently routes commands to the appropriate backend based on configuration profiles.

## Architecture

### Workspace Structure
```
redisctl/
├── crates/
│   ├── redis-cloud/         # Cloud API client library
│   ├── redis-enterprise/    # Enterprise API client library
│   └── redisctl/           # Unified CLI application
├── tests/integration/       # Integration tests
├── docs/                    # mdBook documentation
└── examples/               # Usage examples
```

### Key Components
- **redis-cloud**: Cloud API client with handlers for subscriptions, databases, users, backups, ACLs, peering (100% test coverage)
- **redis-enterprise**: Enterprise API client with handlers for clusters, bdbs, nodes, users, modules, stats (100% test coverage)
- **redisctl**: Main CLI with smart routing logic in `router.rs`, profile management, deployment detection

### CLI Architecture (Three-Tier Design)

1. **Raw API Access** (`api` command) - Direct REST API calls with auth handling
   - `redisctl enterprise api GET /v1/bdbs`
   - `redisctl cloud api POST /subscriptions/123/databases --data @db.json`

2. **Human-Friendly Commands** - Single API call wrappers with nice output
   - `redisctl cloud database list`
   - `redisctl enterprise cluster info`

3. **Workflow Commands** - Multi-step operations with orchestration (planned, see issues #82-85)

## Development Commands

### Building
```bash
# Build all binaries
cargo build --release

# Build specific binary
cargo build --release --bin redisctl
cargo build --release --features cloud-only --bin redis-cloud  
cargo build --release --features enterprise-only --bin redis-enterprise

# Run in development
cargo run -- --help
```

### Testing & Linting
```bash
# Run all tests
cargo test --workspace --all-features

# Run specific package tests
cargo test --package redis-cloud
cargo test --package redis-enterprise  

# Run single test
cargo test test_cloud_config_default

# Linting (MUST pass before committing)
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Pre-commit hooks (recommended)
./scripts/install-hooks.sh  # one-time setup
pre-commit run --all-files  # run manually
```

### Docker Development Environment
```bash
# Start Redis Enterprise cluster
make docker-up

# Quick test against running cluster
make quick-test

# Run CLI in Docker
make docker-cli

# Run integration tests
make docker-test

# Clean up
make docker-down
```

## Key Implementation Details

### Command Routing Logic (router.rs)
- Smart commands (`database`, `user`, `cluster`, `account`) auto-detect deployment type from profile
- Explicit commands (`cloud`, `enterprise`) bypass detection
- Profile resolution: CLI flag > env var > default profile
- Router maps commands to either `commands/cloud.rs` or `commands/enterprise.rs`

### API Authentication
#### Cloud API
- Headers: `x-api-key` and `x-api-secret-key`
- Base URL: `https://api.redislabs.com/v1`
- Database IDs: Format `subscription_id:database_id`

#### Enterprise API  
- Authentication: Basic auth with username/password
- Base URL: `https://cluster:9443` (configurable)
- Database IDs: Simple numeric IDs
- SSL: Option to skip verification with `--insecure` flag

### Profile Management
- Storage locations:
  - Linux: `~/.config/redisctl/config.toml`
  - macOS: `~/Library/Application Support/com.redis.redisctl/config.toml`  
  - Windows: `%APPDATA%\redis\redisctl\config.toml`
- Environment variables override profile settings
- Default profile: `redisctl profile default <name>`

### Error Handling Pattern
- Libraries (`redis-cloud`, `redis-enterprise`): Use `thiserror` for typed errors
- CLI (`redisctl`): Use `anyhow` for user-friendly error messages
- All handlers return `Result<Value>` for consistent JSON output

### Output Formatting
- Formats: JSON (default), YAML, Table
- JMESPath queries: `-q` flag
- Table format: `comfy-table` for pretty printing
- All commands support `--output` or `-o` flag

## Common Development Tasks

### Adding a New Command
1. Define command struct in `crates/redisctl/src/cli.rs`
2. Add handler in appropriate module:
   - Cloud: `crates/redisctl/src/commands/cloud.rs`
   - Enterprise: `crates/redisctl/src/commands/enterprise.rs`
3. Update router in `crates/redisctl/src/router.rs` if it's a smart-routed command
4. Add API client method in library crate (`redis-cloud` or `redis-enterprise`)
5. Add tests in library's test module using wiremock for mocking

### Adding a New API Endpoint
1. Define request/response types in library's `types.rs`
2. Implement client method in library's `client.rs`
3. Add handler module if needed (e.g., `handlers/databases.rs`)
4. Write tests with wiremock mocking the API response
5. Update CLI to expose the new functionality

### Testing Strategy
- Unit tests: In library test modules (`tests/` directory)
- Integration tests: In `tests/integration/` directory
- API mocking: Use `wiremock` for HTTP response mocking
- Docker tests: Full E2E tests against real Enterprise cluster

## Feature Flags
- `default = ["full"]`: Includes both cloud and enterprise
- `cloud-only`: Builds redis-cloud binary only (smaller size)
- `enterprise-only`: Builds redis-enterprise binary only (smaller size)

## API Coverage Status

### Redis Cloud API
- **Coverage**: 95%+ of REST API endpoints implemented
- **Handlers**: 21 total (Account, ACL, Backup, Database, Subscription, Peering, etc.)
- **Test Coverage**: 12/21 handlers have dedicated test files
- **Extended Features**: API Keys, Billing, CRDB/Active-Active, SSO/SAML

### Redis Enterprise API
- **Coverage**: 100% of documented REST API endpoints implemented
- **Handlers**: 29 total (Actions, Alerts, BDB, Cluster, Nodes, Users, etc.)
- **Test Coverage**: 22/29 handlers have dedicated test files
- **Complete CRUD**: All major endpoint categories covered

## Dependencies
- Core: `tokio`, `serde`, `reqwest`, `clap`
- Output: `comfy-table`, `serde_yaml`, `jmespath`
- Config: `config`, `toml`, `directories`
- Testing: `wiremock`, `mockall`, `serial_test`
- Auth: `rpassword` for password input

## CI/CD
- GitHub Actions workflow in `.github/workflows/ci.yml`
- Tests run on: Ubuntu, macOS, Windows
- Required checks: formatting, clippy, all tests
- Release workflow creates platform-specific binaries
- Pre-commit hooks available (see `.pre-commit-config.yaml`)

## Project Standards
- Rust 2024 edition
- Minimum 70% test coverage goal
- All public APIs must have doc comments
- Conventional commits: `feat:`, `fix:`, `docs:`, `test:`, `chore:`, etc.
- Feature branch workflow (never commit directly to main)
- No emoji in code, commits, or documentation
- always squash commits on a branch/pr