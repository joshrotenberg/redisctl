//! Integration tests for Redis Enterprise node commands
//!
//! This module demonstrates node management in Redis Enterprise clusters.
//! Nodes are the physical or virtual machines that make up the Redis Enterprise
//! cluster, each contributing CPU, memory, and storage resources.
//!
//! # Supported Operations
//!
//! - **List nodes** - Get all cluster nodes and their status
//! - **Get node info** - Retrieve detailed node configuration and health
//! - **Get node stats** - Monitor node performance and resource usage
//!
//! # Node Information
//!
//! ## Hardware Details
//! - **CPU Cores** - Processing power available to the node
//! - **RAM** - Total and available memory on the node
//! - **Storage** - Disk space for persistence and logs
//! - **Network** - Network interfaces and connectivity
//!
//! ## Operational Status
//! - **Node Status** - Health state (active, failed, maintenance)
//! - **Role** - Node function (master, slave, quorum)
//! - **Shards** - Database shards hosted on this node
//! - **Services** - Redis Enterprise services running
//!
//! # Usage Examples
//!
//! ## List all cluster nodes
//! ```bash
//! redis-enterprise node list
//! redis-enterprise node list --output json --query '[].{id:id,status:status,address:address}'
//! ```
//!
//! ## Get detailed node information  
//! ```bash
//! redis-enterprise node get 1
//! redis-enterprise node get 2 --query 'total_memory'
//! ```
//!
//! ## Monitor node performance
//! ```bash
//! redis-enterprise node stats 1
//! redis-enterprise node stats 2 --output table
//! ```

use redis_enterprise_cli::handlers::handle_node_command;
use redis_enterprise_cli::commands::NodeCommands;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

mod common;
use common::setup_mock_server;

