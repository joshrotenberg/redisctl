//! Fixed (Essentials) subscription operations handler

use crate::{client::CloudClient, Result};
use serde_json::Value;

/// Handler for Cloud fixed/essentials subscription operations
pub struct CloudFixedHandler {
    client: CloudClient,
}

impl CloudFixedHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudFixedHandler { client }
    }

    /// List all fixed subscriptions
    pub async fn list(&self) -> Result<Value> {
        self.client.get("/fixed/subscriptions").await
    }

    /// Get fixed subscription by ID
    pub async fn get(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/fixed/subscriptions/{}", subscription_id))
            .await
    }

    /// Get fixed subscription databases
    pub async fn databases(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/fixed/subscriptions/{}/databases",
                subscription_id
            ))
            .await
    }

    /// Get a specific database in a fixed subscription
    pub async fn database(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/fixed/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await
    }

    /// List available fixed plans
    pub async fn plans(&self) -> Result<Value> {
        self.client.get("/fixed/plans").await
    }

    /// Get a specific fixed plan
    pub async fn plan(&self, plan_id: u32) -> Result<Value> {
        self.client.get(&format!("/fixed/plans/{}", plan_id)).await
    }
}
