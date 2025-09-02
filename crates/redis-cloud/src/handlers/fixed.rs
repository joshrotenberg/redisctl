//! Fixed (Essentials) subscription operations handler

use crate::{Result, client::CloudClient};
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

    /// Delete a fixed subscription
    pub async fn delete_subscription(&self, subscription_id: u32) -> Result<()> {
        self.client
            .delete(&format!("/fixed/subscriptions/{}", subscription_id))
            .await
    }

    /// Delete a fixed subscription - raw version
    pub async fn delete_subscription_raw(&self, subscription_id: u32) -> Result<serde_json::Value> {
        self.client
            .delete(&format!("/fixed/subscriptions/{}", subscription_id))
            .await?;
        Ok(serde_json::json!({"message": format!("Subscription {} deleted", subscription_id)}))
    }

    /// Delete a database in a fixed subscription
    pub async fn delete_database(&self, subscription_id: u32, database_id: u32) -> Result<()> {
        self.client
            .delete(&format!(
                "/fixed/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await
    }

    /// Delete a database in a fixed subscription - raw version
    pub async fn delete_database_raw(
        &self,
        subscription_id: u32,
        database_id: u32,
    ) -> Result<serde_json::Value> {
        self.client
            .delete(&format!(
                "/fixed/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await?;
        Ok(serde_json::json!({"message": format!("Database {} deleted", database_id)}))
    }

    /// List database tags (fixed)
    pub async fn database_tags(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/fixed/subscriptions/{}/databases/{}/tags",
                subscription_id, database_id
            ))
            .await
    }

    /// Delete database tag (fixed)
    pub async fn delete_database_tag(
        &self,
        subscription_id: u32,
        database_id: u32,
        tag: &str,
    ) -> Result<()> {
        self.client
            .delete(&format!(
                "/fixed/subscriptions/{}/databases/{}/tags/{}",
                subscription_id, database_id, tag
            ))
            .await
    }

    /// Delete database tag (fixed) - raw version
    pub async fn delete_database_tag_raw(
        &self,
        subscription_id: u32,
        database_id: u32,
        tag: &str,
    ) -> Result<serde_json::Value> {
        self.client
            .delete(&format!(
                "/fixed/subscriptions/{}/databases/{}/tags/{}",
                subscription_id, database_id, tag
            ))
            .await?;
        Ok(serde_json::json!({"message": format!("Tag {} deleted", tag)}))
    }
}
