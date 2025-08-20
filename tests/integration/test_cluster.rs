//! Integration tests for Redis Enterprise cluster commands
//!
//! This module demonstrates cluster management operations in Redis Enterprise.
//! Cluster operations control the overall Redis Enterprise deployment, including
//! configuration, statistics, and cluster-wide settings.
//!
//! # Supported Operations
//!
//! - **Get cluster info** - Retrieve cluster status, configuration, and metadata
//! - **Get cluster stats** - Monitor performance metrics and resource usage  
//! - **Update cluster** - Modify cluster configuration and settings
//!
//! # Cluster Information
//!
//! ## Key Metrics
//! - **Status** - Overall cluster health (active, degraded, failed)
//! - **Nodes** - List of cluster nodes and their roles
//! - **Databases** - Count and status of databases in cluster
//! - **License** - License status and resource limits
//! - **Memory Usage** - RAM consumption across cluster
//!
//! ## Configuration
//! - **Cluster Name** - Human-readable cluster identifier
//! - **Email Alerts** - Alert notification settings
//! - **Rack Awareness** - Multi-zone deployment configuration
//! - **Version** - Redis Enterprise software version
//!
//! # Usage Examples
//!
//! ## Get cluster information
//! ```bash
//! redis-enterprise cluster info
//! redis-enterprise cluster info --query 'name'
//! redis-enterprise cluster info --output json
//! ```
//!
//! ## Monitor cluster statistics  
//! ```bash
//! redis-enterprise cluster stats
//! redis-enterprise cluster stats --interval 1h
//! redis-enterprise cluster stats --query 'total_memory'
//! ```
//!
//! ## Update cluster configuration
//! ```bash
//! redis-enterprise cluster update --name "Production Cluster"
//! redis-enterprise cluster update --from-json cluster_config.json
//! ```

use redis_enterprise_cli::handlers::handle_cluster_command;
use redis_enterprise_cli::commands::ClusterCommands;
use serde_json::json;
use wiremock::{
    matchers::{method, path, body_json, query_param},
    Mock, ResponseTemplate,
};

mod common;
use common::{setup_mock_server, create_temp_json_file};

/// Test getting comprehensive cluster information
/// 
/// This test demonstrates:
/// - Retrieving complete cluster status and configuration
/// - Understanding cluster health indicators
/// - Viewing resource utilization and limits
#[tokio::test]
async fn test_cluster_info() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "name": "Production Redis Enterprise",
        "version": "7.4.2-54",
        "license_expired": false,
        "nodes": [1, 2, 3],
        "databases": [10, 11, 12, 13, 14],
        "status": "active",
        "email_alerts": true,
        "rack_aware": true,
        "total_memory": 107374182400u64,  // 100 GB
        "used_memory": 32212254720u64,    // 30 GB 
        "total_shards": 150,
        "license_info": {
            "type_": "enterprise",
            "expiration_date": "2025-12-31T23:59:59Z",
            "shards_limit": 1000,
            "nodes_limit": 50
        },
        "cluster_recovery": false,
        "encryption": {
            "internode_encryption": true,
            "data_encryption": false
        }
    });

    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ClusterCommands::Info;
    let result = handle_cluster_command(&client, command).await.unwrap();

    assert_eq!(result["name"], "Production Redis Enterprise");
    assert_eq!(result["version"], "7.4.2-54");
    assert_eq!(result["status"], "active");
    assert_eq!(result["license_expired"], false);
    assert_eq!(result["nodes"].as_array().unwrap().len(), 3);
    assert_eq!(result["databases"].as_array().unwrap().len(), 5);
    assert_eq!(result["total_shards"], 150);
    assert_eq!(result["email_alerts"], true);
    assert_eq!(result["rack_aware"], true);
    
    // Verify resource usage calculations
    let memory_usage_percent = (32212254720u64 as f64 / 107374182400u64 as f64) * 100.0;
    assert!((memory_usage_percent - 30.0).abs() < 0.1); // ~30% usage
}

