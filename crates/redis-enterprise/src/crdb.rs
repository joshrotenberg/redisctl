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
    /// The GUID of the Active-Active database
    pub guid: String,
    /// Name of Active-Active database
    pub name: String,
    /// Current status of the Active-Active database
    pub status: String,
    /// Database memory size limit, in bytes
    pub memory_size: u64,
    /// List of participating instances in the Active-Active setup
    pub instances: Vec<CrdbInstance>,
    /// Whether communication encryption is enabled
    pub encryption: Option<bool>,
    /// Database on-disk persistence policy
    pub data_persistence: Option<String>,
    /// Whether database replication is enabled
    pub replication: Option<bool>,
    /// Data eviction policy (e.g., 'allkeys-lru', 'volatile-lru')
    pub eviction_policy: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// CRDB instance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdbInstance {
    /// Unique instance ID
    pub id: u32,
    /// Cluster fully qualified name
    pub cluster: String,
    /// Human-readable cluster name
    pub cluster_name: Option<String>,
    /// Current status of this instance
    pub status: String,
    /// List of endpoint addresses for this instance
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
    /// Name of the Active-Active database
    #[builder(setter(into))]
    pub name: String,
    /// Database memory size limit, in bytes
    pub memory_size: u64,
    /// List of participating cluster instances
    pub instances: Vec<CreateCrdbInstance>,
    /// Whether to encrypt communication between instances
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub encryption: Option<bool>,
    /// Database on-disk persistence policy ('disabled', 'aof', 'snapshot')
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub data_persistence: Option<String>,
    /// Data eviction policy when memory limit is reached
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub eviction_policy: Option<String>,
}

/// Create CRDB instance
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreateCrdbInstance {
    /// Cluster fully qualified name, used to uniquely identify the cluster
    #[builder(setter(into))]
    pub cluster: String,
    /// Cluster access URL (e.g., 'https://cluster1.example.com:9443')
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub cluster_url: Option<String>,
    /// Username for cluster authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub username: Option<String>,
    /// Password for cluster authentication
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
