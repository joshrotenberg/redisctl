# CLI Modernization Plan

## Status: Foundation Complete ✅
**Current Branch:** `refactor/cli-modernization`
**Completed PRs:** Foundation (fff4993)

## Architecture Overview

### 3-Layer Design
1. **Layer 1: Raw API Tool** (`redisctl api`) - Direct REST endpoint access
2. **Layer 2: Human-Friendly Interface** (`redisctl cloud/enterprise`) - Typed operations  
3. **Layer 3: Workflow Orchestration** (`redisctl workflow`) - Multi-step operations

## Completed: PR 1 - Core Foundation ✅

### What's Working
- **CLI Structure**: Complete clap-based 3-layer architecture
- **Configuration System**: TOML profiles with env var support
- **Profile Management**: `redisctl profile list/show` commands
- **Error Handling**: Structured errors with library conversions
- **Basic Commands**: Version, help, API stubs all functional

### Key Files Created
- `src/cli.rs` - Complete command structure with all 3 layers defined
- `src/config.rs` - Full configuration management with profiles
- `src/connection.rs` - Connection manager (stub for PR 2)
- `src/error.rs` - Structured error handling
- `src/main.rs` - Basic command routing and execution
- `src/legacy/` - All old CLI code preserved for reference

### Command Structure Implemented
```
redisctl
├── api <deployment> <method> <path>     # Layer 1 (stub ready)
├── profile {list,show,set,remove,default}
├── cloud                                # Layer 2 (structure ready)
│   ├── account {info}
│   ├── subscription {list}  
│   └── database {list}
├── enterprise                           # Layer 2 (structure ready)  
│   ├── cluster {info}
│   ├── database {list}
│   └── user {list}
├── database {list,get,create}           # Smart routing (structure ready)
└── version
```

### Configuration System
- **Location**: `~/.config/redisctl/config.toml` (cross-platform)
- **Profiles**: Named connection configs for Cloud/Enterprise
- **Environment**: `REDISCTL_PROFILE` override support
- **Format**: TOML with proper serde serialization

Example profile structure:
```toml
default_profile = "my-cloud"

[profiles.my-cloud]
deployment_type = "cloud"  
api_key = "key123"
api_secret = "secret456"
api_url = "https://api.redislabs.com/v1"

[profiles.my-enterprise]
deployment_type = "enterprise"
url = "https://cluster:9443"
username = "admin"
password = "password"
insecure = true
```

## Next: PR 2 - Raw API Layer

### Scope
Implement complete `redisctl api` functionality as a curl replacement.

### Implementation Tasks
1. **Connection Management**
   - Complete `connection.rs` with actual client creation
   - Handle Cloud/Enterprise authentication  
   - Environment variable credential overrides
   - SSL/TLS configuration for Enterprise

2. **Raw API Command**
   - HTTP method handling (GET/POST/PUT/PATCH/DELETE)
   - Request body handling (JSON string, @file input)
   - Response formatting (pretty JSON by default)
   - Error handling with proper HTTP status codes

3. **Authentication Integration** 
   - Cloud: API key/secret headers
   - Enterprise: Basic auth with optional password prompting
   - Profile-based credential management

4. **Output Handling**
   - JSON output (default for api commands)
   - Error responses in consistent format
   - Verbose logging with -v flags

### Example Usage (After PR 2)
```bash
# Cloud API calls
redisctl api cloud GET /subscriptions
redisctl api cloud POST /subscriptions --data '{"name":"test"}'
redisctl api cloud GET /subscriptions/123/databases

# Enterprise API calls  
redisctl api enterprise GET /v1/bdbs
redisctl api enterprise POST /v1/bdbs --data @database.json
redisctl api enterprise DELETE /v1/bdbs/1

# With profiles and output control
redisctl --profile prod api cloud GET /subscriptions -o yaml
redisctl -v api enterprise GET /v1/cluster -q 'status'
```

### Files to Complete
- `src/connection.rs` - Full client creation logic
- `src/commands/api.rs` - Raw API command implementation  
- `src/output.rs` - JSON/YAML formatting utilities

## Future: PR 3 - Human-Friendly Layer

### Scope
Implement `redisctl cloud/enterprise` typed commands with table output.

### Key Features
- Table output by default, JSON optional
- Type-safe operations using library handlers
- Better error messages and validation
- Command completion and help

### Example Commands
```bash
redisctl cloud subscription list
redisctl cloud database create --name mydb --memory 1gb
redisctl enterprise cluster info
redisctl enterprise database list --format table
```

## Future: PR 4 - Smart Database Commands

### Scope  
Implement `redisctl database` commands that work with both deployments.

### Features
- Auto-detect deployment type from profile
- Unified interface for common operations
- Consistent output format across both backends

## Future: PR 5 - Workflow Layer

### Scope
Multi-step operations combining multiple API calls.

### Potential Workflows
- Enterprise cluster initialization
- Database creation with backup schedules
- User provisioning workflows
- Environment migration helpers

## Development Guidelines

### Code Quality
- Run `cargo fmt && cargo clippy --all-targets --all-features -- -D warnings` before committing
- All public APIs need documentation
- Use `#[allow(dead_code)]` for foundation code used in future PRs
- Structured error handling with proper context

### Testing Strategy  
- Unit tests for configuration and connection logic
- Integration tests using wiremock for API mocking
- CLI integration tests for command parsing
- Manual testing against real APIs

### Commit Conventions
- `feat(cli): description` for new features
- `fix(cli): description` for bug fixes  
- No pre-commit hooks - manual fmt/clippy instead

## Getting Started After Restart

1. **Checkout the branch**: `git checkout refactor/cli-modernization`
2. **Build and test**: `cargo build --bin redisctl`
3. **Verify foundation**: `./target/debug/redisctl --help`
4. **Check profiles**: `./target/debug/redisctl profile list`
5. **Start PR 2**: Begin implementing `src/connection.rs` client creation logic

The foundation is solid - ready for incremental development! 🚀