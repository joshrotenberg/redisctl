# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**redisctl** is a unified CLI tool for managing both Redis Cloud and Redis Enterprise deployments through their REST APIs. A single binary that intelligently routes commands to the appropriate backend based on configuration profiles, with comprehensive async operation support.

### Key Features
- Unified interface for both Redis Cloud and Redis Enterprise
- Full async operation support with `--wait` flags across all create/update/delete operations
- Multiple output formats (JSON, YAML, Table) with JMESPath filtering
- Profile-based authentication with environment variable support

### Workspace Structure
```
redisctl/
├── crates/
│   ├── redis-cloud/         # Cloud API client library (21 handlers, 95%+ coverage)
│   ├── redis-enterprise/    # Enterprise API client library (29 handlers, 100% coverage) 
│   └── redisctl/           # Unified CLI application
├── scripts/                # Build and release automation
├── tests/                  # Integration tests with wiremock
└── docs/                   # mdBook documentation
```

## Writing Style for Documentation and PRs

**CRITICAL**: Follow these strict guidelines for all documentation, commit messages, and PR descriptions:

### What to AVOID
- ❌ **No emojis** - Never use emojis in commits, PRs, or code (✅ ❌ 🚀 etc.)
- ❌ **No marketing language** - Avoid "exciting", "powerful", "seamless", "game-changing", etc.
- ❌ **No superlatives** - Don't use "best", "perfect", "amazing", "incredible"
- ❌ **No hype** - Write factual, technical descriptions only
- ❌ **No exclamation points** - Use periods for professional tone

### What to DO
- ✓ Use **technical, factual language**
- ✓ Focus on **what changed and why**
- ✓ Be **specific and concrete**
- ✓ Use **imperative mood for commits** (e.g., "Add feature" not "Added feature")
- ✓ Reference **issue numbers** where applicable

### Examples

**Bad**:
```
🚀 feat: Add amazing support package upload feature!

This exciting new feature seamlessly integrates with Files.com to provide
a powerful solution for uploading support packages! ✨
```

**Good**:
```
feat: add support package upload with Files.com integration

Implements support package upload to Files.com via files-sdk 0.3.1.
Adds --upload and --no-save flags to support-package commands.
Supports environment variable and secure keyring storage for API keys.

Closes #123
```

## Build and Development Commands

### Essential Commands
```bash
# Build the project
cargo build --release

# Run tests (CRITICAL - always run before committing)
cargo test --workspace --all-features
cargo test --package redis-cloud         # Test specific package
cargo test --package redis-enterprise
cargo test --lib --all-features          # Library tests only
cargo test --test '*' --all-features     # Integration tests only
cargo test test_database_get             # Run single test by name

# Linting (MUST pass before committing)
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Run the CLI in development
cargo run -- --help
cargo run -- profile list
cargo run -- enterprise cluster info
cargo run -- api cloud get /subscriptions  # Raw API access

# Generate CLI documentation  
./scripts/generate-cli-docs.sh

# Check for dependency vulnerabilities
cargo deny check

# Run with debug logging
RUST_LOG=debug cargo run -- enterprise cluster info

# Platform-specific builds
cargo build --release --features cloud-only --bin redis-cloud
cargo build --release --features enterprise-only --bin redis-enterprise
```

### Docker Testing Environment

#### CRITICAL: Correct Credentials
**The Docker instance uses these EXACT credentials:**
- Username: `admin@redis.local` (this is the default in docker-compose.yml and code)
- Password: `Redis123!` (with the exclamation mark!)
- URL: `https://localhost:9443`

**Note**: Documentation may show `admin@cluster.local` in some examples for historical reasons, but the actual Docker setup and code defaults use `admin@redis.local`.

#### Authentication Precedence (IMPORTANT!)
1. **CLI flags** (highest) - `--username`, `--password`, etc.
2. **Environment variables** - `REDIS_ENTERPRISE_USER`, etc.
3. **Profile/config file** (lowest) - `~/.config/redisctl/config.toml`

⚠️ **Common Pitfall**: Environment variables will override your profile settings!

```bash
# Start local Redis Enterprise cluster for testing
docker compose up -d

# Option 1: Use environment variables (will override profile!)
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_USER="admin@redis.local"  # CORRECT username
export REDIS_ENTERPRISE_PASSWORD="Redis123!"      # CORRECT password with !
export REDIS_ENTERPRISE_INSECURE="true"

# Option 2: Clear env vars and use profile (recommended)
unset REDIS_ENTERPRISE_URL REDIS_ENTERPRISE_USER REDIS_ENTERPRISE_PASSWORD REDIS_ENTERPRISE_INSECURE
cargo run -- --profile enterprise enterprise cluster get

# Test against local cluster
cargo run -- enterprise cluster get

# Clean up
docker compose down -v
```