/// Test listing all cluster nodes
/// 
/// This test demonstrates:
/// - Retrieving complete cluster topology
/// - Understanding node roles and status
/// - Node hardware and capacity overview
#[tokio::test]
async fn test_node_list() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!([
        {
            "uid": 1,
            "id": 1,
            "address": "10.0.1.10",
            "external_address": ["192.168.1.10"],
            "status": "active",
            "role": "master",
            "total_memory": 34359738368u64,  // 32GB
            "available_memory": 17179869184u64, // 16GB available
            "cpu_cores": 8,
            "version": "7.4.2-54",
            "uptime": "15d 8h 30m",
            "shards_count": 12,
            "endpoints_count": 5,
            "recovery_path": "",
            "backup_path": "/var/opt/redislabs/backups",
            "persistent_path": "/var/opt/redislabs/persist",
            "temporary_path": "/var/opt/redislabs/tmp"
        },
        {
            "uid": 2,
            "id": 2,
            "address": "10.0.1.11", 
            "external_address": ["192.168.1.11"],
            "status": "active",
            "role": "master",
            "total_memory": 34359738368u64,  // 32GB
            "available_memory": 20401094656u64, // 19GB available
            "cpu_cores": 8,
            "version": "7.4.2-54",
            "uptime": "15d 8h 25m",
            "shards_count": 8,
            "endpoints_count": 3,
            "recovery_path": "",
            "backup_path": "/var/opt/redislabs/backups",
            "persistent_path": "/var/opt/redislabs/persist", 
            "temporary_path": "/var/opt/redislabs/tmp"
        },
        {
            "uid": 3,
            "id": 3,
            "address": "10.0.1.12",
            "external_address": ["192.168.1.12"],
            "status": "active", 
            "role": "slave",
            "total_memory": 17179869184u64,  // 16GB
            "available_memory": 12884901888u64, // 12GB available
            "cpu_cores": 4,
            "version": "7.4.2-54",
            "uptime": "12d 14h 45m",
            "shards_count": 4,
            "endpoints_count": 2,
            "recovery_path": "",
            "backup_path": "/var/opt/redislabs/backups",
            "persistent_path": "/var/opt/redislabs/persist",
            "temporary_path": "/var/opt/redislabs/tmp"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/v1/nodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = NodeCommands::List;
    let result = handle_node_command(&client, command).await.unwrap();

    let nodes = result.as_array().unwrap();
    assert_eq!(nodes.len(), 3);

    // Verify first node (master with high load)
    assert_eq!(nodes[0]["id"], 1);
    assert_eq!(nodes[0]["status"], "active");
    assert_eq!(nodes[0]["role"], "master");
    assert_eq!(nodes[0]["address"], "10.0.1.10");
    assert_eq!(nodes[0]["total_memory"], 34359738368u64);
    assert_eq!(nodes[0]["cpu_cores"], 8);
    assert_eq!(nodes[0]["shards_count"], 12);
    
    // Verify second node (master with lower load)
    assert_eq!(nodes[1]["id"], 2);
    assert_eq!(nodes[1]["shards_count"], 8);
    assert_eq!(nodes[1]["available_memory"], 20401094656u64);
    
    // Verify third node (slave with smaller capacity)
    assert_eq!(nodes[2]["id"], 3);
    assert_eq!(nodes[2]["role"], "slave");
    assert_eq!(nodes[2]["total_memory"], 17179869184u64); // Smaller node
    assert_eq!(nodes[2]["cpu_cores"], 4);
    assert_eq!(nodes[2]["shards_count"], 4);
    
    // Calculate memory utilization
    let node1_utilization = (34359738368u64 - 17179869184u64) as f64 / 34359738368u64 as f64;
    assert!((node1_utilization - 0.5).abs() < 0.01); // ~50% utilization
}

/// Test getting specific node information
/// 
/// This test demonstrates:
/// - Retrieving detailed node configuration
/// - Understanding node capabilities and limits
/// - Node-specific operational metrics
#[tokio::test]
async fn test_node_get() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "uid": 1,
        "id": 1,
        "address": "10.0.1.10",
        "external_address": ["192.168.1.10", "node1.company.com"],
        "status": "active",
        "role": "master",
        "total_memory": 68719476736u64,  // 64GB
        "available_memory": 34359738368u64, // 32GB available
        "cpu_cores": 16,
        "cpu_threads": 32,
        "version": "7.4.2-54",
        "architecture": "x86_64",
        "uptime": "45d 12h 15m",
        "shards_count": 24,
        "endpoints_count": 8,
        "os": "Ubuntu 20.04.6 LTS",
        "kernel_version": "5.4.0-150-generic",
        "rack_id": "rack-1",
        "zone": "us-west-1a",
        "network_interfaces": [
            {
                "name": "eth0",
                "address": "10.0.1.10",
                "mtu": 9000,
                "speed": "10Gbps"
            },
            {
                "name": "eth1", 
                "address": "192.168.1.10",
                "mtu": 1500,
                "speed": "1Gbps"
            }
        ],
        "storage": {
            "persistent_path": "/var/opt/redislabs/persist",
            "backup_path": "/var/opt/redislabs/backups", 
            "log_path": "/var/opt/redislabs/log",
            "available_disk": 2199023255552u64,  // 2TB available
            "total_disk": 4398046511104u64       // 4TB total
        },
        "services": {
            "cnm_server": "active",
            "cm_server": "active",
            "crdb_coordinator": "active",
            "mdns_server": "active",
            "pdns_server": "active",
            "resource_mgr": "active",
            "stats_archiver": "active"
        }
    });

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = NodeCommands::Get { node_id: 1 };
    let result = handle_node_command(&client, command).await.unwrap();

    assert_eq!(result["id"], 1);
    assert_eq!(result["status"], "active");
    assert_eq!(result["role"], "master");
    assert_eq!(result["address"], "10.0.1.10");
    assert_eq!(result["total_memory"], 68719476736u64);
    assert_eq!(result["available_memory"], 34359738368u64);
    assert_eq!(result["cpu_cores"], 16);
    assert_eq!(result["cpu_threads"], 32);
    assert_eq!(result["architecture"], "x86_64");
    assert_eq!(result["shards_count"], 24);
    assert_eq!(result["rack_id"], "rack-1");
    assert_eq!(result["zone"], "us-west-1a");
    
    // Verify network interfaces
    let interfaces = result["network_interfaces"].as_array().unwrap();
    assert_eq!(interfaces.len(), 2);
    assert_eq!(interfaces[0]["name"], "eth0");
    assert_eq!(interfaces[0]["speed"], "10Gbps");
    assert_eq!(interfaces[1]["name"], "eth1");
    assert_eq!(interfaces[1]["speed"], "1Gbps");
    
    // Verify storage information
    assert_eq!(result["storage"]["total_disk"], 4398046511104u64);
    assert_eq!(result["storage"]["available_disk"], 2199023255552u64);
    
    // Verify services are active
    let services = result["services"].as_object().unwrap();
    assert_eq!(services["cnm_server"], "active");
    assert_eq!(services["cm_server"], "active");
    assert_eq!(services["resource_mgr"], "active");
    
    // Calculate memory utilization percentage
    let memory_used = 68719476736u64 - 34359738368u64; 
    let utilization_percent = (memory_used as f64 / 68719476736u64 as f64) * 100.0;
    assert!((utilization_percent - 50.0).abs() < 1.0); // ~50% utilization
}

