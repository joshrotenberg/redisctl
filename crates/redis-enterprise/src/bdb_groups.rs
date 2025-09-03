//! Database Groups management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BdbGroup {
    pub uid: u32,
    pub name: String,
    #[serde(flatten)]
    pub extra: Value,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBdbGroupRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBdbGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