## Core Design Principles

### Structured Output Support (CRITICAL)
**EVERY command that produces output MUST support structured JSON output via `-o json` flag**

This is essential for:
- **CI/CD Integration**: Parse results programmatically in pipelines
- **Monitoring**: Integrate with monitoring systems
- **Automation**: Script complex workflows
- **Testing**: Verify command results in automated tests
- **Error Handling**: Structured error reporting for debugging

#### JSON Output Requirements
1. **Always Available**: Every command with output must support `-o json`
2. **Consistent Structure**: Include standard fields across all commands:
   ```json
   {
     "success": true,
     "data": { ... },
     "message": "Operation completed",
     "timestamp": "2024-01-15T10:30:00Z",
     "elapsed_seconds": 2
   }
   ```
3. **Error Structure**: Errors should also be structured in JSON mode:
   ```json
   {
     "success": false,
     "error": "Authentication failed",
     "error_code": "AUTH_FAILED",
     "details": { ... },
     "timestamp": "2024-01-15T10:30:00Z"
   }
   ```
4. **Parseable Types**: Use appropriate JSON types (numbers, booleans, not strings)
5. **Exit Codes**: Set proper exit codes (0 for success, non-zero for failure)

#### Implementation Pattern
```rust
// Every command handler should support OutputFormat
pub async fn handle_command(
    // ... other params
    output_format: OutputFormat,  // REQUIRED parameter
) -> CliResult<()> {
    let result = perform_operation()?;
    
    match output_format {
        OutputFormat::Json => {
            let json = json!({
                "success": true,
                "data": result,
                "timestamp": chrono::Utc::now(),
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Table => { /* ... */ }
        OutputFormat::Yaml => { /* ... */ }
    }
}
```

## Architecture

### Three-Layer CLI Design
1. **Raw API Access** (`api` command) - Direct REST calls to any endpoint with HTTP method selection
2. **Human-Friendly Commands** - Type-safe wrappers around single API calls via `cloud`/`enterprise` subcommands
3. **Async Operations** - All create/update/delete operations support `--wait` flags for progress tracking

### Async Operation Support

#### Wait Flag Options
- `--wait` - Wait for operation to complete (default timeout: 600s)
- `--wait-timeout <seconds>` - Custom timeout duration
- `--wait-interval <seconds>` - Polling interval (default: 10s)

#### Supported Async Operations
All create, update, and delete operations across:
- Subscriptions (regular and fixed)
- Databases (regular, fixed, and Active-Active)
- Network connectivity (VPC Peering, PSC, Transit Gateway)
- ACL management (rules, roles, users)
- User and provider account management
- Backup, import, and migration operations

#### Implementation Pattern
All async operations use the centralized `handle_async_response` function in `async_utils.rs`:
```rust
pub async fn handle_async_response(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    response: Value,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
    operation_name: &str,
) -> CliResult<()>
```

### Command System
- **Explicit commands** (`cloud`, `enterprise`) require profiles of the matching type
- **API command** can target either deployment with the `deployment` parameter
- Resolution order: CLI flag → env var → profile settings

### API Client Architecture

#### Cloud API (`crates/redis-cloud/`)
- Auth: `x-api-key` and `x-api-secret-key` headers
- Base URL: `https://api.redislabs.com/v1`
- Database IDs: `subscription_id:database_id` format
- 21 handler modules, 95%+ API coverage
- Test files: `tests/*_tests.rs` using wiremock

#### Enterprise API (`crates/redis-enterprise/`)  
- Auth: Basic auth (username/password)
- Base URL: `https://cluster:9443` (configurable)
- Database IDs: Simple numeric format
- 29 handler modules, 100% API coverage
- SSL verification bypass via `REDIS_ENTERPRISE_INSECURE`
- Test files: `tests/*_tests.rs` using wiremock

### Profile System (`crates/redisctl/src/config.rs`)
Profiles store deployment credentials and settings:
- **Locations**: 
  - Linux: `~/.config/redisctl/config.toml`
  - macOS: `~/.config/redisctl/config.toml` (preferred) or `~/Library/Application Support/com.redis.redisctl/config.toml`
  - Windows: `%APPDATA%\redis\redisctl\config.toml`
- **Cross-platform consistency**: macOS supports Linux-style `~/.config/` path for better dev experience
- **Environment Variable Expansion**: Config files support `${VAR}` and `${VAR:-default}` syntax
- **Override hierarchy**: CLI flags → Environment variables → Profile settings
- **Management**: `redisctl profile set/get/list/default/remove`