/// Test getting node statistics
/// 
/// This test demonstrates:
/// - Node performance monitoring
/// - Resource usage tracking
/// - Real-time node metrics
#[tokio::test]
async fn test_node_stats() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "uid": 2,
        "cpu_system": 15.4,
        "cpu_user": 32.8,
        "cpu_idle": 51.8,
        "free_memory": 42949672960u64,
        "network_bytes_in": 1073741824u64,
        "network_bytes_out": 2147483648u64,
        "persistent_storage_free": 2199023255552u64,
        "ephemeral_storage_free": 536870912000u64,
        "cpu_iowait": 2.1,
        "used_memory": 25769803776u64,
        "usage_percent": 37.5,
        "swap_used": 0,
        "errors_in": 0,
        "errors_out": 0,
        "io_util_percent": 12.5,
        "avg_read_latency_ms": 2.3,
        "avg_write_latency_ms": 4.1,
        "active_processes": 12,
        "total_connections": 234,
        "avg_latency_ms": 0.65,
        "load_avg_1m": 2.45,
        "processes": 156,
        "threads": 1024
    });

    Mock::given(method("GET"))
        .and(path("/v1/nodes/2/stats"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = NodeCommands::Stats { node_id: 2 };
    let result = handle_node_command(&client, command).await.unwrap();

    assert_eq!(result["uid"], 2);
    
    // Verify CPU statistics (direct fields)
    assert_eq!(result["cpu_system"], 15.4);
    assert_eq!(result["cpu_user"], 32.8);
    assert_eq!(result["cpu_idle"], 51.8);
    
    // Verify additional CPU stat
    assert_eq!(result["cpu_iowait"], 2.1);
    
    // Verify memory statistics (from extra fields)
    assert_eq!(result["free_memory"], 42949672960u64);
    assert_eq!(result["used_memory"], 25769803776u64);
    assert_eq!(result["usage_percent"], 37.5);
    assert_eq!(result["swap_used"], 0); // No swap usage
    
    // Verify network statistics (direct fields)
    assert_eq!(result["network_bytes_out"], 2147483648u64);
    assert_eq!(result["network_bytes_in"], 1073741824u64);
    assert_eq!(result["errors_in"], 0);
    assert_eq!(result["errors_out"], 0);
    
    // Verify storage statistics (direct fields)
    assert_eq!(result["persistent_storage_free"], 2199023255552u64);
    assert_eq!(result["ephemeral_storage_free"], 536870912000u64);
    assert_eq!(result["io_util_percent"], 12.5);
    assert_eq!(result["avg_read_latency_ms"], 2.3);
    assert_eq!(result["avg_write_latency_ms"], 4.1);
    
    // Verify Redis process statistics (from extra fields)
    assert_eq!(result["active_processes"], 12);
    assert_eq!(result["total_connections"], 234);
    assert_eq!(result["avg_latency_ms"], 0.65);
    
    // Verify system load (from extra fields)
    assert_eq!(result["load_avg_1m"], 2.45);
    assert_eq!(result["processes"], 156);
    assert_eq!(result["threads"], 1024);
}

/// Test node not found error
/// 
/// This test demonstrates:
/// - Handling non-existent node references
/// - Node lookup failure scenarios
/// - Error response processing
#[tokio::test]
async fn test_node_get_not_found() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Node not found",
            "details": "Node with ID 999 does not exist in this cluster"
        })))
        .mount(&mock_server)
        .await;

    let command = NodeCommands::Get { node_id: 999 };
    let result = handle_node_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("404"));
}

