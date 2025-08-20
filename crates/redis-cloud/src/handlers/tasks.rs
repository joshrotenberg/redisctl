//! Task operations handler

use crate::{client::CloudClient, Result};
use serde_json::Value;

/// Handler for Cloud task operations
pub struct CloudTasksHandler {
    client: CloudClient,
}

impl CloudTasksHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudTasksHandler { client }
    }

    /// List all tasks
    pub async fn list(&self) -> Result<Value> {
        self.client.get("/tasks").await
    }

    /// Get task by ID
    pub async fn get(&self, task_id: &str) -> Result<Value> {
        self.client.get(&format!("/tasks/{}", task_id)).await
    }
}
