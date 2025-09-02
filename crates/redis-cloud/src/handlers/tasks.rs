//! Task operations handler

use crate::{Result, client::CloudClient, models::{Task, TaskList}};

/// Handler for Cloud task operations
pub struct CloudTaskHandler {
    client: CloudClient,
}

impl CloudTaskHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudTaskHandler { client }
    }

    /// List all tasks (typed). Accepts either {"tasks": [...]} or a bare array.
    pub async fn list(&self) -> Result<Vec<Task>> {
        // Try as wrapper
        if let Ok(wrapper) = self.client.get::<TaskList>("/tasks").await {
            return Ok(wrapper.tasks);
        }
        // Fallback: bare array
        let v: serde_json::Value = self.client.get("/tasks").await?;
        if v.as_array().is_some() {
            let tasks: Vec<Task> = serde_json::from_value(v)?;
            Ok(tasks)
        } else {
            Ok(vec![])
        }
    }

    /// Get task by ID
    pub async fn get(&self, task_id: &str) -> Result<Task> {
        self.client.get(&format!("/tasks/{}", task_id)).await
    }
}