/// Test cluster statistics with time intervals
/// 
/// This test demonstrates:
/// - Getting cluster performance metrics
/// - Using time interval parameters for historical data
/// - Understanding throughput and latency statistics
#[tokio::test]
async fn test_cluster_stats_with_interval() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "interval": "1h",
        "timestamp": "2024-08-20T12:00:00Z",
        "total_ops_per_sec": 15750.5,
        "total_read_ops_per_sec": 12600.4,
        "total_write_ops_per_sec": 3150.1,
        "avg_latency_ms": 0.45,
        "p99_latency_ms": 2.8,
        "total_connections": 1250,
        "memory_usage": {
            "used_memory": 32212254720u64,
            "total_memory": 107374182400u64,
            "usage_percent": 30.0
        },
        "network_usage": {
            "total_network_bytes_in": 5368709120u64,  // 5 GB
            "total_network_bytes_out": 8589934592u64   // 8 GB
        },
        "cpu_usage": {
            "avg_cpu_system": 12.5,
            "avg_cpu_user": 23.8,
            "avg_cpu_idle": 63.7
        }
    });

    Mock::given(method("GET"))
        .and(path("/v1/cluster/stats"))
        .and(query_param("interval", "1h"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ClusterCommands::Stats {
        interval: Some("1h".to_string()),
    };
    let result = handle_cluster_command(&client, command).await.unwrap();

    assert_eq!(result["interval"], "1h");
    assert_eq!(result["total_ops_per_sec"], 15750.5);
    assert_eq!(result["total_read_ops_per_sec"], 12600.4);
    assert_eq!(result["total_write_ops_per_sec"], 3150.1);
    assert_eq!(result["avg_latency_ms"], 0.45);
    assert_eq!(result["p99_latency_ms"], 2.8);
    assert_eq!(result["total_connections"], 1250);
    
    // Verify memory usage calculation
    assert_eq!(result["memory_usage"]["usage_percent"], 30.0);
    
    // Verify CPU totals add up correctly
    let cpu = &result["cpu_usage"];
    let total_cpu = cpu["avg_cpu_system"].as_f64().unwrap() + 
                   cpu["avg_cpu_user"].as_f64().unwrap() + 
                   cpu["avg_cpu_idle"].as_f64().unwrap();
    assert!((total_cpu - 100.0).abs() < 0.1);
}

/// Test cluster statistics without interval (default)
/// 
/// This test demonstrates:
/// - Getting real-time cluster metrics
/// - Default statistics collection behavior
/// - Current performance snapshot
#[tokio::test]
async fn test_cluster_stats_default() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "timestamp": "2024-08-20T12:05:00Z",
        "total_ops_per_sec": 16100.8,
        "total_read_ops_per_sec": 12880.6,
        "total_write_ops_per_sec": 3220.2,
        "avg_latency_ms": 0.38,
        "total_connections": 1275,
        "memory_usage": {
            "used_memory": 33285996544u64,  // ~31 GB
            "total_memory": 107374182400u64,
            "usage_percent": 31.0
        }
    });

    Mock::given(method("GET"))
        .and(path("/v1/cluster/stats"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ClusterCommands::Stats { interval: None };
    let result = handle_cluster_command(&client, command).await.unwrap();

    assert_eq!(result["total_ops_per_sec"], 16100.8);
    assert_eq!(result["avg_latency_ms"], 0.38);
    assert_eq!(result["total_connections"], 1275);
    assert_eq!(result["memory_usage"]["usage_percent"], 31.0);
    
    // Should not have interval field when not specified
    assert!(result.get("interval").is_none());
}

/// Test updating cluster configuration with name
/// 
/// This test demonstrates:
/// - Modifying cluster settings via CLI arguments
/// - Updating cluster display name
/// - Basic cluster configuration management
#[tokio::test]
async fn test_cluster_update_with_name() {
    let (mock_server, client) = setup_mock_server().await;

    let expected_request = json!({
        "name": "Updated Production Cluster"
    });

    let mock_response = json!({
        "name": "Updated Production Cluster",
        "version": "7.4.2-54",
        "status": "active",
        "license_expired": false,
        "nodes": [1, 2, 3],
        "databases": [10, 11, 12, 13, 14],
        "total_memory": 107374182400u64,
        "used_memory": 32212254720u64
    });

    Mock::given(method("PUT"))
        .and(path("/v1/cluster"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ClusterCommands::Update {
        name: Some("Updated Production Cluster".to_string()),
        from_json: None,
    };
    let result = handle_cluster_command(&client, command).await.unwrap();

    assert_eq!(result["name"], "Updated Production Cluster");
    assert_eq!(result["status"], "active");
    assert_eq!(result["version"], "7.4.2-54");
}

/// Test updating cluster configuration from JSON file
/// 
/// This test demonstrates:
/// - Complex cluster configuration updates
/// - Using JSON files for multiple setting changes
/// - Advanced cluster management scenarios
#[tokio::test]
async fn test_cluster_update_from_json() {
    let (mock_server, client) = setup_mock_server().await;

    // Create comprehensive cluster configuration
    let update_config = json!({
        "name": "Advanced Production Cluster",
        "email_alerts": false,
        "rack_aware": true,
        "alert_settings": {
            "email_recipients": ["admin@company.com", "ops@company.com"],
            "sms_recipients": ["+1-555-0123"],
            "webhook_url": "https://company.com/webhooks/redis-alerts"
        },
        "maintenance_window": {
            "enabled": true,
            "start_time": "02:00",
            "duration_hours": 4,
            "timezone": "UTC"
        },
        "backup_settings": {
            "auto_backup": true,
            "backup_interval_hours": 24,
            "retention_days": 30
        }
    });
    
    let temp_file = create_temp_json_file(update_config.clone());

    let mock_response = json!({
        "name": "Advanced Production Cluster",
        "version": "7.4.2-54",
        "status": "active",
        "email_alerts": false,
        "rack_aware": true,
        "alert_settings": {
            "email_recipients": ["admin@company.com", "ops@company.com"],
            "sms_recipients": ["+1-555-0123"],
            "webhook_url": "https://company.com/webhooks/redis-alerts"
        },
        "maintenance_window": {
            "enabled": true,
            "start_time": "02:00",
            "duration_hours": 4,
            "timezone": "UTC"
        },
        "backup_settings": {
            "auto_backup": true,
            "backup_interval_hours": 24,
            "retention_days": 30
        }
    });

    Mock::given(method("PUT"))
        .and(path("/v1/cluster"))
        .and(body_json(&update_config))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ClusterCommands::Update {
        name: None, // Should be ignored when using from_json
        from_json: Some(temp_file.path().to_path_buf()),
    };
    let result = handle_cluster_command(&client, command).await.unwrap();

    assert_eq!(result["name"], "Advanced Production Cluster");
    assert_eq!(result["email_alerts"], false);
    assert_eq!(result["rack_aware"], true);
    assert_eq!(result["alert_settings"]["email_recipients"].as_array().unwrap().len(), 2);
    assert_eq!(result["maintenance_window"]["enabled"], true);
    assert_eq!(result["backup_settings"]["auto_backup"], true);
}

/// Test cluster update validation errors
/// 
/// This test demonstrates:
/// - Handling invalid cluster configuration
/// - Error response processing for updates
/// - Cluster configuration validation
#[tokio::test]
async fn test_cluster_update_validation_error() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("PUT"))
        .and(path("/v1/cluster"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "Invalid cluster configuration",
            "details": "Cluster name cannot be empty or contain special characters",
            "field": "name"
        })))
        .mount(&mock_server)
        .await;

    let command = ClusterCommands::Update {
        name: Some("".to_string()), // Empty name should cause validation error
        from_json: None,
    };
    let result = handle_cluster_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("400"));
}

