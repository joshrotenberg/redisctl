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

    /// List all tasks (typed wrapper). Accepts either {"tasks": [...]} or a bare array.
    pub async fn list(&self) -> Result<TaskList> {
        if let Ok(wrapper) = self.client.get::<TaskList>("/tasks").await {
            return Ok(wrapper);
        }
        let v: serde_json::Value = self.client.get("/tasks").await?;
        if v.is_array() {
            let tasks: Vec<Task> = serde_json::from_value(v.clone())?;
            Ok(TaskList { tasks, extra: serde_json::json!({}) })
        } else {
            // Coerce unknown shapes to wrapper for forward-compat
            Ok(TaskList { tasks: vec![], extra: v })
        }
    }

    /// Get task by ID
    pub async fn get(&self, task_id: &str) -> Result<Task> {
        self.client.get(&format!("/tasks/{}", task_id)).await
    }
}
