//! Active-Active (CRDB) database operations handler

use crate::{client::CloudClient, Result};
use serde_json::Value;

/// Handler for Cloud Active-Active database operations
pub struct CloudCrdbHandler {
    client: CloudClient,
}

impl CloudCrdbHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudCrdbHandler { client }
    }

    /// List all Active-Active databases
    pub async fn list(&self) -> Result<Value> {
        self.client.get("/crdb").await
    }

    /// Get Active-Active database by ID
    pub async fn get(&self, crdb_id: u32) -> Result<Value> {
        self.client.get(&format!("/crdb/{}", crdb_id)).await
    }

    /// Create Active-Active database
    pub async fn create(&self, request: Value) -> Result<Value> {
        self.client.post("/crdb", &request).await
    }

    /// Update Active-Active database
    pub async fn update(&self, crdb_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(&format!("/crdb/{}", crdb_id), &request)
            .await
    }

    /// Delete Active-Active database
    pub async fn delete(&self, crdb_id: u32) -> Result<Value> {
        self.client.delete(&format!("/crdb/{}", crdb_id)).await?;
        Ok(serde_json::json!({"message": format!("Active-Active database {} deleted", crdb_id)}))
    }

    /// Get Active-Active database regions
    pub async fn get_regions(&self, crdb_id: u32) -> Result<Value> {
        self.client.get(&format!("/crdb/{}/regions", crdb_id)).await
    }

    /// Add region to Active-Active database
    pub async fn add_region(&self, crdb_id: u32, request: Value) -> Result<Value> {
        self.client
            .post(&format!("/crdb/{}/regions", crdb_id), &request)
            .await
    }

    /// Remove region from Active-Active database
    pub async fn remove_region(&self, crdb_id: u32, region_id: u32) -> Result<Value> {
        self.client
            .delete(&format!("/crdb/{}/regions/{}", crdb_id, region_id))
            .await?;
        Ok(serde_json::json!({"message": format!("Region {} removed from Active-Active database {}", region_id, crdb_id)}))
    }

    /// Get Active-Active database tasks/jobs
    pub async fn get_tasks(&self, crdb_id: u32) -> Result<Value> {
        self.client.get(&format!("/crdb/{}/tasks", crdb_id)).await
    }

    /// Get specific Active-Active task
    pub async fn get_task(&self, crdb_id: u32, task_id: &str) -> Result<Value> {
        self.client
            .get(&format!("/crdb/{}/tasks/{}", crdb_id, task_id))
            .await
    }

    /// Get Active-Active database metrics
    pub async fn get_metrics(&self, crdb_id: u32, metrics: &str, period: &str) -> Result<Value> {
        self.client
            .get(&format!(
                "/crdb/{}/metrics?metrics={}&period={}",
                crdb_id, metrics, period
            ))
            .await
    }

    /// Get Active-Active database backup
    pub async fn backup(&self, crdb_id: u32) -> Result<Value> {
        self.client
            .post(&format!("/crdb/{}/backup", crdb_id), &Value::Null)
            .await
    }

    /// Import data to Active-Active database
    pub async fn import(&self, crdb_id: u32, request: Value) -> Result<Value> {
        self.client
            .post(&format!("/crdb/{}/import", crdb_id), &request)
            .await
    }
}