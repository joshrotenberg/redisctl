//! Integration tests for Redis Enterprise database (BDB) commands
//!
//! This module demonstrates database management in Redis Enterprise.
//! Databases are the core data storage units that can be configured with
//! different persistence, replication, and performance settings.
//!
//! # Supported Operations
//!
//! - **List databases** - Get all databases with filtering and sorting
//! - **Get database info** - Retrieve detailed database configuration
//! - **Create database** - Set up new databases with custom settings
//! - **Update database** - Modify existing database configuration
//! - **Delete database** - Remove databases with confirmation
//! - **Get database stats** - Monitor database performance metrics
//! - **Wait for status** - Poll database until it reaches target state
//!
//! # Database Configuration
//!
//! ## Core Settings
//! - **Memory Size** - RAM allocation (e.g., "1GB", "512MB")
//! - **Port** - Database connection port (auto-assigned if not specified)
//! - **Persistence** - Data durability (aof, snapshot, disabled)
//! - **Replication** - High availability with replica shards
//! - **Eviction Policy** - Memory management strategy
//!
//! ## Advanced Features
//! - **Sharding** - Horizontal scaling across multiple shards
//! - **Redis Modules** - Enhanced functionality (Search, JSON, etc.)
//! - **TLS Encryption** - Secure client connections
//! - **Authentication** - Password protection and ACLs
//!
//! # Usage Examples
//!
//! ## List and filter databases
//! ```bash
//! redis-enterprise database list
//! redis-enterprise database list --status active
//! redis-enterprise database list --sort memory --output json
//! ```
//!
//! ## Create databases with different configurations
//! ```bash
//! # Simple cache database
//! redis-enterprise database create --name "cache" --memory "1GB" --port 12000
//! 
//! # Persistent database with replication
//! redis-enterprise database create --name "sessions" --memory "2GB" --replication --persistence aof
//!
//! # Complex database from JSON
//! redis-enterprise database create --from-json production_db.json
//! ```
//!
//! ## Monitor database performance
//! ```bash
//! redis-enterprise database get my-database
//! redis-enterprise database stats my-database --interval 1h
//! redis-enterprise database wait my-database --status active --timeout 300
//! ```

use redis_enterprise_cli::handlers::handle_database_command;
use redis_enterprise_cli::commands::DatabaseCommands;
use serde_json::json;
use wiremock::{
    matchers::{method, path, body_json, query_param},
    Mock, ResponseTemplate,
};

mod common;
use common::{setup_mock_server, create_temp_json_file};

