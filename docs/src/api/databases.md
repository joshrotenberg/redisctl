# Databases (BDB) API

The Database API (internally called BDB - Big DataBase) provides comprehensive database management capabilities for Redis Enterprise, including creation, configuration, monitoring, and deletion of databases.

## Overview

Databases in Redis Enterprise are multi-tenant, highly available data stores that can be distributed across cluster nodes. Each database can be configured with:

- **Memory limits** and eviction policies
- **Persistence** options (AOF, snapshots)
- **Replication** for high availability
- **Clustering** for horizontal scaling
- **Modules** for extended functionality (Search, JSON, TimeSeries, etc.)
- **Security** with passwords, SSL/TLS, and ACLs

## Handler

```rust
use redis_enterprise::{BdbHandler, DatabaseHandler}; // Both are aliases
```

## Data Structures

### Database Information

```rust
pub struct DatabaseInfo {
    pub uid: u32,                          // Unique database ID
    pub name: String,                      // Database name
    pub port: Option<u16>,                 // Port number
    pub status: Option<String>,            // active, pending, etc.
    pub memory_size: Option<u64>,          // Memory limit in bytes
    pub memory_used: Option<u64>,          // Current memory usage
    pub type_: Option<String>,             // redis or memcached
    pub version: Option<String>,           // Redis version
    pub shards_count: Option<u32>,        // Number of shards
    pub endpoints: Option<Vec<EndpointInfo>>, // Connection endpoints
    pub replication: Option<bool>,         // Replication enabled
    pub persistence: Option<String>,       // aof, snapshot, or disabled
    pub eviction_policy: Option<String>,   // LRU, LFU, etc.
    // ... additional fields
}
```

### Create Database Request

```rust
pub struct CreateDatabaseRequest {
    pub name: String,                      // Required: Database name
    pub memory_size: u64,                  // Required: Memory in bytes
    pub port: Option<u16>,                 // Optional: Specific port
    pub replication: Option<bool>,         // Enable replication
    pub persistence: Option<String>,       // Persistence type
    pub eviction_policy: Option<String>,   // Eviction policy
    pub shards_count: Option<u32>,        // Number of shards
    // Additional optional configurations
}
```

## Operations

### List All Databases

Retrieves all databases in the cluster with their current status and configuration.

```rust
async fn list_databases(client: &EnterpriseClient) -> Result<Vec<DatabaseInfo>> {
    let handler = BdbHandler::new(client.clone());
    handler.list().await
}
```

**Example Response:**
```json
[
  {
    "uid": 1,
    "name": "user-sessions",
    "port": 12000,
    "status": "active",
    "memory_size": 1073741824,
    "memory_used": 524288000,
    "type": "redis",
    "version": "7.2",
    "shards_count": 2,
    "replication": true,
    "persistence": "aof"
  },
  {
    "uid": 2,
    "name": "cache-layer",
    "port": 12001,
    "status": "active",
    "memory_size": 536870912,
    "memory_used": 10485760,
    "eviction_policy": "allkeys-lru"
  }
]
```

### Get Database Information

Retrieves detailed information about a specific database.

```rust
async fn get_database_info(client: &EnterpriseClient, db_id: u32) -> Result<DatabaseInfo> {
    let handler = BdbHandler::new(client.clone());
    handler.info(db_id).await
}
```

### Create a Database

Creates a new database with specified configuration using the builder pattern.

```rust
use redis_enterprise::CreateDatabaseRequest;

async fn create_database(client: &EnterpriseClient) -> Result<DatabaseInfo> {
    let handler = BdbHandler::new(client.clone());
    
    let request = CreateDatabaseRequest::builder()
        .name("my-database")
        .memory_size(1_073_741_824) // 1GB
        .port(12000)
        .replication(true)
        .persistence("aof")
        .eviction_policy("allkeys-lru")
        .shards(2)
        .build()?;
    
    handler.create(request).await
}

// Or use the inline builder helper
async fn create_database_inline(client: &EnterpriseClient) -> Result<DatabaseInfo> {
    let handler = BdbHandler::new(client.clone());
    
    handler.create_with_builder(|b| b
        .name("my-database")
        .memory_size(1_073_741_824)
        .replication(true)
    ).await
}
```

### Advanced Database Creation

Creating a database with modules and advanced configuration:

```rust
use serde_json::json;

async fn create_advanced_database(client: &EnterpriseClient) -> Result<DatabaseInfo> {
    let handler = BdbHandler::new(client.clone());
    
    // Using JSON for complex configurations
    let request = json!({
        "name": "search-database",
        "memory_size": 2147483648, // 2GB
        "port": 12002,
        "replication": true,
        "shards_count": 4,
        "module_list": [
            {
                "module_name": "search",
                "module_args": "PARTITIONS AUTO"
            },
            {
                "module_name": "json"
            }
        ],
        "data_persistence": "aof",
        "aof_policy": "appendfsync-every-sec",
        "snapshot_policy": [{
            "secs": 3600,
            "writes": 100
        }],
        "redis_version": "7.2",
        "ssl": true,
        "tls_mode": "enabled",
        "replica_ha": true,
        "crdt": false
    });
    
    handler.create_raw(request).await
}
```

### Update Database Configuration

Modifies database settings. Note that some settings require database restart.

