//! Integration tests for Redis Enterprise bootstrap commands
//!
//! This module demonstrates cluster bootstrap operations in Redis Enterprise.
//! Bootstrap commands handle the initial setup and configuration of a Redis
//! Enterprise cluster, including node registration and cluster formation.
//!
//! # Supported Operations
//!
//! - **Get bootstrap status** - Check cluster initialization progress
//! - **Raw bootstrap requests** - Send custom bootstrap configuration
//!
//! # Bootstrap Process
//!
//! ## Cluster Initialization
//! - **Node Discovery** - Identify available nodes for cluster formation
//! - **Network Configuration** - Set up inter-node communication
//! - **Security Setup** - Configure authentication and encryption
//! - **Resource Allocation** - Define memory and storage limits
//!
//! ## Bootstrap Status
//! - `not_started` - Bootstrap process has not begun
//! - `in_progress` - Currently forming cluster
//! - `completed` - Cluster successfully initialized
//! - `failed` - Bootstrap encountered errors
//!
//! # Usage Examples
//!
//! ## Check bootstrap status
//! ```bash
//! redis-enterprise bootstrap status
//! redis-enterprise bootstrap status --query 'status'
//! ```
//!
//! ## Initialize cluster with raw JSON
//! ```bash
//! # Basic cluster initialization
//! redis-enterprise bootstrap raw --body '{
//!   "action": "create_cluster",
//!   "cluster": {
//!     "name": "production-cluster",
//!     "username": "admin@company.com",
//!     "password": "secure_password"
//!   }
//! }'
//! ```
//!
//! ## Initialize cluster from configuration file
//! ```bash
//! redis-enterprise bootstrap raw --from-json cluster_init.json
//! ```
//!
//! Example cluster_init.json:
//! ```json
//! {
//!   "action": "create_cluster",
//!   "cluster": {
//!     "name": "Redis Enterprise Cluster",
//!     "username": "admin@company.com",
//!     "password": "SecurePassword123",
//!     "rack_aware": true,
//!     "license": "ENTERPRISE-LICENSE-KEY..."
//!   },
//!   "node": {
//!     "external_addr": "192.168.1.10",
//!     "internal_addr": "10.0.1.10"
//!   }
//! }
//! ```

use redis_enterprise_cli::handlers::handle_bootstrap_command;
use redis_enterprise_cli::commands::BootstrapCommands;
use serde_json::json;
use wiremock::{
    matchers::{method, path, body_json},
    Mock, ResponseTemplate,
};

mod common;
use common::{setup_mock_server, create_temp_json_file};