/// Test listing all databases
/// 
/// This test demonstrates:
/// - Retrieving complete database inventory
/// - Understanding database status indicators
/// - Basic database information overview
#[tokio::test]
async fn test_database_list() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!([
        {
            "uid": 1,
            "name": "cache-db",
            "status": "active",
            "port": 12000,
            "memory_size": 1073741824u64,  // 1GB
            "used_memory": 536870912u64,   // 512MB
            "type": "redis",
            "persistence": "disabled",
            "replication": false,
            "shards_count": 1,
            "endpoint": "redis-12000.cluster.local:12000"
        },
        {
            "uid": 2,
            "name": "sessions-db", 
            "status": "active",
            "port": 12001,
            "memory_size": 2147483648u64,  // 2GB
            "used_memory": 1073741824u64,  // 1GB
            "type": "redis",
            "persistence": "aof",
            "replication": true,
            "shards_count": 2,
            "endpoint": "redis-12001.cluster.local:12001"
        },
        {
            "uid": 3,
            "name": "analytics-db",
            "status": "pending",
            "port": 12002,
            "memory_size": 4294967296u64,  // 4GB
            "used_memory": 0,
            "type": "redis",
            "persistence": "snapshot", 
            "replication": true,
            "shards_count": 4,
            "endpoint": "redis-12002.cluster.local:12002"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/v1/bdbs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::List {
        status: None,
        sort: None,
    };
    let result = handle_database_command(&client, command).await.unwrap();

    let databases = result.as_array().unwrap();
    assert_eq!(databases.len(), 3);
    
    // Verify first database (cache)
    assert_eq!(databases[0]["name"], "cache-db");
    assert_eq!(databases[0]["status"], "active");
    assert_eq!(databases[0]["memory_size"], 1073741824u64);
    assert_eq!(databases[0]["persistence"], "disabled");
    assert_eq!(databases[0]["replication"], false);
    
    // Verify second database (sessions)
    assert_eq!(databases[1]["name"], "sessions-db");
    assert_eq!(databases[1]["persistence"], "aof");
    assert_eq!(databases[1]["replication"], true);
    assert_eq!(databases[1]["shards_count"], 2);
    
    // Verify third database (analytics)
    assert_eq!(databases[2]["name"], "analytics-db");
    assert_eq!(databases[2]["status"], "pending");
    assert_eq!(databases[2]["shards_count"], 4);
}

/// Test listing databases with status filter
/// 
/// This test demonstrates:
/// - Filtering databases by operational status
/// - Client-side filtering implementation
/// - Status-based database management
#[tokio::test]
async fn test_database_list_filtered() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!([
        {
            "uid": 1,
            "name": "active-db-1",
            "status": "active",
            "memory_size": 1073741824u64,
            "port": 12000
        },
        {
            "uid": 2,
            "name": "pending-db",
            "status": "pending", 
            "memory_size": 2147483648u64,
            "port": 12001
        },
        {
            "uid": 3,
            "name": "active-db-2", 
            "status": "active",
            "memory_size": 1073741824u64,
            "port": 12002
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/v1/bdbs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::List {
        status: Some("active".to_string()),
        sort: None,
    };
    let result = handle_database_command(&client, command).await.unwrap();

    let databases = result.as_array().unwrap();
    assert_eq!(databases.len(), 2); // Only active databases
    assert_eq!(databases[0]["name"], "active-db-1");
    assert_eq!(databases[1]["name"], "active-db-2");
    
    // Verify all returned databases have active status
    for db in databases {
        assert_eq!(db["status"], "active");
    }
}

/// Test getting specific database information
/// 
/// This test demonstrates:
/// - Retrieving detailed database configuration
/// - Understanding comprehensive database settings
/// - Database-specific feature information
#[tokio::test]
async fn test_database_get() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "uid": 1,
        "name": "production-db",
        "status": "active",
        "port": 12000,
        "memory_size": 8589934592u64,  // 8GB
        "used_memory": 4294967296u64,  // 4GB
        "type": "redis", 
        "persistence": "aof",
        "replication": true,
        "shards_count": 3,
        "eviction_policy": "allkeys-lru",
        "endpoint": "redis-12000.cluster.local:12000",
        "modules": [
            {
                "module_name": "search",
                "version": "2.6.0"
            },
            {
                "module_name": "json", 
                "version": "2.4.0"
            }
        ],
        "security": {
            "tls_enabled": true,
            "auth_enabled": true,
            "acl_enabled": true
        },
        "backup_policy": {
            "backup_enabled": true,
            "backup_interval": "24h",
            "backup_retention": "7d"
        },
        "created_time": "2024-08-01T10:00:00Z",
        "last_backup_time": "2024-08-20T02:00:00Z"
    });

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::Get {
        database: "1".to_string(),
    };
    let result = handle_database_command(&client, command).await.unwrap();

    assert_eq!(result["name"], "production-db");
    assert_eq!(result["status"], "active");
    assert_eq!(result["memory_size"], 8589934592u64);
    assert_eq!(result["used_memory"], 4294967296u64);
    assert_eq!(result["persistence"], "aof");
    assert_eq!(result["replication"], true);
    assert_eq!(result["shards_count"], 3);
    assert_eq!(result["eviction_policy"], "allkeys-lru");
    
    // Verify modules
    assert_eq!(result["modules"].as_array().unwrap().len(), 2);
    assert_eq!(result["modules"][0]["module_name"], "search");
    assert_eq!(result["modules"][1]["module_name"], "json");
    
    // Verify security settings
    assert_eq!(result["security"]["tls_enabled"], true);
    assert_eq!(result["security"]["auth_enabled"], true);
    assert_eq!(result["security"]["acl_enabled"], true);
    
    // Verify backup configuration
    assert_eq!(result["backup_policy"]["backup_enabled"], true);
    assert_eq!(result["backup_policy"]["backup_interval"], "24h");
}

/// Test creating database with CLI arguments
/// 
/// This test demonstrates:
/// - Creating databases with basic parameters
/// - Using builder pattern for database creation
/// - Simple database setup workflow
#[tokio::test]
async fn test_database_create_with_args() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "uid": 5,
        "name": "new-cache-db",
        "status": "pending",
        "port": 12005,
        "memory_size": 1073741824u64,  // 1GB
        "type": "redis",
        "persistence": "disabled",
        "replication": false,
        "shards_count": 1,
        "eviction_policy": "allkeys-lru",
        "endpoint": "redis-12005.cluster.local:12005"
    });

    Mock::given(method("POST"))
        .and(path("/v1/bdbs"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::Create {
        name: Some("new-cache-db".to_string()),
        memory: Some("1GB".to_string()),
        port: Some(12005),
        replication: false,
        persistence: None, // Should default appropriately
        eviction_policy: Some("allkeys-lru".to_string()),
        shards: None,
        from_json: None,
    };
    let result = handle_database_command(&client, command).await.unwrap();

    assert_eq!(result["uid"], 5);
    assert_eq!(result["name"], "new-cache-db");
    assert_eq!(result["status"], "pending");
    assert_eq!(result["port"], 12005);
    assert_eq!(result["memory_size"], 1073741824u64);
    assert_eq!(result["persistence"], "disabled");
    assert_eq!(result["replication"], false);
    assert_eq!(result["eviction_policy"], "allkeys-lru");
}

/// Test creating database from JSON configuration
/// 
/// This test demonstrates:
/// - Complex database configuration via JSON
/// - Advanced database feature setup
/// - Production-ready database creation
#[tokio::test]
async fn test_database_create_from_json() {
    let (mock_server, client) = setup_mock_server().await;

    // Create comprehensive database configuration
    let database_config = json!({
        "name": "advanced-prod-db",
        "memory_size": 17179869184u64,  // 16GB
        "port": 12010,
        "type": "redis",
        "persistence": "aof",
        "replication": true,
        "shards_count": 4,
        "eviction_policy": "volatile-lru",
        "modules": [
            {"module_name": "search", "version": "2.6.0"},
            {"module_name": "json", "version": "2.4.0"},
            {"module_name": "timeseries", "version": "1.8.0"}
        ],
        "security": {
            "password": "secure-redis-password",
            "tls_enabled": true,
            "client_auth_required": true
        },
        "backup_policy": {
            "backup_enabled": true,
            "backup_interval": "12h",
            "backup_retention": "14d",
            "backup_location": "s3://company-backups/redis/"
        },
        "alert_settings": {
            "memory_threshold": 85,
            "latency_threshold": 5.0,
            "connection_threshold": 1000
        }
    });
    
    let temp_file = create_temp_json_file(database_config.clone());

    let mock_response = json!({
        "uid": 10,
        "name": "advanced-prod-db",
        "status": "pending",
        "port": 12010,
        "memory_size": 17179869184u64,
        "type": "redis",
        "persistence": "aof", 
        "replication": true,
        "shards_count": 4,
        "eviction_policy": "volatile-lru",
        "modules": [
            {"module_name": "search", "version": "2.6.0"},
            {"module_name": "json", "version": "2.4.0"},
            {"module_name": "timeseries", "version": "1.8.0"}
        ],
        "security": {
            "tls_enabled": true,
            "client_auth_required": true,
            "password_protected": true
        },
        "backup_policy": {
            "backup_enabled": true,
            "backup_interval": "12h", 
            "backup_retention": "14d",
            "backup_location": "s3://company-backups/redis/"
        },
        "endpoint": "redis-12010.cluster.local:12010"
    });

    Mock::given(method("POST"))
        .and(path("/v1/bdbs"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::Create {
        name: Some("ignored".to_string()), // Should be ignored when using from_json
        memory: None,
        port: None,
        replication: false,
        persistence: None,
        eviction_policy: None,
        shards: None,
        from_json: Some(temp_file.path().to_path_buf()),
    };
    let result = handle_database_command(&client, command).await.unwrap();

    assert_eq!(result["name"], "advanced-prod-db");
    assert_eq!(result["memory_size"], 17179869184u64);
    assert_eq!(result["port"], 12010);
    assert_eq!(result["shards_count"], 4);
    assert_eq!(result["modules"].as_array().unwrap().len(), 3);
    assert_eq!(result["security"]["tls_enabled"], true);
    assert_eq!(result["backup_policy"]["backup_enabled"], true);
}

/// Test updating database configuration
/// 
/// This test demonstrates:
/// - Modifying existing database settings
/// - Memory and policy updates
/// - Database reconfiguration patterns
#[tokio::test]
async fn test_database_update() {
    let (mock_server, client) = setup_mock_server().await;

    // Mock getting existing database
    let existing_db = json!({
        "uid": 1,
        "name": "test-db",
        "memory_size": 1073741824u64,  // 1GB
        "eviction_policy": "noeviction"
    });

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/test-db"))
        .respond_with(ResponseTemplate::new(404)) // Not found by name
        .mount(&mock_server)
        .await;

    // Try by ID parsing
    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&existing_db))
        .mount(&mock_server)
        .await;

    let update_request = json!({
        "memory_size": 2147483648u64,  // 2GB
        "eviction_policy": "allkeys-lru"
    });

    let mock_response = json!({
        "uid": 1,
        "name": "test-db",
        "status": "active",
        "memory_size": 2147483648u64,  // Updated to 2GB
        "eviction_policy": "allkeys-lru",  // Updated policy
        "port": 12001
    });

    Mock::given(method("PUT"))
        .and(path("/v1/bdbs/1"))
        .and(body_json(&update_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::Update {
        database: "1".to_string(),
        memory: Some("2GB".to_string()),
        eviction_policy: Some("allkeys-lru".to_string()),
        from_json: None,
    };
    let result = handle_database_command(&client, command).await.unwrap();

    assert_eq!(result["memory_size"], 2147483648u64);
    assert_eq!(result["eviction_policy"], "allkeys-lru");
    assert_eq!(result["uid"], 1);
}

/// Test database deletion with confirmation
/// 
/// This test demonstrates:
/// - Safe database deletion workflow
/// - Confirmation bypass for automation
/// - Database removal operations
#[tokio::test]
async fn test_database_delete() {
    let (mock_server, client) = setup_mock_server().await;

    // Mock getting database for confirmation
    let existing_db = json!({
        "uid": 3,
        "name": "old-test-db"
    });

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&existing_db))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/v1/bdbs/3"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::Delete {
        database: "3".to_string(),
        yes: true, // Skip confirmation
    };
    let result = handle_database_command(&client, command).await.unwrap();

    assert_eq!(result["deleted"], true);
    assert_eq!(result["database"], "old-test-db");
}

