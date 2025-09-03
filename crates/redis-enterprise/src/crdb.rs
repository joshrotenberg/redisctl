//! Active-Active (CRDB) database management
//!
//! ## Overview
//! - Create and manage Active-Active databases
//! - Configure cross-region replication
//! - Monitor CRDB status

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

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
///
/// # Examples
///
/// ```rust,no_run
/// use redis_enterprise::{CreateCrdbRequest, CreateCrdbInstance};
///
/// let request = CreateCrdbRequest::builder()
///     .name("global-cache")
///     .memory_size(1024 * 1024 * 1024) // 1GB
///     .instances(vec![
///         CreateCrdbInstance::builder()
///             .cluster("cluster1.example.com")
///             .cluster_url("https://cluster1.example.com:9443")
///             .username("admin")
///             .password("password")
///             .build(),
///         CreateCrdbInstance::builder()
///             .cluster("cluster2.example.com")
///             .cluster_url("https://cluster2.example.com:9443")
///             .username("admin")
///             .password("password")
///             .build()
///     ])
///     .encryption(true)
///     .data_persistence("aof")
///     .build();
/// ```
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreateCrdbRequest {
    #[builder(setter(into))]
    pub name: String,
    pub memory_size: u64,
    pub instances: Vec<CreateCrdbInstance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub encryption: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub data_persistence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub eviction_policy: Option<String>,
}

/// Create CRDB instance
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreateCrdbInstance {
    #[builder(setter(into))]
    pub cluster: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub cluster_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
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