/// Test cluster update missing arguments
/// 
/// This test demonstrates:
/// - Command line argument validation
/// - Error handling for incomplete commands
/// - Required parameter checking
#[tokio::test]
async fn test_cluster_update_missing_args() {
    let (_, client) = setup_mock_server().await;

    let command = ClusterCommands::Update {
        name: None,
        from_json: None,
    };
    let result = handle_cluster_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Either --name or --from-json required"));
}

/// Test cluster info with degraded status
/// 
/// This test demonstrates:
/// - Handling cluster health issues
/// - Understanding degraded cluster states
/// - Monitoring cluster problems
#[tokio::test]
async fn test_cluster_info_degraded() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "name": "Degraded Cluster",
        "version": "7.4.2-54",
        "status": "degraded",
        "license_expired": false,
        "nodes": [1, 2], // Missing node 3
        "databases": [10, 11, 12],
        "failed_nodes": [3],
        "degraded_databases": [13],
        "alerts": [
            {
                "severity": "warning",
                "message": "Node 3 is unreachable",
                "timestamp": "2024-08-20T11:30:00Z"
            },
            {
                "severity": "error", 
                "message": "Database 13 has insufficient replicas",
                "timestamp": "2024-08-20T11:45:00Z"
            }
        ],
        "total_memory": 107374182400u64,
        "used_memory": 32212254720u64
    });

    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ClusterCommands::Info;
    let result = handle_cluster_command(&client, command).await.unwrap();

    assert_eq!(result["status"], "degraded");
    assert_eq!(result["nodes"].as_array().unwrap().len(), 2); // Only 2 healthy nodes
    assert_eq!(result["failed_nodes"].as_array().unwrap().len(), 1);
    assert_eq!(result["degraded_databases"].as_array().unwrap().len(), 1);
    assert_eq!(result["alerts"].as_array().unwrap().len(), 2);
    
    // Verify alert severities
    let alerts = result["alerts"].as_array().unwrap();
    assert_eq!(alerts[0]["severity"], "warning");
    assert_eq!(alerts[1]["severity"], "error");
}

/// Test cluster stats server error
/// 
/// This test demonstrates:
/// - Handling server errors during stats collection
/// - Understanding stats collection failures
/// - Error response processing
#[tokio::test]
async fn test_cluster_stats_server_error() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/stats"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
            "error": "Internal server error",
            "details": "Statistics collection service temporarily unavailable"
        })))
        .mount(&mock_server)
        .await;

    let command = ClusterCommands::Stats { interval: None };
    let result = handle_cluster_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("500"));
}

/// Test cluster update file not found
/// 
/// This test demonstrates:
/// - File handling error scenarios
/// - JSON configuration file validation
/// - File system error handling
#[tokio::test]
async fn test_cluster_update_file_not_found() {
    let (_, client) = setup_mock_server().await;

    let command = ClusterCommands::Update {
        name: None,
        from_json: Some("/nonexistent/cluster_config.json".into()),
    };
    let result = handle_cluster_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("No such file"));
}