//! Cluster management commands for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNode {
    pub id: u32,
    pub address: String,
    pub status: String,
    pub role: Option<String>,
    pub total_memory: Option<u64>,
    pub used_memory: Option<u64>,
    pub cpu_cores: Option<u32>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Cluster information from the REST API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterInfo {
    pub name: String,
    pub version: Option<String>,
    pub license_expired: Option<bool>,
    pub nodes: Option<Vec<u32>>,
    pub databases: Option<Vec<u32>>,
    pub status: Option<String>,
    pub email_alerts: Option<bool>,
    pub rack_aware: Option<bool>,

    // Stats
    pub total_memory: Option<u64>,
    pub used_memory: Option<u64>,
    pub total_shards: Option<u32>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Bootstrap request for creating a new cluster
#[derive(Debug, Serialize)]
pub struct BootstrapRequest {
    pub action: String,
    pub cluster: ClusterBootstrapInfo,
    pub credentials: BootstrapCredentials,
}

/// Cluster information for bootstrap
#[derive(Debug, Serialize)]
pub struct ClusterBootstrapInfo {
    pub name: String,
}

/// Credentials for bootstrap
#[derive(Debug, Serialize)]
pub struct BootstrapCredentials {
    pub username: String,
    pub password: String,
}

/// Cluster handler for executing cluster commands
pub struct ClusterHandler {
    client: RestClient,
}

impl ClusterHandler {
    pub fn new(client: RestClient) -> Self {
        ClusterHandler { client }
    }

    /// Get cluster information (CLUSTER.INFO)
    pub async fn info(&self) -> Result<ClusterInfo> {
        self.client.get("/v1/cluster").await
    }

    /// Bootstrap a new cluster (CLUSTER.BOOTSTRAP)
    pub async fn bootstrap(&self, request: BootstrapRequest) -> Result<Value> {
        // The bootstrap endpoint returns empty response on success
        // Note: Despite docs saying /v1/bootstrap, the actual endpoint is /v1/bootstrap/create_cluster
        self.client
            .post_bootstrap("/v1/bootstrap/create_cluster", &request)
            .await
    }

    /// Update cluster configuration (CLUSTER.UPDATE)
    pub async fn update(&self, updates: Value) -> Result<Value> {
        self.client.put("/v1/cluster", &updates).await
    }

    /// Get cluster stats (CLUSTER.STATS)
    pub async fn stats(&self) -> Result<Value> {
        self.client.get("/v1/cluster/stats").await
    }

    /// Get cluster nodes (CLUSTER.NODES)
    pub async fn nodes(&self) -> Result<Vec<NodeInfo>> {
        self.client.get("/v1/nodes").await
    }

    /// Get cluster license (CLUSTER.LICENSE)
    pub async fn license(&self) -> Result<LicenseInfo> {
        self.client.get("/v1/license").await
    }

    /// Join node to cluster (CLUSTER.JOIN)
    pub async fn join_node(
        &self,
        node_address: &str,
        username: &str,
        password: &str,
    ) -> Result<Value> {
        let body = serde_json::json!({
            "action": "join_cluster",
            "cluster": {
                "nodes": [node_address]
            },
            "credentials": {
                "username": username,
                "password": password
            }
        });
        self.client.post("/v1/bootstrap/join", &body).await
    }

    /// Remove node from cluster (CLUSTER.REMOVE_NODE)
    pub async fn remove_node(&self, node_uid: u32) -> Result<Value> {
        self.client
            .delete(&format!("/v1/nodes/{}", node_uid))
            .await?;
        Ok(serde_json::json!({"message": format!("Node {} removed", node_uid)}))
    }

    /// Reset cluster to factory defaults (CLUSTER.RESET) - DANGEROUS
    pub async fn reset(&self) -> Result<Value> {
        self.client
            .post("/v1/cluster/actions/reset", &serde_json::json!({}))
            .await
    }

    /// Recover cluster from failure (CLUSTER.RECOVER)
    pub async fn recover(&self) -> Result<Value> {
        self.client
            .post("/v1/cluster/actions/recover", &serde_json::json!({}))
            .await
    }

    /// Get cluster settings (CLUSTER.SETTINGS)
    pub async fn settings(&self) -> Result<Value> {
        self.client.get("/v1/cluster/settings").await
    }

    /// Get cluster topology (CLUSTER.TOPOLOGY)
    pub async fn topology(&self) -> Result<Value> {
        self.client.get("/v1/cluster/topology").await
    }
}

/// Node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub uid: u32,
    pub address: String,
    pub status: String,
    pub role: Option<String>,
    pub shards: Option<Vec<u32>>,
    pub total_memory: Option<u64>,
    pub used_memory: Option<u64>,

    #[serde(flatten)]
    pub extra: Value,
}

/// License information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub license_type: Option<String>,
    pub expired: Option<bool>,
    pub expiration_date: Option<String>,
    pub shards_limit: Option<u32>,
    pub features: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: Value,
}