#### Configuration Example with Environment Variables
```toml
default_profile = "cloud-prod"

[profiles.cloud-dev]
deployment_type = "cloud"
api_key = "${REDIS_CLOUD_DEV_KEY}"
api_secret = "${REDIS_CLOUD_DEV_SECRET}"
api_url = "${REDIS_CLOUD_DEV_URL:-https://api.redislabs.com/v1}"

[profiles.cloud-prod]
deployment_type = "cloud"  
api_key = "${REDIS_CLOUD_PROD_KEY}"
api_secret = "${REDIS_CLOUD_PROD_SECRET}"
api_url = "https://api.redislabs.com/v1"

[profiles.enterprise-local]
deployment_type = "enterprise"
url = "${REDIS_ENTERPRISE_URL:-https://localhost:9443}"
username = "${REDIS_ENTERPRISE_USER:-admin@redis.local}"
password = "${REDIS_ENTERPRISE_PASSWORD}"
insecure = true
```

### API Response Type Design Pattern (CRITICAL)

**Philosophy**: Expose ALL known/documented API fields as first-class struct members. Only use `extra: Value` for truly unknown or future fields.

**Why**: Issue #373 highlighted that hiding common fields in `extra` creates friction:
- Forces verbose boilerplate to extract fields from JSON
- No type safety or IDE autocomplete for documented fields
- Painful mapping code in production applications

**Pattern for Response Types**:
```rust
/// Database
///
/// Represents a Redis Cloud database with all known API fields as first-class struct members.
/// The `extra` field is reserved only for truly unknown/future fields that may be added to the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    /// Database ID - always present in API responses
    pub database_id: i32,  // NOT Option if always present!
    
    /// Database name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    // ... ALL ~40 known fields as first-class members
    
    /// Only for truly unknown/future API fields. All documented fields should be first-class members above.
    #[serde(flatten)]
    pub extra: Value,
}
```

**When to use this pattern**:
- ✅ **Response types** - Data the API returns (Database, Subscription, Task, etc.)
- ✅ **Wrapper types** - AccountSubscriptionDatabases, ProcessorResponse, etc.
- ⚠️ **Request types** - Can be more minimal, but still expose common fields

**DO NOT**:
- ❌ Hide documented API fields in `extra`
- ❌ Make fields `Option` if the API always returns them
- ❌ Use `extra` as a lazy catch-all for fields you haven't documented

