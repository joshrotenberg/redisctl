# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**redisctl** is a unified CLI tool for managing both Redis Cloud and Redis Enterprise deployments through their REST APIs. A single binary that intelligently routes commands to the appropriate backend based on configuration profiles.

## Architecture

Workspace with four crates:
- **redis-common**: Shared utilities for config, output formatting (JSON/YAML/Table), JMESPath queries, errors
- **redis-cloud**: Cloud API client with handlers for subscriptions, databases, users, backups, ACLs, peering
- **redis-enterprise**: Enterprise API client with handlers for clusters, bdbs, nodes, users, modules, stats
- **redisctl**: Main CLI with smart routing logic in `router.rs`, profile management, deployment detection


## Development Commands

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

# Run single test
cargo test test_cloud_config_default
cargo test --package redis-cloud
cargo test --package redis-enterprise  
cargo test --package redisctl

# Linting (must pass before committing)
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```


## Key Implementation Details

### Command Routing Logic (router.rs)
- Smart commands (`database`, `user`, `cluster`, `account`) auto-detect deployment type from profile
- Explicit commands (`cloud`, `enterprise`) bypass detection
- Ambiguous commands require `--deployment` flag or explicit routing
- Profile resolution: CLI flag > env var > default profile

### API Authentication Differences
- **Cloud**: Headers `x-api-key` and `x-api-secret-key`  
- **Enterprise**: Basic auth with username/password
- **Cloud DB IDs**: Format `subscription_id:database_id`
- **Enterprise DB IDs**: Simple numeric IDs

### Profile Storage Locations
- Linux: `~/.config/redisctl/config.toml`
- macOS: `~/Library/Application Support/com.redis.redisctl/config.toml`  
- Windows: `%APPDATA%\redis\redisctl\config.toml`

### Error Handling Pattern
- Libraries (`redis-cloud`, `redis-enterprise`): Use `thiserror` for typed errors
- CLI (`redisctl`): Use `anyhow` for user-friendly error messages
- All handlers return `Result<Value>` for consistent JSON output

## Common Tasks

### Adding a New Command
1. Define command struct in `cli.rs` (e.g., `DatabaseCommands`)
2. Add handler in appropriate module (`commands/cloud.rs` or `commands/enterprise.rs`)
3. Update router in `router.rs` if it's a smart-routed command
4. Add tests in lib_tests.rs

### Testing API Calls
Use wiremock for mocking API responses (see examples in `lib_tests.rs`)

### Feature Flags
- `default = ["full"]`: Includes both cloud and enterprise
- `cloud-only`: Builds redis-cloud binary only
- `enterprise-only`: Builds redis-enterprise binary only