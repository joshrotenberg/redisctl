# Cluster API

The Cluster API provides operations for managing Redis Enterprise clusters, including initialization, configuration, and monitoring cluster health.

## Overview

The cluster is the top-level resource in Redis Enterprise, representing a group of nodes working together to host databases. Cluster operations include:

- Getting cluster information and status
- Bootstrapping new clusters
- Managing cluster-wide settings
- Monitoring cluster health
- License management

## Handler

```rust
use redis_enterprise::ClusterHandler;
```

## Operations

### Get Cluster Information

Retrieves comprehensive information about the cluster including nodes, databases, and configuration.

```rust
async fn get_cluster_info(client: &EnterpriseClient) -> Result<ClusterInfo> {
    let handler = ClusterHandler::new(client.clone());
    handler.info().await
}
```

**Response Structure:**

```rust
pub struct ClusterInfo {
    pub name: String,
    pub nodes: Vec<NodeInfo>,
    pub databases: u32,
    pub ram_size: u64,
    pub flash_size: Option<u64>,
    pub version: String,
    pub license: LicenseInfo,
    // ... additional fields
}
```

**Example Response:**

```json
{
  "name": "production-cluster",
  "nodes": [
    {
      "uid": 1,
      "addr": "192.168.1.10",
      "status": "active"
    }
  ],
  "databases": 5,
  "ram_size": 68719476736,
  "version": "7.2.4-92"
}
```

### Bootstrap Cluster

Initializes a new cluster or joins a node to an existing cluster.

```rust
use redis_enterprise::BootstrapRequest;

async fn bootstrap_cluster(client: &EnterpriseClient) -> Result<()> {
    let handler = ClusterHandler::new(client.clone());
    
    let request = BootstrapRequest {
        cluster_name: "my-cluster".to_string(),
        node_address: "192.168.1.10".to_string(),
        username: "admin@example.com".to_string(),
        password: "secure-password".to_string(),
        license_file: Some("/path/to/license.key".to_string()),
        ..Default::default()
    };
    
    handler.bootstrap(request).await
}
```

**Important Notes:**
- Bootstrap can only be called once per cluster
- Requires administrative credentials
- Sets up the first node as the cluster master

### Update Cluster Configuration

Modifies cluster-wide settings such as name, alerts, or resource limits.

```rust
use serde_json::json;

async fn update_cluster(client: &EnterpriseClient) -> Result<ClusterInfo> {
    let handler = ClusterHandler::new(client.clone());
    
    let updates = json!({
        "name": "production-cluster-v2",
        "alert_settings": {
            "cluster_alert_when_down": true,
            "node_alert_when_down": true
        }
    });
    
    handler.update(updates).await
}
```

### Get Cluster Statistics

Retrieves performance metrics and resource utilization.

```rust
async fn get_cluster_stats(client: &EnterpriseClient) -> Result<Value> {
    let handler = ClusterHandler::new(client.clone());
    handler.stats().await
}
```

**Metrics Include:**
- CPU utilization
- Memory usage
- Network throughput
- Request rates
- Cache hit ratios

### License Management

```rust
async fn get_license_info(client: &EnterpriseClient) -> Result<LicenseInfo> {
    let handler = ClusterHandler::new(client.clone());
    handler.license().await
}

async fn update_license(client: &EnterpriseClient, license: &str) -> Result<LicenseInfo> {
    let handler = ClusterHandler::new(client.clone());
    handler.update_license(license.to_string()).await
}
```

## Error Handling

```rust
use redis_enterprise::RestError;

match handler.info().await {
    Ok(cluster) => println!("Cluster: {}", cluster.name),
    Err(RestError::AuthenticationFailed) => {
        eprintln!("Invalid credentials");
    },
    Err(RestError::ConnectionError(msg)) => {
        eprintln!("Cannot connect to cluster: {}", msg);
    },
    Err(e) => eprintln!("Error: {}", e),
}
```

## Common Patterns

### Health Check

```rust
async fn is_cluster_healthy(client: &EnterpriseClient) -> bool {
    let handler = ClusterHandler::new(client.clone());
    
    match handler.info().await {
        Ok(info) => {
            info.nodes.iter().all(|n| n.status == "active") &&
            info.state == "active"
        },
        Err(_) => false,
    }
}
```

### Resource Monitoring

```rust
async fn check_resource_usage(client: &EnterpriseClient) -> Result<()> {
    let handler = ClusterHandler::new(client.clone());
    let info = handler.info().await?;
    
    let used_ram = info.ram_used.unwrap_or(0);
    let total_ram = info.ram_size;
    let usage_percent = (used_ram as f64 / total_ram as f64) * 100.0;
    
    if usage_percent > 80.0 {
        println!("WARNING: RAM usage at {:.1}%", usage_percent);
    }
    
    Ok(())
}
```

## CLI Examples

```bash
# Get cluster information
redis-enterprise cluster info

# Get cluster info as JSON
redis-enterprise cluster info --output json

# Bootstrap a new cluster
redis-enterprise cluster bootstrap \
  --name "my-cluster" \
  --username "admin@example.com" \
  --password "secure-pass"

# Update cluster name
redis-enterprise cluster update --name "new-cluster-name"

# Check license
redis-enterprise cluster license
```

## Best Practices

1. **Always check cluster health** before performing operations
2. **Monitor resource usage** to prevent out-of-memory situations
3. **Keep licenses up to date** to avoid service interruptions
4. **Use connection pooling** when making frequent API calls
5. **Handle errors gracefully** - clusters may be temporarily unavailable

## See Also

For the raw REST API reference, see [Cluster REST API Reference](../rest-api/cluster.md).

## Related Endpoints

- [Nodes](./nodes.md) - Manage individual cluster nodes
- [Databases](./databases.md) - Create and manage databases
- [Statistics](./stats.md) - Detailed performance metrics
- [Alerts](./alerts.md) - Cluster alert configuration
