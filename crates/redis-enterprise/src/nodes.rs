//! Node management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub uid: u32,
    pub address: String,
    pub status: String,
    pub role: Option<String>,
    pub shards: Option<Vec<u32>>,
    pub total_memory: Option<u64>,
    pub used_memory: Option<u64>,
    pub cpu_cores: Option<u32>,
    pub os_version: Option<String>,
    pub ephemeral_storage_size: Option<u64>,
    pub ephemeral_storage_used: Option<u64>,
    pub persistent_storage_size: Option<u64>,
    pub persistent_storage_used: Option<u64>,
    pub rack_id: Option<String>,

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
#[derive(Debug, Serialize)]
pub struct NodeActionRequest {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_uid: Option<u32>,
}

/// Node handler for executing node commands
pub struct NodeHandler {
    client: RestClient,
}

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
    pub async fn execute_action(&self, uid: u32, action: &str) -> Result<Value> {
        let request = NodeActionRequest {
            action: action.to_string(),
            node_uid: Some(uid),
        };
        self.client
            .post(&format!("/v1/nodes/{}/actions", uid), &request)
            .await
    }
}
