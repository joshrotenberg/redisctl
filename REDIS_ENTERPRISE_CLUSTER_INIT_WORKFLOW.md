# Redis Enterprise Cluster Initialization Workflow

## Overview

The `redisctl` tool provides a high-level workflow command to initialize a new Redis Enterprise cluster. This workflow orchestrates multiple API calls to bootstrap a cluster from scratch, handling the complex initialization process through a single command.

## Command Syntax

```bash
redisctl enterprise workflow init-cluster \
  --name "Production Cluster" \
  --username admin@redis.local \
  --password <password> \
  --accept-eula \
  [--license /path/to/license.key] \
  [--with-database database-name]
```

## Workflow Steps

### Step 1: Bootstrap Request Preparation

The workflow begins by preparing a bootstrap configuration request with the following components:

1. **Action Type**: Sets `action: "create_cluster"` to indicate cluster creation
2. **Cluster Configuration**:
   - `name`: The cluster name provided via `--name` parameter
   - Optional DNS suffixes and rack awareness settings
3. **Node Configuration**:
   - `persistent_path`: `/var/opt/redislabs/persist` (default storage path)
   - `ephemeral_path`: `/var/opt/redislabs/tmp` (default temp path)
4. **Credentials**:
   - `username`: Admin username from `--username` parameter
   - `password`: Admin password from `--password` parameter
5. **License** (optional):
   - If `--license` is provided, reads the license file content
   - Includes license content in the `license_file` field

### Step 2: EULA Acceptance Validation

- **Requirement**: User must explicitly accept the End User License Agreement
- **Flag**: `--accept-eula` must be present
- **Validation**: If not provided, workflow fails with error message requiring EULA acceptance

### Step 3: Bootstrap API Call

Executes the cluster bootstrap by sending a POST request:

```
POST /v1/bootstrap/create_cluster
```

With the prepared configuration payload:

```json
{
  "action": "create_cluster",
  "cluster": {
    "name": "Production Cluster"
  },
  "node": {
    "paths": {
      "persistent_path": "/var/opt/redislabs/persist",
      "ephemeral_path": "/var/opt/redislabs/tmp"
    }
  },
  "credentials": {
    "username": "admin@redis.local",
    "password": "secure-password"
  },
  "license_file": "<license-content-if-provided>"
}
```

### Step 4: Wait for Cluster Activation

- **Duration**: 5-second wait period
- **Purpose**: Allow cluster initialization to complete
- **Status**: Cluster transitions from `in_progress` to `active` state

### Step 5: Initial Database Creation (Optional)

If `--with-database` parameter is provided:

1. **Intent**: Create an initial database after cluster initialization
2. **Current Limitation**: Requires authenticated client with new credentials
3. **Status**: Currently displays warning message to create database manually
4. **Future Enhancement**: Will automatically create database once authentication handling is implemented

## Required Parameters

| Parameter | Description | Required |
|-----------|-------------|----------|
| `--name` | Cluster name | Yes |
| `--username` | Admin username for cluster | Yes |
| `--password` | Admin password for cluster | Yes |
| `--accept-eula` | Explicit EULA acceptance | Yes |

## Optional Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `--license` | Path to license key file | None (trial mode) |
| `--with-database` | Name of initial database to create | None |

## Internal Implementation Details

### File Structure

The workflow is implemented across multiple files:

1. **`crates/redisctl/src/workflows/cluster.rs`**: Main workflow implementation
   - `init_cluster()` function orchestrates the steps
   - Handles parameter validation and API calls

2. **`crates/redis-enterprise/src/bootstrap.rs`**: Bootstrap API client
   - Defines data structures (`BootstrapConfig`, `ClusterBootstrap`, etc.)
   - Provides `BootstrapHandler` for API interactions

3. **`crates/redisctl/src/workflows/mod.rs`**: Workflow command routing
   - Maps CLI commands to workflow functions
   - Handles command dispatching

### Data Flow

1. **CLI Input** → Parse command-line arguments
2. **Validation** → Check EULA acceptance and required parameters
3. **Configuration** → Build bootstrap request payload
4. **API Call** → Send POST to `/v1/bootstrap/create_cluster`
5. **Wait** → Allow cluster to initialize
6. **Response** → Return success/failure status

### Error Handling

- **Missing EULA**: Returns error requiring `--accept-eula` flag
- **API Failures**: Propagates HTTP errors from bootstrap endpoint
- **License Issues**: Reports file read errors for license path
- **Database Creation**: Currently logs warning for manual creation

## Success Response

On successful initialization:

```json
{
  "success": true,
  "message": "Cluster 'Production Cluster' initialized successfully"
}
```

## Bootstrap Status States

The bootstrap process can be in the following states:

- `not_started`: Bootstrap has not been initiated
- `in_progress`: Cluster initialization is ongoing
- `completed`: Cluster successfully initialized
- `failed`: Bootstrap process encountered an error

## Related Workflows

### Planned/Stub Implementations

1. **`setup-ha`**: Configure high availability with replica nodes (not yet implemented)
2. **`create-database`**: Standalone database creation workflow

## Testing

The workflow is tested through:

1. **Unit Tests**: `bootstrap_tests.rs` validates bootstrap API interactions
2. **Integration Tests**: Docker-based tests against real Redis Enterprise clusters
3. **Mock Testing**: Uses `wiremock` for HTTP response simulation

## Prerequisites

Before running the initialization workflow:

1. **Network Access**: Ensure connectivity to target node (default port 9443)
2. **Clean Node**: Target should be a fresh Redis Enterprise installation
3. **License File** (optional): Valid license key for production deployments
4. **Credentials**: Choose secure admin username and password

## Example Usage

### Basic Cluster Initialization

```bash
redisctl enterprise workflow init-cluster \
  --name "Development Cluster" \
  --username dev@redis.local \
  --password SecurePass123! \
  --accept-eula
```

### Production Cluster with License

```bash
redisctl enterprise workflow init-cluster \
  --name "Production Cluster" \
  --username admin@company.com \
  --password SuperSecure456! \
  --accept-eula \
  --license /path/to/redis-enterprise.key
```

### With Initial Database (Future)

```bash
redisctl enterprise workflow init-cluster \
  --name "App Cluster" \
  --username admin@app.local \
  --password AppPass789! \
  --accept-eula \
  --with-database app-cache
```

## Notes and Limitations

1. **Single Node**: Currently initializes single-node clusters only
2. **Database Creation**: Manual step required after initialization
3. **HA Setup**: High availability configuration requires additional steps
4. **Timeout**: Fixed 5-second wait may be insufficient for large deployments
5. **Progress Tracking**: No real-time progress updates during initialization

## Security Considerations

- **Credentials**: Use strong passwords for admin accounts
- **License**: Protect license files with appropriate permissions
- **Network**: Use SSL/TLS for API communications (configurable via profile)
- **EULA**: Explicit acceptance required for compliance

## Future Enhancements

1. **Progress Monitoring**: Poll bootstrap status endpoint for real-time updates
2. **Database Creation**: Implement authenticated database creation post-bootstrap
3. **Multi-Node**: Support for initializing multi-node clusters
4. **Validation**: Pre-flight checks for node readiness
5. **Rollback**: Cleanup on initialization failure