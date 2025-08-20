//! Database operations handler

use crate::{
    Result,
    client::CloudClient,
    models::{CloudDatabase, CreateDatabaseRequest, UpdateDatabaseRequest},
};
use serde_json::Value;

/// Handler for Cloud database operations
pub struct CloudDatabaseHandler {
    client: CloudClient,
}

impl CloudDatabaseHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudDatabaseHandler { client }
    }

    /// Get database by ID
    pub async fn get(&self, subscription_id: u32, database_id: u32) -> Result<CloudDatabase> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await
    }

    /// Create a new database
    pub async fn create(
        &self,
        subscription_id: u32,
        request: CreateDatabaseRequest,
    ) -> Result<Value> {
        self.client
            .post(
                &format!("/subscriptions/{}/databases", subscription_id),
                &request,
            )
            .await
    }

    /// Update database
    pub async fn update(
        &self,
        subscription_id: u32,
        database_id: u32,
        request: UpdateDatabaseRequest,
    ) -> Result<CloudDatabase> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}",
                    subscription_id, database_id
                ),
                &request,
            )
            .await
    }

    /// Delete database
    pub async fn delete(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await?;
        Ok(serde_json::json!({"message": format!("Database {} deleted", database_id)}))
    }

    /// Resize database
    pub async fn resize(
        &self,
        subscription_id: u32,
        database_id: u32,
        memory_limit_in_gb: f64,
    ) -> Result<CloudDatabase> {
        let request = UpdateDatabaseRequest {
            memory_limit_in_gb: Some(memory_limit_in_gb),
            name: None,
            data_persistence: None,
            replication: None,
            data_eviction: None,
            password: None,
        };
        self.update(subscription_id, database_id, request).await
    }

    /// List all databases across all subscriptions
    pub async fn list_all(&self) -> Result<Vec<CloudDatabase>> {
        let response: Value = self.client.get("/databases").await?;
        if let Some(dbs) = response.get("databases") {
            serde_json::from_value(dbs.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// List databases for subscription as Value
    pub async fn list(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/subscriptions/{}/databases", subscription_id))
            .await
    }
    
    /// Get database as Value
    pub async fn get_raw(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await
    }
    
    /// Create database with Value
    pub async fn create_raw(&self, subscription_id: u32, request: Value) -> Result<Value> {
        self.client
            .post(
                &format!("/subscriptions/{}/databases", subscription_id),
                &request,
            )
            .await
    }
    
    /// Update database with Value
    pub async fn update_raw(&self, subscription_id: u32, database_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}",
                    subscription_id, database_id
                ),
                &request,
            )
            .await
    }
    
    /// Backup database
    pub async fn backup(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/backup",
                    subscription_id, database_id
                ),
                &Value::Null,
            )
            .await
    }
    
    /// Import data
    pub async fn import(&self, subscription_id: u32, database_id: u32, request: Value) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/import",
                    subscription_id, database_id
                ),
                &request,
            )
            .await
    }
    
    /// Get metrics
    pub async fn get_metrics(&self, subscription_id: u32, database_id: u32, metrics: &str, period: &str) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/metrics?metrics={}&period={}",
                subscription_id, database_id, metrics, period
            ))
            .await
    }
    
    /// Get database backup status
    pub async fn backup_status(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/backup",
                subscription_id, database_id
            ))
            .await
    }

}
