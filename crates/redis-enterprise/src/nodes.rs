//! Nodes management for Redis Enterprise
//!
//! ## Overview
//! - List and query resources
//! - Create and update configurations
//! - Monitor status and metrics

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Response from node action operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeActionResponse {
    /// The action UID for tracking async operations
    pub action_uid: String,
    /// Description of the action
    pub description: Option<String>,
    /// Additional fields from the response
    #[serde(flatten)]
    pub extra: Value,
}

/// Node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Cluster unique ID of node (read-only)
    pub uid: u32,

    /// Internal IP address of node
    #[serde(rename = "addr")]
    pub addr: Option<String>,

    /// Node status (read-only)
    pub status: String,

    /// Node accepts new shards if true
    pub accept_servers: Option<bool>,

    /// Hardware architecture (read-only)
    pub architecture: Option<String>,

    /// Total number of CPU cores (read-only)
    #[serde(rename = "cores")]
    pub cores: Option<u32>,

    /// External IP addresses of node
    pub external_addr: Option<Vec<String>>,

    /// Total memory in bytes
    pub total_memory: Option<u64>,

    /// Installed OS version (read-only)
    pub os_version: Option<String>,
    /// Operating system name (read-only)
    pub os_name: Option<String>,
    /// Operating system family (read-only)
    pub os_family: Option<String>,
    /// Full version number (read-only)
    pub os_semantic_version: Option<String>,

    /// Ephemeral storage size in bytes (read-only)
    pub ephemeral_storage_size: Option<f64>,
    /// Persistent storage size in bytes (read-only)
    pub persistent_storage_size: Option<f64>,

    /// Ephemeral storage path (read-only)
    pub ephemeral_storage_path: Option<String>,
    /// Persistent storage path (read-only)
    pub persistent_storage_path: Option<String>,
    /// Flash storage path (read-only)
    pub bigredis_storage_path: Option<String>,

    /// Rack ID where node is installed
    pub rack_id: Option<String>,
    /// Second rack ID where node is installed
    pub second_rack_id: Option<String>,

    /// Number of shards on the node (read-only)
    pub shard_count: Option<u32>,
    /// Cluster unique IDs of all node shards
    pub shard_list: Option<Vec<u32>>,
    /// RAM shard count
    pub ram_shard_count: Option<u32>,
    /// Flash shard count
    pub flash_shard_count: Option<u32>,

    /// Flash storage enabled for Auto Tiering databases
    pub bigstore_enabled: Option<bool>,
    /// FIPS mode enabled
    pub fips_enabled: Option<bool>,
    /// Use internal IPv6
    pub use_internal_ipv6: Option<bool>,

    /// Maximum number of listeners on the node
    pub max_listeners: Option<u32>,
    /// Maximum number of shards on the node
    pub max_redis_servers: Option<u32>,
    /// Maximum background processes forked from shards
    pub max_redis_forks: Option<i32>,
    /// Maximum simultaneous replica full syncs
    pub max_slave_full_syncs: Option<i32>,

    /// Node uptime in seconds
    pub uptime: Option<u64>,
    /// Installed Redis Enterprise cluster software version (read-only)
    pub software_version: Option<String>,

    /// Supported database versions
    pub supported_database_versions: Option<Vec<Value>>,

    /// Bigstore driver name (deprecated)
    pub bigstore_driver: Option<String>,

    /// Storage size of bigstore storage (read-only)
    pub bigstore_size: Option<u64>,

    /// Public IP address (deprecated)
    pub public_addr: Option<String>,

    /// Recovery files path
    pub recovery_path: Option<String>,

    /// Capture any additional fields not explicitly defined
    #[serde(flatten)]
    pub extra: Value,
}

/// Node stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    pub uid: u32,
    pub cpu_user: Option<f64>,
    pub cpu_system: Option<f64>,
    pub cpu_idle: Option<f64>,
    pub free_memory: Option<u64>,
    pub network_bytes_in: Option<u64>,
    pub network_bytes_out: Option<u64>,
    pub persistent_storage_free: Option<u64>,
    pub ephemeral_storage_free: Option<u64>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Node action request
#[derive(Debug, Serialize, TypedBuilder)]
pub struct NodeActionRequest {
    #[builder(setter(into))]
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub node_uid: Option<u32>,
}

/// Node handler for executing node commands
pub struct NodeHandler {
    client: RestClient,
}

/// Alias for backwards compatibility and intuitive plural naming
pub type NodesHandler = NodeHandler;

impl NodeHandler {
    pub fn new(client: RestClient) -> Self {
        NodeHandler { client }
    }

