//! Cluster bootstrap and node joining operations
//!
//! ## Overview
//! - Bootstrap new clusters
//! - Join nodes to existing clusters
//! - Configure initial settings

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bootstrap configuration for cluster initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapConfig {
    /// Action to perform (e.g., 'create', 'join', 'recover_cluster')
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Cluster configuration for initialization
    pub cluster: Option<ClusterBootstrap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Node configuration for bootstrap
    pub node: Option<NodeBootstrap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Admin credentials for cluster access
    pub credentials: Option<CredentialsBootstrap>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Cluster bootstrap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterBootstrap {
    /// Cluster name for identification
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNS suffixes for cluster FQDN resolution
    pub dns_suffixes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Enable rack-aware placement for high availability
    pub rack_aware: Option<bool>,
}

/// Node bootstrap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeBootstrap {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Storage paths configuration for the node
    pub paths: Option<NodePaths>,
}

/// Node paths configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePaths {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Path for persistent storage (databases, configuration, logs)
    pub persistent_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Path for ephemeral storage (temporary files, caches)
    pub ephemeral_path: Option<String>,
}

/// Credentials bootstrap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsBootstrap {
    /// Admin username for cluster management
    pub username: String,
    /// Admin password for authentication
    pub password: String,
}

/// Bootstrap status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapStatus {
    /// Current status of the bootstrap operation
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Progress percentage (0.0-100.0) of the bootstrap operation
    pub progress: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Status message or error description
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

    /// Validate bootstrap for a specific UID - POST /v1/bootstrap/validate/{uid}
    pub async fn validate_for(&self, uid: u32, body: Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/bootstrap/validate/{}", uid), &body)
            .await
    }

    /// Post a specific bootstrap action - POST /v1/bootstrap/{action}
    pub async fn post_action(&self, action: &str, body: Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/bootstrap/{}", action), &body)
            .await
    }
}
