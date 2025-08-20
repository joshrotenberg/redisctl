//! Bootstrap operations for Redis Enterprise cluster initialization

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bootstrap configuration for cluster initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapConfig {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<ClusterBootstrap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<NodeBootstrap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<CredentialsBootstrap>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Cluster bootstrap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterBootstrap {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_suffixes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_aware: Option<bool>,
}

/// Node bootstrap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeBootstrap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<NodePaths>,
}

/// Node paths configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePaths {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_path: Option<String>,
}

/// Credentials bootstrap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsBootstrap {
    pub username: String,
    pub password: String,
}

/// Bootstrap status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapStatus {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Bootstrap handler for cluster initialization
pub struct BootstrapHandler {
    client: RestClient,
}

impl BootstrapHandler {
    pub fn new(client: RestClient) -> Self {
        BootstrapHandler { client }
    }

    /// Initialize cluster bootstrap
    pub async fn create(&self, config: BootstrapConfig) -> Result<BootstrapStatus> {
        self.client.post("/v1/bootstrap", &config).await
    }

    /// Get bootstrap status
    pub async fn status(&self) -> Result<BootstrapStatus> {
        self.client.get("/v1/bootstrap").await
    }

    /// Join node to existing cluster
    pub async fn join(&self, config: BootstrapConfig) -> Result<BootstrapStatus> {
        self.client.post("/v1/bootstrap/join", &config).await
    }

    /// Reset bootstrap (dangerous operation)
    pub async fn reset(&self) -> Result<()> {
        self.client.delete("/v1/bootstrap").await
    }
}

