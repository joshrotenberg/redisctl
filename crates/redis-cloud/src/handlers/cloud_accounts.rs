//! Cloud account operations handler

use crate::{client::CloudClient, Result};
use serde_json::Value;

/// Handler for Cloud account operations
pub struct CloudAccountsHandler {
    client: CloudClient,
}

impl CloudAccountsHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudAccountsHandler { client }
    }

    /// List all cloud accounts
    pub async fn list(&self) -> Result<Value> {
        self.client.get("/cloud-accounts").await
    }

    /// Get cloud account by ID
    pub async fn get(&self, account_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/cloud-accounts/{}", account_id))
            .await
    }

    /// Create a new cloud account
    pub async fn create(&self, request: Value) -> Result<Value> {
        self.client.post("/cloud-accounts", &request).await
    }

    /// Update cloud account
    pub async fn update(&self, account_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(&format!("/cloud-accounts/{}", account_id), &request)
            .await
    }

    /// Delete cloud account
    pub async fn delete(&self, account_id: u32) -> Result<Value> {
        self.client
            .delete(&format!("/cloud-accounts/{}", account_id))
            .await?;
        Ok(serde_json::json!({"message": format!("Cloud account {} deleted", account_id)}))
    }
}
