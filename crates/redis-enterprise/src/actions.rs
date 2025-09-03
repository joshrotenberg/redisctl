//! Action management for Redis Enterprise async operations
//!
//! ## Overview
//! - Track long-running operations
//! - Query action status
//! - Cancel or wait for actions

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

    // Versioned sub-handlers for clearer API
    pub fn v1(&self) -> v1::ActionsV1 {
        v1::ActionsV1::new(self.client.clone())
    }

    pub fn v2(&self) -> v2::ActionsV2 {
        v2::ActionsV2::new(self.client.clone())
    }
}

pub mod v1 {
    use super::{Action, RestClient};
    use crate::error::Result;

    pub struct ActionsV1 {
        client: RestClient,
    }

    impl ActionsV1 {
        pub(crate) fn new(client: RestClient) -> Self {
            Self { client }
        }

        pub async fn list(&self) -> Result<Vec<Action>> {
            self.client.get("/v1/actions").await
        }

        pub async fn get(&self, action_uid: &str) -> Result<Action> {
            self.client
                .get(&format!("/v1/actions/{}", action_uid))
                .await
        }

        pub async fn cancel(&self, action_uid: &str) -> Result<()> {
            self.client
                .delete(&format!("/v1/actions/{}", action_uid))
                .await
        }

        pub async fn list_for_bdb(&self, bdb_uid: u32) -> Result<Vec<Action>> {
            self.client
                .get(&format!("/v1/actions/bdb/{}", bdb_uid))
                .await
        }
    }
}

pub mod v2 {
    use super::{Action, RestClient};
    use crate::error::Result;

    pub struct ActionsV2 {
        client: RestClient,
    }

    impl ActionsV2 {
        pub(crate) fn new(client: RestClient) -> Self {
            Self { client }
        }

        pub async fn list(&self) -> Result<Vec<Action>> {
            self.client.get("/v2/actions").await
        }

        pub async fn get(&self, action_uid: &str) -> Result<Action> {
            self.client
                .get(&format!("/v2/actions/{}", action_uid))
                .await
        }
    }
}