```rust
async fn update_database(client: &EnterpriseClient, db_id: u32) -> Result<DatabaseInfo> {
    let handler = BdbHandler::new(client.clone());
    
    let updates = json!({
        "memory_size": 2147483648, // Increase to 2GB
        "eviction_policy": "volatile-lru",
        "password": "new-secure-password"
    });
    
    handler.update(db_id, updates).await
}
```

### Delete a Database

Permanently removes a database. This operation cannot be undone.

```rust
async fn delete_database(client: &EnterpriseClient, db_id: u32) -> Result<()> {
    let handler = BdbHandler::new(client.clone());
    
    // Optionally verify database is empty or backed up
    let info = handler.info(db_id).await?;
    if info.memory_used.unwrap_or(0) > 0 {
        println!("Warning: Database contains data!");
    }
    
    handler.delete(db_id).await
}
```

### Database Statistics

Retrieves performance metrics and statistics.

```rust
async fn get_database_stats(client: &EnterpriseClient, db_id: u32) -> Result<Value> {
    let handler = BdbHandler::new(client.clone());
    handler.stats(db_id).await
}
```

**Metrics include:**
- Operations per second
- Latency percentiles
- Memory fragmentation
- Network traffic
- Hit/miss ratios
- Connected clients

## Common Patterns

### Database Health Check

```rust
async fn check_database_health(client: &EnterpriseClient, db_id: u32) -> Result<bool> {
    let handler = BdbHandler::new(client.clone());
    let info = handler.info(db_id).await?;
    
    Ok(info.status == Some("active".to_string()) &&
       info.memory_used.unwrap_or(0) < info.memory_size.unwrap_or(u64::MAX))
}
```

### Find Database by Name

```rust
async fn find_database_by_name(
    client: &EnterpriseClient, 
    name: &str
) -> Result<Option<DatabaseInfo>> {
    let handler = BdbHandler::new(client.clone());
    let databases = handler.list().await?;
    
    Ok(databases.into_iter()
        .find(|db| db.name == name))
}
```

### Monitor Memory Usage

```rust
async fn monitor_database_memory(client: &EnterpriseClient) -> Result<()> {
    let handler = BdbHandler::new(client.clone());
    let databases = handler.list().await?;
    
    for db in databases {
        let used = db.memory_used.unwrap_or(0) as f64;
        let limit = db.memory_size.unwrap_or(1) as f64;
        let usage_pct = (used / limit) * 100.0;
        
        println!("Database '{}' ({}): {:.1}% memory used", 
                 db.name, db.uid, usage_pct);
                 
        if usage_pct > 90.0 {
            println!("  ⚠️  WARNING: High memory usage!");
        }
    }
    
    Ok(())
}
```

### Backup Configuration

```rust
async fn backup_database_config(
    client: &EnterpriseClient, 
    db_id: u32
) -> Result<String> {
    let handler = BdbHandler::new(client.clone());
    let info = handler.info(db_id).await?;
    
    // Serialize configuration for backup
    let config = serde_json::to_string_pretty(&info)?;
    
    // Save to file
    std::fs::write(format!("backup_db_{}.json", db_id), &config)?;
    
    Ok(config)
}
```

## CLI Examples

```bash
# List all databases
redis-enterprise database list

# Get database info
redis-enterprise database info 1

# Create a simple database
redis-enterprise database create \
  --name "my-cache" \
  --memory 1GB \
  --port 12000

# Create database with replication
redis-enterprise database create \
  --name "ha-database" \
  --memory 2GB \
  --replication \
  --persistence aof

# Update database memory
redis-enterprise database update 1 \
  --memory 4GB

# Delete a database
redis-enterprise database delete 1 --confirm

# Get database statistics
redis-enterprise database stats 1 --output json
```

## Best Practices

1. **Plan Memory Allocation**: Consider peak usage and leave headroom for operations
2. **Enable Replication**: For production databases, always enable replication
3. **Configure Persistence**: Choose appropriate persistence based on data criticality
4. **Set Eviction Policies**: Configure policies before reaching memory limits
5. **Monitor Metrics**: Regularly check stats to identify performance issues
6. **Use Sharding**: For databases larger than node memory, enable clustering
7. **Secure Databases**: Always set passwords and consider SSL/TLS
8. **Test Configuration**: Test database settings in dev before production

## Performance Considerations

- **Sharding**: Improves throughput but adds complexity
- **Persistence**: AOF safer but impacts performance vs snapshots
- **Replication**: Adds latency but crucial for HA
- **Modules**: Some modules (e.g., Search) require additional memory

## Error Handling

```rust
use redis_enterprise::RestError;

match handler.create(request).await {
    Ok(db) => println!("Created database: {}", db.name),
    Err(RestError::ApiError { code: 409, .. }) => {
        eprintln!("Database already exists");
    },
    Err(RestError::ApiError { code: 400, message }) => {
        eprintln!("Invalid configuration: {}", message);
    },
    Err(RestError::ApiError { code: 507, .. }) => {
        eprintln!("Insufficient resources in cluster");
    },
    Err(e) => eprintln!("Failed to create database: {}", e),
}
```

## See Also

For the raw REST API reference, see [Databases REST API Reference](../rest-api/databases.md).

## Related Endpoints

- [Shards](./shards.md) - Manage database shards
- [Endpoints](./endpoints.md) - Database connection endpoints
- [Redis ACLs](./redis-acls.md) - Access control lists
- [Modules](./modules.md) - Redis module management
- [Statistics](./stats.md) - Detailed performance metrics