/// Test database statistics collection
/// 
/// This test demonstrates:
/// - Database performance monitoring
/// - Statistics collection with intervals
/// - Database-specific metrics
#[tokio::test]
async fn test_database_stats() {
    let (mock_server, client) = setup_mock_server().await;

    // Mock getting database first
    let existing_db = json!({
        "uid": 2,
        "name": "stats-test-db"
    });

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/stats-test-db"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&existing_db))
        .mount(&mock_server)
        .await;

    let stats_response = json!({
        "interval": "1h",
        "timestamp": "2024-08-20T12:00:00Z",
        "ops_per_sec": 1250.5,
        "read_ops_per_sec": 875.3,
        "write_ops_per_sec": 375.2,
        "avg_latency_ms": 0.85,
        "p99_latency_ms": 3.2,
        "connections": 45,
        "memory_usage": {
            "used_memory": 536870912u64,    // 512MB
            "allocated_memory": 1073741824u64, // 1GB
            "usage_percent": 50.0
        },
        "hit_rate_percent": 94.5,
        "evicted_keys": 125,
        "expired_keys": 89
    });

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/2/stats"))
        .and(query_param("interval", "1h"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&stats_response))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::Stats {
        database: "2".to_string(),
        interval: Some("1h".to_string()),
    };
    let result = handle_database_command(&client, command).await.unwrap();

    assert_eq!(result["ops_per_sec"], 1250.5);
    assert_eq!(result["read_ops_per_sec"], 875.3);
    assert_eq!(result["write_ops_per_sec"], 375.2);
    assert_eq!(result["avg_latency_ms"], 0.85);
    assert_eq!(result["connections"], 45);
    assert_eq!(result["hit_rate_percent"], 94.5);
    assert_eq!(result["memory_usage"]["usage_percent"], 50.0);
}

