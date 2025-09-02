//! Node management for Redis Enterprise
//!
//! Overview
//! - List/get/update/remove nodes
//! - Node actions (e.g., maintenance_on/off) with typed responses
//! - Status and watchdog status (per-node and aggregate)
//! - Shards and proxies per-node

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
    pub uid: u32,

    /// IP address of the node (renamed from 'address' to match API)
    #[serde(rename = "addr")]
    pub addr: Option<String>,

    pub status: String,

    /// Whether node accepts new shards
    pub accept_servers: Option<bool>,

    /// System architecture (e.g., "aarch64", "x86_64")
    pub architecture: Option<String>,

    /// CPU cores (renamed from 'cpu_cores' to match API)
    #[serde(rename = "cores")]
    pub cores: Option<u32>,

    /// External IP addresses
    pub external_addr: Option<Vec<String>>,

    /// Total memory in bytes
    pub total_memory: Option<u64>,

    /// OS version information
    pub os_version: Option<String>,
    pub os_name: Option<String>,
    pub os_family: Option<String>,
    pub os_semantic_version: Option<String>,

    /// Storage sizes (API returns f64, not u64)
    pub ephemeral_storage_size: Option<f64>,
    pub persistent_storage_size: Option<f64>,

    /// Storage paths
    pub ephemeral_storage_path: Option<String>,
    pub persistent_storage_path: Option<String>,
    pub bigredis_storage_path: Option<String>,

    /// Rack configuration
    pub rack_id: Option<String>,
    pub second_rack_id: Option<String>,

    /// Shard information
    pub shard_count: Option<u32>,
    pub shard_list: Option<Vec<u32>>,
    pub ram_shard_count: Option<u32>,
    pub flash_shard_count: Option<u32>,

    /// Features and capabilities
    pub bigstore_enabled: Option<bool>,
    pub fips_enabled: Option<bool>,
    pub use_internal_ipv6: Option<bool>,

    /// Limits and settings
    pub max_listeners: Option<u32>,
    pub max_redis_servers: Option<u32>,
    pub max_redis_forks: Option<i32>,
    pub max_slave_full_syncs: Option<i32>,

    /// Runtime information
    pub uptime: Option<u64>,
    pub software_version: Option<String>,

    /// Supported Redis versions
    pub supported_database_versions: Option<Vec<Value>>,

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

    /// Execute node action (e.g., "maintenance_on", "maintenance_off") - typed version
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
