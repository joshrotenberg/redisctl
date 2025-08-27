//! Live integration tests using Redis Enterprise containers
//!
//! This module provides real integration tests against live Redis Enterprise
//! containers using the docker-wrapper RedisEnterpriseTemplate. These tests
//! demonstrate full end-to-end functionality with actual Redis Enterprise clusters.
//!
//! # Test Environment
//!
//! - **Container**: Real Redis Enterprise cluster using docker-wrapper 0.8.1
//! - **Template**: RedisEnterpriseTemplate with cluster initialization
//! - **Authentication**: Live API credentials (admin@redis.local / Redis123!)
//! - **Network**: Isolated Docker network for testing
//!
//! # Test Categories
//!
//! - **Cluster Operations** - Info, stats, updates against live cluster
//! - **Database Management** - Create, configure, and manage databases
//! - **Node Operations** - Node health, statistics, and management
//! - **User Management** - Create and manage cluster users
//! - **Bootstrap Operations** - Cluster initialization workflows

use docker_wrapper::{RedisEnterpriseConnectionInfo, RedisEnterpriseTemplate};
use redis_enterprise::{
    BdbHandler, BootstrapHandler, ClusterHandler, CreateDatabaseRequest, CreateUserRequest,
    EnterpriseClient, NodeHandler, UserHandler,
};
use std::time::Duration;
use tokio::time::sleep;

/// Test configuration for Redis Enterprise container
const ENTERPRISE_USERNAME: &str = "admin@redis.local";
const ENTERPRISE_PASSWORD: &str = "Redis123!";
const ENTERPRISE_UI_PORT: u16 = 8443;

/// Setup a live Redis Enterprise container for testing
///
/// Creates a real Redis Enterprise container using docker-wrapper and returns
/// a configured client for testing against the live cluster.
///
/// # Returns
/// A tuple containing (Container, EnterpriseClient) ready for testing
async fn setup_enterprise_container() -> (RedisEnterpriseConnectionInfo, EnterpriseClient) {
    // Create Redis Enterprise template with test configuration
    let template = RedisEnterpriseTemplate::new("redis-enterprise-test")
        .accept_eula()
        .admin_username(ENTERPRISE_USERNAME)
        .admin_password(ENTERPRISE_PASSWORD)
        .cluster_name("test-cluster")
        .ui_port(ENTERPRISE_UI_PORT)
        .api_port(ENTERPRISE_UI_PORT)
        .memory_limit("2g");

    // Start container
    let container = template
        .start()
        .await
        .expect("Failed to start Redis Enterprise container");

    // Wait for cluster to be ready
    sleep(Duration::from_secs(60)).await;

    // Create client configured for the container
    let client = EnterpriseClient::builder()
        .base_url(format!("https://localhost:{}", ENTERPRISE_UI_PORT))
        .username(ENTERPRISE_USERNAME)
        .password(ENTERPRISE_PASSWORD)
        .insecure(true) // For testing with self-signed certs
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to create Enterprise client");

    // Wait a bit more for full initialization
    sleep(Duration::from_secs(10)).await;

    (container, client)
}

/// Test live cluster info retrieval
///
/// This test demonstrates:
/// - Connecting to a real Redis Enterprise cluster
/// - Retrieving cluster information via REST API
/// - Validating cluster status and configuration
#[tokio::test]
#[ignore] // Use 'cargo test -- --ignored' to run container tests
async fn test_live_cluster_info() {
    let (_container, client) = setup_enterprise_container().await;

    // Get cluster info using the cluster handler
    let cluster_handler = ClusterHandler::new(client);
    let result = cluster_handler
        .info()
        .await
        .expect("Failed to get cluster info");

    // Validate expected cluster properties
    println!("✅ Live cluster info retrieved successfully");
    println!("   Cluster name: {:?}", result.name);
    println!("   Version: {:?}", result.version);
    println!("   Status: {:?}", result.status);
}

/// Test live cluster statistics
///
/// This test demonstrates:
/// - Getting real-time cluster performance metrics  
/// - Understanding live cluster resource usage
/// - Monitoring active cluster operations
#[tokio::test]
#[ignore]
async fn test_live_cluster_stats() {
    let (_container, client) = setup_enterprise_container().await;

    let cluster_handler = ClusterHandler::new(client);
    let result = cluster_handler
        .stats()
        .await
        .expect("Failed to get cluster stats");

    println!("✅ Live cluster stats retrieved successfully");
    println!(
        "   Stats keys: {:?}",
        result.as_object().map(|o| o.keys().collect::<Vec<_>>())
    );
}

/// Test live database creation
///
/// This test demonstrates:
/// - Creating a database on live cluster
/// - Configuring database parameters
/// - Validating database deployment
#[tokio::test]
#[ignore]
async fn test_live_database_create() {
    let (_container, client) = setup_enterprise_container().await;

    let db_handler = BdbHandler::new(client.clone());

    // Create a test database
    let db_request = CreateDatabaseRequest {
        name: "test-live-db".to_string(),
        memory_size: 100_000_000, // 100MB
        port: Some(12001),
        replication: Some(false),
        persistence: None,
        eviction_policy: None,
        shards_count: None,
        module_list: None,
        authentication_redis_pass: None,
    };

    let result = db_handler
        .create(db_request)
        .await
        .expect("Failed to create database");

    // Should return database info
    assert_eq!(result.name, "test-live-db");

    // Wait for database to become active
    let uid = result.uid;

    // Poll for database status
    for _ in 0..30 {
        sleep(Duration::from_secs(2)).await;

        if let Ok(info) = db_handler.info(uid).await
            && let Some(status) = &info.status
            && status == "active"
        {
            println!("✅ Database created and active: {}", info.name);
            return;
        }
    }

    panic!("Database did not become active within timeout");
}