/// Test getting bootstrap status - not started
/// 
/// This test demonstrates:
/// - Checking cluster initialization status
/// - Understanding bootstrap process stages
/// - Cluster readiness assessment
#[tokio::test]
async fn test_bootstrap_status_not_started() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "status": "not_started",
        "message": "Cluster bootstrap has not been initiated",
        "available_nodes": [
            {
                "ip": "10.0.1.10",
                "hostname": "redis-node1.local",
                "status": "available"
            },
            {
                "ip": "10.0.1.11", 
                "hostname": "redis-node2.local",
                "status": "available"
            },
            {
                "ip": "10.0.1.12",
                "hostname": "redis-node3.local", 
                "status": "available"
            }
        ],
        "requirements": {
            "min_nodes": 1,
            "recommended_nodes": 3,
            "available_memory": "48GB",
            "available_storage": "1TB"
        }
    });

    Mock::given(method("GET"))
        .and(path("/v1/bootstrap"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = BootstrapCommands::Status;
    let result = handle_bootstrap_command(&client, command).await.unwrap();

    assert_eq!(result["status"], "not_started");
    assert_eq!(result["message"], "Cluster bootstrap has not been initiated");
    
    // Verify available nodes
    let nodes = result["available_nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0]["ip"], "10.0.1.10");
    assert_eq!(nodes[0]["status"], "available");
    assert_eq!(nodes[1]["hostname"], "redis-node2.local");
    assert_eq!(nodes[2]["ip"], "10.0.1.12");
    
    // Verify requirements
    assert_eq!(result["requirements"]["min_nodes"], 1);
    assert_eq!(result["requirements"]["recommended_nodes"], 3);
    assert_eq!(result["requirements"]["available_memory"], "48GB");
}

/// Test getting bootstrap status - in progress
/// 
/// This test demonstrates:
/// - Monitoring ongoing cluster initialization
/// - Understanding bootstrap progress indicators
/// - Cluster formation process tracking
#[tokio::test]
async fn test_bootstrap_status_in_progress() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "status": "in_progress",
        "message": "Cluster initialization is in progress",
        "progress": {
            "current_step": "configuring_network",
            "steps_completed": 2,
            "total_steps": 5,
            "progress_percent": 40.0
        },
        "steps": [
            {
                "name": "validate_nodes",
                "status": "completed",
                "duration_ms": 1500
            },
            {
                "name": "setup_security",
                "status": "completed", 
                "duration_ms": 3200
            },
            {
                "name": "configuring_network",
                "status": "in_progress",
                "started_at": "2024-08-20T12:00:00Z"
            },
            {
                "name": "initialize_storage",
                "status": "pending"
            },
            {
                "name": "finalize_cluster",
                "status": "pending"
            }
        ],
        "estimated_completion": "2024-08-20T12:05:00Z"
    });

    Mock::given(method("GET"))
        .and(path("/v1/bootstrap"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = BootstrapCommands::Status;
    let result = handle_bootstrap_command(&client, command).await.unwrap();

    assert_eq!(result["status"], "in_progress");
    assert_eq!(result["progress"]["current_step"], "configuring_network");
    assert_eq!(result["progress"]["steps_completed"], 2);
    assert_eq!(result["progress"]["total_steps"], 5);
    assert_eq!(result["progress"]["progress_percent"], 40.0);
    
    // Verify step details
    let steps = result["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 5);
    assert_eq!(steps[0]["status"], "completed");
    assert_eq!(steps[1]["status"], "completed");
    assert_eq!(steps[2]["status"], "in_progress");
    assert_eq!(steps[3]["status"], "pending");
    assert_eq!(steps[4]["status"], "pending");
}

/// Test getting bootstrap status - completed
/// 
/// This test demonstrates:
/// - Successful cluster initialization completion
/// - Understanding post-bootstrap cluster state
/// - Cluster readiness confirmation
#[tokio::test]
async fn test_bootstrap_status_completed() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "status": "completed",
        "message": "Cluster successfully initialized",
        "cluster_info": {
            "name": "Redis Enterprise Production",
            "uid": "12345678-1234-5678-9abc-123456789abc",
            "nodes": 3,
            "created_at": "2024-08-20T12:00:00Z",
            "bootstrap_duration_ms": 45000
        },
        "endpoints": {
            "admin_console": "https://redis-cluster.local:8443",
            "api_url": "https://redis-cluster.local:9443",
            "database_endpoint": "redis-cluster.local:12000"
        },
        "next_steps": [
            "Configure DNS for cluster endpoints",
            "Upload enterprise license",
            "Create your first database",
            "Set up monitoring and alerts"
        ]
    });

    Mock::given(method("GET"))
        .and(path("/v1/bootstrap"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = BootstrapCommands::Status;
    let result = handle_bootstrap_command(&client, command).await.unwrap();

    assert_eq!(result["status"], "completed");
    assert_eq!(result["message"], "Cluster successfully initialized");
    assert_eq!(result["cluster_info"]["name"], "Redis Enterprise Production");
    assert_eq!(result["cluster_info"]["nodes"], 3);
    assert_eq!(result["cluster_info"]["bootstrap_duration_ms"], 45000);
    
    // Verify endpoints
    assert_eq!(result["endpoints"]["admin_console"], "https://redis-cluster.local:8443");
    assert_eq!(result["endpoints"]["api_url"], "https://redis-cluster.local:9443");
    
    // Verify next steps
    let next_steps = result["next_steps"].as_array().unwrap();
    assert_eq!(next_steps.len(), 4);
    assert_eq!(next_steps[0], "Configure DNS for cluster endpoints");
    assert_eq!(next_steps[1], "Upload enterprise license");
}

/// Test bootstrap raw request with CLI body
/// 
/// This test demonstrates:
/// - Initiating cluster bootstrap with JSON configuration
/// - Basic cluster creation workflow
/// - Bootstrap request processing
#[tokio::test]
async fn test_bootstrap_raw_with_body() {
    let (mock_server, client) = setup_mock_server().await;

    let request_body = json!({
        "action": "create_cluster",
        "cluster": {
            "name": "Test Cluster",
            "username": "admin@test.com",
            "password": "testpassword123"
        },
        "node": {
            "external_addr": "192.168.1.100",
            "internal_addr": "10.0.1.100"
        }
    });

    let mock_response = json!({
        "status": "initiated",
        "message": "Cluster bootstrap process started",
        "bootstrap_id": "bootstrap-12345",
        "cluster": {
            "name": "Test Cluster",
            "estimated_completion": "2024-08-20T12:05:00Z"
        },
        "progress_url": "/v1/bootstrap/status/bootstrap-12345"
    });

    Mock::given(method("POST"))
        .and(path("/v1/bootstrap"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(202).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = BootstrapCommands::Raw {
        body: Some(serde_json::to_string(&request_body).unwrap()),
        from_json: None,
    };
    let result = handle_bootstrap_command(&client, command).await.unwrap();

    assert_eq!(result["status"], "initiated");
    assert_eq!(result["message"], "Cluster bootstrap process started");
    assert_eq!(result["bootstrap_id"], "bootstrap-12345");
    assert_eq!(result["cluster"]["name"], "Test Cluster");
}

/// Test bootstrap raw request from JSON file
/// 
/// This test demonstrates:
/// - Complex cluster initialization from configuration file
/// - Advanced bootstrap settings and features
/// - File-based cluster configuration workflow
#[tokio::test]
async fn test_bootstrap_raw_from_json_file() {
    let (mock_server, client) = setup_mock_server().await;

    // Create comprehensive bootstrap configuration
    let bootstrap_config = json!({
        "action": "create_cluster",
        "cluster": {
            "name": "Production Redis Enterprise",
            "username": "admin@company.com",
            "password": "SecurePassword123!",
            "rack_aware": true,
            "flash_enabled": true,
            "license": "ENTERPRISE-2024-ABC123..."
        },
        "node": {
            "external_addr": "192.168.1.10",
            "internal_addr": "10.0.1.10",
            "rack_id": "rack-1"
        },
        "network": {
            "cluster_port": 9443,
            "data_port_range": "10000-19999",
            "discovery_port": 8001
        },
        "security": {
            "cipher_suites": ["TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384"],
            "min_tls_version": "1.2",
            "require_ssl": true
        },
        "persistence": {
            "persistent_path": "/var/opt/redislabs/persist",
            "backup_path": "/var/opt/redislabs/backups"
        }
    });
    
    let temp_file = create_temp_json_file(bootstrap_config.clone());

    let mock_response = json!({
        "status": "initiated",
        "message": "Advanced cluster bootstrap process started",
        "bootstrap_id": "bootstrap-advanced-67890",
        "cluster": {
            "name": "Production Redis Enterprise",
            "features": ["rack_aware", "flash_enabled", "ssl_enabled"],
            "estimated_completion": "2024-08-20T12:10:00Z"
        },
        "security": {
            "ssl_enabled": true,
            "min_tls_version": "1.2"
        },
        "progress_url": "/v1/bootstrap/status/bootstrap-advanced-67890"
    });

    Mock::given(method("POST"))
        .and(path("/v1/bootstrap"))
        .and(body_json(&bootstrap_config))
        .respond_with(ResponseTemplate::new(202).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = BootstrapCommands::Raw {
        body: None,
        from_json: Some(temp_file.path().to_path_buf()),
    };
    let result = handle_bootstrap_command(&client, command).await.unwrap();

    assert_eq!(result["status"], "initiated");
    assert_eq!(result["cluster"]["name"], "Production Redis Enterprise");
    assert_eq!(result["security"]["ssl_enabled"], true);
    assert_eq!(result["security"]["min_tls_version"], "1.2");
    
    // Verify advanced features
    let features = result["cluster"]["features"].as_array().unwrap();
    assert_eq!(features.len(), 3);
    assert!(features.contains(&json!("rack_aware")));
    assert!(features.contains(&json!("flash_enabled")));
    assert!(features.contains(&json!("ssl_enabled")));
}

/// Test bootstrap status with failed state
/// 
/// This test demonstrates:
/// - Handling bootstrap failure scenarios
/// - Understanding failure reasons and recovery
/// - Bootstrap error diagnosis
#[tokio::test]
async fn test_bootstrap_status_failed() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "status": "failed",
        "message": "Cluster bootstrap failed",
        "error": {
            "code": "INSUFFICIENT_RESOURCES",
            "message": "Not enough memory available for cluster initialization",
            "details": "Minimum 8GB RAM required per node, only 4GB available"
        },
        "failed_step": "initialize_storage",
        "failure_time": "2024-08-20T12:03:30Z",
        "recovery_suggestions": [
            "Ensure each node has at least 8GB RAM available",
            "Free up disk space on persistent storage paths",
            "Check network connectivity between nodes",
            "Verify license key is valid and not expired"
        ],
        "logs": [
            {
                "timestamp": "2024-08-20T12:00:00Z",
                "level": "info",
                "message": "Starting cluster bootstrap process"
            },
            {
                "timestamp": "2024-08-20T12:01:15Z",
                "level": "warn",
                "message": "Low memory detected on node 10.0.1.10"
            },
            {
                "timestamp": "2024-08-20T12:03:30Z",
                "level": "error",
                "message": "Bootstrap failed: insufficient memory for storage initialization"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/v1/bootstrap"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = BootstrapCommands::Status;
    let result = handle_bootstrap_command(&client, command).await.unwrap();

    assert_eq!(result["status"], "failed");
    assert_eq!(result["error"]["code"], "INSUFFICIENT_RESOURCES");
    assert_eq!(result["failed_step"], "initialize_storage");
    
    // Verify recovery suggestions
    let suggestions = result["recovery_suggestions"].as_array().unwrap();
    assert_eq!(suggestions.len(), 4);
    assert_eq!(suggestions[0], "Ensure each node has at least 8GB RAM available");
    
    // Verify logs
    let logs = result["logs"].as_array().unwrap();
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0]["level"], "info");
    assert_eq!(logs[1]["level"], "warn");
    assert_eq!(logs[2]["level"], "error");
}

/// Test bootstrap raw request validation error
/// 
/// This test demonstrates:
/// - Handling invalid bootstrap configuration
/// - Configuration validation and error reporting
/// - Bootstrap parameter constraints
#[tokio::test]
async fn test_bootstrap_raw_validation_error() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v1/bootstrap"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "Invalid bootstrap configuration",
            "details": "Required fields are missing or have invalid values",
            "validation_errors": [
                {
                    "field": "cluster.username",
                    "message": "Must be a valid email address"
                },
                {
                    "field": "cluster.password",
                    "message": "Password must be at least 8 characters"
                },
                {
                    "field": "node.external_addr",
                    "message": "Must be a valid IP address or hostname"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let invalid_body = json!({
        "action": "create_cluster",
        "cluster": {
            "name": "Test",
            "username": "invalid-email",
            "password": "weak"
        },
        "node": {
            "external_addr": "invalid-ip"
        }
    });

    let command = BootstrapCommands::Raw {
        body: Some(serde_json::to_string(&invalid_body).unwrap()),
        from_json: None,
    };
    let result = handle_bootstrap_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("400"));
}

/// Test bootstrap raw missing arguments
/// 
/// This test demonstrates:
/// - Command line argument validation
/// - Error handling for incomplete commands
/// - Required parameter checking
#[tokio::test]
async fn test_bootstrap_raw_missing_args() {
    let (_, client) = setup_mock_server().await;

    let command = BootstrapCommands::Raw {
        body: None,
        from_json: None,
    };
    let result = handle_bootstrap_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Either --body or --from-json required"));
}

/// Test bootstrap raw file not found
/// 
/// This test demonstrates:
/// - File handling error scenarios
/// - JSON configuration file validation
/// - File system error handling
#[tokio::test]
async fn test_bootstrap_raw_file_not_found() {
    let (_, client) = setup_mock_server().await;

    let command = BootstrapCommands::Raw {
        body: None,
        from_json: Some("/nonexistent/bootstrap.json".into()),
    };
    let result = handle_bootstrap_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("No such file"));
}

/// Test bootstrap server error
/// 
/// This test demonstrates:
/// - Handling server errors during bootstrap operations
/// - Understanding bootstrap service failures
/// - Error response processing
#[tokio::test]
async fn test_bootstrap_server_error() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v1/bootstrap"))
        .respond_with(ResponseTemplate::new(503).set_body_json(json!({
            "error": "Service unavailable",
            "details": "Bootstrap service is temporarily down for maintenance"
        })))
        .mount(&mock_server)
        .await;

    let command = BootstrapCommands::Status;
    let result = handle_bootstrap_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("503"));
}

/// Test bootstrap raw with malformed JSON
/// 
/// This test demonstrates:
/// - JSON parsing error handling
/// - Invalid configuration format detection
/// - Client-side validation errors
#[tokio::test]
async fn test_bootstrap_raw_malformed_json() {
    let (_, client) = setup_mock_server().await;

    let command = BootstrapCommands::Raw {
        body: Some("{invalid json}".to_string()),
        from_json: None,
    };
    let result = handle_bootstrap_command(&client, command).await;

    assert!(result.is_err());
    // Should fail during JSON parsing before making any network request
}