/// Test node stats collection failure
/// 
/// This test demonstrates:
/// - Handling stats collection errors
/// - Node monitoring service issues
/// - Statistics service unavailability
#[tokio::test]
async fn test_node_stats_error() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/stats"))
        .respond_with(ResponseTemplate::new(503).set_body_json(json!({
            "error": "Statistics service unavailable",
            "details": "Node statistics collector is temporarily down for maintenance"
        })))
        .mount(&mock_server)
        .await;

    let command = NodeCommands::Stats { node_id: 1 };
    let result = handle_node_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("503"));
}

/// Test listing nodes when cluster is degraded
/// 
/// This test demonstrates:
/// - Cluster health issues in node listing
/// - Failed node identification
/// - Partial cluster visibility
#[tokio::test]
async fn test_node_list_degraded_cluster() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!([
        {
            "uid": 1,
            "id": 1,
            "address": "10.0.1.10",
            "status": "active",
            "role": "master",
            "total_memory": 34359738368u64,
            "available_memory": 17179869184u64,
            "cpu_cores": 8,
            "shards_count": 15
        },
        {
            "uid": 2,
            "id": 2,
            "address": "10.0.1.11",
            "status": "failed",
            "role": "master",
            "total_memory": 34359738368u64,
            "available_memory": 0,  // Failed node has no available memory
            "cpu_cores": 8,
            "shards_count": 0,      // No shards on failed node
            "last_seen": "2024-08-20T10:30:00Z",
            "failure_reason": "Network connectivity lost"
        },
        {
            "uid": 3,
            "id": 3,
            "address": "10.0.1.12",
            "status": "active",
            "role": "slave",
            "total_memory": 17179869184u64,
            "available_memory": 8589934592u64,
            "cpu_cores": 4,
            "shards_count": 8       // Taking over shards from failed node
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/v1/nodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = NodeCommands::List;
    let result = handle_node_command(&client, command).await.unwrap();

    let nodes = result.as_array().unwrap();
    assert_eq!(nodes.len(), 3);
    
    // Verify active master node
    assert_eq!(nodes[0]["status"], "active");
    assert_eq!(nodes[0]["shards_count"], 15);
    
    // Verify failed node
    assert_eq!(nodes[1]["status"], "failed");
    assert_eq!(nodes[1]["available_memory"], 0);
    assert_eq!(nodes[1]["shards_count"], 0);
    assert_eq!(nodes[1]["failure_reason"], "Network connectivity lost");
    
    // Verify slave node (likely handling extra load)
    assert_eq!(nodes[2]["status"], "active");
    assert_eq!(nodes[2]["role"], "slave");
    assert_eq!(nodes[2]["shards_count"], 8);
}

/// Test node with maintenance status
/// 
/// This test demonstrates:
/// - Planned maintenance scenarios
/// - Node operational status tracking
/// - Maintenance window management
#[tokio::test]
async fn test_node_maintenance_status() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "uid": 1,
        "id": 1,
        "address": "10.0.1.10",
        "status": "maintenance",
        "role": "master",
        "total_memory": 34359738368u64,
        "available_memory": 34359738368u64,  // All memory available during maintenance
        "cpu_cores": 8,
        "shards_count": 0,                   // Shards migrated away
        "maintenance_info": {
            "started": "2024-08-20T02:00:00Z",
            "estimated_completion": "2024-08-20T06:00:00Z", 
            "reason": "Security patches and hardware upgrades",
            "automated": true
        },
        "version": "7.4.2-54"
    });

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = NodeCommands::Get { node_id: 1 };
    let result = handle_node_command(&client, command).await.unwrap();

    assert_eq!(result["status"], "maintenance");
    assert_eq!(result["shards_count"], 0); // Shards migrated during maintenance
    assert_eq!(result["maintenance_info"]["automated"], true);
    assert_eq!(result["maintenance_info"]["reason"], "Security patches and hardware upgrades");
    
    // All memory should be available since no shards are running
    assert_eq!(result["available_memory"], result["total_memory"]);
}