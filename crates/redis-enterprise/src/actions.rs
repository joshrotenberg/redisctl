//! Action management for Redis Enterprise async operations

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Action information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_uid: String,
    pub name: String,
    pub status: String,
    pub progress: Option<f32>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub description: Option<String>,
    pub error: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Action handler for tracking async operations
pub struct ActionHandler {
    client: RestClient,
}

impl ActionHandler {
    pub fn new(client: RestClient) -> Self {
        ActionHandler { client }
    }

    /// List all actions
    pub async fn list(&self) -> Result<Vec<Action>> {
        self.client.get("/v1/actions").await
    }

    /// Get specific action status
    pub async fn get(&self, action_uid: &str) -> Result<Action> {
        self.client
            .get(&format!("/v1/actions/{}", action_uid))
            .await
    }

    /// Cancel an action
    pub async fn cancel(&self, action_uid: &str) -> Result<()> {
        self.client
            .delete(&format!("/v1/actions/{}", action_uid))
            .await
    }

    /// List actions via v2 API - GET /v2/actions
    pub async fn list_v2(&self) -> Result<Vec<Action>> {
        self.client.get("/v2/actions").await
    }

    /// Get action via v2 API - GET /v2/actions/{uid}
    pub async fn get_v2(&self, action_uid: &str) -> Result<Action> {
        self.client
            .get(&format!("/v2/actions/{}", action_uid))
            .await
    }

    /// List actions for a database - GET /v1/actions/bdb/{uid}
    pub async fn list_for_bdb(&self, bdb_uid: u32) -> Result<Vec<Action>> {
        self.client
            .get(&format!("/v1/actions/bdb/{}", bdb_uid))
            .await
    }
}