    /// List all nodes
    pub async fn list(&self) -> Result<Vec<Node>> {
        self.client.get("/v1/nodes").await
    }

    /// Get specific node info
    pub async fn get(&self, uid: u32) -> Result<Node> {
        self.client.get(&format!("/v1/nodes/{}", uid)).await
    }

    /// Update node configuration
    pub async fn update(&self, uid: u32, updates: Value) -> Result<Node> {
        self.client
            .put(&format!("/v1/nodes/{}", uid), &updates)
            .await
    }

    /// Remove node from cluster
    pub async fn remove(&self, uid: u32) -> Result<()> {
        self.client.delete(&format!("/v1/nodes/{}", uid)).await
    }

    /// Get node stats
    pub async fn stats(&self, uid: u32) -> Result<NodeStats> {
        self.client.get(&format!("/v1/nodes/{}/stats", uid)).await
    }

    /// Get node actions
    pub async fn actions(&self, uid: u32) -> Result<Value> {
        self.client.get(&format!("/v1/nodes/{}/actions", uid)).await
    }

    /// Execute node action (e.g., "maintenance_on", "maintenance_off")
    pub async fn execute_action(&self, uid: u32, action: &str) -> Result<NodeActionResponse> {
        let request = NodeActionRequest {
            action: action.to_string(),
            node_uid: Some(uid),
        };
        self.client
            .post(&format!("/v1/nodes/{}/actions", uid), &request)
            .await
    }

    // raw variant removed in favor of typed execute_action

    /// List all available node actions (global) - GET /v1/nodes/actions
    pub async fn list_actions(&self) -> Result<Value> {
        self.client.get("/v1/nodes/actions").await
    }

    /// Get node action detail - GET /v1/nodes/{uid}/actions/{action}
    pub async fn action_detail(&self, uid: u32, action: &str) -> Result<Value> {
        self.client
            .get(&format!("/v1/nodes/{}/actions/{}", uid, action))
            .await
    }

    /// Execute named node action - POST /v1/nodes/{uid}/actions/{action}
    pub async fn action_execute(&self, uid: u32, action: &str, body: Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/nodes/{}/actions/{}", uid, action), &body)
            .await
    }

    /// Delete node action - DELETE /v1/nodes/{uid}/actions/{action}
    pub async fn action_delete(&self, uid: u32, action: &str) -> Result<()> {
        self.client
            .delete(&format!("/v1/nodes/{}/actions/{}", uid, action))
            .await
    }

    /// List snapshots for a node - GET /v1/nodes/{uid}/snapshots
    pub async fn snapshots(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/nodes/{}/snapshots", uid))
            .await
    }

    /// Create a snapshot - POST /v1/nodes/{uid}/snapshots/{name}
    pub async fn snapshot_create(&self, uid: u32, name: &str) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/nodes/{}/snapshots/{}", uid, name),
                &serde_json::json!({}),
            )
            .await
    }

    /// Delete a snapshot - DELETE /v1/nodes/{uid}/snapshots/{name}
    pub async fn snapshot_delete(&self, uid: u32, name: &str) -> Result<()> {
        self.client
            .delete(&format!("/v1/nodes/{}/snapshots/{}", uid, name))
            .await
    }

    /// All nodes status - GET /v1/nodes/status
    pub async fn status_all(&self) -> Result<Value> {
        self.client.get("/v1/nodes/status").await
    }

    /// Watchdog status for all nodes - GET /v1/nodes/wd_status
    pub async fn wd_status_all(&self) -> Result<Value> {
        self.client.get("/v1/nodes/wd_status").await
    }

    /// Node status - GET /v1/nodes/{uid}/status
    pub async fn status(&self, uid: u32) -> Result<Value> {
        self.client.get(&format!("/v1/nodes/{}/status", uid)).await
    }

    /// Node watchdog status - GET /v1/nodes/{uid}/wd_status
    pub async fn wd_status(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/nodes/{}/wd_status", uid))
            .await
    }

    /// All node alerts - GET /v1/nodes/alerts
    pub async fn alerts_all(&self) -> Result<Value> {
        self.client.get("/v1/nodes/alerts").await
    }

    /// Alerts for node - GET /v1/nodes/alerts/{uid}
    pub async fn alerts_for(&self, uid: u32) -> Result<Value> {
        self.client.get(&format!("/v1/nodes/alerts/{}", uid)).await
    }

    /// Alert detail - GET /v1/nodes/alerts/{uid}/{alert}
    pub async fn alert_detail(&self, uid: u32, alert: &str) -> Result<Value> {
        self.client
            .get(&format!("/v1/nodes/alerts/{}/{}", uid, alert))
            .await
    }
}