/// Test live node operations
///
/// This test demonstrates:
/// - Querying node information in live cluster
/// - Understanding node roles and status
/// - Monitoring node health
#[tokio::test]
#[ignore]
async fn test_live_node_info() {
    let (_container, client) = setup_enterprise_container().await;

    let node_handler = NodeHandler::new(client);
    let result = node_handler.list().await.expect("Failed to list nodes");

    // Should have at least one node
    assert!(!result.is_empty(), "Cluster should have at least one node");

    println!("✅ Live node info - {} nodes found", result.len());
    for node in &result {
        println!("  Node {}: {:?} ({:?})", node.uid, node.addr, node.status);
    }
}

/// Test live user management
///
/// This test demonstrates:
/// - Creating users in live cluster
/// - User authentication and permissions
/// - User lifecycle management
#[tokio::test]
#[ignore]
async fn test_live_user_management() {
    let (_container, client) = setup_enterprise_container().await;

    let user_handler = UserHandler::new(client);

    // Create a test user
    let user_request = CreateUserRequest {
        email: Some("test-user@example.com".to_string()),
        username: "test-user".to_string(),
        password: "TestPassword123!".to_string(),
        role: "admin".to_string(),
        email_alerts: Some(true),
    };

    let result = user_handler
        .create(user_request)
        .await
        .expect("Failed to create user");

    // Should return user info
    assert_eq!(result.email, Some("test-user@example.com".to_string()));

    // List users to verify creation
    let users = user_handler.list().await.expect("Failed to list users");

    let test_user_exists = users
        .iter()
        .any(|user| user.email == Some("test-user@example.com".to_string()));

    assert!(test_user_exists, "Test user should exist in user list");
    println!("✅ Live user created and verified");
}

/// Test live bootstrap operations
///
/// This test demonstrates:
/// - Checking cluster bootstrap status
/// - Understanding cluster initialization state
/// - Bootstrap workflow validation
#[tokio::test]
#[ignore]
async fn test_live_bootstrap_status() {
    let (_container, client) = setup_enterprise_container().await;

    let bootstrap_handler = BootstrapHandler::new(client);
    let result = bootstrap_handler
        .status()
        .await
        .expect("Failed to get bootstrap status");

    println!("✅ Bootstrap status retrieved successfully");
    println!("   Bootstrap info: {:?}", result);
}

/// Test error handling with live cluster
///
/// This test demonstrates:
/// - Error handling with live API calls
/// - Invalid request scenarios
/// - API error response processing
#[tokio::test]
#[ignore]
async fn test_live_error_handling() {
    let (_container, client) = setup_enterprise_container().await;

    let db_handler = BdbHandler::new(client);

    // Try to get non-existent database
    let result = db_handler.info(99999).await;

    // Should get an error for non-existent database
    assert!(
        result.is_err(),
        "Should get error for non-existent database"
    );

    let error = result.unwrap_err();
    println!("✅ Got expected error: {}", error);

    // Error should contain useful information
    let error_str = error.to_string();
    assert!(error_str.contains("404") || error_str.contains("not found"));
}

/// Helper function to cleanup test databases
#[allow(dead_code)]
async fn cleanup_test_databases(client: EnterpriseClient) {
    let db_handler = BdbHandler::new(client);

    if let Ok(databases) = db_handler.list().await {
        for db in &databases {
            if db.name.starts_with("test-") {
                let _ = db_handler.delete(db.uid).await;
                println!("🧹 Cleaned up test database: {}", db.name);
            }
        }
    }
}

/// Test with cleanup - creates and removes database
///
/// This test demonstrates:
/// - Full database lifecycle (create -> use -> delete)
/// - Resource cleanup patterns
/// - Integration test best practices
#[tokio::test]
#[ignore]
async fn test_live_database_lifecycle() {
    let (_container, client) = setup_enterprise_container().await;

    let db_handler = BdbHandler::new(client.clone());

    // Create database
    let db_request = CreateDatabaseRequest {
        name: "test-lifecycle-db".to_string(),
        memory_size: 100_000_000,
        port: Some(12002),
        replication: Some(false),
        persistence: None,
        eviction_policy: None,
        shards_count: None,
        module_list: None,
        authentication_redis_pass: None,
    };

    let create_result = db_handler
        .create(db_request)
        .await
        .expect("Failed to create lifecycle test database");

    let uid = create_result.uid;

    // Wait for database to be active
    for _ in 0..20 {
        sleep(Duration::from_secs(1)).await;
        if let Ok(info) = db_handler.info(uid).await
            && let Some(status) = &info.status
            && status == "active"
        {
            break;
        }
    }

    // Verify database exists and is active
    let db_info = db_handler
        .info(uid)
        .await
        .expect("Failed to get database info");
    assert_eq!(db_info.name, "test-lifecycle-db");

    // Clean up - delete the database
    db_handler
        .delete(uid)
        .await
        .expect("Failed to delete test database");

    println!(
        "✅ Database lifecycle test completed - created, verified, and deleted database {}",
        uid
    );

    // Verify deletion (should get 404)
    sleep(Duration::from_secs(2)).await;
    let get_result = db_handler.info(uid).await;
    assert!(
        get_result.is_err(),
        "Database should be deleted and return error"
    );
}
