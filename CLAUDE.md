# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**redisctl** is a unified CLI tool for managing both Redis Cloud and Redis Enterprise deployments through their REST APIs. A single binary that intelligently routes commands to the appropriate backend based on configuration profiles.

## Architecture

### Workspace Structure
```
redisctl/
├── crates/
│   ├── redis-common/        # Shared utilities (config, output, errors)
│   ├── redis-cloud/         # Cloud API client library
│   ├── redis-enterprise/    # Enterprise API client library
│   └── redisctl/           # Unified CLI application
├── tests/integration/       # Integration tests
├── docs/                    # mdBook documentation
└── examples/               # Usage examples
```

### Key Crates
- **redis-common**: Shared utilities for config, output formatting (JSON/YAML/Table), JMESPath queries, errors
- **redis-cloud**: Cloud API client with handlers for subscriptions, databases, users, backups, ACLs, peering
- **redis-enterprise**: Enterprise API client with handlers for clusters, bdbs, nodes, users, modules, stats
- **redisctl**: Main CLI with smart routing logic in `router.rs`, profile management, deployment detection

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
cargo run -- profile list
```

### Testing & Linting
```bash
# Run all tests
cargo test --workspace
cargo test --all-features

# Run specific package tests
cargo test --package redis-cloud
cargo test --package redis-enterprise  
cargo test --package redisctl

# Run single test
cargo test test_cloud_config_default

# Linting (must pass before committing)
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Quick validation before commit
make pre-commit  # runs fmt, test, clippy
```

### Docker Development Environment
```bash
# Start Redis Enterprise cluster
make docker-up

# Run CLI in Docker
make docker-cli

# Run integration tests
make docker-test
make docker-integration

# Create example databases
make docker-examples

# Clean up
make docker-down

# Quick test against running cluster
make quick-test
```

## Key Implementation Details

### Command Routing Logic (router.rs)
- Smart commands (`database`, `user`, `cluster`, `account`) auto-detect deployment type from profile
- Explicit commands (`cloud`, `enterprise`) bypass detection
- Ambiguous commands require `--deployment` flag or explicit routing
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
- Default profile can be set with `redisctl profile default <name>`

### Error Handling Pattern
- Libraries (`redis-cloud`, `redis-enterprise`): Use `thiserror` for typed errors
- CLI (`redisctl`): Use `anyhow` for user-friendly error messages
- All handlers return `Result<Value>` for consistent JSON output
- API errors are wrapped with context about the failed operation

### Output Formatting
- Formats: JSON (default), YAML, Table
- JMESPath queries supported with `-q` flag
- Table format uses `comfy-table` for pretty printing
- All commands support `--output` or `-o` flag

## Common Development Tasks

### Adding a New Command
1. Define command struct in `crates/redisctl/src/cli.rs` (e.g., `DatabaseCommands`)
2. Add handler in appropriate module:
   - Cloud: `crates/redisctl/src/commands/cloud.rs`
   - Enterprise: `crates/redisctl/src/commands/enterprise.rs`
3. Update router in `crates/redisctl/src/router.rs` if it's a smart-routed command
4. Add API client method in library crate (`redis-cloud` or `redis-enterprise`)
5. Add tests in `src/lib_tests.rs` using wiremock for mocking

### Adding a New API Endpoint
1. Define request/response types in library's `types.rs`
2. Implement client method in library's `client.rs`
3. Add handler module if needed (e.g., `handlers/databases.rs`)
4. Write tests with wiremock mocking the API response
5. Update CLI to expose the new functionality

### Testing Strategy
- Unit tests: In `lib_tests.rs` for each crate
- Integration tests: In `tests/integration/` directory
- API mocking: Use `wiremock` for HTTP response mocking
- Docker tests: Full E2E tests against real Enterprise cluster
- Test commands: `cargo test`, `make docker-test`

### CI/CD Workflow
- GitHub Actions runs on all PRs
- Tests matrix: Ubuntu, macOS, Windows
- Checks: formatting, clippy, all tests
- Release workflow creates binaries for all platforms

## Feature Flags
- `default = ["full"]`: Includes both cloud and enterprise
- `cloud-only`: Builds redis-cloud binary only (smaller size)
- `enterprise-only`: Builds redis-enterprise binary only (smaller size)
- Used to create platform-specific binaries

## API Coverage Status

### Redis Cloud API
- ✅ Subscriptions (list, get, create, update, delete)
- ✅ Databases (full CRUD operations)
- ✅ Cloud Accounts (AWS, GCP, Azure integration)
- ✅ Users & ACLs
- ✅ Backup & Import
- ✅ VPC Peering
- ✅ Transit Gateway
- 🚧 Active-Active databases
- 🚧 SAML SSO configuration

### Redis Enterprise API
- ✅ Cluster management
- ✅ Database (BDB) operations
- ✅ Users & roles
- ✅ Modules management
- ✅ Bootstrap & initialization
- ✅ Backup & restore
- 🚧 CRDB (Active-Active)
- 🚧 LDAP integration
- 🚧 Certificates (OCSP)

## Dependencies
- Core: `tokio`, `serde`, `reqwest`, `clap`
- Output: `comfy-table`, `serde_yaml`, `jmespath`
- Config: `config`, `toml`, `directories`
- Testing: `wiremock`, `mockall`, `serial_test`
- Auth: `rpassword` for password input

## Next Session Starting Points

**Priority Focus:**
1. **API Completeness Audit**: Verify Enterprise and Cloud API libraries have complete endpoint coverage
2. **Testing Coverage**: Ensure all API endpoints are properly tested  
3. **CLI Raw Access**: Verify basic/raw CLI access to both APIs is complete and tested
4. **Documentation**: Update mdBook docs with latest features

**Session Checklist:**
1. Read this CLAUDE.md file
2. Check open PRs and merge if ready
3. Run `cargo test --workspace` to ensure everything is working
4. Check for outdated dependencies: `cargo outdated`
5. Review GitHub issues for priority tasks