//! Database group configuration
//!
//! ## Overview
//! - Manage database groups
//! - Configure group settings
//! - Query group membership

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Database group information
/// Represents a group of databases that share a memory pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BdbGroup {
    /// Cluster unique ID of the database group
    pub uid: u32,
    /// Group name (may be auto-generated if not specified)
    pub name: String,
    /// The common memory pool size limit for all databases in the group, expressed in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<u64>,
    /// A list of UIDs of member databases (read-only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Handler for database group operations
pub struct BdbGroupsHandler {
    client: RestClient,
}

impl BdbGroupsHandler {
    pub fn new(client: RestClient) -> Self {
        BdbGroupsHandler { client }
    }

    pub async fn list(&self) -> Result<Vec<BdbGroup>> {
        self.client.get("/v1/bdb_groups").await
    }

    pub async fn get(&self, uid: u32) -> Result<BdbGroup> {
        self.client.get(&format!("/v1/bdb_groups/{}", uid)).await
    }

    pub async fn create(&self, body: CreateBdbGroupRequest) -> Result<BdbGroup> {
        self.client.post("/v1/bdb_groups", &body).await
    }

    pub async fn update(&self, uid: u32, body: UpdateBdbGroupRequest) -> Result<BdbGroup> {
        self.client
            .put(&format!("/v1/bdb_groups/{}", uid), &body)
            .await
    }

    pub async fn delete(&self, uid: u32) -> Result<()> {
        self.client.delete(&format!("/v1/bdb_groups/{}", uid)).await
    }
}

/// Request to create a new database group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBdbGroupRequest {
    pub name: String,
}

/// Request to update an existing database group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBdbGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
