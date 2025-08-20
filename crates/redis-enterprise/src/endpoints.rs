//! Endpoint management and statistics for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub uid: String,
    pub bdb_uid: u32,
    pub node_uid: u32,
    pub addr: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Endpoint statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointStats {
    pub uid: String,
    pub intervals: Vec<StatsInterval>,

    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsInterval {
    pub interval: String,
    pub timestamps: Vec<i64>,
    pub values: Vec<Value>,
}

/// Endpoints handler
pub struct EndpointsHandler {
    client: RestClient,
}

impl EndpointsHandler {
    pub fn new(client: RestClient) -> Self {
        EndpointsHandler { client }
    }

    /// List all endpoints
    pub async fn list(&self) -> Result<Vec<Endpoint>> {
        self.client.get("/v1/endpoints").await
    }

    /// Get specific endpoint
    pub async fn get(&self, uid: &str) -> Result<Endpoint> {
        self.client.get(&format!("/v1/endpoints/{}", uid)).await
    }

    /// Get endpoint statistics
    pub async fn stats(&self, uid: &str) -> Result<EndpointStats> {
        self.client
            .get(&format!("/v1/endpoints/{}/stats", uid))
            .await
    }

    /// Get all endpoint statistics
    pub async fn all_stats(&self) -> Result<Vec<EndpointStats>> {
        self.client.get("/v1/endpoints/stats").await
    }

    /// Get endpoints for a specific database
    pub async fn list_by_database(&self, bdb_uid: u32) -> Result<Vec<Endpoint>> {
        self.client
            .get(&format!("/v1/bdbs/{}/endpoints", bdb_uid))
            .await
    }

    /// Get endpoints for a specific node
    pub async fn list_by_node(&self, node_uid: u32) -> Result<Vec<Endpoint>> {
        self.client
            .get(&format!("/v1/nodes/{}/endpoints", node_uid))
            .await
    }
}
