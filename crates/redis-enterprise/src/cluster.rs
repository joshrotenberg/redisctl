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
