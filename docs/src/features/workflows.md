# Workflows

Workflows are a powerful feature of redisctl that automate complex, multi-step operations. Instead of running multiple commands manually and managing the state between them, workflows handle the entire process with proper error handling, progress tracking, and rollback capabilities.

## Overview

Workflows solve common challenges when managing Redis deployments:

- **Complex operations** requiring multiple API calls in sequence
- **Asynchronous operations** that need polling and status checking
- **Error recovery** with proper cleanup and state management
- **Progress visibility** for long-running operations
- **Reproducibility** through consistent execution patterns

## How Workflows Work

Each workflow is a self-contained operation that:

1. **Validates prerequisites** - Checks current state before making changes
2. **Executes steps sequentially** - Performs operations in the correct order
3. **Handles async operations** - Waits for tasks to complete with progress feedback
4. **Manages errors gracefully** - Provides clear error messages and recovery options
5. **Returns structured results** - Outputs can be consumed programmatically

## Available Workflows

### Redis Enterprise

- **init-cluster** - Complete cluster initialization with bootstrap and database setup

### Redis Cloud (Future)

- **provision-subscription** - Create subscription with databases and networking
- **setup-aa-database** - Configure Active-Active database across regions

## Using Workflows

### Interactive Mode

Run workflows with human-readable output:

```bash
redisctl enterprise workflow init-cluster \
  --username "admin@cluster.local" \
  --password "SecurePass123"
```

Output:
```
Initializing Redis Enterprise cluster...
Bootstrap completed successfully
Cluster is ready
Creating default database 'default-db'...
Database created successfully (ID: 1)
Database connectivity verified (PING successful)

Cluster initialization completed successfully

Cluster name: redis-cluster
Admin user: admin@cluster.local
Database: default-db (1GB)

Access endpoints:
  Web UI: https://localhost:8443
  API: https://localhost:9443
```

### Programmatic Mode

Use structured output for automation:

```bash
# Get JSON output
redisctl enterprise workflow init-cluster \
  --username "admin@cluster.local" \
  --password "SecurePass123" \
  --output json \
  --skip-database
```

```json
{
  "success": true,
  "message": "Cluster initialized successfully",
  "outputs": {
    "cluster_name": "redis-cluster",
    "username": "admin@cluster.local",
    "database_created": false,
    "database_name": "default-db"
  }
}
```

### CI/CD Integration

Workflows are ideal for CI/CD pipelines:

```yaml
# GitHub Actions example
- name: Initialize Redis Enterprise
  run: |
    redisctl enterprise workflow init-cluster \
      --username "${{ secrets.REDIS_USER }}" \
      --password "${{ secrets.REDIS_PASSWORD }}" \
      --output json \
      --wait-timeout 300
```

## Async Operation Handling

Workflows handle asynchronous operations transparently:

```bash
# Workflows support standard async flags
redisctl enterprise workflow init-cluster \
  --username "admin@cluster.local" \
  --password "SecurePass123" \
  --wait \
  --wait-timeout 600
```

The workflow will:
- Submit operations asynchronously
- Poll for completion status
- Show progress indicators
- Handle timeouts gracefully

## Error Handling

Workflows provide robust error handling:

### Partial Success
If a workflow partially completes (e.g., cluster initialized but database creation fails):
- The successful steps are preserved
- Clear error messages explain what failed
- Recovery instructions are provided

### Idempotency
Workflows check current state before making changes:
- Running init-cluster on an initialized cluster returns success without re-bootstrapping
- Operations are safe to retry

### Validation
Prerequisites are checked before execution:
- Required permissions are verified
- Resource availability is confirmed
- Configuration validity is checked

## Workflow Architecture

### Trait-Based Design

Workflows implement a common trait for consistency:

```rust
pub trait Workflow: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, context: WorkflowContext, args: WorkflowArgs) 
        -> Pin<Box<dyn Future<Output = Result<WorkflowResult>> + Send>>;
}
```

### Registry Pattern

Workflows are registered at startup:

```rust
let registry = WorkflowRegistry::new();
registry.register(InitClusterWorkflow::new());
registry.register(UpgradeClusterWorkflow::new());
```

### Context and Arguments

Each workflow receives:
- **Context**: Connection manager, profile, output format, timeouts
- **Arguments**: User-provided parameters as key-value pairs

### Results

Workflows return structured results:
- **Success/failure status**
- **Human-readable message**
- **Structured outputs** for programmatic consumption

## Best Practices

### When to Use Workflows

Use workflows for:
- **Initial setup** - Bootstrapping new environments
- **Complex migrations** - Multi-step data or configuration changes
- **Disaster recovery** - Automated failover and recovery procedures
- **Routine maintenance** - Standardized update and backup procedures

### When to Use Direct Commands

Use direct commands for:
- **Simple queries** - Getting status or configuration
- **Single operations** - Creating one resource
- **Debugging** - Investigating specific issues
- **Custom scripts** - Operations not covered by workflows

## Creating Custom Workflows

While redisctl provides built-in workflows, you can create custom workflows by:

1. **Scripting existing commands** - Combine redisctl commands in bash/python
2. **Using the libraries** - Build Rust applications with redis-cloud/redis-enterprise crates
3. **Contributing workflows** - Submit PRs for commonly needed workflows

Example custom workflow script:

```bash
#!/bin/bash
# Custom workflow: setup-monitoring.sh

# Create monitoring database
DB_ID=$(redisctl enterprise database create \
  --name "monitoring" \
  --memory-gb 1 \
  --output json | jq -r '.uid')

# Configure alerts
redisctl enterprise database update $DB_ID \
  --alert-settings '{"memory_threshold": 80}'

# Setup metrics export
redisctl enterprise stats config \
  --database $DB_ID \
  --export-interval 60

echo "Monitoring setup complete for database $DB_ID"
```

## Future Enhancements

Planned workflow improvements:

- **Workflow templates** - Parameterized workflows for common patterns
- **Conditional logic** - Branching based on state or user input
- **Rollback support** - Automatic undo for failed operations
- **Workflow composition** - Building complex workflows from simpler ones
- **Progress streaming** - Real-time updates for long operations

## See Also

- [Enterprise Workflows](../enterprise/workflows.md) - Enterprise-specific workflow documentation
- [Async Operations](./async-operations.md) - Understanding async operation handling
- [Output Formats](./output-formats.md) - Working with structured output