/// Test waiting for database status
/// 
/// This test demonstrates:
/// - Polling database until target status reached
/// - Timeout handling for status changes
/// - Database provisioning workflows
#[tokio::test]
async fn test_database_wait_success() {
    let (mock_server, client) = setup_mock_server().await;

    // First call - database still pending
    let pending_response = json!({
        "uid": 4,
        "name": "wait-test-db",
        "status": "pending"
    });

    // Second call - database now active
    let active_response = json!({
        "uid": 4,
        "name": "wait-test-db", 
        "status": "active",
        "port": 12004,
        "endpoint": "redis-12004.cluster.local:12004"
    });

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/wait-test-db"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    // First call returns pending
    Mock::given(method("GET"))
        .and(path("/v1/bdbs/4"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&pending_response))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // Subsequent calls return active
    Mock::given(method("GET"))
        .and(path("/v1/bdbs/4"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&active_response))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::Wait {
        database: "4".to_string(),
        status: "active".to_string(),
        timeout: 10, // Short timeout for testing
    };
    let result = handle_database_command(&client, command).await.unwrap();

    assert_eq!(result["status"], "active");
    assert_eq!(result["uid"], 4);
    assert_eq!(result["name"], "wait-test-db");
}

/// Test database creation validation errors
/// 
/// This test demonstrates:
/// - Handling invalid database configuration
/// - Parameter validation errors
/// - Database creation constraints
#[tokio::test]
async fn test_database_create_missing_args() {
    let (_, client) = setup_mock_server().await;

    let command = DatabaseCommands::Create {
        name: None, // Missing required name
        memory: None, // Missing required memory
        port: None,
        replication: false,
        persistence: None,
        eviction_policy: None,
        shards: None,
        from_json: None,
    };
    let result = handle_database_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("--name required"));
}

/// Test database not found error
/// 
/// This test demonstrates:
/// - Handling non-existent database references
/// - Database lookup failure scenarios
/// - Error response processing
#[tokio::test]
async fn test_database_get_not_found() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/nonexistent"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    // Also mock numeric ID attempt
    Mock::given(method("GET"))
        .and(path("/v1/bdbs/999"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::Get {
        database: "nonexistent".to_string(),
    };
    let result = handle_database_command(&client, command).await;

    assert!(result.is_err());
}

/// Test database wait timeout
/// 
/// This test demonstrates:
/// - Timeout handling during status polling
/// - Long-running operation management
/// - Database provisioning time limits
#[tokio::test]
async fn test_database_wait_timeout() {
    let (mock_server, client) = setup_mock_server().await;

    // Database stays in pending state (never reaches active)
    let pending_response = json!({
        "uid": 5,
        "name": "slow-db",
        "status": "pending"
    });

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/slow-db"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&pending_response))
        .mount(&mock_server)
        .await;

    let command = DatabaseCommands::Wait {
        database: "5".to_string(),
        status: "active".to_string(),
        timeout: 1, // Very short timeout to trigger timeout quickly
    };
    let result = handle_database_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Timeout"));
}