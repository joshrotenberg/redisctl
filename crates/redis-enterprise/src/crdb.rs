//! Active-Active Database (CRDB) management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// CRDB (Active-Active Database) information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crdb {
    pub guid: String,
    pub name: String,
    pub status: String,
    pub memory_size: u64,
    pub instances: Vec<CrdbInstance>,
    pub encryption: Option<bool>,
    pub data_persistence: Option<String>,
    pub replication: Option<bool>,
    pub eviction_policy: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// CRDB instance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdbInstance {
    pub id: u32,
    pub cluster: String,
    pub cluster_name: Option<String>,
    pub status: String,
    pub endpoints: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create CRDB request
#[derive(Debug, Serialize)]
pub struct CreateCrdbRequest {
    pub name: String,
    pub memory_size: u64,
    pub instances: Vec<CreateCrdbInstance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<String>,
}

/// Create CRDB instance
#[derive(Debug, Serialize)]
pub struct CreateCrdbInstance {
    pub cluster: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// CRDB handler for managing Active-Active databases
pub struct CrdbHandler {
    client: RestClient,
}

impl CrdbHandler {
    pub fn new(client: RestClient) -> Self {
        CrdbHandler { client }
    }

    /// List all CRDBs
    pub async fn list(&self) -> Result<Vec<Crdb>> {
        self.client.get("/v1/crdbs").await
    }

    /// Get specific CRDB
    pub async fn get(&self, guid: &str) -> Result<Crdb> {
        self.client.get(&format!("/v1/crdbs/{}", guid)).await
    }

    /// Create new CRDB
    pub async fn create(&self, request: CreateCrdbRequest) -> Result<Crdb> {
        self.client.post("/v1/crdbs", &request).await
    }

    /// Update CRDB
    pub async fn update(&self, guid: &str, updates: Value) -> Result<Crdb> {
        self.client
            .put(&format!("/v1/crdbs/{}", guid), &updates)
            .await
    }

    /// Delete CRDB
    pub async fn delete(&self, guid: &str) -> Result<()> {
        self.client.delete(&format!("/v1/crdbs/{}", guid)).await
    }

    /// Get CRDB tasks
    pub async fn tasks(&self, guid: &str) -> Result<Value> {
        self.client.get(&format!("/v1/crdbs/{}/tasks", guid)).await
    }
}
