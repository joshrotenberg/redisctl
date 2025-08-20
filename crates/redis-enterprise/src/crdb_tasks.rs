//! Active-Active (CRDB) task management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// CRDB task information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdbTask {
    pub task_id: String,
    pub crdb_guid: String,
    pub task_type: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// CRDB task creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCrdbTaskRequest {
    pub crdb_guid: String,
    pub task_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

/// CRDB tasks handler
pub struct CrdbTasksHandler {
    client: RestClient,
}

impl CrdbTasksHandler {
    pub fn new(client: RestClient) -> Self {
        CrdbTasksHandler { client }
    }

    /// List all CRDB tasks
    pub async fn list(&self) -> Result<Vec<CrdbTask>> {
        self.client.get("/v1/crdb_tasks").await
    }

    /// Get specific CRDB task
    pub async fn get(&self, task_id: &str) -> Result<CrdbTask> {
        self.client
            .get(&format!("/v1/crdb_tasks/{}", task_id))
            .await
    }

    /// Create a new CRDB task
    pub async fn create(&self, request: CreateCrdbTaskRequest) -> Result<CrdbTask> {
        self.client.post("/v1/crdb_tasks", &request).await
    }

    /// Cancel a CRDB task
    pub async fn cancel(&self, task_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/v1/crdb_tasks/{}", task_id))
            .await
    }

    /// Get tasks for a specific CRDB
    pub async fn list_by_crdb(&self, crdb_guid: &str) -> Result<Vec<CrdbTask>> {
        self.client
            .get(&format!("/v1/crdbs/{}/tasks", crdb_guid))
            .await
    }
}