**Recent improvements** (PR #373):
- Database: 3 → ~40 first-class fields
- Subscription: 11 → 17 first-class fields
- Eliminates user boilerplate like `db.extra.get("name").and_then(|v| v.as_str())`

### Error Handling Strategy
- **Libraries** (`redis-cloud`, `redis-enterprise`): Use `thiserror` for typed errors
- **CLI** (`redisctl`): Use `anyhow` for context-rich error messages
- **Handlers**: Return `Result<serde_json::Value>` for uniform JSON output
- **API errors**: Wrapped with operation context
- **Async operations**: Include task ID and status in error messages

### Output System (`crates/redisctl/src/output.rs`)
- **Formats**: JSON (default), YAML, Table (via `--output`/`-o`)
- **Filtering**: JMESPath queries via `-q` flag
- **Table rendering**: `comfy-table` for human-readable output
- **Auto-detection**: Uses terminal detection to choose format
- **Progress indicators**: Animated spinners for async operations

### Parameter Grouping Pattern
To avoid clippy's `too_many_arguments` warning, functions with >7 parameters use parameter structs:
```rust
pub struct OperationParams<'a> {
    pub conn_mgr: &'a ConnectionManager,
    pub profile_name: Option<&'a str>,
    pub async_ops: &'a AsyncOperationArgs,
    pub output_format: OutputFormat,
    pub query: Option<&'a str>,
}
```

This pattern is used in:
- `ConnectivityOperationParams` - VPC, PSC, TGW operations
- `CloudAccountOperationParams` - Provider account operations
- `AclOperationParams` - ACL management operations

## Adding New Features

### CRITICAL: Verify API Endpoints Exist
**NEVER create or implement endpoints that don't exist in the actual APIs**
- Always verify endpoints exist in the official API documentation
- For Redis Enterprise: Check `./tmp/rest-html/` or https://redis.io/docs/latest/operate/rs/references/rest-api/requests/
- For Redis Cloud: Check the official API documentation
- DO NOT hallucinate or assume endpoints exist based on what seems logical
- If an endpoint doesn't exist in the API, DO NOT implement it in the library

#### How to Verify an Endpoint Exists
```bash
# 1. Check the documentation
grep -r "endpoint_name" ./tmp/rest-html/

# 2. Test it directly with curl
curl -k -u "admin@redis.local:Redis123!" https://localhost:9443/v1/endpoint -I

# 3. Check response - 404 means it doesn't exist!
# HTTP/1.1 200 OK = endpoint exists
# HTTP/1.1 404 Not Found = endpoint DOES NOT exist
# HTTP/1.1 401 Unauthorized = endpoint exists but needs auth

# 4. For binary endpoints, check Content-Type
curl -k -u "admin@redis.local:Redis123!" https://localhost:9443/v1/debuginfo/all -I | grep -i content-type
# Content-Type: application/x-gzip = binary tar.gz file
# Content-Type: application/json = JSON response
```

### Adding a Command
1. **FIRST**: Verify the API endpoint actually exists in the documentation
2. Define command enum variant in `crates/redisctl/src/cli.rs`
3. Add `AsyncOperationArgs` for create/update/delete operations:
   ```rust
   #[command(flatten)]
   async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
   ```
4. Implement handler in appropriate command module
5. Use `handle_async_response` for operations that return task IDs
6. Add client method in library crate handler module (ONLY if endpoint exists)
7. Write tests using `wiremock` for API mocking

### Adding an API Endpoint
1. Define types in library's `types.rs` or handler module
2. Implement method in handler (e.g., `handlers/databases.rs`)
3. Add tests with `wiremock` mocking in `tests/[handler]_tests.rs`
4. Expose in CLI command handler
5. Add async support if operation returns task ID

### Handler Implementation Pattern
```rust
// For async operations
pub async fn handle_create_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let payload = parse_data_input(data)?;
    
    let response = client
        .post_raw("/subscriptions/{}/databases", payload)
        .await
        .context("Failed to create database")?;
    
    handle_async_response(
        conn_mgr,
        profile_name,
        response,
        async_ops,
        output_format,
        query,
        "database creation",
    )
    .await
}
```

### API Version Strategy (v1 vs v2)
Redis Enterprise has both v1 and v2 endpoints for some operations. Our strategy:
1. **Always use v2 when available** - v1 endpoints are deprecated as of Redis Enterprise 7.2
2. **Default to v2** - Primary methods should use v2 endpoints
3. **No v1 fallback** - Don't maintain v1 methods unless absolutely necessary for backwards compatibility
4. **Async handling** - v2 endpoints often return `action_uid` for async tracking

Examples:
- Module upload: Use `/v2/modules` (returns action_uid)
- BDB creation: Use `/v2/bdbs` for async operations
- Actions: Use `/v2/actions` for listing and status

### Active-Active (CRDB) Pattern

Redis Cloud has separate functions for Active-Active (CRDB) operations to ensure type safety:

#### When to Use Active-Active Functions
Active-Active databases (Conflict-free Replicated Databases) require special handling for multi-region deployments. The `_active_active` suffix indicates functions designed specifically for these databases.

**Key Active-Active Operations:**
- VPC Peering: `create_active_active()`, `get_active_active()`, `update_active_active()`, `delete_active_active()`
- PSC: `create_service_active_active()`, `create_endpoint_active_active()`, etc.
- Transit Gateway: `create_attachment_active_active()`, `get_attachments_active_active()`, etc.

**Why Separate Functions?**
1. **Different request structures** - AA operations often require region-specific parameters
2. **Type safety** - Prevents accidentally calling regular endpoints with AA-specific data
3. **Clear API surface** - Explicit function names make intent clear

**Example:**
```rust
// Regular database VPC peering
let vpc = handler.create(subscription_id, &request).await?;

// Active-Active database VPC peering (requires region specification)
let vpc = handler.create_active_active(subscription_id, region_id, &request).await?;
```

**Note:** Despite having separate functions, these map to the same API endpoints. The separation is for developer ergonomics and type safety.

### Common Code Patterns
- **Client creation**: Use `create_cloud_client()` or `create_enterprise_client()` helpers
- **Error context**: Always add `.context()` to errors for debugging
- **Output handling**: Use `print_output()` helper for consistent formatting
- **Async operations**: Use `handle_async_response()` for task-based operations
- **Parameter grouping**: Create params struct when >7 parameters needed
- **Testing**: Mock all HTTP calls with `wiremock`, never make real API calls in tests
- **Active-Active**: Use `_active_active` suffixed functions for CRDB operations

## OpenAPI Specification Validation

### Automated Validation Tests
The `redis-cloud` crate includes automated tests that validate our implementation against the official OpenAPI specification:

**Test File**: `crates/redis-cloud/tests/openapi_validation.rs`

**What is Validated**:
1. **Spec Integrity** - OpenAPI spec loads and has required sections
2. **Endpoint Count** - Verify we have the expected number of endpoints
3. **Schema Definitions** - Key response types exist in spec
4. **Endpoint Categories** - All expected API categories are present
5. **Response Type Coverage** - Core fields match spec definitions

**Running Validation**:
```bash
cargo test --package redis-cloud --test openapi_validation
```

### OpenAPI Spec Location
- **Local Copy**: `crates/redis-cloud/tests/fixtures/cloud_openapi.json`
- **Official Source**: https://redis.io/docs/latest/operate/rc/api/api-reference/openapi.json

### API Documentation Pattern
All handler functions include OpenAPI references in their documentation:

```rust
/// Get cloud accounts
///
/// Gets a list of all configured cloud accounts.
///
/// # API Endpoint
///
/// `GET /cloud-accounts`
///
/// See [OpenAPI Spec](https://redis.io/docs/latest/operate/rc/api/api-reference/openapi.json) - `getCloudAccounts`
pub async fn get_cloud_accounts(&self) -> Result<CloudAccounts> {
    // ...
}
```

This pattern:
- Links to the official OpenAPI specification
- References the specific operationId from the spec
- Makes it easy to cross-reference implementation with API docs

## Testing Approach

### MANDATORY Testing Requirements
**ALL new Enterprise API features MUST:**
1. **Be tested against the Docker instance FIRST** - Before writing mocks, test the actual endpoint with real data
2. **Capture real responses** - Use the actual response data from Docker testing for mock responses
3. **Verify response types** - Confirm if endpoint returns JSON or binary (tar.gz, etc.)
4. **Test both success and error cases** - Include authentication failures, 404s, etc.
5. **Document the testing** - Add comments showing which Docker command was used to generate mock data

#### Testing Workflow for New Enterprise Features
```bash
# 1. Start Docker instance
docker compose up -d

# 2. Test the real endpoint
curl -k -u "admin@redis.local:Redis123!" https://localhost:9443/v1/your/endpoint | jq

# 3. Capture response for mock (example for binary endpoints)
curl -k -u "admin@redis.local:Redis123!" https://localhost:9443/v1/debuginfo/all -o test_response.tar.gz
file test_response.tar.gz  # Verify it's actually gzip

# 4. Use captured data in tests
# Add comment in test file: // Mock data captured from: curl -k -u "admin@redis.local:Redis123!" ...
```

### Standard Testing Practices
- **Unit tests**: `src/lib_tests.rs` in each crate
- **Integration tests**: `tests/*_tests.rs` with wiremock mocks based on REAL responses
- **E2E testing**: Docker Compose with real Redis Enterprise
- **Coverage goal**: 70%+ for libraries, focus on critical paths
- **Test organization**: One test file per handler module
- **Mock setup**: Use `common::setup_mock_server()` helper
- **Binary responses**: Test with actual gzip headers and binary content

### Running Tests
```bash
# Run all tests
cargo test --workspace --all-features

# Run tests for specific crate
cargo test --package redis-cloud
cargo test --package redis-enterprise

# Run specific test
cargo test test_database_list

# Run tests with output
cargo test -- --nocapture

# Run tests in single thread (for debugging)
cargo test -- --test-threads=1
```

## Release Process
Automated via GitHub Actions:
1. Push conventional commits → `release-plz` creates version PR
2. Merge PR → publishes to crates.io, creates `redisctl-v*` tag
3. Tag triggers `cargo-dist` → builds platform binaries
4. Release created → Docker image published to Docker Hub

### Conventional Commits
- `feat:` New feature (minor version bump)
- `fix:` Bug fix (patch version bump)
- `docs:` Documentation only
- `style:` Formatting, missing semi colons, etc
- `refactor:` Code change that neither fixes a bug nor adds a feature
- `test:` Adding missing tests
- `chore:` Updating build tasks, package manager configs, etc

## VHS Terminal Recording Demos

The project includes VHS (terminal recording) demos to showcase CLI features:

### Demo Files
- `vhs/quick-start.tape` - Basic usage demonstration
- `vhs/profile-management.tape` - Profile creation and management
- `vhs/demo-config.toml` - Clean configuration for recordings
- `vhs/setup-demo-env.sh` - Environment setup for recording
- `vhs/restore-config.sh` - Restore original configuration
- `vhs/img/` - Generated GIF output directory

### Recording New Demos
```bash
# Set up demo environment (backs up real config)
./vhs/setup-demo-env.sh

# Record demo
vhs vhs/quick-start.tape

# Restore original config
./vhs/restore-config.sh
```

### VHS Tape Format Notes
- VHS executes actual commands, not just displays them
- Avoid complex quoted strings that break VHS parsing
- Use simple commands that work without API access for demos
- Focus on profile management and help commands

## Important Files
- `crates/redisctl/src/cli.rs` - Command definitions and CLI structure
- `crates/redisctl/src/commands/` - Command implementation directory
  - `cloud/async_utils.rs` - Async operation handling utilities
  - `cloud/connectivity/` - Network connectivity commands
  - `cloud/acl.rs` & `acl_impl.rs` - ACL management
- `crates/redisctl/src/connection.rs` - Client creation helpers
- `crates/redisctl/src/config.rs` - Profile and configuration management
- `crates/redisctl/src/output.rs` - Output formatting system
- `crates/redis-cloud/src/handlers/` - Cloud API endpoint handlers
- `crates/redis-enterprise/src/handlers/` - Enterprise API endpoint handlers
- `release-plz.toml` - Release automation config
- `cliff.toml` - Changelog generation config
- `docker-compose.yml` - Local test environment
- `./tmp/rest-html/` - Redis Enterprise API documentation (for verifying endpoints)

## Dependencies and Versions
- **Rust**: 1.89+ (edition 2024, per Cargo.toml)
- **Tokio**: 1.40+ async runtime with full features
- **Reqwest**: 0.12+ HTTP client with rustls-tls (no native TLS)
- **Clap**: 4.5+ CLI parsing with derive, env, and string features
- **Serde**: 1.0+ JSON/YAML serialization with derive macros
- **Wiremock**: 0.6+ API mocking for comprehensive testing
- **Anyhow/Thiserror**: Error handling (anyhow for CLI, thiserror for libraries)
- **Comfy-table**: 7.2+ for table output formatting
- **JMESPath**: 0.4+ for JSON query filtering
- **Indicatif**: 0.17+ for progress indicators and spinners

### Key Feature Flags
- `cloud-only`: Builds Cloud-only binary (`redis-cloud`)
- `enterprise-only`: Builds Enterprise-only binary (`redis-enterprise`)
- Default: Unified binary with both APIs (`redisctl`)

## Recent Major Updates

### Documentation Reorganization (PR #362, Issue #361)
Completely restructured documentation for better navigation and discoverability:
- Organized primarily by deployment type (Cloud/Enterprise) instead of flat lists
- Commands grouped by functionality (Core Resources, Access Control, Operations, etc.)
- Reduced Enterprise docs from 34 files to ~6-7 logical sections
- Added comprehensive getting-started examples
- Created Common Features section for cross-deployment functionality
- Added JMESPath query and secure storage documentation
- Fixed all broken internal links
- Note: mdbook-lint temporarily disabled - needs 133 lint errors fixed

### Secure Credential Storage (PR #360, Issue #180)
Implemented OS keyring integration for secure credential storage:
- Optional `secure-storage` feature flag
- Supports macOS Keychain, Windows Credential Store, Linux Secret Service
- Profile commands support `--use-keyring` flag
- Credentials stored encrypted in OS keyring, config only contains references
- Full backward compatibility with plaintext storage

### Async Operation Support (Issues #175-#199)
Added comprehensive `--wait` flag support across all create/update/delete operations:
- Database operations (regular, fixed, Active-Active)
- Subscription management (regular and fixed)
- Network connectivity (VPC Peering, PSC, Transit Gateway)
- ACL management (rules, roles, users)
- User and provider account management
- Backup, import, and migration operations

### Parameter Grouping Refactor
Introduced parameter structs to avoid `too_many_arguments` clippy warnings:
- `ConnectivityOperationParams` for network operations
- `CloudAccountOperationParams` for provider accounts
- `AclOperationParams` for ACL management

### Module Reorganization
- Consolidated connectivity commands under single module
- Improved code organization for better maintainability
- Enhanced test coverage across all modules

### Module Management Commands (Issue #153)
Added module management commands for Redis Enterprise:
- `module list` - List all available modules
- `module get <uid>` - Get specific module details
- `module config-bdb <bdb_uid> --data <json>` - Configure modules for database
  - Format: `{"modules": [{"module_name": "search", "module_args": "PARTITIONS AUTO"}]}`
  - Uses `module_name` and `module_args` fields (NOT module_uid or config)
- `module upload --file <path>` - Upload new module
  - Uses multipart/form-data with v2 endpoint (falls back to v1 if v2 not available)
  - Note: Module upload may not be enabled on all Redis Enterprise instances
- `module delete <uid>` - Delete module
- Note: upgrade-bdb endpoint documented but doesn't exist in actual API

## API Endpoint Reference

### Redis Enterprise API - Complete Verified Endpoint List
**CRITICAL**: This is the COMPLETE list of endpoints that exist in the Redis Enterprise API.
If an endpoint is not in this list, it DOES NOT exist. Do not implement it.

#### Logs Endpoints
- `GET /v1/logs` - Get cluster event logs (the ONLY logs endpoint)
  - Query params: `stime`, `etime`, `order`, `limit`, `offset`
  - Returns array of events with `time`, `type`, and event-specific fields

**These endpoints DO NOT exist** (despite what might seem logical):
- ❌ `GET /v1/logs/{id}` - No endpoint for specific log entry
- ❌ `GET /v1/logs/files` - No log files endpoint
- ❌ `POST /v1/logs/rotate` - No log rotation endpoint
- ❌ `GET /v1/logs/config` - No log configuration endpoint
- ❌ `GET /v1/nodes/{id}/logs` - No node-specific logs endpoint
- ❌ `GET /v1/bdbs/{id}/logs` - No database-specific logs endpoint

#### Complete Endpoint List (for reference)
<details>
<summary>Click to expand full endpoint list</summary>

```
# Actions
GET /v1/actions
GET /v1/actions/:action_uid
GET /v1/actions/bdb/:bdb_uid

# BDB Groups
GET /v1/bdb_groups
GET /v1/bdb_groups/:uid
POST /v1/bdb_groups
PUT /v1/bdb_groups/:uid
DELETE /v1/bdb_groups/:uid

# Databases (BDBs)
GET /v1/bdbs
GET /v1/bdbs/:uid
POST /v1/bdbs
PUT /v1/bdbs/:uid
DELETE /v1/bdbs/:uid
GET /v1/bdbs/:uid/availability
GET /v1/bdbs/:uid/shards
GET /v1/bdbs/:uid/syncer_state
GET /v1/bdbs/:uid/syncer_state/crdt
GET /v1/bdbs/:uid/syncer_state/replica
POST /v1/bdbs/:uid/command
POST /v1/bdbs/:uid/passwords
PUT /v1/bdbs/:uid/passwords
DELETE /v1/bdbs/:uid/passwords
POST /v1/bdbs/:uid/upgrade
POST /v1/bdbs/:uid/actions/export
POST /v1/bdbs/:uid/actions/import
POST /v1/bdbs/:uid/actions/recover
POST /v1/bdbs/:uid/actions/resume_traffic
POST /v1/bdbs/:uid/actions/stop_traffic
PUT /v1/bdbs/:uid/actions/backup_reset_status
PUT /v1/bdbs/:uid/actions/export_reset_status
PUT /v1/bdbs/:uid/actions/import_reset_status
PUT /v1/bdbs/:uid/actions/rebalance
PUT /v1/bdbs/:uid/actions/revamp
GET /v1/bdbs/:uid/actions/optimize_shards_placement
GET /v1/bdbs/:uid/actions/recover

# Database Stats
GET /v1/bdbs/stats
GET /v1/bdbs/stats/:uid
GET /v1/bdbs/stats/last
GET /v1/bdbs/stats/last/:uid
GET /v1/bdbs/:bdb_uid/peer_stats
GET /v1/bdbs/:bdb_uid/peer_stats/:uid
GET /v1/bdbs/:bdb_uid/sync_source_stats
GET /v1/bdbs/:bdb_uid/sync_source_stats/:uid

# Database Alerts
GET /v1/bdbs/alerts
GET /v1/bdbs/alerts/:uid
GET /v1/bdbs/alerts/:uid/:alert
POST /v1/bdbs/alerts/:uid

# Cluster
GET /v1/cluster
PUT /v1/cluster
GET /v1/cluster/actions
GET /v1/cluster/actions/:action
POST /v1/cluster/actions/:action
DELETE /v1/cluster/actions/:action
GET /v1/cluster/alerts
GET /v1/cluster/alerts/:alert
GET /v1/cluster/stats
GET /v1/cluster/stats/last
GET /v1/cluster/policy
PUT /v1/cluster/policy
PUT /v1/cluster/policy/restore_default
GET /v1/cluster/services_configuration
PUT /v1/cluster/services_configuration
GET /v1/cluster/witness_disk
GET /v1/cluster/module/capabilities
GET /v1/cluster/certificates
DELETE /v1/cluster/certificates/:certificate_name
POST /v1/cluster/certificates/rotate
PUT /v1/cluster/update_cert

# Nodes
GET /v1/nodes
GET /v1/nodes/:uid
PUT /v1/nodes/:uid
GET /v1/nodes/:uid/status
GET /v1/nodes/:uid/wd_status
GET /v1/nodes/status
GET /v1/nodes/wd_status
GET /v1/nodes/actions
GET /v1/nodes/:node_uid/actions
GET /v1/nodes/:node_uid/actions/:action
POST /v1/nodes/:node_uid/actions/:action
DELETE /v1/nodes/:node_uid/actions/:action
GET /v1/nodes/alerts
GET /v1/nodes/alerts/:uid
GET /v1/nodes/alerts/:uid/:alert
GET /v1/nodes/stats
GET /v1/nodes/stats/:uid
GET /v1/nodes/stats/last
GET /v1/nodes/stats/last/:uid
GET /v1/nodes/:node_uid/snapshots
POST /v1/nodes/:node_uid/snapshots/:snapshot_name
DELETE /v1/nodes/:node_uid/snapshots/:snapshot_name

# Users & Auth
GET /v1/users
GET /v1/users/:uid
POST /v1/users
PUT /v1/users/:uid
DELETE /v1/users/:uid
POST /v1/users/password
PUT /v1/users/password
DELETE /v1/users/password
POST /v1/users/authorize
POST /v1/users/refresh_jwt
GET /v1/users/permissions
GET /v1/users/permissions/:role

# Roles
GET /v1/roles
GET /v1/roles/:uid
POST /v1/roles
PUT /v1/roles/:uid
DELETE /v1/roles/:uid

# Redis ACLs
GET /v1/redis_acls
GET /v1/redis_acls/:uid
POST /v1/redis_acls
PUT /v1/redis_acls/:uid
DELETE /v1/redis_acls/:uid
POST /v1/redis_acls/validate

# Modules
GET /v1/modules
GET /v1/modules/:uid
POST /v1/modules
DELETE /v1/modules/:uid
POST /v1/modules/config/bdb/:uid
POST /v1/modules/upgrade/bdb/:uid
POST /v1/bdbs/:uid/modules/config
POST /v1/bdbs/:uid/modules/upgrade

# Shards
GET /v1/shards
GET /v1/shards/:shard_uid
GET /v1/shards/stats
GET /v1/shards/stats/:uid
GET /v1/shards/stats/last
GET /v1/shards/stats/last/:uid
POST /v1/shards/actions/failover
POST /v1/shards/actions/migrate
POST /v1/shards/:uid/actions/failover
POST /v1/shards/:uid/actions/migrate

# LDAP
GET /v1/cluster/ldap
PUT /v1/cluster/ldap
DELETE /v1/cluster/ldap
GET /v1/ldap_mappings
GET /v1/ldap_mappings/:uid
POST /v1/ldap_mappings
PUT /v1/ldap_mappings/:uid
DELETE /v1/ldap_mappings/:uid

# Other Endpoints
GET /v1/bootstrap
POST /v1/bootstrap/:action
POST /v1/bootstrap/validate/:action
GET /v1/cm_settings
PUT /v1/cm_settings
GET /v1/debuginfo/all
GET /v1/debuginfo/all/bdb/:bdb_uid
GET /v1/debuginfo/node
GET /v1/debuginfo/node/bdb/:bdb_uid
GET /v1/diagnostics
PUT /v1/diagnostics
GET /v1/endpoints/stats
GET /v1/job_scheduler
PUT /v1/job_scheduler
GET /v1/jsonschema
GET /v1/license
PUT /v1/license
GET /v1/local/bdbs/:uid/endpoint/availability
GET /v1/local/node/master_healthcheck
GET /v1/local/services
POST /v1/local/services
GET /v1/logs  # ← THE ONLY LOGS ENDPOINT
GET /v1/migrations/:uid
GET /v1/ocsp
GET /v1/ocsp/status
PUT /v1/ocsp
POST /v1/ocsp/test
GET /v1/proxies
GET /v1/proxies/:uid
PUT /v1/proxies
PUT /v1/proxies/:uid
POST /v1/services
GET /v1/suffix/:name
GET /v1/suffixes
GET /v1/usage_report
GET /v1/cluster/auditing/db_conns
PUT /v1/cluster/auditing/db_conns
DELETE /v1/cluster/auditing/db_conns

# V2 Endpoints
GET /v2/actions
GET /v2/actions/:action_uid
POST /v2/bdbs
POST /v2/modules
DELETE /v2/modules/:uid
```

</details>

## Development Workflow
The project follows strict development standards:
- **Branch Strategy**: Feature branches only, never commit to main
- **Branch Naming**: `feat/`, `fix/`, `docs/`, `refactor/`, `test/` prefixes
- **Quality Gates**: All code must pass `cargo fmt`, `cargo clippy`, and `cargo test`
- **Test Coverage**: 70%+ for libraries with comprehensive `wiremock` integration tests
- **Documentation**: mdBook for user docs, comprehensive inline docs for APIs
- **PR Process**: Create PR from feature branch, CI must pass, maintainer reviews
- **Commit Messages**: Use conventional commits format for automatic versioning

## Troubleshooting

### Common Issues
- **SSL errors with Enterprise**: Set `REDIS_ENTERPRISE_INSECURE=true` for self-signed certs
- **Authentication failures**: Use `redisctl auth test` to validate credentials
- **Test failures**: Ensure Docker is running for integration tests
- **Clippy warnings**: Must be fixed before committing, run `cargo clippy --fix`
- **Profile not found**: Check `redisctl profile list` and set default with `redisctl profile default <name>`
- **Windows build issues**: May need to install OpenSSL or use rustls feature
- **Async timeout**: Increase timeout with `--wait-timeout` for long operations
- **Task not found**: Some operations may not return task IDs in certain contexts