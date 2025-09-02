//! Active-Active (CRDB) database operations handler

use crate::{
    Result,
    client::CloudClient,
    models::{
        CloudCrdb, CloudCrdbRegion, CrdbMetrics, CrdbTask, CreateCrdbRequest, UpdateCrdbRequest,
    },
};

/// Handler for Cloud Active-Active database operations
pub struct CloudCrdbHandler {
    client: CloudClient,
}

impl CloudCrdbHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudCrdbHandler { client }
    }

    /// List all Active-Active databases (typed)
    pub async fn list(&self) -> Result<Vec<CloudCrdb>> {
        let v: serde_json::Value = self.client.get("/crdb").await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("crdbs") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get Active-Active database by ID
    pub async fn get(&self, crdb_id: u32) -> Result<CloudCrdb> {
        self.client.get(&format!("/crdb/{}", crdb_id)).await
    }

    /// Create Active-Active database
    pub async fn create(&self, request: serde_json::Value) -> Result<CloudCrdb> {
        self.client.post("/crdb", &request).await
    }

    /// Update Active-Active database
    pub async fn update(&self, crdb_id: u32, request: serde_json::Value) -> Result<CloudCrdb> {
        self.client
            .put(&format!("/crdb/{}", crdb_id), &request)
            .await
    }

    /// Delete Active-Active database
    pub async fn delete(&self, crdb_id: u32) -> Result<()> {
        self.client.delete(&format!("/crdb/{}", crdb_id)).await
    }

    /// Get Active-Active database regions
    pub async fn get_regions(&self, crdb_id: u32) -> Result<Vec<CloudCrdbRegion>> {
        let v: serde_json::Value = self.client.get(&format!("/crdb/{}/regions", crdb_id)).await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("regions") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Add region to Active-Active database
    pub async fn add_region(&self, crdb_id: u32, region: serde_json::Value) -> Result<CloudCrdb> {
        self.client
            .post(&format!("/crdb/{}/regions", crdb_id), &region)
            .await
    }

    /// Remove region from Active-Active database
    pub async fn remove_region(&self, crdb_id: u32, region_id: u32) -> Result<()> {
        self.client
            .delete(&format!("/crdb/{}/regions/{}", crdb_id, region_id))
            .await
    }

    /// Get Active-Active database tasks/jobs
    pub async fn get_tasks(&self, crdb_id: u32) -> Result<Vec<CrdbTask>> {
        let v: serde_json::Value = self.client.get(&format!("/crdb/{}/tasks", crdb_id)).await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("tasks") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get specific Active-Active task
    pub async fn get_task(&self, crdb_id: u32, task_id: &str) -> Result<CrdbTask> {
        self.client
            .get(&format!("/crdb/{}/tasks/{}", crdb_id, task_id))
            .await
    }

    /// Get Active-Active database metrics
    pub async fn get_metrics(&self, crdb_id: u32, metrics: &str, period: &str) -> Result<CrdbMetrics> {
        self.client
            .get(&format!(
                "/crdb/{}/metrics?metrics={}&period={}",
                crdb_id, metrics, period
            ))
            .await
    }

    /// Get Active-Active database backup
    pub async fn backup(&self, crdb_id: u32) -> Result<serde_json::Value> {
        // Some CRDB backup APIs return task or status; keep as raw JSON result but via typed handler signature
        self.client
            .post(&format!("/crdb/{}/backup", crdb_id), &serde_json::Value::Null)
            .await
    }

    /// Import data to Active-Active database
    pub async fn import(&self, crdb_id: u32, request: serde_json::Value) -> Result<serde_json::Value> {
        self.client
            .post(&format!("/crdb/{}/import", crdb_id), &request)
            .await
    }
}
