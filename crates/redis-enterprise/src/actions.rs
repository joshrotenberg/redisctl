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
}
